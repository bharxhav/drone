mod domain;
mod error;
mod response;

use std::collections::HashMap;

pub use error::Error;
pub use response::*;

use domain::Domain;

pub struct Client {
    http: reqwest::Client,
    cache: HashMap<Vec<String>, Documentation>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
            cache: HashMap::new(),
        }
    }

    pub async fn get<S>(&self, scope: &[S]) -> Result<Documentation, Error>
    where
        S: AsRef<str>,
    {
        let Some((domain, scope)) = scope.split_first() else {
            return Ok(Domain::ALL.into());
        };

        let domain = Domain::from_scope(domain.as_ref())
            .ok_or_else(|| Error::NotFound(domain.as_ref().to_owned()))?;

        domain.get(self, scope).await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
