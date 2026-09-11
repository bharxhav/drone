use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Rid, ontology::object_property_data_type::ObjectPropertyDataType};

use super::object_type::TypeClass;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceType {
    pub ontology_rid: Rid,
    pub rid: Rid,
    pub api_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub properties: HashMap<String, SharedProperty>,
    pub all_properties: HashMap<String, SharedProperty>,
    pub properties_v2: HashMap<String, InterfaceProperty>,
    pub all_properties_v2: HashMap<String, ResolvedInterfaceProperty>,
    pub extends_interfaces: Vec<String>,
    pub all_extends_interfaces: Vec<String>,
    pub implemented_by_object_types: Vec<String>,
    pub links: HashMap<String, InterfaceLink>,
    pub all_links: HashMap<String, InterfaceLink>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum InterfaceProperty {
    InterfaceDefinedPropertyType(DefinedProperty),
    InterfaceSharedPropertyType(SharedProperty),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinedProperty {
    pub rid: Rid,
    pub api_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub data_type: ObjectPropertyDataType,
    pub value_type_api_name: Option<String>,
    pub require_implementation: bool,
    pub type_classes: Vec<TypeClass>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedProperty {
    pub rid: Rid,
    pub api_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub data_type: ObjectPropertyDataType,
    pub value_type_api_name: Option<String>,
    pub required: bool,
    pub type_classes: Vec<TypeClass>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedInterfaceProperty {
    pub rid: Rid,
    pub api_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub data_type: ObjectPropertyDataType,
    pub value_type_api_name: Option<String>,
    pub require_implementation: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceLink {
    pub rid: Rid,
    pub api_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub linked_entity_api_name: LinkedEntity,
    pub cardinality: Cardinality,
    pub required: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LinkedEntity {
    ObjectTypeApiName { object_type_api_name: String },
    InterfaceTypeApiName { interface_type_api_name: String },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Cardinality {
    One,
    Many,
}
