//! Shared models used across namespaces (display name, pagination params, etc.)

use serde::{Deserialize, Serialize};

/// Display name for a resource.
pub type DisplayName = String;

/// Common page size parameter.
pub type PageSize = u32;

/// Preview mode flag for beta features.
pub type PreviewMode = bool;

/// Resource type enum (used across filesystem/core).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceType {
    Dataset,
    Folder,
    Project,
    #[serde(other)]
    Unknown,
}
