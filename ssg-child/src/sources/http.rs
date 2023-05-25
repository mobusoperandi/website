use futures::{future::BoxFuture, FutureExt, TryFutureExt};
use reqwest::Url;

use crate::disk_caching_http_client::HTTP_CLIENT;

use super::{bytes_with_file_spec_safety::Targets, FileSource};

#[derive(Debug, Clone)]
pub struct Http(Url);

impl From<Url> for Http {
    fn from(url: Url) -> Self {
        Self(url)
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
struct HttpError(#[from] reqwest_middleware::Error);

impl FileSource for Http {
    fn obtain_content(
        &self,
        _targets: Targets,
    ) -> BoxFuture<'static, Result<Vec<u8>, Box<dyn std::error::Error + Send>>> {
        let url = self.0.clone();

        async {
            Ok(HTTP_CLIENT
                .get(url)
                .send()
                .await?
                .error_for_status()
                .map_err(reqwest_middleware::Error::Reqwest)?
                .bytes()
                .await
                .map_err(reqwest_middleware::Error::Reqwest)?
                .to_vec())
        }
        .map_err(|error: HttpError| -> Box<dyn std::error::Error + Send> { Box::new(error) })
        .boxed()
    }
}
