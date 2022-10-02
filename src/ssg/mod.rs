mod disk_caching_http_client;

use anyhow::{anyhow, Result};
use futures::{Future, FutureExt};
use pathdiff::diff_paths;
use readext::ReadExt;
use reqwest::Url;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    path,
};
use tokio::{fs, io::AsyncWriteExt};

pub fn generate_static_site(
    output_dir: path::PathBuf,
    mappings: BTreeMap<path::PathBuf, impl Future<Output = Source>>,
) -> impl Iterator<Item = (path::PathBuf, impl Future<Output = Result<()>>)> {
    let paths = mappings
        .iter()
        .map(|(path, _source)| path.to_owned())
        .collect::<BTreeSet<_>>();
    mappings.into_iter().map(move |(path, source)| {
        let this_path = path.to_owned();
        let paths = paths.clone();
        let output_dir = output_dir.clone();
        let result = source.then(|source| async {
            let contents = match source {
                Source::Bytes(bytes) => bytes.clone(),
                Source::BytesWithAssetSafety(function) => {
                    let assets = Assets {
                        this_path: this_path.clone(),
                        paths,
                    };
                    function(assets)?
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
            let mut file_handle = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(
                    [output_dir, this_path]
                        .into_iter()
                        .collect::<path::PathBuf>(),
                )
                .await?;
            file_handle.write_all(&contents).await?;
            Ok(())
        });
        (path, result)
    })
}

pub enum Source {
    Bytes(Vec<u8>),
    BytesWithAssetSafety(Box<dyn FnOnce(Assets) -> Result<Vec<u8>> + Send>),
    GoogleFont(GoogleFont),
    Http(Url),
}

pub struct Assets {
    this_path: path::PathBuf,
    paths: BTreeSet<path::PathBuf>,
}

impl Assets {
    pub fn relative(&self, path: path::PathBuf) -> Result<path::PathBuf> {
        diff_paths(
            self.paths
                .get(&path)
                .ok_or_else(|| anyhow!("No such path"))?,
            self.this_path.clone(),
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
                "https://google-webfonts-helper.herokuapp.com/api/fonts/{}",
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
