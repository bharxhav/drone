//! Functions namespace wire types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type DataValue = Value;
pub type ExecutionId = String;
pub type FunctionRid = String;
pub type FunctionVersion = String;
pub type ParameterId = String;
pub type QueryApiName = String;
pub type TransactionId = String;
pub type TypeReferenceIdentifier = String;
pub type ValueTypeRid = String;
pub type ValueTypeVersion = String;
pub type ValueTypeVersionId = String;
pub type ValueTypeApiName = String;
pub type ValueTypeDescription = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub api_name: QueryApiName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub parameters: HashMap<ParameterId, Parameter>,
    pub output: QueryDataType,
    pub rid: FunctionRid,
    pub version: FunctionVersion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_references: Option<HashMap<TypeReferenceIdentifier, QueryDataType>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub data_type: QueryDataType,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum QueryDataType {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "struct")]
    Struct { fields: Vec<QueryStructField> },
    #[serde(rename = "set")]
    Set {
        #[serde(rename = "subType")]
        sub_type: Box<QueryDataType>,
    },
    #[serde(rename = "void")]
    Void,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "threeDimensionalAggregation")]
    ThreeDimensionalAggregation {
        #[serde(rename = "keyType")]
        key_type: QueryAggregationKeyType,
        #[serde(rename = "valueType")]
        value_type: TwoDimensionalAggregation,
    },
    #[serde(rename = "union")]
    Union {
        #[serde(rename = "unionTypes")]
        union_types: Vec<QueryDataType>,
    },
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "mediaReference")]
    MediaReference,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "array")]
    Array {
        #[serde(rename = "subType")]
        sub_type: Box<QueryDataType>,
    },
    #[serde(rename = "twoDimensionalAggregation")]
    TwoDimensionalAggregation {
        #[serde(rename = "keyType")]
        key_type: QueryAggregationKeyType,
        #[serde(rename = "valueType")]
        value_type: QueryAggregationValueType,
    },
    #[serde(rename = "valueTypeReference")]
    ValueTypeReference {
        rid: ValueTypeRid,
        #[serde(rename = "versionId")]
        version_id: ValueTypeVersionId,
    },
    #[serde(rename = "typeReference")]
    TypeReference {
        #[serde(rename = "typeId")]
        type_id: TypeReferenceIdentifier,
    },
    #[serde(rename = "timestamp")]
    Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryStructField {
    pub name: String,
    pub field_type: QueryDataType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum QueryAggregationKeyType {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "range")]
    Range {
        #[serde(rename = "subType")]
        sub_type: QueryAggregationRangeSubType,
    },
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "timestamp")]
    Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum QueryAggregationRangeSubType {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "timestamp")]
    Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum QueryAggregationValueType {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "timestamp")]
    Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoDimensionalAggregation {
    pub key_type: QueryAggregationKeyType,
    pub value_type: QueryAggregationValueType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQueryRequest {
    pub parameters: HashMap<ParameterId, Option<DataValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<FunctionVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteAsyncQueryRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ontology: Option<String>,
    pub parameters: HashMap<ParameterId, Option<DataValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<FunctionVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

pub type StreamingExecuteQueryRequest = ExecuteAsyncQueryRequest;
pub type StreamingExecuteEventsQueryRequest = ExecuteAsyncQueryRequest;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteQueryResponse {
    pub value: DataValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExecuteQueryAsyncResponse {
    #[serde(rename = "submitted")]
    Submitted {
        #[serde(rename = "executionId")]
        execution_id: ExecutionId,
    },
    #[serde(rename = "completed")]
    Completed { value: DataValue },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelExecutionResponse {
    pub id: ExecutionId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResultExecutionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GetExecutionResultResponse {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "succeeded")]
    Succeeded { value: DataValue },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetByRidQueriesBatchRequestElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_prerelease: Option<bool>,
    pub rid: FunctionRid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<FunctionVersion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetByRidQueriesBatchResponse {
    pub data: Vec<Query>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StreamingExecuteQueryResponse {
    #[serde(rename = "data")]
    Data { value: DataValue },
    #[serde(rename = "error")]
    Error {
        #[serde(rename = "errorCode")]
        error_code: String,
        #[serde(rename = "errorName")]
        error_name: String,
        #[serde(rename = "errorInstanceId")]
        error_instance_id: String,
        #[serde(
            rename = "errorDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        error_description: Option<String>,
        parameters: HashMap<String, Value>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueType {
    pub rid: ValueTypeRid,
    pub version: ValueTypeVersion,
    pub version_id: ValueTypeVersionId,
    pub api_name: ValueTypeApiName,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<ValueTypeDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<ValueTypeDataType>,
    pub constraints: Vec<ValueTypeConstraint>,
}

pub type VersionId = ValueType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ValueTypeConstraint {
    #[serde(rename = "struct")]
    Struct {
        fields: HashMap<String, ValueTypeApiName>,
    },
    #[serde(rename = "structV1")]
    StructV1 {
        fields: HashMap<String, ValueTypeConstraint>,
    },
    #[serde(rename = "regex")]
    Regex {
        pattern: String,
        #[serde(rename = "partialMatch")]
        partial_match: bool,
    },
    #[serde(rename = "nullable")]
    Nullable { value: NullableConstraintValue },
    #[serde(rename = "array")]
    Array {
        #[serde(
            rename = "minimumSize",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        minimum_size: Option<i64>,
        #[serde(
            rename = "maximumSize",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        maximum_size: Option<i64>,
        #[serde(rename = "uniqueValues")]
        unique_values: bool,
        #[serde(
            rename = "valueConstraint",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        value_constraint: Option<Box<ValueTypeConstraint>>,
    },
    #[serde(rename = "length")]
    Length {
        #[serde(
            rename = "minimumLength",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        minimum_length: Option<i64>,
        #[serde(
            rename = "maximumLength",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        maximum_length: Option<i64>,
    },
    #[serde(rename = "range")]
    Range {
        #[serde(
            rename = "minimumValue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        minimum_value: Option<Value>,
        #[serde(
            rename = "maximumValue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        maximum_value: Option<Value>,
    },
    #[serde(rename = "rid")]
    Rid,
    #[serde(rename = "map")]
    Map {
        #[serde(rename = "keyConstraints")]
        key_constraints: Vec<ValueTypeConstraint>,
        #[serde(rename = "valueConstraints")]
        value_constraints: Vec<ValueTypeConstraint>,
        #[serde(rename = "uniqueValues")]
        unique_values: bool,
    },
    #[serde(rename = "uuid")]
    Uuid,
    #[serde(rename = "enum")]
    Enum { options: Vec<Value> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NullableConstraintValue {
    #[serde(rename = "NULLABLE")]
    Nullable,
    #[serde(rename = "NOT_NULLABLE")]
    NotNullable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ValueTypeDataType {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "struct")]
    Struct {
        fields: Vec<ValueTypeDataTypeStructElement>,
    },
    #[serde(rename = "string")]
    String,
    #[serde(rename = "byte")]
    Byte,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "optional")]
    Optional {
        #[serde(rename = "wrappedType")]
        wrapped_type: Box<ValueTypeDataType>,
    },
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "union")]
    Union {
        #[serde(rename = "memberTypes")]
        member_types: Vec<ValueTypeDataType>,
    },
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "array")]
    Array {
        #[serde(rename = "subType")]
        sub_type: Box<ValueTypeDataType>,
    },
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "valueTypeReference")]
    ValueTypeReference {
        rid: ValueTypeRid,
        #[serde(rename = "versionId")]
        version_id: ValueTypeVersionId,
    },
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "map")]
    Map {
        #[serde(rename = "keyType")]
        key_type: Box<ValueTypeDataType>,
        #[serde(rename = "valueType")]
        value_type: Box<ValueTypeDataType>,
    },
    #[serde(rename = "timestamp")]
    Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueTypeDataTypeStructElement {
    pub name: String,
    pub field_type: ValueTypeDataType,
}
