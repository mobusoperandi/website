mod disk_caching_http_client;

use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, Future, FutureExt};
use pathdiff::diff_paths;
use readext::ReadExt;
use reqwest::Url;
use std::{
    collections::BTreeSet,
    fmt::Display,
    path::{Path, PathBuf},
};
use tokio::{fs, io::AsyncWriteExt};

pub fn generate_static_site(
    output_dir: PathBuf,
    assets: impl IntoIterator<Item = Asset>,
) -> Result<impl Iterator<Item = (PathBuf, impl Future<Output = Result<()>>)>> {
    let (paths, assets) = assets.into_iter().try_fold(
        (BTreeSet::<PathBuf>::new(), BTreeSet::<Asset>::new()),
        |(mut paths, mut assets), asset| {
            let newly_inserted = paths.insert(asset.target.clone());
            if !newly_inserted {
                return Err(anyhow!("Duplicate target: {}", asset.target.display()));
            }
            assets.insert(asset);
            Ok((paths, assets))
        },
    )?;
    Ok(assets.into_iter().map(move |Asset { source, target }| {
        let this_target = target.to_owned();
        let targets = paths.clone();
        let output_dir = output_dir.clone();
        let result = source.then(|source| async {
            let contents = match source {
                Source::Bytes(bytes) => bytes.clone(),
                Source::BytesWithAssetSafety(function) => {
                    let targets = Targets {
                        this_target: this_target.clone(),
                        all_targets: targets,
                    };
                    function(targets)?
                }
                Source::GoogleFont(google_font) => google_font.download().await?,
                Source::Http(url) => {
                    let client = disk_caching_http_client::create();
                    client
                        .get(url.clone())
                        .send()
                        .await?
                        .bytes()
                        .await?
                        .to_vec()
                }
            };
            let file_path = [output_dir, this_target].into_iter().collect::<PathBuf>();
            fs::create_dir_all(file_path.parent().unwrap())
                .await
                .unwrap();
            let mut file_handle = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path)
                .await?;
            file_handle.write_all(&contents).await?;
            Ok(())
        });
        (target, result)
    }))
}

pub struct Asset {
    pub(crate) source: BoxFuture<'static, Source>,
    pub(crate) target: PathBuf,
}

impl Asset {
    pub fn new(target: PathBuf, source: impl Future<Output = Source> + Send + 'static) -> Self {
        Self {
            source: source.boxed(),
            target,
        }
    }
}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
    }
}

impl Eq for Asset {}

impl PartialOrd for Asset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Asset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.target.cmp(&other.target)
    }
}

pub enum Source {
    Bytes(Vec<u8>),
    BytesWithAssetSafety(Box<dyn FnOnce(Targets) -> Result<Vec<u8>> + Send>),
    GoogleFont(GoogleFont),
    Http(Url),
}

#[derive(Debug, Clone)]
pub struct Targets {
    this_target: PathBuf,
    all_targets: BTreeSet<PathBuf>,
}

impl Targets {
    pub fn relative(&self, path: impl AsRef<Path>) -> Result<PathBuf> {
        diff_paths(
            self.all_targets
                .get(&path.as_ref().to_path_buf())
                .ok_or_else(|| anyhow!("No such path {:?}", path.as_ref()))?,
            self.this_target.clone(),
        )
        .ok_or_else(|| anyhow!("Failed to obtain relative path"))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GoogleFont {
    pub name: &'static str,
    pub version: u8,
    pub subset: &'static str,
    pub variant: &'static str,
}

impl GoogleFont {
    pub(crate) async fn download(&self) -> Result<Vec<u8>> {
        // TODO: Consider reusing the client ->
        let url = Url::parse_with_params(
            &format!(
                "https://gwfh.mranftl.com/api/fonts/{}",
                self.name.to_lowercase(),
            ),
            [
                ("download", "zip"),
                ("subsets", self.subset),
                ("variants", self.variant),
            ],
        )?;
        let client = disk_caching_http_client::create();
        let archive = client.get(url.clone()).send().await?.bytes().await?;
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(archive))?;
        let mut font_file = archive.by_name(&format!(
            "{}-v{}-{}-{}.woff2",
            self.name.to_lowercase(),
            self.version,
            self.subset,
            self.variant
        ))?;
        let font_contents = font_file.read_into_vec()?;
        Ok(font_contents)
    }
}

impl Display for GoogleFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
