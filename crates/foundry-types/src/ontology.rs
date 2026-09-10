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
