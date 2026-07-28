//! Filesystem namespace wire types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::transport::pagination::PageToken;

pub type FolderRid = String;
pub type ProjectRid = String;
pub type SpaceRid = String;
pub type ResourceRid = String;
pub type ResourceDisplayName = String;
pub type ResourcePath = String;
pub type UsageAccountRid = String;
pub type FileSystemId = String;
pub type TagRid = String;
pub type ResourceTagDisplayName = String;
pub type SpaceMavenIdentifier = String;
pub type ProjectTemplateRid = String;
pub type ProjectTemplateVariableId = String;
pub type ProjectTemplateVariableValue = String;
pub type OrganizationRid = String;
pub type EnrollmentRid = String;
pub type MarkingId = String;
pub type RoleId = String;
pub type RoleSetId = String;
pub type PrincipalId = String;
pub type UserId = String;
pub type Rid = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrashStatus {
    DirectlyTrashed,
    AncestorTrashed,
    NotTrashed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FolderType {
    Folder,
    Space,
    Project,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrincipalType {
    User,
    Group,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectResourceReferenceType {
    External,
    Filesystem,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceType {
    AipProfile,
    AipAgentsAgent,
    AipAgentsSession,
    AipAssistFlowCapture,
    AipAssistWalkthrough,
    ArtifactsRepository,
    BellasoCipherChannel,
    BellasoCipherLicense,
    BlacksmithDocument,
    BlobsterArchive,
    BlobsterAudio,
    BlobsterBlob,
    BlobsterCode,
    BlobsterConfiguration,
    BlobsterDocument,
    BlobsterImage,
    BlobsterJupyternotebook,
    BlobsterPdf,
    BlobsterPresentation,
    BlobsterSpreadsheet,
    BlobsterVideo,
    BlobsterXml,
    CarbonWorkspace,
    CompassFolder,
    CompassWebLink,
    ContourAnalysis,
    DataHealthMonitoringView,
    DecisionsExploration,
    DreddiePipeline,
    EddieLogic,
    EddiePipeline,
    FformsForm,
    FlowWorkflow,
    FoundryDataset,
    FoundryDeployedApp,
    FoundryAcademyTutorial,
    FoundryContainerServiceContainer,
    FoundryMlObjective,
    FoundryTemplatesTemplate,
    FusionDocument,
    GeotimeCatalogIntegration,
    GpsView,
    HubbleExplorationLayout,
    HyperautoIntegration,
    LogicFlowsConnectedFlow,
    MachineryDocument,
    MagritteAgent,
    MagritteDriver,
    MagritteExport,
    MagritteSource,
    MarketplaceBlockSetInstallation,
    MarketplaceBlockSetRepo,
    MarketplaceLocal,
    MarketplaceRemoteStore,
    MioMediaSet,
    ModelsModel,
    ModelsModelVersion,
    MonocleGraph,
    NotepadNotepad,
    NotepadNotepadTemplate,
    ObjectSentinelMonitor,
    ObjectSetVersionedObjectSet,
    OpusGraph,
    OpusGraphTemplate,
    OpusMap,
    OpusMapLayer,
    OpusMapTemplate,
    OpusSearchAround,
    QuiverAnalysis,
    QuiverArtifact,
    QuiverDashboard,
    QuiverFunction,
    QuiverObjectSetPath,
    ReportReport,
    SlateDocument,
    SolutionDesignDiagram,
    StemmaRepository,
    TablesTable,
    TaurusWorkflow,
    ThirdPartyApplicationsApplication,
    TimeSeriesCatalogSync,
    VectorTemplate,
    VectorWorkbook,
    WorkshopModule,
    WorkshopState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub rid: ResourceRid,
    pub display_name: ResourceDisplayName,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub path: ResourcePath,
    #[serde(rename = "type")]
    pub resource_type: ResourceType,
    pub created_by: String,
    pub updated_by: String,
    pub created_time: String,
    pub updated_time: String,
    pub trash_status: TrashStatus,
    pub parent_folder_rid: FolderRid,
    pub project_rid: ProjectRid,
    pub space_rid: SpaceRid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub rid: FolderRid,
    pub display_name: ResourceDisplayName,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub path: ResourcePath,
    #[serde(rename = "type")]
    pub folder_type: FolderType,
    pub created_by: String,
    pub updated_by: String,
    pub created_time: String,
    pub updated_time: String,
    pub trash_status: TrashStatus,
    pub parent_folder_rid: FolderRid,
    pub project_rid: Option<ProjectRid>,
    pub space_rid: SpaceRid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub rid: ProjectRid,
    pub display_name: ResourceDisplayName,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub path: ResourcePath,
    pub created_by: String,
    pub updated_by: String,
    pub created_time: String,
    pub updated_time: String,
    pub trash_status: TrashStatus,
    pub space_rid: SpaceRid,
    pub resource_level_role_grants_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub rid: SpaceRid,
    pub display_name: ResourceDisplayName,
    pub description: Option<String>,
    pub path: ResourcePath,
    pub file_system_id: FileSystemId,
    pub usage_account_rid: UsageAccountRid,
    pub organizations: Vec<OrganizationRid>,
    pub deletion_policy_organizations: Vec<OrganizationRid>,
    pub default_role_set_id: RoleSetId,
    pub space_maven_identifier: Option<SpaceMavenIdentifier>,
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

page!(ListChildrenOfFolderResponse, Resource);
page!(ListSpacesResponse, Space);
page!(ListMarkingsOfResourceResponse, MarkingId);
page!(ListOrganizationsOfProjectResponse, OrganizationRid);
page!(
    ListProjectResourceReferencesResponse,
    ProjectResourceReference
);
page!(ListResourceRolesResponse, ResourceRole);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFoldersBatchResponse {
    pub data: HashMap<FolderRid, Folder>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResourcesBatchResponse {
    pub data: HashMap<ResourceRid, Resource>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetByPathResourcesBatchResponse {
    pub data: Vec<Resource>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourceTagsResponse {
    pub data: Vec<ResourceTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessRequirements {
    pub organizations: Vec<Organization>,
    pub markings: Vec<Marking>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marking {
    pub marking_id: MarkingId,
    pub is_directly_applied: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub marking_id: MarkingId,
    pub organization_rid: OrganizationRid,
    pub is_directly_applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AddResourceReferenceRequest {
    #[serde(rename = "external")]
    External {
        #[serde(rename = "resourceRid")]
        resource_rid: Rid,
        #[serde(rename = "importName")]
        import_name: String,
    },
    #[serde(rename = "filesystem")]
    Filesystem {
        #[serde(rename = "resourceRid")]
        resource_rid: ResourceRid,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ProjectResourceReferenceUnion {
    #[serde(rename = "external")]
    External {
        #[serde(rename = "resourceRid")]
        resource_rid: Rid,
        name: String,
        #[serde(rename = "importedAt")]
        imported_at: String,
        #[serde(rename = "importedBy")]
        imported_by: UserId,
    },
    #[serde(rename = "filesystem")]
    Filesystem {
        #[serde(rename = "resourceRid")]
        resource_rid: ResourceRid,
        name: String,
        #[serde(rename = "importedAt")]
        imported_at: String,
        #[serde(rename = "importedBy")]
        imported_by: UserId,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectResourceReference {
    pub reference: ProjectResourceReferenceUnion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResourceRolePrincipal {
    #[serde(rename = "principalWithId")]
    PrincipalWithId {
        #[serde(rename = "principalId")]
        principal_id: PrincipalId,
        #[serde(rename = "principalType")]
        principal_type: PrincipalType,
    },
    #[serde(rename = "everyone")]
    Everyone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResourceRolePrincipalIdentifier {
    #[serde(rename = "principalIdOnly")]
    PrincipalIdOnly {
        #[serde(rename = "principalId")]
        principal_id: PrincipalId,
    },
    #[serde(rename = "everyone")]
    Everyone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrincipalWithId {
    pub principal_id: PrincipalId,
    pub principal_type: PrincipalType,
    #[serde(rename = "type")]
    pub kind: PrincipalWithIdType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrincipalWithIdType {
    #[serde(rename = "principalWithId")]
    PrincipalWithId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRole {
    pub resource_role_principal: ResourceRolePrincipal,
    pub role_id: RoleId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRoleIdentifier {
    pub resource_role_principal: ResourceRolePrincipalIdentifier,
    pub role_id: RoleId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTag {
    pub tag_rid: TagRid,
    pub display_name: ResourceTagDisplayName,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub parent_folder_rid: FolderRid,
    pub display_name: ResourceDisplayName,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceFolderRequest {
    pub parent_folder_rid: FolderRid,
    pub display_name: ResourceDisplayName,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFoldersBatchRequestElement {
    pub folder_rid: FolderRid,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResourcesBatchRequestElement {
    pub resource_rid: ResourceRid,
}
#[derive(Debug, Clone, Serialize)]
pub struct GetByPathResourcesBatchRequestElement {
    pub path: ResourcePath,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarkingsRequest {
    pub marking_ids: Vec<MarkingId>,
}
pub type RemoveMarkingsRequest = AddMarkingsRequest;
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOrganizationsRequest {
    pub organization_rids: Vec<OrganizationRid>,
}
pub type RemoveOrganizationsRequest = AddOrganizationsRequest;
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddResourceRolesRequest {
    pub roles: Vec<ResourceRoleIdentifier>,
}
pub type RemoveResourceRolesRequest = AddResourceRolesRequest;
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddResourceTagsRequest {
    pub tag_rids: Vec<TagRid>,
}
pub type RemoveResourceTagsRequest = AddResourceTagsRequest;
#[derive(Debug, Clone, Serialize)]
pub struct AddProjectResourceReferencesRequest {
    pub resources: Vec<AddResourceReferenceRequest>,
}
#[derive(Debug, Clone, Serialize)]
pub struct RemoveProjectResourceReferencesRequest {
    pub resources: Vec<Rid>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    pub display_name: ResourceDisplayName,
    pub description: Option<String>,
    pub space_rid: SpaceRid,
    pub role_grants: HashMap<RoleId, Vec<PrincipalWithId>>,
    pub default_roles: Vec<RoleId>,
    pub organization_rids: Vec<OrganizationRid>,
    pub resource_level_role_grants_allowed: Option<bool>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectFromTemplateRequest {
    pub template_rid: ProjectTemplateRid,
    pub variable_values: HashMap<ProjectTemplateVariableId, ProjectTemplateVariableValue>,
    pub default_roles: Option<Vec<RoleId>>,
    pub organization_rids: Option<Vec<OrganizationRid>>,
    pub project_description: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceProjectRequest {
    pub display_name: ResourceDisplayName,
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSpaceRequest {
    pub enrollment_rid: EnrollmentRid,
    pub usage_account_rid: Option<UsageAccountRid>,
    pub file_system_id: Option<FileSystemId>,
    pub display_name: ResourceDisplayName,
    pub organizations: Vec<OrganizationRid>,
    pub description: Option<String>,
    pub deletion_policy_organizations: Vec<OrganizationRid>,
    pub default_role_set_id: Option<RoleSetId>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceSpaceRequest {
    pub usage_account_rid: Option<UsageAccountRid>,
    pub display_name: ResourceDisplayName,
    pub description: Option<String>,
    pub default_role_set_id: Option<RoleSetId>,
}
