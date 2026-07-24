mod domain;
mod error;
mod response;

pub use domain::Domain;
pub use error::Error;
pub use response::{Documentation, Link, NavItem as Item, Page};

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
        let scope: Vec<&str> = scope.iter().map(AsRef::as_ref).collect();

        match domain {
            Domain::Product => domain::product::get(self, domain.route().as_ref(), &scope).await,
            Domain::Platform => domain::platform::get(self, domain.route().as_ref(), &scope).await,
            Domain::Updates => domain::updates::get(self, domain.route().as_ref(), &scope).await,
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
