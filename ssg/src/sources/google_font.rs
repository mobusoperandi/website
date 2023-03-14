use std::fmt::Display;

use readext::ReadExt;
use reqwest::Url;
use thiserror::Error;

use crate::disk_caching_http_client;

#[derive(Debug, Clone, Copy)]
pub struct GoogleFont {
    pub name: &'static str,
    pub version: u8,
    pub subset: &'static str,
    pub variant: &'static str,
}

#[derive(Debug, Error)]
pub(crate) enum DownloadError {
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    #[error(transparent)]
    RequestMiddleware(#[from] reqwest_middleware::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl GoogleFont {
    pub(crate) async fn download(&self) -> Result<Vec<u8>, DownloadError> {
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
