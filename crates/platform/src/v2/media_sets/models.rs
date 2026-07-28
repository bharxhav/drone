//! Media Sets namespace wire types.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
pub type Rid = String;
pub type TransactionId = String;
pub type TransformationJobId = String;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaSchema {
    Audio,
    Dicom,
    Document,
    Imagery,
    Model3d,
    Multimodal,
    Spreadsheet,
    StreamingVideo,
    Video,
    Email,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionPolicy {
    #[serde(rename = "batchTransactions")]
    BatchTransactions,
    #[serde(rename = "noTransactions")]
    NoTransactions,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMediaSetResponse {
    pub rid: Rid,
    pub media_schema: MediaSchema,
    pub default_branch_name: String,
    pub transaction_policy: TransactionPolicy,
    pub paths_required: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMediaItemInfoResponse {
    pub view_rid: Rid,
    #[serde(default)]
    pub path: Option<String>,
    pub logical_timestamp: i64,
    #[serde(default)]
    pub attribution: Option<MediaAttribution>,
    #[serde(default)]
    pub originally_uploaded_file_mime_type: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub size_bytes: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAttribution {
    pub creator_id: String,
    pub creation_timestamp: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMediaItemRidByPathResponse {
    #[serde(default)]
    pub media_item_rid: Option<Rid>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutMediaItemResponse {
    pub media_item_rid: Rid,
    pub media_set_view_rid: Rid,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaReference {
    pub mime_type: String,
    pub reference: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterMediaItemRequest {
    pub physical_item_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_item_path: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterMediaItemResponse {
    pub media_item_rid: Rid,
    pub media_type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransformationJobStatus {
    Pending,
    Failed,
    Successful,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransformationJobStatusResponse {
    pub status: TransformationJobStatus,
    pub job_id: TransformationJobId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformMediaItemResponse {
    pub status: TransformationJobStatus,
    pub job_id: TransformationJobId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TrackedTransformationResponse {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "successful")]
    Successful,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioSpecification {
    pub bit_rate: i32,
    pub duration_seconds: f64,
    #[serde(default)]
    pub number_of_channels: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSpecification {
    pub bit_rate: i32,
    pub duration_seconds: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MediaItemMetadata {
    #[serde(rename = "document")]
    Document {
        format: String,
        #[serde(default)]
        pages: Option<i32>,
        #[serde(rename = "sizeBytes")]
        size_bytes: i64,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        author: Option<String>,
    },
    #[serde(rename = "imagery")]
    Imagery {
        format: String,
        #[serde(default)]
        dimensions: Option<Dimensions>,
        bands: Vec<Value>,
        attributes: HashMap<String, HashMap<String, String>>,
        #[serde(default, rename = "iccProfile")]
        icc_profile: Option<String>,
        #[serde(default)]
        geo: Option<Value>,
        #[serde(default)]
        pages: Option<i32>,
        #[serde(default)]
        orientation: Option<Value>,
        #[serde(rename = "sizeBytes")]
        size_bytes: i64,
    },
    #[serde(rename = "spreadsheet")]
    Spreadsheet {
        format: String,
        #[serde(rename = "sheetNames")]
        sheet_names: Vec<String>,
        #[serde(rename = "sizeBytes")]
        size_bytes: i64,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        author: Option<String>,
    },
    #[serde(rename = "untyped")]
    Untyped {
        #[serde(rename = "sizeBytes")]
        size_bytes: i64,
    },
    #[serde(rename = "audio")]
    Audio {
        format: String,
        specification: AudioSpecification,
        #[serde(rename = "sizeBytes")]
        size_bytes: i64,
    },
    #[serde(rename = "model3d")]
    Model3d {
        format: String,
        #[serde(rename = "modelType")]
        model_type: String,
        #[serde(rename = "sizeBytes")]
        size_bytes: i64,
    },
    #[serde(rename = "video")]
    Video {
        format: String,
        specification: VideoSpecification,
        #[serde(rename = "sizeBytes")]
        size_bytes: i64,
    },
    #[serde(rename = "dicom")]
    Dicom {
        #[serde(rename = "metaInformation")]
        meta_information: Value,
        #[serde(rename = "mediaType")]
        media_type: String,
        #[serde(rename = "commonDataElements")]
        common_data_elements: Value,
        #[serde(rename = "otherDataElements")]
        other_data_elements: HashMap<String, Value>,
        #[serde(rename = "sizeBytes")]
        size_bytes: i64,
    },
    #[serde(rename = "email")]
    Email {
        format: String,
        #[serde(rename = "sizeBytes")]
        size_bytes: i64,
        sender: Vec<Value>,
        date: String,
        #[serde(rename = "attachmentCount")]
        attachment_count: i32,
        to: Vec<Value>,
        cc: Vec<Value>,
        #[serde(default)]
        subject: Option<String>,
        attachments: Vec<Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Transformation {
    #[serde(rename = "emailToText")]
    EmailToText { operation: Value },
    #[serde(rename = "image")]
    Image {
        encoding: Value,
        operations: Vec<Value>,
    },
    #[serde(rename = "spreadsheetToText")]
    SpreadsheetToText { operation: Value },
    #[serde(rename = "videoToAudio")]
    VideoToAudio { encoding: Value, operation: Value },
    #[serde(rename = "audioToText")]
    AudioToText { operation: Value },
    #[serde(rename = "emailToAttachment")]
    EmailToAttachment { operation: Value },
    #[serde(rename = "videoToArchive")]
    VideoToArchive { encoding: Value, operation: Value },
    #[serde(rename = "videoToText")]
    VideoToText { operation: Value },
    #[serde(rename = "imageToText")]
    ImageToText { operation: Value },
    #[serde(rename = "videoToImage")]
    VideoToImage { encoding: Value, operation: Value },
    #[serde(rename = "video")]
    Video { encoding: Value, operation: Value },
    #[serde(rename = "imageToDocument")]
    ImageToDocument { operation: Value },
    #[serde(rename = "dicomToImage")]
    DicomToImage { encoding: Value, operation: Value },
    #[serde(rename = "documentToDocument")]
    DocumentToDocument { encoding: Value, operation: Value },
    #[serde(rename = "documentToImage")]
    DocumentToImage { encoding: Value, operation: Value },
    #[serde(rename = "imageToEmbedding")]
    ImageToEmbedding { operation: Value },
    #[serde(rename = "audio")]
    Audio { operation: Value },
    #[serde(rename = "documentToText")]
    DocumentToText { operation: Value },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformMediaItemRequest {
    pub transformation: Transformation,
}
