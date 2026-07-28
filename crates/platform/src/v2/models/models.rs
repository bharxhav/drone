//! Models namespace wire types.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type Rid = String;
pub type ModelRid = Rid;
pub type ModelVersionRid = Rid;
pub type ExperimentRid = Rid;
pub type ModelStudioRid = Rid;
pub type LiveDeploymentRid = Rid;
pub type TrainerId = Rid;
pub type ModelStudioConfigVersionNumber = i32;
pub type EpochMillis = i64;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub rid: ModelRid,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelRequest {
    pub name: String,
    pub parent_folder_rid: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromoteVersionModelRequest {
    pub source_model_version_rid: ModelVersionRid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelFiles {
    #[serde(rename = "dill")]
    Dill {
        #[serde(rename = "serializedModelFunction")]
        serialized_model_function: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelApiDataType {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "array")]
    Array {
        #[serde(rename = "itemType")]
        item_type: Box<ModelApiDataType>,
    },
    #[serde(rename = "map")]
    Map {
        #[serde(rename = "keyType")]
        key_type: Box<ModelApiDataType>,
        #[serde(rename = "valueType")]
        value_type: Box<ModelApiDataType>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelApiColumn {
    pub name: String,
    #[serde(default)]
    pub required: Option<bool>,
    pub data_type: ModelApiDataType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelApiInput {
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "parameter")]
    Parameter {
        name: String,
        #[serde(default)]
        required: Option<bool>,
        #[serde(rename = "dataType")]
        data_type: ModelApiDataType,
    },
    #[serde(rename = "tabular")]
    Tabular {
        name: String,
        #[serde(default)]
        required: Option<bool>,
        columns: Vec<ModelApiColumn>,
        #[serde(default)]
        format: Option<ModelApiTabularFormat>,
    },
}
pub type ModelApiOutput = ModelApiInput;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModelApiTabularFormat {
    Pandas,
    Spark,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelApi {
    pub inputs: Vec<ModelApiInput>,
    pub outputs: Vec<ModelApiOutput>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelVersionRequest {
    pub model_files: ModelFiles,
    pub backing_repositories: Vec<Rid>,
    pub conda_requirements: Vec<String>,
    pub model_api: ModelApi,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelVersion {
    pub rid: ModelVersionRid,
    pub model_api: ModelApi,
    pub conda_requirements: Vec<String>,
    pub backing_repositories: Vec<Rid>,
    pub created_time: String,
    #[serde(default)]
    pub source: Option<ModelVersionSource>,
    #[serde(default)]
    pub linked_experiment: Option<ExperimentRid>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelVersionSource {
    #[serde(rename = "importedContainerizedModel")]
    Containerized,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "codeWorkspace")]
    CodeWorkspace {
        #[serde(rename = "codeWorkspaceRid")]
        code_workspace_rid: Rid,
        branch: String,
    },
    #[serde(rename = "modelStudio")]
    ModelStudio {
        #[serde(rename = "modelStudioRid")]
        model_studio_rid: Rid,
    },
    #[serde(rename = "codeRepository")]
    CodeRepository {
        #[serde(rename = "repositoryRid")]
        repository_rid: Rid,
        branch: String,
    },
    #[serde(rename = "sdk")]
    Sdk,
    #[serde(rename = "promoted")]
    Promoted {
        #[serde(rename = "previousModelRid")]
        previous_model_rid: ModelRid,
        #[serde(rename = "previousModelVersionRid")]
        previous_model_version_rid: ModelVersionRid,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModelVersionsResponse {
    pub data: Vec<ModelVersion>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelFunction {
    pub function_rid: Rid,
    pub function_version: String,
    pub display_name: String,
    pub api_name: String,
    pub is_row_wise: bool,
    #[serde(default)]
    pub ontology_binding: Option<Rid>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelFunctionRequest {
    pub api_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontology_binding: Option<Rid>,
    pub is_row_wise: bool,
    pub display_name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceModelFunctionRequest {
    pub api_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontology_binding: Option<Rid>,
    pub is_row_wise: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParameterValue {
    #[serde(rename = "datetime")]
    Datetime { value: String },
    #[serde(rename = "boolean")]
    Boolean { value: bool },
    #[serde(rename = "string")]
    String { value: String },
    #[serde(rename = "double")]
    Double { value: f64 },
    #[serde(rename = "integer")]
    Integer { value: i64 },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: ParameterValue,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExperimentSource {
    #[serde(rename = "codeWorkspace")]
    CodeWorkspace {
        #[serde(rename = "containerRid")]
        container_rid: Rid,
        #[serde(default, rename = "deploymentRid")]
        deployment_rid: Option<Rid>,
    },
    #[serde(rename = "authoring")]
    Authoring {
        #[serde(rename = "stemmaRid")]
        stemma_rid: Rid,
    },
    #[serde(rename = "sdk")]
    Sdk,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExperimentStatus {
    Running,
    Succeeded,
    Failed,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SeriesAggregationsValue {
    #[serde(rename = "double")]
    Double {
        #[serde(default)]
        min: Option<f64>,
        #[serde(default)]
        max: Option<f64>,
        #[serde(default)]
        last: Option<f64>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesAggregations {
    pub name: String,
    #[serde(default)]
    pub length: Option<i64>,
    pub value: SeriesAggregationsValue,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SummaryMetricAggregation {
    Min,
    Max,
    Last,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryMetric {
    pub series_name: String,
    pub aggregation: SummaryMetricAggregation,
    pub value: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExperimentArtifactDetails {
    #[serde(rename = "table")]
    Table {
        #[serde(rename = "rowCount")]
        row_count: i64,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentArtifactMetadata {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub size_bytes: i64,
    pub details: ExperimentArtifactDetails,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Experiment {
    pub rid: ExperimentRid,
    pub model_rid: ModelRid,
    pub created_time: String,
    pub created_by: String,
    pub source: ExperimentSource,
    pub status: ExperimentStatus,
    #[serde(default)]
    pub status_message: Option<String>,
    pub branch: String,
    pub parameters: Vec<Parameter>,
    pub series: Vec<SeriesAggregations>,
    pub summary_metrics: Vec<SummaryMetric>,
    pub artifacts: HashMap<String, ExperimentArtifactMetadata>,
    pub tags: Vec<String>,
    #[serde(default)]
    pub linked_model_version: Option<ModelVersionRid>,
    #[serde(default)]
    pub job_rid: Option<Rid>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoubleSeriesValueV1 {
    pub value: f64,
    pub timestamp: EpochMillis,
    pub step: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Series {
    #[serde(rename = "doubleV1")]
    DoubleV1 { series: Vec<DoubleSeriesValueV1> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchExperimentsNumericFilterOperator {
    Eq,
    Gt,
    Lt,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchExperimentsParameterFilterOperator {
    Eq,
    Gt,
    Lt,
    Contains,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SearchExperimentsFilter {
    #[serde(rename = "seriesFilter")]
    Series {
        #[serde(rename = "seriesName")]
        series_name: String,
        field: String,
        operator: SearchExperimentsNumericFilterOperator,
        value: Value,
    },
    #[serde(rename = "contains")]
    Contains { field: String, value: Value },
    #[serde(rename = "not")]
    Not { value: Box<SearchExperimentsFilter> },
    #[serde(rename = "or")]
    Or {
        filters: Vec<SearchExperimentsFilter>,
    },
    #[serde(rename = "and")]
    And {
        filters: Vec<SearchExperimentsFilter>,
    },
    #[serde(rename = "parameterFilter")]
    Parameter {
        #[serde(rename = "parameterName")]
        parameter_name: String,
        operator: SearchExperimentsParameterFilterOperator,
        value: Value,
    },
    #[serde(rename = "summaryMetricFilter")]
    SummaryMetric {
        #[serde(rename = "seriesName")]
        series_name: String,
        aggregation: SummaryMetricAggregation,
        operator: SearchExperimentsNumericFilterOperator,
        value: Value,
    },
    #[serde(rename = "eq")]
    Equals { field: String, value: Value },
    #[serde(rename = "startsWith")]
    StartsWith { field: String, value: Value },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderByDirection {
    Asc,
    Desc,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchExperimentsOrderBy {
    pub field: String,
    pub direction: OrderByDirection,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchExperimentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#where: Option<SearchExperimentsFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<SearchExperimentsOrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchExperimentsResponse {
    pub data: Vec<Experiment>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GpuType {
    A100,
    A10G,
    A16,
    H100,
    H200,
    L4,
    L40S,
    T4,
    V100,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveDeploymentGpu {
    pub count: i32,
    #[serde(default)]
    pub r#type: Option<GpuType>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveDeploymentScalingConfiguration {
    pub scale_up_load_threshold: f64,
    pub scale_up_delay: String,
    pub scale_down_delay: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveDeploymentRuntimeConfiguration {
    pub min_replicas: i32,
    pub max_replicas: i32,
    #[serde(default)]
    pub cpu: Option<f64>,
    #[serde(default)]
    pub memory: Option<String>,
    #[serde(default)]
    pub gpu: Option<LiveDeploymentGpu>,
    #[serde(default)]
    pub thread_count: Option<i32>,
    #[serde(default)]
    pub scaling_configuration: Option<LiveDeploymentScalingConfiguration>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CreateLiveDeploymentTarget {
    #[serde(rename = "direct")]
    Direct {
        #[serde(rename = "modelRid")]
        model_rid: ModelRid,
        branch: String,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLiveDeploymentRequest {
    pub deployment_type: CreateLiveDeploymentTarget,
    pub runtime_configuration: LiveDeploymentRuntimeConfiguration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceLiveDeploymentRequest {
    pub runtime_configuration: LiveDeploymentRuntimeConfiguration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveDeploymentModelVersion {
    pub model_rid: ModelRid,
    pub model_version_rid: ModelVersionRid,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LiveDeploymentState {
    Active,
    Starting,
    Degraded,
    Disabled,
    Failed,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveDeploymentStatus {
    pub state: LiveDeploymentState,
    pub is_ready: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveDeployment {
    pub rid: LiveDeploymentRid,
    pub model_version: LiveDeploymentModelVersion,
    #[serde(default)]
    pub branch: Option<String>,
    pub runtime_configuration: LiveDeploymentRuntimeConfiguration,
    pub status: LiveDeploymentStatus,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLiveDeploymentsResponse {
    pub data: Vec<LiveDeployment>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformJsonLiveDeploymentRequest {
    pub input: HashMap<String, Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformLiveDeploymentResponse {
    pub output: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelStudio {
    pub rid: ModelStudioRid,
    pub folder_rid: String,
    pub created_time: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelStudioRequest {
    pub name: String,
    pub parent_folder_rid: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfiguration {
    pub memory: String,
    pub cpu: String,
    #[serde(default)]
    pub gpu: Option<GpuType>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelStudioInput {
    #[serde(rename = "dataset")]
    Dataset {
        rid: Rid,
        #[serde(rename = "columnMapping")]
        column_mapping: HashMap<String, Vec<String>>,
        #[serde(rename = "ignoreColumns")]
        ignore_columns: Vec<String>,
        #[serde(rename = "selectColumns")]
        select_columns: Vec<String>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelStudioOutput {
    #[serde(rename = "model")]
    Model {
        #[serde(rename = "modelRid")]
        model_rid: ModelRid,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelStudioWorkerConfig {
    #[serde(default)]
    pub custom_config: Option<HashMap<String, Value>>,
    pub inputs: HashMap<String, ModelStudioInput>,
    pub outputs: HashMap<String, ModelStudioOutput>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainerVersionLocator {
    pub trainer_id: TrainerId,
    pub version: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelStudioConfigVersionRequest {
    pub name: String,
    pub resources: ResourceConfiguration,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changelog: Option<String>,
    pub worker_config: ModelStudioWorkerConfig,
    pub trainer_id: TrainerId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelStudioConfigVersion {
    pub name: String,
    pub version: i32,
    pub trainer_id: TrainerId,
    pub trainer: TrainerVersionLocator,
    pub worker_config: ModelStudioWorkerConfig,
    pub resources: ResourceConfiguration,
    #[serde(default)]
    pub changelog: Option<String>,
    pub created_by: String,
    pub created_time: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModelStudioConfigVersionsResponse {
    pub data: Vec<ModelStudioConfigVersion>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelStudioRunOutput {
    #[serde(rename = "model")]
    Model {
        #[serde(rename = "modelRid")]
        model_rid: ModelRid,
        #[serde(rename = "modelVersionRid")]
        model_version_rid: ModelVersionRid,
        #[serde(default, rename = "experimentRid")]
        experiment_rid: Option<ExperimentRid>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelStudioRun {
    pub run_id: String,
    pub build_rid: Rid,
    pub job_rid: Rid,
    pub config_version: i32,
    pub started_by: String,
    pub started_time: String,
    #[serde(default)]
    pub build_status: Option<Value>,
    pub resolved_outputs: HashMap<String, ModelStudioRunOutput>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModelStudioRunsResponse {
    pub data: Vec<ModelStudioRun>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelStudioTrainer {
    pub trainer_id: TrainerId,
    pub version: String,
    pub name: String,
    pub r#type: TrainerType,
    pub description: String,
    pub custom_config_schema: Value,
    pub inputs: Value,
    pub outputs: Value,
    pub experimental: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrainerType {
    Generic,
    TimeSeries,
    TabularRegression,
    TabularClassification,
    LlmFinetuning,
    VlmFinetuning,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelStudioTrainersResponse {
    pub data: Vec<ModelStudioTrainer>,
}
