mod domain;
mod error;
mod response;

pub use domain::Domain;
pub use error::Error;
pub use response::*;

pub struct Client {
    http: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    pub async fn get<S>(&self, domain: Domain, scope: &[S]) -> Result<Documentation, Error>
    where
        S: AsRef<str>,
    {
        domain.get(self, scope).await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
