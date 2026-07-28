use serde::{Deserialize, Serialize};

/// A single page of results from a paginated endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub data: Vec<T>,
    pub next_page_token: Option<PageToken>,
}

/// Opaque page token for cursor-based pagination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PageToken(pub String);

impl PageToken {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for PageToken {
    fn from(s: String) -> Self {
        Self(s)
    }
}
