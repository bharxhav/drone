//! Checkpoints namespace wire types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ApprovalsSubtaskId = String;
pub type ApprovalsTaskId = String;
pub type CheckpointType = String;
pub type ConfigRid = String;
pub type InteractionRid = String;
pub type NamespaceRid = String;
pub type OrganizationRid = String;
pub type ProjectRid = String;
pub type RecordCreatedAt = String;
pub type RecordRid = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Scope {
    UserScoped,
    ResourceScoped,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortDirection {
    Asc,
    Desc,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RedactionType {
    UserRedacted,
    ResourceRedacted,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JustificationMatchType {
    Exact,
    Contains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedactableString {
    pub value: Option<String>,
    pub redaction_type: Option<RedactionType>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActingUser {
    pub user_id: String,
    pub username: RedactableString,
    pub organization_rid: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalsMetadata {
    pub approvals_task_id: String,
    pub approvals_subtask_ids: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropdownSelection {
    pub selected_option: String,
    pub additional_response: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Justification {
    #[serde(rename = "responseJustification")]
    Response {
        response: String,
        prompt: String,
        description: Option<String>,
        title: String,
    },
    #[serde(rename = "dropdownJustification")]
    Dropdown {
        #[serde(rename = "selectedOptions")]
        selected_options: Vec<DropdownSelection>,
        prompt: String,
        description: Option<String>,
        title: String,
    },
    #[serde(rename = "reauthenticationJustification")]
    Reauthentication {
        #[serde(rename = "reauthenticationId")]
        reauthentication_id: String,
        prompt: String,
        description: Option<String>,
        title: String,
    },
    #[serde(rename = "acknowledgementJustification")]
    Acknowledgement {
        prompt: String,
        description: Option<String>,
        title: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckpointedOntology {
    pub ontology_rid: String,
    pub ontology_version: String,
    pub namespace_rid: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckpointedOntologyWithObjectTypes {
    pub ontology: CheckpointedOntology,
    pub object_type_rids: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckpointedVersionedObjectSet {
    pub versioned_object_set_rid: String,
    pub object_set_version: String,
    pub object_types: Vec<CheckpointedOntologyWithObjectTypes>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckpointedObjectSetTypesProxy {
    pub object_types: Vec<CheckpointedOntologyWithObjectTypes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CheckpointedItem {
    #[serde(rename = "checkpointedIssue")]
    Issue {
        #[serde(rename = "issueRid")]
        issue_rid: String,
    },
    #[serde(rename = "checkpointedJob")]
    Job {
        #[serde(rename = "jobRid")]
        job_rid: String,
    },
    #[serde(rename = "checkpointedSchedule")]
    Schedule {
        #[serde(rename = "scheduleRid")]
        schedule_rid: String,
    },
    #[serde(rename = "checkpointedResource")]
    Resource {
        rid: String,
        #[serde(rename = "resourceType")]
        resource_type: String,
        name: Option<RedactableString>,
        #[serde(rename = "projectRid")]
        project_rid: Option<String>,
        #[serde(rename = "namespaceRid")]
        namespace_rid: Option<String>,
        #[serde(rename = "compassPath")]
        compass_path: RedactableString,
        #[serde(rename = "orgMarkings")]
        org_markings: Vec<String>,
    },
    #[serde(rename = "checkpointedJobSpecification")]
    JobSpecification {
        #[serde(rename = "jobSpecRid")]
        job_spec_rid: String,
    },
    #[serde(rename = "checkpointedLanguageModel")]
    LanguageModel {
        #[serde(rename = "modelRid")]
        model_rid: String,
    },
    #[serde(rename = "checkpointedGroup")]
    Group {
        #[serde(rename = "groupId")]
        group_id: String,
    },
    #[serde(rename = "checkpointedUserIntakeSubmission")]
    UserIntakeSubmission {
        #[serde(rename = "submissionRid")]
        submission_rid: String,
    },
    #[serde(rename = "checkpointedObjectSet")]
    ObjectSet {
        versioned: Option<CheckpointedVersionedObjectSet>,
        #[serde(rename = "typesProxy")]
        types_proxy: Option<CheckpointedObjectSetTypesProxy>,
    },
    #[serde(rename = "checkpointedMarking")]
    Marking {
        #[serde(rename = "markingId")]
        marking_id: String,
    },
    #[serde(rename = "checkpointedMarketplaceProduct")]
    MarketplaceProduct {
        #[serde(rename = "productId")]
        product_id: String,
    },
    #[serde(rename = "checkpointedPeeringJob")]
    PeeringJob {
        #[serde(rename = "jobId")]
        job_id: String,
        #[serde(rename = "relationshipRid")]
        relationship_rid: String,
    },
    #[serde(rename = "checkpointedRole")]
    Role {
        #[serde(rename = "roleId")]
        role_id: String,
    },
    #[serde(rename = "checkpointedIntervention")]
    Intervention {
        #[serde(rename = "interventionRid")]
        intervention_rid: String,
    },
    #[serde(rename = "checkpointedLanguageModelSession")]
    LanguageModelSession {
        #[serde(rename = "sessionRid")]
        session_rid: String,
    },
    #[serde(rename = "checkpointedToken")]
    Token {
        #[serde(rename = "tokenId")]
        token_id: String,
        #[serde(rename = "tokenType")]
        token_type: String,
    },
    #[serde(rename = "checkpointedUserIntakeFormInput")]
    UserIntakeFormInput {
        #[serde(rename = "inputId")]
        input_id: String,
    },
    #[serde(rename = "checkpointedPrincipal")]
    Principal {
        id: String,
        username: RedactableString,
        #[serde(rename = "organizationRid")]
        organization_rid: Option<String>,
        role: String,
    },
    #[serde(rename = "checkpointedActionType")]
    ActionType {
        #[serde(rename = "actionTypeRid")]
        action_type_rid: String,
        ontology: CheckpointedOntology,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub rid: String,
    pub config_rid: Option<String>,
    pub r#type: String,
    pub scope: Scope,
    pub acting_user: ActingUser,
    pub delegate_user_id: Option<String>,
    pub created_at: String,
    pub checkpointed_items: Vec<CheckpointedItem>,
    pub justification: Justification,
    pub project_rid: Option<String>,
    pub organization_rid: Option<String>,
    pub namespace_rid: Option<String>,
    pub interaction_rid: Option<String>,
    pub approvals_metadata: Option<ApprovalsMetadata>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordsBatchRequestElement {
    pub record_rid: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecordsBatchResponse {
    pub data: HashMap<String, Record>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CheckpointedItemId {
    #[serde(rename = "checkpointedJobRid")]
    JobRid { rid: String },
    #[serde(rename = "checkpointedMarkingId")]
    MarkingId { id: String },
    #[serde(rename = "checkpointedTokenId")]
    TokenId { id: String },
    #[serde(rename = "checkpointedGroupId")]
    GroupId { id: String },
    #[serde(rename = "checkpointedObjectSetVersionedRid")]
    ObjectSetVersionedRid { rid: String },
    #[serde(rename = "checkpointedObjectSetTypesProxyRids")]
    ObjectSetTypesProxyRids { rids: Vec<String> },
    #[serde(rename = "checkpointedResourceRid")]
    ResourceRid { rid: String },
    #[serde(rename = "checkpointedPeeringJobId")]
    PeeringJobId {
        id: String,
        #[serde(rename = "relationshipRid")]
        relationship_rid: String,
    },
    #[serde(rename = "checkpointedIssueRid")]
    IssueRid { rid: String },
    #[serde(rename = "checkpointedInterventionRid")]
    InterventionRid { rid: String },
    #[serde(rename = "checkpointedJobSpecRid")]
    JobSpecRid { rid: String },
    #[serde(rename = "checkpointedActionTypeRid")]
    ActionTypeRid { rid: String },
    #[serde(rename = "checkpointedScheduleRid")]
    ScheduleRid { rid: String },
    #[serde(rename = "checkpointedRoleId")]
    RoleId { id: String },
    #[serde(rename = "checkpointedUserIntakeFormInputId")]
    UserIntakeFormInputId { id: String },
    #[serde(rename = "checkpointedMarketplaceProductId")]
    MarketplaceProductId { id: String },
    #[serde(rename = "checkpointedLanguageModelRid")]
    LanguageModelRid { rid: String },
    #[serde(rename = "checkpointedPrincipalId")]
    PrincipalId { id: String },
    #[serde(rename = "checkpointedLanguageModelSessionRid")]
    LanguageModelSessionRid { rid: String },
    #[serde(rename = "checkpointedUserIntakeSubmissionRid")]
    UserIntakeSubmissionRid { rid: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SearchCheckpointRecordsFilter {
    #[serde(rename = "not")]
    Not {
        filter: Box<SearchCheckpointRecordsFilter>,
    },
    #[serde(rename = "or")]
    Or {
        filters: Vec<SearchCheckpointRecordsFilter>,
    },
    #[serde(rename = "and")]
    And {
        filters: Vec<SearchCheckpointRecordsFilter>,
    },
    #[serde(rename = "textSearch")]
    TextSearch {
        field: String,
        query: String,
        #[serde(rename = "matchType")]
        match_type: JustificationMatchType,
    },
    #[serde(rename = "lt")]
    Lt { field: String, value: String },
    #[serde(rename = "gte")]
    Gte { field: String, value: String },
    #[serde(rename = "eq")]
    Equals { field: String, value: String },
    #[serde(rename = "checkpointedItemId")]
    CheckpointedItemId {
        #[serde(rename = "checkpointedItemId")]
        checkpointed_item_id: CheckpointedItemId,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCheckpointRecordsRequest {
    pub filter: SearchCheckpointRecordsFilter,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRecordsRequest {
    pub r#where: SearchCheckpointRecordsRequest,
    pub page_token: Option<String>,
    pub page_size: Option<u32>,
    pub sort_direction: Option<SortDirection>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchCheckpointRecordsResponse {
    pub data: Vec<Record>,
    pub next_page_token: Option<String>,
}
