use serde::{Deserialize, Serialize};

use crate::{ApiName, Rid};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkTypeSide {
    pub api_name: ApiName,
    pub display_name: String,
    pub status: Status,
    pub object_type_api_name: ApiName,
    pub cardinality: Cardinality,
    pub foreign_key_property_api_name: Option<ApiName>,
    pub link_type_rid: Rid,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Active,
    Endorsed,
    Experimental,
    Deprecated,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Cardinality {
    One,
    Many,
}
