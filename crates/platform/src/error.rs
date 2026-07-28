use http::StatusCode;

/// Top-level error type for the platform SDK.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("configuration: {0}")]
    Config(String),

    #[error("transport: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("{0}")]
    Api(Box<ApiError>),

    #[error("decode: {0}")]
    Decode(#[from] serde_json::Error),
}

/// Structured error returned by the Foundry API.
#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: StatusCode,
    pub error_name: Option<String>,
    pub error_code: Option<String>,
    pub error_instance_id: Option<String>,
    pub error_description: Option<String>,
    pub parameters: serde_json::Value,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.status)?;
        if let Some(name) = &self.error_name {
            write!(f, "{name}")?;
        }
        if let Some(desc) = &self.error_description {
            write!(f, ": {desc}")?;
        }
        Ok(())
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Self::Api(Box::new(e))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
