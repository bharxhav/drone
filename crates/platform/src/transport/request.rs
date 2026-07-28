use bytes::Bytes;
use reqwest::header::HeaderValue;

/// Body accepted by an authenticated transport request.
#[derive(Debug, Clone)]
pub enum RequestBody {
    Bytes(Bytes),
    Json(serde_json::Value),
}

impl From<Bytes> for RequestBody {
    fn from(value: Bytes) -> Self {
        Self::Bytes(value)
    }
}

impl From<Vec<u8>> for RequestBody {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value.into())
    }
}

impl From<serde_json::Value> for RequestBody {
    fn from(value: serde_json::Value) -> Self {
        Self::Json(value)
    }
}

impl RequestBody {
    pub(crate) fn apply(self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match self {
            Self::Bytes(body) => request.body(body),
            Self::Json(body) => request.json(&body),
        }
    }
}

pub(crate) fn content_type(value: &str) -> crate::error::Result<HeaderValue> {
    HeaderValue::from_str(value).map_err(|error| crate::error::Error::Config(error.to_string()))
}
