mod disk_caching_http_client;

use anyhow::Result;
use futures::{Future, FutureExt};
use readext::ReadExt;
use reqwest::Url;
use std::{fmt::Display, path};
use tokio::{fs, io::AsyncWriteExt};

pub fn generate_static_site(
    mappings: impl IntoIterator<Item = (path::PathBuf, impl Future<Output = Source>)>,
) -> impl Iterator<Item = (path::PathBuf, impl Future<Output = Result<()>>)> {
    mappings.into_iter().map(|(path, source)| {
        let path_clone = path.clone();
        let result = source.then(|source| async {
            let contents = match source {
                Source::Bytes(bytes) => bytes.clone(),
                Source::GoogleFont(google_font) => google_font.download().await?,
            };
            let mut file_handle = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path_clone)
                .await?;
            file_handle.write_all(&contents).await?;
            Ok(())
        });
        (path, result)
    })
}

#[derive(Clone, Debug)]
pub enum Source {
    Bytes(Vec<u8>),
    GoogleFont(GoogleFont),
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
