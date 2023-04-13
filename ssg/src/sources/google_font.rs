use std::fmt::Display;

use futures::{future::BoxFuture, FutureExt, TryFutureExt};
use getset::Getters;
use readext::ReadExt;
use reqwest::Url;

use crate::disk_caching_http_client;

use super::{bytes_with_file_spec_safety::Targets, FileSource};

#[derive(Debug, Clone, Getters)]
pub struct GoogleFont {
    #[getset(get = "pub")]
    family: String,
    version: u8,
    subset: String,
    variant: String,
}

impl GoogleFont {
    pub fn new(family: String, version: u8, subset: String, variant: String) -> Self {
        Self {
            family,
            version,
            subset,
            variant,
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum DownloadError {
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    #[error(transparent)]
    Http(#[from] reqwest_middleware::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl FileSource for GoogleFont {
    fn obtain_content(
        &self,
        _targets: Targets,
    ) -> BoxFuture<'static, Result<Vec<u8>, Box<dyn std::error::Error + Send>>> {
        let Self {
            family,
            version,
            subset,
            variant,
        } = self.clone();

        async move {
            // TODO: Consider reusing the client ->
            let url = Url::parse_with_params(
                &format!(
                    "https://gwfh.mranftl.com/api/fonts/{}",
                    family.to_lowercase(),
                ),
                [
                    ("download", "zip"),
                    ("subsets", &subset),
                    ("variants", &variant),
                ],
            )?;

            let client = disk_caching_http_client::create();

            let archive = client
                .get(url.clone())
                .send()
                .await?
                .error_for_status()
                .map_err(reqwest_middleware::Error::Reqwest)?
                .bytes()
                .await
                .map_err(reqwest_middleware::Error::Reqwest)?;

            let mut archive = zip::ZipArchive::new(std::io::Cursor::new(archive))?;

            let mut font_file = archive.by_name(&format!(
                "{}-v{}-{}-{}.woff2",
                family.to_lowercase(),
                version,
                subset,
                variant
            ))?;

            Ok(font_file.read_into_vec()?)
        }
        .map_err(
            move |error: DownloadError| -> Box<dyn std::error::Error + Send> { Box::new(error) },
        )
        .boxed()
    }
}

impl Display for GoogleFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.family)
    }
}
