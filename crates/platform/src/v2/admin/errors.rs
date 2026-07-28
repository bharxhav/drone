//! Typed views over generic Foundry API errors.

use crate::error::ApiError;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AdminApiError {
    pub name: String,
    pub parameters: Value,
    pub error_instance_id: Option<String>,
}

impl AdminApiError {
    pub fn from_api(error: &ApiError) -> Option<Self> {
        Some(Self {
            name: error.error_name.clone()?,
            parameters: error.parameters.clone(),
            error_instance_id: error.error_instance_id.clone(),
        })
    }
}
