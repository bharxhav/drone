use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ObjectPropertyDataType {
    Date,
    Struct {
        fields: Vec<StructField>,
    },
    String,
    Byte,
    Double,
    Geopoint,
    GeotimeSeriesReference,
    Integer,
    Float,
    Geoshape,
    Long,
    Boolean,
    CipherText,
    Marking,
    Attachment,
    MediaReference,
    Timeseries,
    Array {
        #[serde(rename = "subType")]
        sub_type: Box<ObjectPropertyDataType>,
    },
    Short,
    Vector,
    Decimal,
    Timestamp,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructField {
    pub name: String,
    pub data_type: ObjectPropertyDataType,
    pub required: bool,
}
