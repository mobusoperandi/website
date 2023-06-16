use std::fmt::Display;

use futures::{future::BoxFuture, FutureExt, TryFutureExt};
use getset::Getters;
use lazy_regex::regex_captures;
use readext::ReadExt;
use reqwest::{header::CONTENT_DISPOSITION, Url};

use crate::disk_caching_http_client::HTTP_CLIENT;

use super::{FileContents, FileSource};

#[derive(Debug, Clone, Getters)]
pub struct GoogleFont {
    #[getset(get = "pub")]
    family: String,
    subset: String,
    variant: String,
}

impl GoogleFont {
    #[must_use]
    pub fn new(family: String, subset: String, variant: String) -> Self {
        Self {
            family,
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
    #[error("no Content-Disposition header present")]
    NoContentDisposition,
    #[error("Content-Disposition value not UTF-8")]
    ContentDispositionNotUtf8(#[from] reqwest::header::ToStrError),
    #[error("could not parse value of Content-Disposition")]
    BadContentDisposition(String),
}

impl FileSource for GoogleFont {
    fn obtain_content(
        &self,
    ) -> BoxFuture<'static, Result<FileContents, Box<dyn std::error::Error + Send>>> {
        let Self {
            family,
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

            let response = HTTP_CLIENT
                .get(url.clone())
                .send()
                .await?
                .error_for_status()
                .map_err(reqwest_middleware::Error::Reqwest)?;

            let content_disposition = response
                .headers()
                .get(CONTENT_DISPOSITION)
                .ok_or(DownloadError::NoContentDisposition)?
                .to_str()?;

            let (_, version) = regex_captures!(
                // `attachment; filename=vollkorn-v22-latin.zip`
                r"attachment; filename=.*-v(\d*)-.*",
                content_disposition
            )
            .ok_or_else(|| DownloadError::BadContentDisposition(content_disposition.to_owned()))?;

            let version = version.to_owned();

            let archive = response
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

            Ok(FileContents::new(font_file.read_into_vec()?, None))
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
