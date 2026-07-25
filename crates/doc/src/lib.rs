mod domain;
mod error;
mod response;
mod scope;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub use error::Error;
pub use response::*;
pub use scope::Scope;

use domain::Domain;

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
        let documentation = if cache_key.is_empty() {
            Domain::ALL.into()
        } else {
            let domain = Domain::from_scope(&cache_key[0])
                .ok_or_else(|| Error::NotFound(cache_key[0].clone()))?;

            domain.get(&self.http, &cache_key[1..]).await?
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
