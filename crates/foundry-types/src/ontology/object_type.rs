use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Rid, ontology::DataType};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectType {
    pub api_name: String,
    pub description: String,
    pub display_name: String,
    pub status: Status,
    pub primary_key: String,
    pub properties: HashMap<String, Property>,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub description: Option<String>,
    pub data_type: DataType,
    pub rid: Rid,
}
