//! Orchestration namespace wire types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::transport::pagination::PageToken;

pub type AbortOnFailure = bool;
pub type BranchName = String;
pub type BuildRid = String;
pub type BuildableRid = String;
pub type CreatedBy = String;
pub type CreatedTime = String;
pub type CronExpression = String;
pub type DatasetRid = String;
pub type Duration = String;
pub type FallbackBranches = Vec<BranchName>;
pub type ForceBuild = bool;
pub type JobRid = String;
pub type JobStartedTime = String;
pub type MediaSetRid = String;
pub type NotificationsEnabled = bool;
pub type ProjectRid = String;
pub type RetryBackoffDuration = Duration;
pub type RetryCount = i64;
pub type SchedulePaused = bool;
pub type ScheduleRid = String;
pub type ScheduleRunRid = String;
pub type ScheduleVersionRid = String;
pub type TableRid = String;
pub type TransactionRid = String;
pub type UpdatedBy = String;
pub type UpdatedTime = String;
pub type ZoneId = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuildStatus {
    Running,
    Succeeded,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Waiting,
    Running,
    Succeeded,
    Failed,
    Canceled,
    DidNotRun,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScheduleRunErrorName {
    Targetresolutionfailure,
    Cyclicdependency,
    Incompatibletargets,
    Permissiondenied,
    Jobspecnotfound,
    Scheduleownernotfound,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchBuildsEqualsFilterField {
    CreatedBy,
    BranchName,
    Status,
    Rid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchBuildsTimeFilterField {
    StartedTime,
    FinishedTime,
}

pub type SearchBuildsGteFilterField = SearchBuildsTimeFilterField;
pub type SearchBuildsLtFilterField = SearchBuildsTimeFilterField;
pub type SearchBuildsOrderByField = SearchBuildsTimeFilterField;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderByDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum BuildTarget {
    Upstream {
        target_rids: Vec<BuildableRid>,
        ignored_rids: Vec<BuildableRid>,
    },
    Manual {
        target_rids: Vec<BuildableRid>,
    },
    Connecting {
        input_rids: Vec<BuildableRid>,
        target_rids: Vec<BuildableRid>,
        ignored_rids: Vec<BuildableRid>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ScheduleRequestBuildTarget {
    Upstream {
        #[serde(skip_serializing_if = "Option::is_none")]
        ignored_rids: Option<Vec<BuildableRid>>,
        target_rids: Vec<BuildableRid>,
    },
    Manual {
        target_rids: Vec<BuildableRid>,
    },
    Connecting {
        #[serde(skip_serializing_if = "Option::is_none")]
        ignored_rids: Option<Vec<BuildableRid>>,
        target_rids: Vec<BuildableRid>,
        input_rids: Vec<BuildableRid>,
    },
}

pub type CreateScheduleRequestBuildTarget = ScheduleRequestBuildTarget;
pub type ReplaceScheduleRequestBuildTarget = ScheduleRequestBuildTarget;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Trigger {
    JobSucceeded {
        dataset_rid: DatasetRid,
        branch_name: BranchName,
    },
    Or {
        triggers: Vec<Trigger>,
    },
    NewLogic {
        branch_name: BranchName,
        dataset_rid: DatasetRid,
    },
    TableUpdated {
        table_rid: TableRid,
        branch_name: BranchName,
    },
    And {
        triggers: Vec<Trigger>,
    },
    DatasetUpdated {
        dataset_rid: DatasetRid,
        branch_name: BranchName,
    },
    ScheduleSucceeded {
        schedule_rid: ScheduleRid,
    },
    MediaSetUpdated {
        media_set_rid: MediaSetRid,
        branch_name: BranchName,
    },
    Time {
        cron_expression: CronExpression,
        time_zone: ZoneId,
    },
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ScopeMode {
    Project { project_rids: Vec<ProjectRid> },
    User,
}

pub type CreateScheduleRequestScopeMode = ScopeMode;
pub type ReplaceScheduleRequestScopeMode = ScopeMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub target: BuildTarget,
    pub branch_name: BranchName,
    pub fallback_branches: FallbackBranches,
    pub force_build: ForceBuild,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<RetryCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_backoff_duration: Option<RetryBackoffDuration>,
    pub abort_on_failure: AbortOnFailure,
    pub notifications_enabled: NotificationsEnabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub rid: BuildRid,
    pub branch_name: BranchName,
    pub created_time: CreatedTime,
    pub created_by: CreatedBy,
    pub fallback_branches: FallbackBranches,
    pub job_rids: Vec<JobRid>,
    pub retry_count: RetryCount,
    pub retry_backoff_duration: RetryBackoffDuration,
    pub abort_on_failure: AbortOnFailure,
    pub status: BuildStatus,
    pub finished_time: Option<String>,
    pub schedule_rid: Option<ScheduleRid>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBuildRequest {
    pub target: BuildTarget,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<BranchName>,
    pub fallback_branches: FallbackBranches,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_build: Option<ForceBuild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<RetryCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_backoff_duration: Option<RetryBackoffDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_on_failure: Option<AbortOnFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<NotificationsEnabled>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRequestAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_on_failure: Option<AbortOnFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_build: Option<ForceBuild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_backoff_duration: Option<RetryBackoffDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<RetryCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_branches: Option<FallbackBranches>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<BranchName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<NotificationsEnabled>,
    pub target: ScheduleRequestBuildTarget,
}

pub type CreateScheduleRequestAction = ScheduleRequestAction;
pub type ReplaceScheduleRequestAction = ScheduleRequestAction;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateScheduleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub action: CreateScheduleRequestAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_mode: Option<CreateScheduleRequestScopeMode>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceScheduleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub action: ReplaceScheduleRequestAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_mode: Option<ReplaceScheduleRequestScopeMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum JobOutput {
    DatasetJobOutput {
        dataset_rid: DatasetRid,
        output_transaction_rid: Option<TransactionRid>,
    },
    TransactionalMediaSetJobOutput {
        media_set_rid: MediaSetRid,
        transaction_id: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub rid: JobRid,
    pub build_rid: BuildRid,
    pub started_time: JobStartedTime,
    pub latest_attempt_start_time: Option<String>,
    pub finished_time: Option<String>,
    pub job_status: JobStatus,
    pub outputs: Vec<JobOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub rid: ScheduleRid,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub current_version_rid: ScheduleVersionRid,
    pub created_time: CreatedTime,
    pub created_by: CreatedBy,
    pub updated_time: UpdatedTime,
    pub updated_by: UpdatedBy,
    pub paused: SchedulePaused,
    pub trigger: Option<Trigger>,
    pub action: Action,
    pub scope_mode: ScopeMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleVersion {
    pub rid: ScheduleVersionRid,
    pub schedule_rid: ScheduleRid,
    pub created_time: CreatedTime,
    pub created_by: CreatedBy,
    pub trigger: Option<Trigger>,
    pub action: Action,
    pub scope_mode: ScopeMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ScheduleRunResult {
    Ignored,
    Submitted {
        build_rid: BuildRid,
    },
    Error {
        error_name: ScheduleRunErrorName,
        description: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRun {
    pub rid: ScheduleRunRid,
    pub schedule_rid: ScheduleRid,
    pub schedule_version_rid: ScheduleVersionRid,
    pub created_time: CreatedTime,
    pub created_by: Option<CreatedBy>,
    pub result: Option<ScheduleRunResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedResourcesResponse {
    pub datasets: Vec<BuildableRid>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBuildsBatchRequestElement {
    pub build_rid: BuildRid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBuildsBatchResponse {
    pub data: HashMap<BuildRid, Build>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetJobsBatchRequestElement {
    pub job_rid: JobRid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetJobsBatchResponse {
    pub data: HashMap<JobRid, Job>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSchedulesBatchRequestElement {
    pub schedule_rid: ScheduleRid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSchedulesBatchResponse {
    pub data: HashMap<ScheduleRid, Schedule>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListJobsOfBuildResponse {
    pub data: Vec<Job>,
    pub next_page_token: Option<PageToken>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRunsOfScheduleResponse {
    pub data: Vec<ScheduleRun>,
    pub next_page_token: Option<PageToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SearchBuildsFilter {
    Not {
        value: Box<SearchBuildsFilter>,
    },
    Or {
        items: Vec<SearchBuildsFilter>,
    },
    And {
        items: Vec<SearchBuildsFilter>,
    },
    Lt {
        field: SearchBuildsLtFilterField,
        value: Value,
    },
    Gte {
        field: SearchBuildsGteFilterField,
        value: Value,
    },
    Eq {
        field: SearchBuildsEqualsFilterField,
        value: Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchBuildsOrderBy {
    pub fields: Vec<SearchBuildsOrderByItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchBuildsOrderByItem {
    pub field: SearchBuildsOrderByField,
    pub direction: OrderByDirection,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchBuildsRequest {
    pub r#where: SearchBuildsFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<SearchBuildsOrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchBuildsResponse {
    pub data: Vec<Build>,
    pub next_page_token: Option<PageToken>,
}
