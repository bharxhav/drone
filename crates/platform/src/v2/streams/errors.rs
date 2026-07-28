//! Typed parameter payloads for Streams API errors.

use serde::{Deserialize, Serialize};
use serde_json::Value;

macro_rules! params { ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => { #[derive(Debug, Clone, Serialize, Deserialize)] #[serde(rename_all = "camelCase")] pub struct $name { $(pub $field: $ty,)* #[serde(flatten)] pub extra: std::collections::HashMap<String, Value> } }; }
params!(DatasetParameters {
    dataset_rid: String
});
params!(StreamParameters {
    dataset_rid: String,
    stream_branch_name: String
});
params!(SubscriberParameters {
    dataset_rid: String,
    subscriber_subscriber_id: String,
    stream_branch_name: String
});
params!(InvalidStreamNoSchemaParameters { dataset_rid: String, branch_name: String, view_rid: Option<String> });
params!(InvalidStreamTypeParameters {
    stream_type: String
});
params!(RecordDoesNotMatchStreamSchemaParameters { dataset_rid: String, branch_name: String, view_rid: Option<String> });
params!(SubscriberAlreadyExistsParameters {
    subscriber_id: String,
    existing_dataset_rid: String,
    existing_branch_name: String
});
params!(SubscriberNotFoundParameters {
    subscriber_id: String
});
params!(ViewNotFoundParameters { view_rid: String });
params!(CannotCreateStreamingDatasetInUserFolderParameters {
    parent_folder_rid: String
});
pub type CannotWriteToTrashedStreamParameters = DatasetParameters;
pub type CommitSubscriberOffsetsPermissionDeniedParameters = SubscriberParameters;
pub type CreateStreamPermissionDeniedParameters = StreamParameters;
pub type CreateSubscriberPermissionDeniedParameters = SubscriberParameters;
pub type DeleteSubscriberPermissionDeniedParameters = SubscriberParameters;
pub type GetEndOffsetsForStreamPermissionDeniedParameters = StreamParameters;
pub type GetRecordsFromStreamPermissionDeniedParameters = StreamParameters;
pub type GetSubscriberReadPositionPermissionDeniedParameters = SubscriberParameters;
pub type PublishBinaryRecordToStreamPermissionDeniedParameters = StreamParameters;
pub type PublishRecordToStreamPermissionDeniedParameters = StreamParameters;
pub type PublishRecordsToStreamPermissionDeniedParameters = StreamParameters;
pub type ReadRecordsFromSubscriberPermissionDeniedParameters = SubscriberParameters;
pub type ResetStreamPermissionDeniedParameters = StreamParameters;
pub type ResetSubscriberOffsetsPermissionDeniedParameters = SubscriberParameters;
pub type StreamNotFoundParameters = StreamParameters;
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmptyParameters {
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
pub type CreateStreamingDatasetPermissionDeniedParameters = EmptyParameters;
pub type FailedToProcessBinaryRecordParameters = EmptyParameters;
pub type RecordTooLargeParameters = EmptyParameters;
