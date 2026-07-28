//! Typed parameter payloads for SQL Queries API errors.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmptyParameters {
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyObjectTypeNotFoundParameters {
    pub object_type_rid: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessageParameters {
    pub error_message: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyQueryInvalidObjectBackendParameters {
    pub object_type_rids: Vec<String>,
    pub link_type_rids: Vec<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyQueryNestedObjectSetTooLargeParameters {
    pub nested_object_set_size: i64,
    pub max_allowed_nested_object_set_size: i64,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyQueryStringColumnTooLongParameters {
    pub column_name: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadQueryInputsPermissionDeniedParameters {
    pub rids: Vec<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
pub type CancelSqlQueryPermissionDeniedParameters = EmptyParameters;
pub type ColumnTypesNotSupportedParameters = EmptyParameters;
pub type ExecuteOntologySqlQueryPermissionDeniedParameters = EmptyParameters;
pub type ExecuteSqlQueryPermissionDeniedParameters = EmptyParameters;
pub type GetResultsSqlQueryPermissionDeniedParameters = EmptyParameters;
pub type GetStatusSqlQueryPermissionDeniedParameters = EmptyParameters;
pub type OntologyQueryFailedParameters = ErrorMessageParameters;
pub type QueryCanceledParameters = EmptyParameters;
pub type QueryFailedParameters = ErrorMessageParameters;
pub type QueryParseErrorParameters = ErrorMessageParameters;
pub type QueryPermissionDeniedParameters = EmptyParameters;
pub type QueryRunningParameters = EmptyParameters;
