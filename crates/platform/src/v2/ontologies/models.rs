//! Ontologies namespace wire types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type OntologyObject = HashMap<String, Value>;
pub type DataValue = Value;
pub type ObjectSet = Value;
pub type SearchQuery = Value;
pub type Aggregation = Value;
pub type AggregationGroupBy = Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ontology {
    pub api_name: String,
    pub display_name: String,
    pub description: String,
    pub rid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOntologiesResponse {
    pub data: Vec<Ontology>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadOntologyMetadataRequest {
    pub object_types: Vec<String>,
    pub link_types: Vec<String>,
    pub action_types: Vec<String>,
    pub query_types: Vec<String>,
    pub interface_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyFullMetadata {
    pub ontology: Ontology,
    pub object_types: HashMap<String, Value>,
    pub action_types: HashMap<String, Value>,
    pub query_types: HashMap<String, Value>,
    pub interface_types: HashMap<String, Value>,
    pub shared_property_types: HashMap<String, Value>,
    pub branch: Option<Value>,
    pub value_types: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct CommonQuery<'a> {
    pub branch: Option<&'a str>,
    pub sdk_package_rid: Option<&'a str>,
    pub sdk_version: Option<&'a str>,
}

#[derive(Debug, Clone, Default)]
pub struct ObjectQuery<'a> {
    pub common: CommonQuery<'a>,
    pub exclude_rid: Option<bool>,
    pub select: Option<Vec<&'a str>>,
}

#[derive(Debug, Clone, Default)]
pub struct ListObjectsQuery<'a> {
    pub object: ObjectQuery<'a>,
    pub order_by: Option<&'a str>,
    pub page_size: Option<u32>,
    pub page_token: Option<&'a str>,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectsPage {
    pub data: Vec<OntologyObject>,
    pub next_page_token: Option<String>,
    pub total_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchOrderBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    pub fields: Vec<SearchOrdering>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchOrdering {
    pub field: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyIdentifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_api_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_property_type_api_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceSigningOptions {
    pub expires_after_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchObjectsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#where: Option<SearchQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<SearchOrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    pub select: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_v2: Option<Vec<PropertyIdentifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_rid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_signing_options: Option<ReferenceSigningOptions>,
}

pub type SearchObjectsResponse = ObjectsPage;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateObjectsRequest {
    pub aggregation: Vec<Aggregation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#where: Option<SearchQuery>,
    pub group_by: Vec<AggregationGroupBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationMetricResult {
    pub name: String,
    pub value: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateObjectsResponseItem {
    pub group: HashMap<String, Option<Value>>,
    pub metrics: Vec<AggregationMetricResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateObjectsResponse {
    pub excluded_items: Option<u64>,
    pub accuracy: String,
    pub data: Vec<AggregateObjectsResponseItem>,
    pub compute_usage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountObjectsResponse {
    pub count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyActionRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_edits: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyActionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ApplyActionRequestOptions>,
    pub parameters: HashMap<String, Option<DataValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyActionOverrides {
    pub unique_identifier_link_id_values: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_execution_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyActionWithOverridesRequest {
    pub request: ApplyActionRequest,
    pub overrides: ApplyActionOverrides,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchApplyActionRequestItem {
    pub parameters: HashMap<String, Option<DataValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchApplyActionRequestItemWithOverrides {
    pub parameters: HashMap<String, Option<DataValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<ApplyActionOverrides>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchApplyActionRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_edits: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchApplyActionRequest<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<BatchApplyActionRequestOptions>,
    pub requests: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyActionResponse {
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub validation: Option<Value>,
    pub edits: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchApplyActionResponse {
    pub edits: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteQueryRequest {
    pub parameters: HashMap<String, Option<DataValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteQueryResponse {
    pub value: DataValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub rid: String,
    pub filename: String,
    pub size_bytes: u64,
    pub media_type: String,
    #[serde(rename = "type")]
    pub attachment_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateObjectSetRequest {
    pub aggregation: Vec<Aggregation>,
    pub object_set: ObjectSet,
    pub group_by: Vec<AggregationGroupBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_compute_usage: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemporaryObjectSetRequest {
    pub object_set: ObjectSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemporaryObjectSetResponse {
    pub object_set_rid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadObjectSetRequest {
    pub object_set: ObjectSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<SearchOrderBy>,
    pub select: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_v2: Option<Vec<PropertyIdentifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_rid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_property_securities: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_compute_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_signing_options: Option<ReferenceSigningOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadObjectSetResponse {
    pub data: Vec<OntologyObject>,
    pub next_page_token: Option<String>,
    pub total_count: u64,
    pub compute_usage: Option<f64>,
    pub property_securities: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadObjectSetLinksRequest {
    pub object_set: ObjectSet,
    pub links: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_compute_usage: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadObjectSetLinksResponse {
    pub data: Vec<Value>,
    pub next_page_token: Option<String>,
    pub compute_usage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeseriesEntry {
    pub time: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TimeRange {
    Absolute {
        #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
        start_time: Option<String>,
        #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
        end_time: Option<String>,
    },
    Relative {
        start_time: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamTimeSeriesValuesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<TimeRange>,
}
