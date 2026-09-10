use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use url::Url;

use crate::Error;

pub struct Client {
    pub(super) url: Url,
    pub(super) http: reqwest::Client,
}

impl Client {
    pub fn new(url: &str, token: &str) -> Result<Self, Error> {
        let url = Url::parse(url)?;

        let mut headers = HeaderMap::new();
        let mut authorization = HeaderValue::from_str(&format!("Bearer {token}"))?;
        authorization.set_sensitive(true);
        headers.insert(AUTHORIZATION, authorization);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { url, http })
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn creates_client() {
        assert!(Client::new("https://example.palantirfoundry.com", "token").is_ok());
    }

    #[test]
    fn rejects_invalid_url() {
        assert!(Client::new("://", "token").is_err());
    }

    #[test]
    fn rejects_invalid_token() {
        assert!(Client::new("https://example.palantirfoundry.com", "invalid\ntoken").is_err());
    }
}
