//! Data Health namespace API errors.

use crate::error::ApiError;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DataHealthApiError {
    pub name: String,
    pub parameters: Value,
    pub error_instance_id: Option<String>,
}
impl DataHealthApiError {
    pub fn from_api(error: &ApiError) -> Option<Self> {
        Some(Self {
            name: error.error_name.clone()?,
            parameters: error.parameters.clone(),
            error_instance_id: error.error_instance_id.clone(),
        })
    }
}

pub const ERROR_NAMES: &[&str] = &[
    "CheckAlreadyExists",
    "CheckNotFound",
    "CheckReportLimitAboveMaximum",
    "CheckReportLimitBelowMinimum",
    "CheckReportNotFound",
    "CheckTypeNotSupported",
    "CreateCheckPermissionDenied",
    "DeleteCheckPermissionDenied",
    "GetLatestCheckReportsPermissionDenied",
    "InvalidNumericColumnCheckConfig",
    "InvalidPercentageCheckConfig",
    "InvalidTimeCheckConfig",
    "InvalidTransactionTimeCheckConfig",
    "InvalidTrendConfig",
    "ModifyingCheckTypeNotSupported",
    "PercentageValueAboveMaximum",
    "PercentageValueBelowMinimum",
    "ReplaceCheckPermissionDenied",
];
