use serde::{Deserialize, Serialize};

use crate::{ApiName, Rid};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueType {
    pub api_name: ApiName,
    pub description: Option<String>,
    pub display_name: String,
    pub status: Status,
    pub field_type: ValueTypeFieldType,
    pub rid: Rid,
    pub ontology_rid: Rid,
    pub constraints: Vec<ValueTypeConstraint>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Active,
    Deprecated,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ValueTypeFieldType {
    Date,
    Struct {
        fields: Vec<StructField>,
    },
    String,
    Byte,
    Double,
    Optional {
        #[serde(rename = "wrappedType")]
        wrapped_type: Option<Box<ValueTypeFieldType>>,
    },
    Integer,
    Union {
        #[serde(rename = "memberTypes")]
        member_types: Vec<ValueTypeFieldType>,
    },
    Float,
    Long,
    Reference,
    Boolean,
    Array {
        #[serde(rename = "subType")]
        sub_type: Option<Box<ValueTypeFieldType>>,
    },
    Binary,
    Short,
    Decimal,
    Map {
        #[serde(rename = "keyType")]
        key_type: Option<Box<ValueTypeFieldType>>,
        #[serde(rename = "valueType")]
        value_type: Option<Box<ValueTypeFieldType>>,
    },
    Timestamp,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructField {
    pub name: Option<String>,
    pub field_type: Option<ValueTypeFieldType>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ValueTypeConstraint {
    Struct,
    Regex {
        regex: RegexConstraint,
    },
    Unsupported,
    Array,
    Length {
        length: LengthConstraint,
    },
    Range {
        range: RangeConstraint,
    },
    Rid,
    Uuid,
    Enum {
        #[serde(rename = "enum")]
        constraint: EnumConstraint,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LengthConstraint {
    pub minimum_length: Option<u64>,
    pub maximum_length: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegexConstraint {
    pub pattern: String,
    pub partial_match: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeConstraint {
    pub minimum_value: Option<serde_json::Value>,
    pub maximum_value: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnumConstraint {
    pub options: Vec<serde_json::Value>,
}
