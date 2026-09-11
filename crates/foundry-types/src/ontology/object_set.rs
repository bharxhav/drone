use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Rid;

use super::object::PropertyValue;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ObjectSet {
    Base {
        object_type: String,
    },
    InterfaceBase {
        interface_type: String,
        include_all_base_object_properties: Option<bool>,
    },
    Static {
        objects: Vec<Rid>,
    },
    Reference {
        reference: Rid,
    },
    Filter {
        object_set: Box<ObjectSet>,
        r#where: SearchQuery,
    },
    Union {
        object_sets: Vec<ObjectSet>,
    },
    Intersect {
        object_sets: Vec<ObjectSet>,
    },
    Subtract {
        object_sets: Vec<ObjectSet>,
    },
    SearchAround {
        object_set: Box<ObjectSet>,
        link: String,
    },
    InterfaceLinkSearchAround {
        object_set: Box<ObjectSet>,
        interface_link: String,
    },
    AsType {
        entity_type: String,
        object_set: Box<ObjectSet>,
    },
    AsBaseObjectTypes {
        object_set: Box<ObjectSet>,
    },
    NearestNeighbors {
        object_set: Box<ObjectSet>,
        property: String,
        num_neighbors: u32,
        similarity_threshold: Option<f64>,
        query: NearestNeighborsQuery,
    },
    WithProperties {
        object_set: Box<ObjectSet>,
        derived_properties: HashMap<String, DerivedProperty>,
    },
    MethodInput,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum NearestNeighborsQuery {
    Vector { values: Vec<f64> },
    Text { value: String },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DerivedProperty {
    Add {
        properties: Vec<DerivedProperty>,
    },
    AbsoluteValue {
        property: Box<DerivedProperty>,
    },
    Extract {
        property: Box<DerivedProperty>,
        part: ExtractDatePart,
    },
    Negate {
        property: Box<DerivedProperty>,
    },
    Subtract {
        left: Box<DerivedProperty>,
        right: Box<DerivedProperty>,
    },
    Property {
        api_name: String,
    },
    Least {
        properties: Vec<DerivedProperty>,
    },
    Divide {
        left: Box<DerivedProperty>,
        right: Box<DerivedProperty>,
    },
    Multiply {
        properties: Vec<DerivedProperty>,
    },
    Greatest {
        properties: Vec<DerivedProperty>,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExtractDatePart {
    Year,
    Quarter,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SearchQuery {
    And {
        value: Vec<SearchQuery>,
    },
    Or {
        value: Vec<SearchQuery>,
    },
    Not {
        value: Box<SearchQuery>,
    },
    Eq {
        field: String,
        value: PropertyValue,
    },
    Lt {
        field: String,
        value: PropertyValue,
    },
    Lte {
        field: String,
        value: PropertyValue,
    },
    Gt {
        field: String,
        value: PropertyValue,
    },
    Gte {
        field: String,
        value: PropertyValue,
    },
    In {
        field: String,
        value: Vec<PropertyValue>,
    },
    IsNull {
        field: String,
        value: bool,
    },
    Contains {
        field: String,
        value: PropertyValue,
    },
    ContainsAllTerms {
        field: String,
        value: String,
    },
    ContainsAnyTerm {
        field: String,
        value: String,
    },
    ContainsAllTermsInOrder {
        field: String,
        value: String,
    },
    StartsWith {
        field: String,
        value: String,
    },
    Wildcard {
        field: String,
        value: String,
    },
    Regex {
        field: String,
        value: String,
    },
}
