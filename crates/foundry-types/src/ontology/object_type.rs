pub mod link_type;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{ApiName, Rid, ontology::object_property_data_type::ObjectPropertyDataType};
use link_type::LinkTypeSide;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectTypeFullMetadata {
    pub ontology_rid: Rid,
    pub object_type: ObjectType,
    pub link_types: Vec<LinkTypeSide>,
    pub implements_interfaces: Vec<ApiName>,
    pub implements_interfaces2: HashMap<ApiName, InterfaceImplementation>,
    pub shared_property_type_mapping: HashMap<ApiName, ApiName>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectType {
    pub api_name: ApiName,
    pub description: Option<String>,
    pub display_name: String,
    pub plural_display_name: String,
    pub status: Status,
    pub icon: Icon,
    pub primary_key: ApiName,
    pub title_property: ApiName,
    pub properties: HashMap<ApiName, Property>,
    pub rid: Rid,
    pub visibility: Option<Visibility>,
    pub aliases: Vec<String>,
    pub datasources: Vec<Datasource>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Active,
    Endorsed,
    Experimental,
    Deprecated,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Icon {
    Blueprint { color: String, name: String },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Visibility {
    Normal,
    Prominent,
    Hidden,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub data_type: ObjectPropertyDataType,
    pub rid: Rid,
    pub status: Option<PropertyStatus>,
    pub visibility: Option<Visibility>,
    pub value_type_api_name: Option<ApiName>,
    pub value_formatting: Option<serde_json::Value>,
    pub type_classes: Vec<TypeClass>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PropertyStatus {
    Active,
    Deprecated {
        message: String,
        deadline: String,
        replaced_by: Option<Rid>,
    },
    Experimental,
    Example,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TypeClass {
    pub kind: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceImplementation {
    pub api_name: Option<ApiName>,
    pub rid: Option<Rid>,
    pub properties: HashMap<ApiName, ApiName>,
    pub properties_v2: HashMap<String, serde_json::Value>,
    pub links: HashMap<ApiName, Vec<ApiName>>,
    pub action_types: HashMap<ApiName, ApiName>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Datasource {
    pub rid: Rid,
    pub definition: DatasourceDefinition,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DatasourceDefinition {
    TimeSeries {
        time_series_sync_rid: Rid,
        properties: Vec<String>,
    },
    Unsupported {
        unsupported_type: String,
        properties: Vec<String>,
    },
    RestrictedView {
        restricted_view_rid: Rid,
        property_mapping: HashMap<String, PropertyMapping>,
    },
    Stream {
        stream_rid: Rid,
        branch: Option<String>,
        property_mapping: HashMap<String, PropertyMapping>,
    },
    MediaSetView {
        media_set_view_rid: Rid,
        properties: Vec<String>,
    },
    Direct {
        direct_source_rid: Rid,
        property_mapping: HashMap<String, PropertyMapping>,
    },
    GeotimeSeries {
        geotime_series_integration_rid: Rid,
        properties: Vec<String>,
    },
    EditsOnly,
    Dataset {
        dataset_rid: Rid,
        branch: Option<String>,
        property_mapping: HashMap<String, PropertyMapping>,
    },
    Table {
        table_rid: Rid,
        branch: Option<String>,
        property_mapping: HashMap<String, PropertyMapping>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PropertyMapping {
    Struct {
        column: String,
        fields: HashMap<String, StructFieldMapping>,
    },
    Column {
        column: String,
    },
    EditOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructFieldMapping {
    pub api_name: String,
}
