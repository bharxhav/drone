//! Third Party Applications namespace API errors.

use crate::error::ApiError;
use serde_json::Value;
#[derive(Debug, Clone)]
pub struct ThirdPartyApplicationsApiError {
    pub name: String,
    pub parameters: Value,
    pub error_instance_id: Option<String>,
}
impl ThirdPartyApplicationsApiError {
    pub fn from_api(error: &ApiError) -> Option<Self> {
        Some(Self {
            name: error.error_name.clone()?,
            parameters: error.parameters.clone(),
            error_instance_id: error.error_instance_id.clone(),
        })
    }
}
pub const ERROR_NAMES: &[&str] = &[
    "CannotDeleteDeployedVersion",
    "DeleteVersionPermissionDenied",
    "DeployWebsitePermissionDenied",
    "FileCountLimitExceeded",
    "FileSizeLimitExceeded",
    "InvalidVersion",
    "ScanningErrored",
    "ScanningInProgress",
    "SiteAssetHasVulnerabilities",
    "ThirdPartyApplicationNotFound",
    "UndeployWebsitePermissionDenied",
    "UploadSnapshotVersionPermissionDenied",
    "UploadVersionPermissionDenied",
    "VersionAlreadyExists",
    "VersionLimitExceeded",
    "VersionNotFound",
    "WebsiteNotFound",
];
