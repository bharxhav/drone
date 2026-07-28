//! Audit namespace wire types.

use serde::{Deserialize, Serialize};
pub type FileId = String;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFile {
    pub id: FileId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLogFilesResponse {
    pub data: Vec<LogFile>,
    pub next_page_token: Option<String>,
}
