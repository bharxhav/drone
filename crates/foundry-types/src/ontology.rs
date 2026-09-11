pub mod action_parameter_data_type;
pub mod action_type;
pub mod interface_type;
pub mod object;
pub mod object_property_data_type;
pub mod object_type;
pub mod query_type;
pub mod value_type;

use serde::{Deserialize, Serialize};

use crate::Rid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ontology {
    pub api_name: String,
    pub display_name: String,
    pub description: String,
    pub rid: Rid,
}
