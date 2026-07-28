//! AIP Agents namespace wire types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type AgentRid = String;
pub type AgentVersionString = String;
pub type AgentMarkdownResponse = String;
pub type SessionRid = String;
pub type SessionTraceId = String;
pub type MessageId = String;
pub type ParameterId = String;
pub type ToolInputName = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub rid: AgentRid,
    pub version: AgentVersionString,
    pub metadata: AgentMetadata,
    pub parameters: HashMap<ParameterId, Parameter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentMetadata {
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_placeholder: Option<String>,
    pub suggested_prompts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentVersion {
    pub string: AgentVersionString,
    pub version: AgentVersionDetails,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentVersionDetails {
    pub major: i64,
    pub minor: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub rid: SessionRid,
    pub metadata: SessionMetadata,
    pub agent_rid: AgentRid,
    pub agent_version: AgentVersionString,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionMetadata {
    pub title: String,
    pub created_time: String,
    pub updated_time: String,
    pub message_count: i64,
    pub estimated_expires_time: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentsSessionsPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    pub data: Vec<Session>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSessionsResponse {
    pub data: Vec<Session>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAgentVersionsResponse {
    pub data: Vec<AgentVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<AgentVersionString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserTextInput {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockingContinueSessionRequest {
    pub user_input: UserTextInput,
    pub parameter_inputs: HashMap<ParameterId, ParameterValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts_override: Option<Vec<InputContext>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_trace_id: Option<SessionTraceId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingContinueSessionRequest {
    pub user_input: UserTextInput,
    pub parameter_inputs: HashMap<ParameterId, ParameterValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts_override: Option<Vec<InputContext>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_trace_id: Option<SessionTraceId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelSessionRequest {
    pub message_id: MessageId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<AgentMarkdownResponse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelSessionResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<SessionExchangeResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRagContextForSessionRequest {
    pub user_input: UserTextInput,
    pub parameter_inputs: HashMap<ParameterId, ParameterValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateSessionTitleRequest {
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content {
    pub exchanges: Vec<SessionExchange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionExchange {
    pub user_input: UserTextInput,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<SessionExchangeContexts>,
    pub result: SessionExchangeResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionExchangeContexts {
    pub object_contexts: Vec<ObjectContext>,
    pub function_retrieved_contexts: Vec<FunctionRetrievedContext>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionExchangeResult {
    pub agent_markdown_response: AgentMarkdownResponse,
    pub parameter_updates: HashMap<ParameterId, ParameterValueUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tokens_used: Option<i64>,
    pub interrupted_output: bool,
    pub session_trace_id: SessionTraceId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSessionRagContextResponse {
    pub object_contexts: Vec<ObjectContext>,
    pub function_retrieved_contexts: Vec<FunctionRetrievedContext>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputContext {
    #[serde(rename = "functionRetrievedContext")]
    FunctionRetrievedContext {
        #[serde(rename = "functionRid")]
        function_rid: String,
        #[serde(rename = "functionVersion")]
        function_version: String,
        #[serde(rename = "retrievedPrompt")]
        retrieved_prompt: String,
    },
    #[serde(rename = "objectContext")]
    ObjectContext {
        #[serde(rename = "objectRids")]
        object_rids: Vec<String>,
        #[serde(rename = "propertyTypeRids")]
        property_type_rids: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionRetrievedContext {
    pub function_rid: String,
    pub function_version: String,
    pub retrieved_prompt: String,
    #[serde(
        rename = "type",
        default = "function_context_type",
        skip_serializing_if = "is_function_context_type"
    )]
    pub context_type: String,
}
fn function_context_type() -> String {
    "functionRetrievedContext".into()
}
fn is_function_context_type(value: &str) -> bool {
    value == "functionRetrievedContext"
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectContext {
    pub object_rids: Vec<String>,
    pub property_type_rids: Vec<String>,
    #[serde(
        rename = "type",
        default = "object_context_type",
        skip_serializing_if = "is_object_context_type"
    )]
    pub context_type: String,
}
fn object_context_type() -> String {
    "objectContext".into()
}
fn is_object_context_type(value: &str) -> bool {
    value == "objectContext"
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "parameterType")]
    pub parameter_type: ParameterType,
    pub access: ParameterAccessMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterAccessMode {
    #[serde(rename = "READ_ONLY")]
    ReadOnly,
    #[serde(rename = "READ_WRITE")]
    ReadWrite,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParameterType {
    #[serde(rename = "string")]
    String {
        #[serde(
            rename = "defaultValue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        default_value: Option<String>,
    },
    #[serde(rename = "objectSet")]
    ObjectSet {
        #[serde(rename = "expectedObjectTypes")]
        expected_object_types: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParameterValue {
    #[serde(rename = "string")]
    String { value: String },
    #[serde(rename = "objectSet")]
    ObjectSet {
        #[serde(rename = "objectSet")]
        object_set: Value,
        ontology: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParameterValueUpdate {
    #[serde(rename = "string")]
    String { value: String },
    #[serde(rename = "objectSet")]
    ObjectSet { value: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionTrace {
    pub id: SessionTraceId,
    pub status: SessionTraceStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<SessionExchangeContexts>,
    pub tool_call_groups: Vec<ToolCallGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionTraceStatus {
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "COMPLETE")]
    Complete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallGroup {
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCall {
    pub tool_metadata: ToolMetadata,
    pub input: ToolCallInput,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<ToolCallOutput>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCallInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thought: Option<String>,
    pub inputs: HashMap<ToolInputName, ToolInputValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    #[serde(rename = "type")]
    pub tool_type: ToolType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ToolType {
    Function,
    Action,
    OntologySemanticSearch,
    ObjectQuery,
    UpdateApplicationVariable,
    RequestClarification,
    ObjectQueryWithSql,
    CodeExecution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolInputValue {
    #[serde(rename = "string")]
    String { value: String },
    #[serde(rename = "rid")]
    Rid { rid: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolOutputValue {
    #[serde(rename = "string")]
    String { value: String },
    #[serde(rename = "rid")]
    Rid { rid: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolCallOutput {
    #[serde(rename = "success")]
    Success { output: ToolOutputValue },
    #[serde(rename = "failure")]
    Failure {
        #[serde(rename = "correctionMessage")]
        correction_message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelPurpose {
    #[serde(rename = "PRIMARY_AGENT")]
    PrimaryAgent,
    #[serde(rename = "QUESTION_SUGGESTER")]
    QuestionSuggester,
}
