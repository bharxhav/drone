pub mod link_type;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Rid, ontology::object_property_data_type::ObjectPropertyDataType};
use link_type::LinkType;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectType {
    pub api_name: String,
    pub description: String,
    pub display_name: String,
    pub status: Status,
    pub primary_key: String,
    pub properties: HashMap<String, Property>,
    pub outgoing_links: Vec<LinkType>,
    pub rid: Rid,
    pub ontology_rid: Rid,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Active,
    Endorsed,
    Experimental,
    Deprecated,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub description: Option<String>,
    pub data_type: ObjectPropertyDataType,
    pub value_type_api_name: Option<String>,
    pub rid: Rid,
}
