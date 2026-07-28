//! Typed parameter payloads for Datasets API errors.

use serde::{Deserialize, Serialize};
use serde_json::Value;

macro_rules! params {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name { $(pub $field: $ty,)* #[serde(flatten)] pub extra: std::collections::HashMap<String, Value> }
    };
}

params!(DatasetParameters {
    dataset_rid: String
});
params!(DatasetBranchParameters {
    dataset_rid: String,
    branch_name: String
});
params!(DatasetTransactionParameters {
    dataset_rid: String,
    transaction_rid: String
});
params!(DatasetFileParameters {
    dataset_rid: String,
    file_path: String
});
params!(ViewParameters {
    view_dataset_rid: String
});
params!(BranchAlreadyExistsParameters {
    dataset_rid: String,
    branch_name: String
});
params!(BranchNotFoundParameters {
    dataset_rid: String,
    branch_name: String
});
params!(CreateDatasetPermissionDeniedParameters {
    parent_folder_rid: String,
    name: String
});
params!(CreateTransactionPermissionDeniedParameters { dataset_rid: String, branch_name: Option<String> });
params!(DatasetViewNotFoundParameters {
    dataset_rid: String,
    branch: String
});
params!(DeleteSchemaPermissionDeniedParameters { dataset_rid: String, branch_name: String, transaction_id: Option<String> });
params!(FileAlreadyExistsParameters {
    dataset_rid: String,
    transaction_rid: String,
    path: String
});
params!(FileNotFoundOnBranchParameters {
    dataset_rid: String,
    branch_name: String,
    path: String
});
params!(FileNotFoundOnTransactionRangeParameters { dataset_rid: String, start_transaction_rid: Option<String>, end_transaction_rid: String, path: String });
params!(FileSizeLimitExceededParameters {
    dataset_rid: String,
    path: String,
    file_size_bytes: i64,
    max_file_size_bytes: i64
});
params!(InputBackingDatasetNotInOutputViewProjectParameters { view_project_rid: String, invalid_backing_datasets: Vec<String> });
params!(InvalidBranchNameParameters {
    branch_name: String
});
params!(InvalidTransactionTypeParameters {
    dataset_rid: String,
    transaction_rid: String,
    transaction_type: super::models::TransactionType
});
params!(InvalidViewPrimaryKeyColumnTypeParameters { primary_key_columns: Vec<String>, invalid_columns: Vec<String> });
params!(InvalidViewPrimaryKeyDeletionColumnParameters {
    deletion_column: String,
    deletion_column_type: Value
});
params!(NotAllColumnsInPrimaryKeyArePresentParameters { primary_key_columns: Vec<String>, missing_columns: Vec<String> });
params!(OpenTransactionAlreadyExistsParameters {
    dataset_rid: String,
    branch_name: String
});
params!(ReadTableErrorParameters {
    dataset_rid: String,
    message: String
});
params!(SchemaNotFoundParameters { dataset_rid: String, branch_name: String, transaction_rid: Option<String> });
params!(TransactionStatusParameters {
    dataset_rid: String,
    transaction_rid: String,
    transaction_status: super::models::TransactionStatus
});
params!(UploadFilePermissionDeniedParameters {
    dataset_rid: String,
    transaction_rid: String,
    path: String
});
params!(ViewNotFoundParameters {
    view_dataset_rid: String,
    branch: String
});
params!(ViewPrimaryKeyDeletionColumnNotInDatasetSchemaParameters {
    deletion_column: String
});

pub type AbortTransactionPermissionDeniedParameters = DatasetTransactionParameters;
pub type AddBackingDatasetsPermissionDeniedParameters = ViewParameters;
pub type AddPrimaryKeyPermissionDeniedParameters = ViewParameters;
pub type BuildTransactionPermissionDeniedParameters = DatasetTransactionParameters;
pub type ColumnTypesNotSupportedParameters = DatasetParameters;
pub type CommitTransactionPermissionDeniedParameters = DatasetTransactionParameters;
pub type CreateBranchPermissionDeniedParameters = DatasetBranchParameters;
pub type DeleteBranchPermissionDeniedParameters = DatasetBranchParameters;
pub type DeleteFilePermissionDeniedParameters = DatasetFileParameters;
pub type FileNotFoundParameters = DatasetFileParameters;
pub type GetBranchTransactionHistoryPermissionDeniedParameters = DatasetBranchParameters;
pub type GetDatasetHealthCheckReportsPermissionDeniedParameters = DatasetParameters;
pub type GetDatasetHealthChecksPermissionDeniedParameters = DatasetParameters;
pub type GetDatasetJobsPermissionDeniedParameters = DatasetParameters;
pub type GetDatasetSchedulesPermissionDeniedParameters = DatasetParameters;
pub type GetDatasetSchemaPermissionDeniedParameters = DatasetParameters;
pub type GetFileContentPermissionDeniedParameters = DatasetFileParameters;
pub type JobTransactionPermissionDeniedParameters = DatasetTransactionParameters;
pub type PutDatasetSchemaPermissionDeniedParameters = DatasetParameters;
pub type PutSchemaPermissionDeniedParameters = DatasetBranchParameters;
pub type ReadTableDatasetPermissionDeniedParameters = DatasetParameters;
pub type ReadTableRowLimitExceededParameters = DatasetParameters;
pub type ReadTableTimeoutParameters = DatasetParameters;
pub type RemoveBackingDatasetsPermissionDeniedParameters = ViewParameters;
pub type ReplaceBackingDatasetsPermissionDeniedParameters = ViewParameters;
pub type TransactionNotCommittedParameters = TransactionStatusParameters;
pub type TransactionNotFoundParameters = DatasetTransactionParameters;
pub type TransactionNotOpenParameters = TransactionStatusParameters;
pub type ViewDatasetCleanupFailedParameters = ViewParameters;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmptyParameters {
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

pub type CreateViewPermissionDeniedParameters = EmptyParameters;
pub type InvalidViewBackingDatasetParameters = EmptyParameters;
pub type ViewPrimaryKeyCannotBeModifiedParameters = EmptyParameters;
pub type ViewPrimaryKeyMustContainAtLeastOneColumnParameters = EmptyParameters;
pub type ViewPrimaryKeyRequiresBackingDatasetsParameters = EmptyParameters;
