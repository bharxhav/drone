use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{ApiName, Rid};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryType {
    pub ontology_rid: Rid,
    pub api_name: ApiName,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub parameters: HashMap<String, QueryParameter>,
    pub output: QueryDataType,
    pub rid: Rid,
    pub version: String,
    pub type_references: HashMap<String, QueryDataType>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParameter {
    pub description: Option<String>,
    pub data_type: QueryDataType,
    pub required: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum QueryDataType {
    Date,
    InterfaceObject {
        interface_api_name: ApiName,
    },
    Struct {
        fields: Vec<QueryStructField>,
    },
    String,
    Integer,
    ThreeDimensionalAggregation,
    Float,
    Long,
    Unsupported,
    Attachment,
    Array {
        #[serde(rename = "subType")]
        sub_type: Box<QueryDataType>,
    },
    ObjectSet {
        object_api_name: ApiName,
    },
    TwoDimensionalAggregation,
    TypeReference {
        type_id: String,
    },
    Timestamp,
    Set {
        #[serde(rename = "subType")]
        sub_type: Box<QueryDataType>,
    },
    Void,
    EntrySet {
        key_type: Box<QueryDataType>,
        value_type: Box<QueryDataType>,
    },
    Double,
    Union {
        union_types: Vec<QueryDataType>,
    },
    Boolean,
    MediaReference,
    Null,
    InterfaceObjectSet {
        interface_api_name: ApiName,
    },
    Object {
        object_api_name: ApiName,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryStructField {
    pub name: String,
    pub field_type: QueryDataType,
}
