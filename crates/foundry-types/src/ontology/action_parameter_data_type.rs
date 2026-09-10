use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ActionParameterDataType {
    Date,
    String,
    Byte,
    Double,
    Integer,
    Float,
    Long,
    Boolean,
    Attachment,
    MediaReference,
    Geoshape,
    Geohash,
    ScenarioReference,
    ObjectType,
    Object,
    ObjectSet,
    InterfaceObject,
    Marking,
    Vector,
    Array {
        #[serde(rename = "subType")]
        sub_type: Box<ActionParameterDataType>,
    },
    Struct {
        fields: Vec<StructField>,
    },
    Short,
    Decimal,
    Timestamp,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructField {
    pub name: String,
    pub data_type: ActionParameterDataType,
    pub required: bool,
}
