//! Typed parameter payloads for Checkpoints API errors.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordParameters {
    pub record_rid: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
pub type CheckpointRecordNotFoundParameters = RecordParameters;
pub type CheckpointRecordPermissionDeniedParameters = RecordParameters;
pub type RecordNotFoundParameters = RecordParameters;
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchRecordsPermissionDeniedParameters {
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
