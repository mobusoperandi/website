mod disk_caching_http_client;

use std::{
    collections::BTreeSet,
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, Future, FutureExt};
use readext::ReadExt;
use reqwest::Url;
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

    let iterator = assets.into_iter().map(move |Asset { source, target }| {
        let this_target = target.to_owned();
        let targets = paths.clone();
        let output_dir = output_dir.clone();

        let result = source
            .then(|source| generate_file_from_asset(source, targets, this_target, output_dir));

        (target, result)
    });

    Ok(iterator)
}

async fn generate_file_from_asset(
    source: Source,
    targets: BTreeSet<PathBuf>,
    this_target: PathBuf,
    output_dir: PathBuf,
) -> Result<()> {
    let contents = match source {
        Source::Bytes(bytes) => bytes.clone(),
        Source::BytesWithAssetSafety(function) => {
            let targets = Targets {
                all: targets,
                current: this_target.clone(),
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

    let this_target_relative = this_target.iter().skip(1).collect();
    let file_path = [output_dir, this_target_relative]
        .into_iter()
        .collect::<PathBuf>();

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
}

pub struct Asset {
    pub(crate) source: BoxFuture<'static, Source>,
    pub(crate) target: PathBuf,
}

impl Asset {
    pub fn new<T>(target: T, source: impl Future<Output = Source> + Send + 'static) -> Self
    where
        PathBuf: From<T>,
    {
        let target: PathBuf = target.into();

        assert!(target.is_absolute(), "path not absolute: {target:?}");

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
    current: PathBuf,
    all: BTreeSet<PathBuf>,
}

impl Targets {
    pub fn path_of(&self, path: impl AsRef<Path>) -> Result<String> {
        let path = path.as_ref();

        assert!(path.is_absolute(), "path not absolute: {path:?}");

        self.all
            .contains(path)
            .then(|| {
                PathBuf::from_iter([PathBuf::from("/"), path.to_owned()])
                    .to_str()
                    .map(|path| path.to_owned())
            })
            .flatten()
            .map(|path| {
                if path == "/index.html" {
                    String::from("/")
                } else {
                    path
                }
            })
            .ok_or_else(|| anyhow!("no target with path: {path:?}"))
    }
    pub fn current_path(&self) -> String {
        self.path_of(&self.current).unwrap()
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
