mod error;

use reqwest::header::{HeaderMap, HeaderValue};
use url::Url;

pub use error::FoundryError;

pub struct Client {
    pub hostname: Url,
    http: reqwest::Client,
}

impl Client {
    pub fn new(hostname: &str, token: &str) -> Result<Self, FoundryError> {
        let hostname = Url::parse(hostname)?;

        let mut headers = HeaderMap::new();
        let mut authorization = HeaderValue::from_str(&format!("Bearer {token}"))?;
        authorization.set_sensitive(true);
        headers.insert(reqwest::header::AUTHORIZATION, authorization);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { hostname, http })
    }
}
