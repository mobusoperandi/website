use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use once_cell::sync::Lazy;
use reqwest::{Client, Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next, Result};
use task_local_extensions::Extensions;

struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let url = req.url().clone();

        println!("request {url}");

        let response = next.run(req, extensions).await;

        if let Ok(response) = &response {
            let header_x_cache = response.headers().get("x-cache").unwrap();

            println!("response {header_x_cache:?} {url}");
        }

        response
    }
}

pub(crate) static HTTP_CLIENT: Lazy<ClientWithMiddleware> = Lazy::new(|| {
    const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");
    const UUID: &str = "a6c07a0f-7599-468d-8627-88b85ede9fde";

    let cache_path = dirs::cache_dir()
        .unwrap()
        .join(format!("{CRATE_NAME}.{UUID}"));

    ClientBuilder::new(Client::new())
        .with(LoggingMiddleware)
        .with(Cache(HttpCache {
            // TODO don't leave it as ForceCache
            mode: CacheMode::ForceCache,
            manager: CACacheManager {
                path: cache_path.to_str().unwrap().to_owned(),
            },
            options: None,
        }))
        .build()
});
