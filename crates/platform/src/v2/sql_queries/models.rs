//! SQL Queries namespace wire types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type MapParameterKey = String;
pub type ParameterMapping = HashMap<ParameterName, ParameterValue>;
pub type ParameterName = String;
pub type ScenarioRid = String;
pub type SqlQueryId = String;
pub type TableName = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SerializationFormat {
    Arrow,
    Csv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ColumnType {
    #[serde(rename = "date")]
    Date {},
    #[serde(rename = "struct")]
    Struct { fields: Vec<StructColumnFieldType> },
    #[serde(rename = "string")]
    String {},
    #[serde(rename = "double")]
    Double {},
    #[serde(rename = "integer")]
    Integer {},
    #[serde(rename = "float")]
    Float {},
    #[serde(rename = "list")]
    List {
        #[serde(rename = "elementType")]
        element_type: Box<ColumnType>,
    },
    #[serde(rename = "any")]
    Any {},
    #[serde(rename = "long")]
    Long {},
    #[serde(rename = "boolean")]
    Boolean {},
    #[serde(rename = "binary")]
    Binary {},
    #[serde(rename = "short")]
    Short {},
    #[serde(rename = "decimal")]
    Decimal { precision: i32, scale: i32 },
    #[serde(rename = "map")]
    Map {
        #[serde(rename = "keyType")]
        key_type: Box<ColumnType>,
        #[serde(rename = "valueType")]
        value_type: Box<ColumnType>,
    },
    #[serde(rename = "timestamp")]
    Timestamp {},
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructColumnFieldType {
    pub name: String,
    pub r#type: ColumnType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StructElementName {
    #[serde(rename = "structFieldRid")]
    Rid { value: String },
    #[serde(rename = "structFieldKey")]
    Key { value: String },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructElement {
    pub struct_element_name: StructElementName,
    pub struct_element_value: ParameterValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParameterValue {
    #[serde(rename = "date")]
    Date { value: String },
    #[serde(rename = "struct")]
    Struct {
        #[serde(rename = "structElements")]
        struct_elements: Vec<StructElement>,
    },
    #[serde(rename = "string")]
    String { value: String },
    #[serde(rename = "double")]
    Double { value: f64 },
    #[serde(rename = "integer")]
    Integer { value: i32 },
    #[serde(rename = "float")]
    Float { value: f64 },
    #[serde(rename = "list")]
    List {
        values: Vec<ParameterValue>,
        #[serde(rename = "elementType")]
        element_type: ColumnType,
    },
    #[serde(rename = "any")]
    Any { value: serde_json::Value },
    #[serde(rename = "long")]
    Long { value: i64 },
    #[serde(rename = "boolean")]
    Boolean { value: bool },
    #[serde(rename = "null")]
    Null {},
    #[serde(rename = "short")]
    Short { value: i32 },
    #[serde(rename = "decimal")]
    Decimal { value: serde_json::Number },
    #[serde(rename = "map")]
    Map {
        values: HashMap<String, ParameterValue>,
    },
    #[serde(rename = "timestamp")]
    Timestamp { value: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Parameters {
    #[serde(rename = "unnamedParameterValues")]
    Unnamed { values: Vec<ParameterValue> },
    #[serde(rename = "namedParameterMapping")]
    Named { mapping: ParameterMapping },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteOntologySqlQueryRequest {
    pub query: String,
    pub parameters: Option<Parameters>,
    pub row_limit: Option<i32>,
    pub dry_run: Option<bool>,
    pub branch: Option<serde_json::Value>,
    pub scenario_rid: Option<String>,
    pub ontology_identifier: Option<serde_json::Value>,
    pub table_providers: Option<HashMap<String, serde_json::Value>>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteSqlQueryRequest {
    pub query: String,
    pub fallback_branch_ids: Option<Vec<String>>,
    pub serialization_format: Option<SerializationFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum QueryStatus {
    #[serde(rename = "running")]
    Running {
        #[serde(rename = "queryId")]
        query_id: String,
    },
    #[serde(rename = "canceled")]
    Canceled {},
    #[serde(rename = "failed")]
    Failed {
        #[serde(rename = "errorMessage")]
        error_message: String,
    },
    #[serde(rename = "succeeded")]
    Succeeded {
        #[serde(rename = "queryId")]
        query_id: String,
    },
}
