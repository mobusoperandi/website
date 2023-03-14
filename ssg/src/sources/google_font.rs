use std::fmt::Display;

use futures::{future::BoxFuture, FutureExt, TryFutureExt};
use readext::ReadExt;
use reqwest::Url;

use crate::disk_caching_http_client;

use super::{bytes_with_file_spec_safety::Targets, FileSource};

#[derive(Debug, Clone, Copy)]
pub struct GoogleFont {
    pub name: &'static str,
    pub version: u8,
    pub subset: &'static str,
    pub variant: &'static str,
}

#[derive(Debug, thiserror::Error)]
enum DownloadError {
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

impl FileSource for GoogleFont {
    fn obtain_content(
        &self,
        _targets: Targets,
    ) -> BoxFuture<'static, Result<Vec<u8>, Box<dyn std::error::Error + Send>>> {
        let &Self {
            name,
            version,
            subset,
            variant,
        } = self;

        async move {
            // TODO: Consider reusing the client ->
            let url = Url::parse_with_params(
                &format!("https://gwfh.mranftl.com/api/fonts/{}", name.to_lowercase(),),
                [
                    ("download", "zip"),
                    ("subsets", subset),
                    ("variants", variant),
                ],
            )?;

            let client = disk_caching_http_client::create();
            let archive = client.get(url.clone()).send().await?.bytes().await?;

            let mut archive = zip::ZipArchive::new(std::io::Cursor::new(archive))?;

            let mut font_file = archive.by_name(&format!(
                "{}-v{}-{}-{}.woff2",
                name.to_lowercase(),
                version,
                subset,
                variant
            ))?;

            let font_contents = font_file.read_into_vec()?;

            Ok(font_contents)
        }
        .map_err(
            move |error: DownloadError| -> Box<dyn std::error::Error + Send> { Box::new(error) },
        )
        .boxed()
    }
}

impl Display for GoogleFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
