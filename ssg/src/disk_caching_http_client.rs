use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

pub(crate) fn create() -> ClientWithMiddleware {
    ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            // TODO don't leave it as ForceCache
            mode: CacheMode::ForceCache,
            // TODO where do I want the cache?
            manager: CACacheManager::default(),
            options: None,
        }))
        .build()
}
