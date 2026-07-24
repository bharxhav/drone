use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum FoundryError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Client(#[from] ClientError),
}

#[derive(Debug, Diagnostic, Error)]
pub enum ClientError {
    #[error("invalid hostname")]
    #[diagnostic(code(platform::client::hostname))]
    Url(#[from] url::ParseError),

    #[error("invalid Foundry API token")]
    #[diagnostic(code(platform::client::token))]
    Header(#[from] reqwest::header::InvalidHeaderValue),

    #[error("HTTP client error")]
    #[diagnostic(code(platform::client::http))]
    Http(#[from] reqwest::Error),
}

impl From<url::ParseError> for FoundryError {
    fn from(error: url::ParseError) -> Self {
        Self::Client(error.into())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for FoundryError {
    fn from(error: reqwest::header::InvalidHeaderValue) -> Self {
        Self::Client(error.into())
    }
}

impl From<reqwest::Error> for FoundryError {
    fn from(error: reqwest::Error) -> Self {
        Self::Client(error.into())
    }
}
