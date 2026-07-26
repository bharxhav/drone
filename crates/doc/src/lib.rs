mod documentation;
mod domain;
mod error;
mod scope;
mod wire;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub use documentation::*;
pub use error::Error;
pub use scope::Scope;

pub struct Client {
    http: reqwest::Client,
    cache: RwLock<HashMap<Scope, Arc<Page>>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(&self, scope: Scope) -> Result<Documentation, Error> {
        // 1. Use the normalized scope as the cache key.
        let cache_key = scope;

        // 2. Early Return on Cache Hit.
        if let Some(page) = self.cache.read().await.get(&cache_key) {
            return Ok(Documentation::Page(Arc::clone(page)));
        }

        // 3. Build the root index or fetch the requested domain document.
        let documentation = if cache_key.is_root() {
            Documentation::Index(Vec::new())
        } else {
            let domain = cache_key.domain().ok_or_else(|| {
                Error::NotFound(cache_key.domain_name().unwrap_or_default().into())
            })?;

            domain.get(&self.http, cache_key.tail()).await?
        };

        // 4. Cache only terminal pages. Navigation is context-specific and may expose an
        // incomplete route tree, so caching it could incorrectly reject valid routes.
        if let Documentation::Page(page) = &documentation {
            let mut cache = self.cache.write().await;
            cache.entry(cache_key).or_insert_with(|| Arc::clone(page));
        }

        // 5. Return the fetched document.
        Ok(documentation)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
