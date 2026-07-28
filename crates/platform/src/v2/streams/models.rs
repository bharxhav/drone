//! Streams namespace wire types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Compressed = bool;
pub type GetEndOffsetsResponse = HashMap<PartitionId, i64>;
pub type GetRecordsResponse = Vec<RecordWithOffset>;
pub type PartitionId = String;
pub type PartitionOffsets = HashMap<PartitionId, i64>;
pub type PartitionRecords = Vec<RecordWithOffset>;
pub type PartitionsCount = i32;
pub type Record = HashMap<String, Option<serde_json::Value>>;
pub type SubscriberId = String;
pub type ViewRid = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamType {
    LowLatency,
    HighThroughput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStreamRequestStreamSchema {
    pub key_field_names: Option<Vec<String>>,
    pub fields: Vec<serde_json::Value>,
    pub change_data_capture: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStreamRequest {
    #[serde(rename = "schema")]
    pub schema: CreateStreamRequestStreamSchema,
    pub partitions_count: Option<i32>,
    pub stream_type: Option<StreamType>,
    pub branch_name: String,
    pub compressed: Option<bool>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStreamingDatasetRequest {
    pub name: String,
    pub parent_folder_rid: String,
    #[serde(rename = "schema")]
    pub schema: serde_json::Value,
    pub branch_name: Option<String>,
    pub partitions_count: Option<i32>,
    pub stream_type: Option<StreamType>,
    pub compressed: Option<bool>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubscriberRequest {
    pub subscriber_id: String,
    pub read_position: Option<ReadPosition>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    pub rid: String,
    pub name: String,
    pub parent_folder_rid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReadPosition {
    #[serde(rename = "earliest")]
    Earliest {},
    #[serde(rename = "latest")]
    Latest {},
    #[serde(rename = "specific")]
    Specific { offsets: PartitionOffsets },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishRecordToStreamRequest {
    pub record: Record,
    pub view_rid: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishRecordsToStreamRequest {
    pub records: Vec<Record>,
    pub view_rid: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordWithOffset {
    pub offset: i64,
    pub value: Record,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetStreamRequest {
    #[serde(rename = "schema")]
    pub schema: Option<serde_json::Value>,
    pub partitions_count: Option<i32>,
    pub stream_type: Option<StreamType>,
    pub compressed: Option<bool>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitSubscriberOffsetsRequest {
    pub view_rid: Option<String>,
    pub offsets: PartitionOffsets,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadRecordsFromSubscriberRequest {
    pub view_rid: Option<String>,
    pub limit: Option<i32>,
    pub partition_ids: Option<Vec<String>>,
    pub auto_commit: Option<bool>,
}
#[derive(Debug, Clone, Serialize)]
pub struct ResetSubscriberOffsetsRequest {
    pub position: ReadPosition,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadSubscriberRecordsResponse {
    pub records_by_partition: HashMap<PartitionId, PartitionRecords>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub branch_name: String,
    #[serde(rename = "schema")]
    pub schema: serde_json::Value,
    pub view_rid: String,
    pub partitions_count: i32,
    pub stream_type: StreamType,
    pub compressed: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscriber {
    pub subscriber_id: String,
    pub read_position: Option<ReadPosition>,
    pub dataset_rid: String,
    pub branch_name: String,
    pub view_rid: String,
    pub start_offsets: PartitionOffsets,
    pub created_time: String,
}
