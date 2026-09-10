use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Rid, ontology::action_parameter_data_type::ActionParameterDataType};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionType {
    pub api_name: String,
    pub description: String,
    pub parameters: HashMap<String, Parameter>,
    pub rid: Rid,
    pub ontology_rid: Rid,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub data_type: ActionParameterDataType,
}
