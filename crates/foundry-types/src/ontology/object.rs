use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Rid;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub ontology_rid: Rid,
    pub object_type_api_name: String,
    pub rid: Rid,
    pub primary_key: PropertyValue,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum PropertyValue {
    Array(Vec<PropertyValue>),
    Attachment(Attachment),
    Boolean(bool),
    Byte(i8),
    CipherText(String),
    Date(String),
    Decimal(String),
    Double(f64),
    Float(f32),
    Geopoint(GeoJson),
    GeotimeSeriesReference(Value),
    Geoshape(GeoJson),
    Integer(i32),
    Long(i64),
    Marking(Value),
    MediaReference(MediaReference),
    Secured(Box<SecuredPropertyValue>),
    Short(i16),
    String(String),
    Struct(HashMap<String, PropertyValue>),
    Timestamp(String),
    Timeseries(TimeseriesProperty),
    Vector(Vec<f64>),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Attachment {
    pub rid: Rid,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaReference {
    pub mime_type: String,
    pub reference: MediaReferenceValue,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MediaReferenceValue {
    MediaSetViewItem {
        media_set_view_item: MediaSetViewItem,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSetViewItem {
    pub media_set_rid: Rid,
    pub media_set_view_rid: Rid,
    pub media_item_rid: Rid,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TimeseriesProperty {
    SeriesId(String),
    Series(TimeseriesSeries),
    Template(TimeseriesTemplate),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesSeries {
    pub series_id: String,
    pub sync_rid: Rid,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesTemplate {
    pub template_rid: Rid,
    pub template_version: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GeoJson {
    pub r#type: String,
    pub coordinates: Value,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuredPropertyValue {
    pub value: Option<Value>,
    pub property_security_index: Option<u32>,
}
