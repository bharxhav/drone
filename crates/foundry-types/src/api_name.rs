use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ApiName(String);

impl ApiName {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Display for ApiName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<String> for ApiName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<ApiName> for String {
    fn from(api_name: ApiName) -> Self {
        api_name.0
    }
}
