//! Public APIs namespace wire types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type ApiDefinitionDeprecated = bool;
pub type ApiDefinitionName = String;
pub type ApiDefinitionRid = String;
pub type OpenApiDefinitionDeprecated = bool;
pub type OpenApiDefinitionValue = Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiVersion {
    V1,
    V2,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IrVersion {
    V1,
    V2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDefinition {
    pub version: IrVersion,
    pub rid: ApiDefinitionRid,
    pub name: ApiDefinitionName,
    pub deprecated: ApiDefinitionDeprecated,
    pub ir: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiDefinition {
    pub api_version: ApiVersion,
    pub deprecated: OpenApiDefinitionDeprecated,
    pub value: OpenApiDefinitionValue,
}
