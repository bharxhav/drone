use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{ApiName, Rid};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub ontology_rid: Rid,
    pub object_type_api_name: ApiName,
    pub rid: Rid,
    pub primary_key: PropertyValue,
    pub properties: HashMap<ApiName, PropertyValue>,
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
    GeotimeSeriesReference(GeotimeSeriesReference),
    Geoshape(GeoJson),
    Integer(i32),
    Long(i64),
    Marking(Marking),
    MediaReference(MediaReference),
    Secured(Box<SecuredPropertyValue>),
    Short(i16),
    String(String),
    Struct(HashMap<ApiName, PropertyValue>),
    Timestamp(String),
    Timeseries(TimeseriesProperty),
    Vector(Vec<f64>),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub rid: Rid,
    pub filename: String,
    pub size_bytes: u64,
    pub media_type: String,
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
    #[serde(flatten)]
    pub geometry: GeoJsonGeometry,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum GeoJsonGeometry {
    Point {
        coordinates: Position,
    },
    MultiPoint {
        coordinates: Vec<Position>,
    },
    LineString {
        coordinates: Vec<Position>,
    },
    MultiLineString {
        coordinates: Vec<Vec<Position>>,
    },
    Polygon {
        coordinates: Vec<Vec<Position>>,
    },
    MultiPolygon {
        coordinates: Vec<Vec<Vec<Position>>>,
    },
}

pub type Position = Vec<f64>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeotimeSeriesReference {
    pub series_id: String,
    pub integration_rid: Rid,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Marking {
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuredPropertyValue {
    pub value: Option<Box<PropertyValue>>,
    pub property_security_index: Option<u32>,
}
