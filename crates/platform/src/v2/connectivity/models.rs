//! Connectivity namespace wire types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::transport::pagination::PageToken;

pub type BranchName = String;
pub type BuildRid = String;
pub type CloudIdentityRid = String;
pub type ConnectionDisplayName = String;
pub type ConnectionRid = String;
pub type DatasetRid = String;
pub type FileImportDisplayName = String;
pub type FileImportRid = String;
pub type FolderRid = String;
pub type JdbcDriverArtifactName = String;
pub type JdbcProperties = HashMap<String, String>;
pub type MarkingId = String;
pub type NetworkEgressPolicyRid = String;
pub type PlaintextValue = String;
pub type Region = String;
pub type SecretName = String;
pub type TableImportDisplayName = String;
pub type TableImportQuery = String;
pub type TableImportRid = String;
pub type TableName = String;
pub type TableRid = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FileImportMode {
    Snapshot,
    Append,
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TableImportMode {
    Snapshot,
    Append,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FileProperty {
    LastModified,
    Size,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FileFormat {
    Avro,
    Csv,
    Parquet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Protocol {
    Http,
    Https,
}

pub type UriScheme = Protocol;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SmbProxyType {
    Http,
    Socks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EncryptedProperty {
    #[serde(rename = "asSecretName")]
    AsSecretName { value: SecretName },
    #[serde(rename = "asPlaintextValue")]
    AsPlaintextValue { value: PlaintextValue },
}

pub type CreateConnectionRequestEncryptedProperty = EncryptedProperty;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicCredentials {
    pub username: String,
    pub password: EncryptedProperty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RestRequestApiKeyLocation {
    #[serde(rename = "header")]
    Header { header_name: String },
    #[serde(rename = "queryParameter")]
    QueryParameter { query_parameter_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RestAuthenticationMode {
    #[serde(rename = "bearerToken")]
    BearerToken {
        #[serde(rename = "bearerToken")]
        bearer_token: EncryptedProperty,
    },
    #[serde(rename = "apiKey")]
    ApiKey {
        location: RestRequestApiKeyLocation,
        #[serde(rename = "apiKey")]
        api_key: EncryptedProperty,
    },
    #[serde(rename = "basic")]
    Basic {
        username: String,
        password: EncryptedProperty,
    },
    #[serde(rename = "oauth2")]
    Oauth2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub scheme: Option<UriScheme>,
    pub host: String,
    pub port: Option<i32>,
    pub auth: Option<RestAuthenticationMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RestConnectionAdditionalSecrets {
    #[serde(rename = "asSecretsWithPlaintextValues")]
    AsSecretsWithPlaintextValues {
        secrets: HashMap<SecretName, PlaintextValue>,
    },
    #[serde(rename = "asSecretsNames")]
    AsSecretsNames {
        #[serde(rename = "secretNames")]
        secret_names: Vec<SecretName>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum S3AuthenticationMode {
    #[serde(rename = "awsAccessKey")]
    AwsAccessKey {
        #[serde(rename = "accessKeyId")]
        access_key_id: String,
        #[serde(rename = "secretAccessKey")]
        secret_access_key: EncryptedProperty,
    },
    #[serde(rename = "cloudIdentity")]
    CloudIdentity {
        #[serde(rename = "cloudIdentityRid")]
        cloud_identity_rid: CloudIdentityRid,
    },
    #[serde(rename = "oidc")]
    Oidc {
        audience: String,
        #[serde(rename = "issuerUrl", skip_serializing_if = "Option::is_none")]
        issuer_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<ConnectionRid>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3KmsConfiguration {
    pub kms_key: String,
    pub kms_region: Option<Region>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3ProxyConfiguration {
    pub host: String,
    pub port: i32,
    pub non_proxy_hosts: Option<Vec<String>>,
    pub protocol: Option<Protocol>,
    pub credentials: Option<BasicCredentials>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StsRoleConfiguration {
    pub role_arn: String,
    pub role_session_name: String,
    pub role_session_duration: Option<Value>,
    pub external_id: Option<String>,
    pub sts_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmbProxyConfiguration {
    pub hostname: String,
    pub port: i32,
    pub protocol: SmbProxyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SmbAuth {
    #[serde(rename = "usernamePassword")]
    UsernamePassword {
        username: String,
        password: EncryptedProperty,
        domain: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DatabricksAuthenticationMode {
    #[serde(rename = "workflowIdentityFederation")]
    WorkflowIdentityFederation {
        #[serde(rename = "servicePrincipalApplicationId")]
        service_principal_application_id: Option<String>,
        #[serde(rename = "issuerUrl", skip_serializing_if = "Option::is_none")]
        issuer_url: Option<String>,
        audience: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<ConnectionRid>,
    },
    #[serde(rename = "oauthM2M")]
    OauthMachineToMachine {
        #[serde(rename = "clientID")]
        client_id: String,
        #[serde(rename = "clientSecret")]
        client_secret: EncryptedProperty,
    },
    #[serde(rename = "personalAccessToken")]
    PersonalAccessToken {
        #[serde(rename = "personalAccessToken")]
        personal_access_token: EncryptedProperty,
    },
    #[serde(rename = "basic")]
    Basic {
        username: String,
        password: EncryptedProperty,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SnowflakeAuthenticationMode {
    #[serde(rename = "externalOauth")]
    ExternalOauth {
        #[serde(skip_serializing_if = "Option::is_none")]
        audience: Option<String>,
        #[serde(rename = "issuerUrl", skip_serializing_if = "Option::is_none")]
        issuer_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<ConnectionRid>,
    },
    #[serde(rename = "keyPair")]
    KeyPair {
        user: String,
        #[serde(rename = "privateKey")]
        private_key: EncryptedProperty,
    },
    #[serde(rename = "basic")]
    Basic {
        username: String,
        password: EncryptedProperty,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectionConfiguration {
    #[serde(rename = "s3")]
    S3(Box<S3ConnectionConfiguration>),

    #[serde(rename = "rest")]
    Rest {
        domains: Vec<Domain>,
        #[serde(rename = "additionalSecrets")]
        additional_secrets: Option<RestConnectionAdditionalSecrets>,
        #[serde(rename = "oauth2ClientRid")]
        oauth2_client_rid: Option<String>,
    },
    #[serde(rename = "snowflake")]
    Snowflake {
        #[serde(rename = "accountIdentifier")]
        account_identifier: String,
        database: Option<String>,
        role: Option<String>,
        schema: Option<String>,
        warehouse: Option<String>,
        #[serde(rename = "authenticationMode")]
        authentication_mode: SnowflakeAuthenticationMode,
        #[serde(rename = "jdbcProperties")]
        jdbc_properties: JdbcProperties,
    },
    #[serde(rename = "databricks")]
    Databricks {
        #[serde(rename = "hostName")]
        host_name: String,
        #[serde(rename = "httpPath")]
        http_path: String,
        authentication: DatabricksAuthenticationMode,
        #[serde(rename = "jdbcProperties")]
        jdbc_properties: JdbcProperties,
    },
    #[serde(rename = "smb")]
    Smb {
        hostname: String,
        port: Option<i32>,
        proxy: Option<SmbProxyConfiguration>,
        share: String,
        #[serde(rename = "baseDirectory")]
        base_directory: Option<String>,
        auth: SmbAuth,
        #[serde(rename = "requireMessageSigning")]
        require_message_signing: Option<bool>,
    },
    #[serde(rename = "jdbc")]
    Jdbc {
        url: String,
        #[serde(rename = "driverClass")]
        driver_class: String,
        #[serde(rename = "uploadedJdbcDrivers", default)]
        uploaded_jdbc_drivers: Vec<JdbcDriverArtifactName>,
        #[serde(rename = "jdbcProperties")]
        jdbc_properties: JdbcProperties,
        credentials: Option<BasicCredentials>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3ConnectionConfiguration {
    #[serde(rename = "bucketUrl")]
    pub bucket_url: String,
    #[serde(rename = "s3Endpoint")]
    pub s3_endpoint: Option<String>,
    pub region: Option<Region>,
    #[serde(rename = "authenticationMode")]
    pub authentication_mode: Option<S3AuthenticationMode>,
    #[serde(rename = "s3EndpointSigningRegion")]
    pub s3_endpoint_signing_region: Option<Region>,
    #[serde(rename = "clientKmsConfiguration")]
    pub client_kms_configuration: Option<S3KmsConfiguration>,
    #[serde(rename = "stsRoleConfiguration")]
    pub sts_role_configuration: Option<StsRoleConfiguration>,
    #[serde(rename = "proxyConfiguration")]
    pub proxy_configuration: Option<S3ProxyConfiguration>,
    #[serde(rename = "maxConnections")]
    pub max_connections: Option<i32>,
    #[serde(rename = "connectionTimeoutMillis")]
    pub connection_timeout_millis: Option<String>,
    #[serde(rename = "socketTimeoutMillis")]
    pub socket_timeout_millis: Option<String>,
    #[serde(rename = "maxErrorRetry")]
    pub max_error_retry: Option<i32>,
    #[serde(rename = "matchSubfolderExactly")]
    pub match_subfolder_exactly: Option<bool>,
    #[serde(rename = "enableRequesterPays")]
    pub enable_requester_pays: Option<bool>,
}

pub type CreateConnectionRequestConnectionConfiguration = ConnectionConfiguration;
pub type CreateConnectionRequestDatabricksAuthenticationMode = DatabricksAuthenticationMode;
pub type CreateConnectionRequestSnowflakeAuthenticationMode = SnowflakeAuthenticationMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectionWorker {
    #[serde(rename = "unknownWorker")]
    UnknownWorker,
    #[serde(rename = "foundryWorker")]
    FoundryWorker {
        #[serde(rename = "networkEgressPolicyRids")]
        network_egress_policy_rids: Vec<NetworkEgressPolicyRid>,
    },
}

pub type CreateConnectionRequestConnectionWorker = ConnectionWorker;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionExportSettings {
    pub exports_enabled: bool,
    pub export_enabled_without_markings_validation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub rid: ConnectionRid,
    pub parent_folder_rid: FolderRid,
    pub display_name: ConnectionDisplayName,
    pub export_settings: ConnectionExportSettings,
    pub worker: ConnectionWorker,
    pub configuration: ConnectionConfiguration,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConnectionRequest {
    pub parent_folder_rid: FolderRid,
    pub configuration: CreateConnectionRequestConnectionConfiguration,
    pub display_name: ConnectionDisplayName,
    pub worker: CreateConnectionRequestConnectionWorker,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExportSettingsForConnectionRequest {
    pub export_settings: ConnectionExportSettings,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateSecretsForConnectionRequest {
    pub secrets: HashMap<SecretName, PlaintextValue>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationConnectionsBatchRequestElement {
    pub connection_rid: ConnectionRid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetConfigurationConnectionsBatchResponse {
    pub data: HashMap<ConnectionRid, ConnectionConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FileImportFilter {
    #[serde(rename = "pathNotMatchesFilter")]
    PathNotMatches { regex: String },
    #[serde(rename = "anyPathMatchesFilter")]
    AnyPathMatches { regex: String },
    #[serde(rename = "filesCountLimitFilter")]
    FilesCountLimit {
        #[serde(rename = "filesCount")]
        files_count: i32,
    },
    #[serde(rename = "changedSinceLastUploadFilter")]
    ChangedSinceLastUpload {
        #[serde(rename = "fileProperties")]
        file_properties: Vec<FileProperty>,
    },
    #[serde(rename = "customFilter")]
    Custom { config: Value },
    #[serde(rename = "lastModifiedAfterFilter")]
    LastModifiedAfter {
        #[serde(rename = "afterTimestamp")]
        after_timestamp: Option<String>,
    },
    #[serde(rename = "pathMatchesFilter")]
    PathMatches { regex: String },
    #[serde(rename = "atLeastCountFilter")]
    AtLeastCount {
        #[serde(rename = "minFilesCount")]
        min_files_count: i32,
    },
    #[serde(rename = "fileSizeFilter")]
    FileSize { gt: Option<i64>, lt: Option<i64> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileImport {
    pub rid: FileImportRid,
    pub connection_rid: ConnectionRid,
    pub dataset_rid: DatasetRid,
    pub branch_name: Option<BranchName>,
    pub display_name: FileImportDisplayName,
    pub file_import_filters: Vec<FileImportFilter>,
    pub import_mode: FileImportMode,
    pub subfolder: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFileImportRequest {
    pub dataset_rid: DatasetRid,
    pub import_mode: FileImportMode,
    pub display_name: FileImportDisplayName,
    pub branch_name: Option<BranchName>,
    pub subfolder: Option<String>,
    pub file_import_filters: Vec<FileImportFilter>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceFileImportRequest {
    pub import_mode: FileImportMode,
    pub display_name: FileImportDisplayName,
    pub subfolder: Option<String>,
    pub file_import_filters: Vec<FileImportFilter>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFileImportsResponse {
    pub data: Vec<FileImport>,
    pub next_page_token: Option<PageToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TableImportInitialIncrementalState {
    #[serde(rename = "stringColumnInitialIncrementalState")]
    String {
        #[serde(rename = "columnName")]
        column_name: String,
        #[serde(rename = "currentValue")]
        current_value: String,
    },
    #[serde(rename = "dateColumnInitialIncrementalState")]
    Date {
        #[serde(rename = "columnName")]
        column_name: String,
        #[serde(rename = "currentValue")]
        current_value: String,
    },
    #[serde(rename = "integerColumnInitialIncrementalState")]
    Integer {
        #[serde(rename = "columnName")]
        column_name: String,
        #[serde(rename = "currentValue")]
        current_value: i32,
    },
    #[serde(rename = "timestampColumnInitialIncrementalState")]
    Timestamp {
        #[serde(rename = "columnName")]
        column_name: String,
        #[serde(rename = "currentValue")]
        current_value: String,
    },
    #[serde(rename = "longColumnInitialIncrementalState")]
    Long {
        #[serde(rename = "columnName")]
        column_name: String,
        #[serde(rename = "currentValue")]
        current_value: String,
    },
    #[serde(rename = "decimalColumnInitialIncrementalState")]
    Decimal {
        #[serde(rename = "columnName")]
        column_name: String,
        #[serde(rename = "currentValue")]
        current_value: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TableImportConfig {
    #[serde(rename = "databricksImportConfig")]
    Databricks {
        query: TableImportQuery,
        #[serde(rename = "initialIncrementalState")]
        initial_incremental_state: Option<TableImportInitialIncrementalState>,
    },
    #[serde(rename = "jdbcImportConfig")]
    Jdbc {
        query: TableImportQuery,
        #[serde(rename = "initialIncrementalState")]
        initial_incremental_state: Option<TableImportInitialIncrementalState>,
    },
    #[serde(rename = "microsoftSqlServerImportConfig")]
    MicrosoftSqlServer {
        query: TableImportQuery,
        #[serde(rename = "initialIncrementalState")]
        initial_incremental_state: Option<TableImportInitialIncrementalState>,
    },
    #[serde(rename = "postgreSqlImportConfig")]
    PostgreSql {
        query: TableImportQuery,
        #[serde(rename = "initialIncrementalState")]
        initial_incremental_state: Option<TableImportInitialIncrementalState>,
    },
    #[serde(rename = "microsoftAccessImportConfig")]
    MicrosoftAccess {
        query: TableImportQuery,
        #[serde(rename = "initialIncrementalState")]
        initial_incremental_state: Option<TableImportInitialIncrementalState>,
    },
    #[serde(rename = "snowflakeImportConfig")]
    Snowflake {
        query: TableImportQuery,
        #[serde(rename = "initialIncrementalState")]
        initial_incremental_state: Option<TableImportInitialIncrementalState>,
    },
    #[serde(rename = "oracleImportConfig")]
    Oracle {
        query: TableImportQuery,
        #[serde(rename = "initialIncrementalState")]
        initial_incremental_state: Option<TableImportInitialIncrementalState>,
    },
}

pub type CreateTableImportRequestTableImportConfig = TableImportConfig;
pub type ReplaceTableImportRequestTableImportConfig = TableImportConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableImport {
    pub rid: TableImportRid,
    pub connection_rid: ConnectionRid,
    pub dataset_rid: DatasetRid,
    pub branch_name: Option<BranchName>,
    pub display_name: TableImportDisplayName,
    pub import_mode: TableImportMode,
    pub allow_schema_changes: bool,
    pub config: TableImportConfig,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTableImportRequest {
    pub dataset_rid: DatasetRid,
    pub import_mode: TableImportMode,
    pub display_name: TableImportDisplayName,
    pub allow_schema_changes: Option<bool>,
    pub branch_name: Option<BranchName>,
    pub config: CreateTableImportRequestTableImportConfig,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceTableImportRequest {
    pub import_mode: TableImportMode,
    pub display_name: TableImportDisplayName,
    pub allow_schema_changes: Option<bool>,
    pub config: ReplaceTableImportRequestTableImportConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTableImportsResponse {
    pub data: Vec<TableImport>,
    pub next_page_token: Option<PageToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VirtualTableConfig {
    #[serde(rename = "snowflake")]
    Snowflake {
        database: String,
        schema: String,
        table: String,
    },
    #[serde(rename = "unity")]
    Unity {
        catalog: String,
        schema: String,
        table: String,
    },
    #[serde(rename = "glue")]
    Glue { database: String, table: String },
    #[serde(rename = "delta")]
    Delta { path: String },
    #[serde(rename = "iceberg")]
    Iceberg {
        #[serde(rename = "tableIdentifier")]
        table_identifier: String,
        #[serde(rename = "warehousePath")]
        warehouse_path: Option<String>,
    },
    #[serde(rename = "files")]
    Files { format: FileFormat, path: String },
    #[serde(rename = "bigquery")]
    BigQuery {
        project: String,
        dataset: String,
        table: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualTable {
    pub rid: TableRid,
    pub name: TableName,
    pub parent_rid: FolderRid,
    pub config: VirtualTableConfig,
    pub markings: Option<Vec<MarkingId>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVirtualTableRequest {
    pub markings: Option<Vec<MarkingId>>,
    pub parent_rid: FolderRid,
    pub name: TableName,
    pub config: VirtualTableConfig,
}
