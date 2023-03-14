use reqwest::Url;

use crate::disk_caching_http_client;

pub(crate) async fn download(url: Url) -> reqwest_middleware::Result<Vec<u8>> {
    Ok(disk_caching_http_client::create()
        .get(url.clone())
        .send()
        .await?
        .bytes()
        .await?
        .to_vec())
}
