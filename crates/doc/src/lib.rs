mod domain;
mod error;
mod response;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub use error::Error;
pub use response::*;

use domain::Domain;

pub struct Client {
    http: reqwest::Client,
    cache: RwLock<HashMap<Vec<String>, Arc<Page>>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get<S>(&self, scope: &[S]) -> Result<Documentation, Error>
    where
        S: AsRef<str>,
    {
        // 1. Normalize the requested path into its cache key.
        let cache_key: Vec<String> = scope
            .iter()
            .map(|segment| segment.as_ref().trim_matches('/').to_owned())
            .collect();

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
