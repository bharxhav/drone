//! Datasets namespace wire types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::transport::pagination::PageToken;

pub type BranchName = String;
pub type BuildRid = String;
pub type CheckRid = String;
pub type DatasetName = String;
pub type DatasetRid = String;
pub type DatasetSchema = serde_json::Value;
pub type FilePath = String;
pub type FileUpdatedTime = String;
pub type JobRid = String;
pub type MarkingId = String;
pub type ScheduleRid = String;
pub type TransactionCreatedTime = String;
pub type TransactionRid = String;
pub type VersionId = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataframeReader {
    Avro,
    Csv,
    Parquet,
    Datasource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TableExportFormat {
    Arrow,
    Csv,
}

impl TableExportFormat {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Arrow => "ARROW",
            Self::Csv => "CSV",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Aborted,
    Committed,
    Open,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Append,
    Update,
    Snapshot,
    Delete,
}

impl TransactionType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Append => "APPEND",
            Self::Update => "UPDATE",
            Self::Snapshot => "SNAPSHOT",
            Self::Delete => "DELETE",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetDatasetJobsComparisonType {
    Gte,
    Lt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetDatasetJobsSortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetDatasetJobsSortType {
    ByStartedTime,
    ByFinishedTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetDatasetJobsTimeFilterField {
    SubmittedTime,
    FinishedTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    pub rid: DatasetRid,
    pub name: DatasetName,
    pub parent_folder_rid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Branch {
    pub name: BranchName,
    pub transaction_rid: Option<TransactionRid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub path: FilePath,
    pub transaction_rid: TransactionRid,
    pub size_bytes: Option<i64>,
    pub updated_time: FileUpdatedTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub rid: TransactionRid,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub created_time: TransactionCreatedTime,
    pub closed_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub view_name: DatasetName,
    pub dataset_rid: DatasetRid,
    pub parent_folder_rid: String,
    pub branch: Option<BranchName>,
    pub backing_datasets: Vec<ViewBackingDataset>,
    pub primary_key: Option<ViewPrimaryKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewBackingDataset {
    pub branch: Option<BranchName>,
    pub dataset_rid: DatasetRid,
    pub stop_propagating_marking_ids: Vec<MarkingId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewPrimaryKey {
    pub columns: Vec<String>,
    pub resolution: ViewPrimaryKeyResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ViewPrimaryKeyResolution {
    #[serde(rename = "unique")]
    Unique(PrimaryKeyResolutionUnique),
    #[serde(rename = "duplicate")]
    Duplicate(PrimaryKeyResolutionDuplicate),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrimaryKeyResolutionUnique {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryKeyResolutionDuplicate {
    pub deletion_column: Option<String>,
    pub resolution_strategy: PrimaryKeyResolutionStrategy,
}

pub type PrimaryKeyResolutionStrategy = PrimaryKeyLatestWinsResolutionStrategy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryKeyLatestWinsResolutionStrategy {
    pub columns: Vec<String>,
    #[serde(rename = "type")]
    pub strategy_type: LatestWinsType,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum LatestWinsType {
    #[default]
    #[serde(rename = "latestWins")]
    LatestWins,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GetDatasetJobsQuery {
    #[serde(rename = "or")]
    Or(GetDatasetJobsOrFilter),
    #[serde(rename = "and")]
    And(GetDatasetJobsAndFilter),
    #[serde(rename = "timeFilter")]
    TimeFilter(GetDatasetJobsTimeFilter),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDatasetJobsOrFilter {
    pub items: Vec<GetDatasetJobsQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDatasetJobsAndFilter {
    pub items: Vec<GetDatasetJobsQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDatasetJobsTimeFilter {
    pub field: GetDatasetJobsTimeFilterField,
    pub comparison_type: GetDatasetJobsComparisonType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDatasetJobsSort {
    pub sort_type: GetDatasetJobsSortType,
    pub sort_direction: GetDatasetJobsSortDirection,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatasetRequest {
    pub parent_folder_rid: String,
    pub name: DatasetName,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBranchRequest {
    pub transaction_rid: Option<TransactionRid>,
    pub name: BranchName,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRequest {
    pub transaction_type: TransactionType,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutDatasetSchemaRequest {
    pub branch_name: Option<BranchName>,
    pub dataframe_reader: Option<DataframeReader>,
    pub end_transaction_rid: Option<TransactionRid>,
    #[serde(rename = "schema")]
    pub schema_: DatasetSchema,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDatasetJobsRequest {
    pub r#where: Option<GetDatasetJobsQuery>,
    pub order_by: Vec<GetDatasetJobsSort>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSchemaDatasetsBatchRequestElement {
    pub end_transaction_rid: Option<TransactionRid>,
    pub dataset_rid: DatasetRid,
    pub version_id: Option<VersionId>,
    pub branch_name: Option<BranchName>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateViewRequest {
    pub parent_folder_rid: String,
    pub view_name: DatasetName,
    pub backing_datasets: Vec<ViewBackingDataset>,
    pub branch: Option<BranchName>,
    pub primary_key: Option<ViewPrimaryKey>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddBackingDatasetsRequest {
    pub branch: Option<BranchName>,
    pub backing_datasets: Vec<ViewBackingDataset>,
}

pub type RemoveBackingDatasetsRequest = AddBackingDatasetsRequest;
pub type ReplaceBackingDatasetsRequest = AddBackingDatasetsRequest;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddPrimaryKeyRequest {
    pub branch: Option<BranchName>,
    pub primary_key: ViewPrimaryKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDatasetSchemaResponse {
    pub branch_name: BranchName,
    pub end_transaction_rid: TransactionRid,
    #[serde(rename = "schema")]
    pub schema_: DatasetSchema,
    pub version_id: VersionId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetHealthCheckReportsResponse {
    pub data: HashMap<CheckRid, Option<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobDetails {
    pub job_rid: JobRid,
}

macro_rules! page {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub data: Vec<$item>,
            pub next_page_token: Option<PageToken>,
        }
    };
}

page!(GetJobResponse, JobDetails);
page!(ListBranchesResponse, Branch);
page!(ListFilesResponse, File);
page!(ListSchedulesResponse, ScheduleRid);
page!(ListTransactionsOfDatasetResponse, Transaction);
page!(ListTransactionsResponse, Transaction);

#[derive(Debug, Clone, Deserialize)]
pub struct ListHealthChecksResponse {
    pub data: Vec<CheckRid>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSchemaDatasetsBatchResponse {
    pub data: HashMap<DatasetRid, GetDatasetSchemaResponse>,
}
