use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid Foundry URL")]
    Url(#[from] url::ParseError),

    #[error("invalid Foundry API token")]
    Token(#[from] reqwest::header::InvalidHeaderValue),

    #[error("could not create Foundry API client")]
    Http(#[from] reqwest::Error),
}
