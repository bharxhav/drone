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
    cache: RwLock<HashMap<Vec<String>, Arc<Documentation>>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get<S>(&self, scope: &[S]) -> Result<Arc<Documentation>, Error>
    where
        S: AsRef<str>,
    {
        // 1. Normalize the requested path into its cache key.
        let cache_key: Vec<String> = scope
            .iter()
            .map(|segment| segment.as_ref().trim_matches('/').to_owned())
            .collect();

        // 2. Early Return on Cache Hit.
        if let Some(documentation) = self.cache.read().await.get(&cache_key) {
            return Ok(Arc::clone(documentation));
        }

        // 3. Build the root index or fetch the requested domain document.
        let documentation = if cache_key.is_empty() {
            Domain::ALL.into()
        } else {
            let domain = Domain::from_scope(&cache_key[0])
                .ok_or_else(|| Error::NotFound(cache_key[0].clone()))?;

            domain.get(&self.http, &cache_key[1..]).await?
        };

        let documentation = Arc::new(documentation);

        // 4. Cache the result, preserving a value inserted by a concurrent request.

        // 5. Return a shared handle to the cached document.
        Ok(Arc::clone(documentation))
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
