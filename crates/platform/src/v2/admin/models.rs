//! Admin namespace wire types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PrincipalId = String;
pub type GroupId = String;
pub type UserId = String;
pub type MarkingId = String;
pub type RoleId = String;
pub type EnrollmentRid = String;
pub type OrganizationRid = String;
pub type AuthenticationProviderRid = String;
pub type MarkingCategoryId = String;
pub type Attributes = HashMap<String, Vec<String>>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrincipalType {
    User,
    Group,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStatus {
    Active,
    Deleted,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarkingCategoryRole {
    Administer,
    View,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarkingCategoryType {
    Conjunctive,
    Disjunctive,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarkingRole {
    Administer,
    Declassify,
    Use,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarkingType {
    Mandatory,
    Cbac,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CertificateUsageType {
    Encryption,
    Signing,
    Unspecified,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClassificationBannerDisplayType {
    BannerLine,
    PortionMarking,
    #[serde(other)]
    Unknown,
}

impl ClassificationBannerDisplayType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::BannerLine => "BANNER_LINE",
            Self::PortionMarking => "PORTION_MARKING",
            Self::Unknown => "UNKNOWN",
        }
    }
}
impl UserStatus {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Deleted => "DELETED",
            Self::Unknown => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AuthenticationProtocol {
    Saml {
        #[serde(rename = "serviceProviderMetadata")]
        service_provider_metadata: SamlServiceProviderMetadata,
    },
    Oidc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateInfo {
    pub pem_certificate: String,
    pub common_name: Option<String>,
    pub expiry_date: String,
    pub usage_type: CertificateUsageType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SamlServiceProviderMetadata {
    pub entity_id: String,
    pub metadata_url: String,
    pub acs_urls: Vec<String>,
    pub logout_urls: Vec<String>,
    pub certificates: Vec<CertificateInfo>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationProvider {
    pub rid: AuthenticationProviderRid,
    pub name: String,
    pub realm: String,
    pub enabled: bool,
    pub supported_hosts: Vec<String>,
    pub supported_username_patterns: Vec<String>,
    pub protocol: AuthenticationProtocol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CbacBanner {
    pub classification_string: String,
    pub markings: Vec<MarkingId>,
    pub text_color: String,
    pub background_colors: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CbacMarkingRestrictions {
    pub disallowed_markings: Vec<MarkingId>,
    pub implied_markings: Vec<MarkingId>,
    pub required_markings: Vec<Vec<MarkingId>>,
    pub user_satisfies_markings: bool,
    pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enrollment {
    pub rid: EnrollmentRid,
    pub name: String,
    pub created_time: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentRoleAssignment {
    pub principal_type: PrincipalType,
    pub principal_id: PrincipalId,
    pub role_id: RoleId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub description: Option<String>,
    pub realm: String,
    pub organizations: Vec<OrganizationRid>,
    pub attributes: Attributes,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub principal_type: PrincipalType,
    pub principal_id: PrincipalId,
    pub expiration: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembership {
    pub group_id: GroupId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembershipExpirationPolicy {
    pub maximum_value: Option<String>,
    pub maximum_duration: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupProviderInfo {
    pub provider_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub host_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marking {
    pub id: MarkingId,
    pub category_id: MarkingCategoryId,
    pub name: String,
    pub description: Option<String>,
    pub organization: Option<OrganizationRid>,
    pub created_time: String,
    pub created_by: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkingCategory {
    pub id: MarkingCategoryId,
    pub name: String,
    pub description: String,
    pub category_type: MarkingCategoryType,
    pub marking_type: MarkingType,
    pub markings: Vec<MarkingId>,
    pub created_time: String,
    pub created_by: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkingCategoryPermissions {
    pub organization_rids: Vec<OrganizationRid>,
    pub roles: Vec<MarkingCategoryRoleAssignment>,
    pub is_public: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkingCategoryRoleAssignment {
    pub role: MarkingCategoryRole,
    pub principal_id: PrincipalId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkingMember {
    pub principal_type: PrincipalType,
    pub principal_id: PrincipalId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkingRoleAssignment {
    pub principal_type: PrincipalType,
    pub principal_id: PrincipalId,
    pub role: MarkingRole,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkingRoleUpdate {
    pub role: MarkingRole,
    pub principal_id: PrincipalId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub rid: OrganizationRid,
    pub name: String,
    pub description: Option<String>,
    pub marking_id: MarkingId,
    pub host: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationGuestMember {
    pub principal_type: PrincipalType,
    pub principal_id: PrincipalId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationRoleAssignment {
    pub principal_type: PrincipalType,
    pub principal_id: PrincipalId,
    pub role_id: RoleId,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: RoleId,
    pub display_name: String,
    pub description: String,
    pub operations: Vec<String>,
    pub can_assigns: Vec<RoleId>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email: Option<String>,
    pub realm: String,
    pub organization: Option<OrganizationRid>,
    pub status: UserStatus,
    pub attributes: Attributes,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProviderInfo {
    pub provider_id: String,
}

macro_rules! list_response {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub data: Vec<$item>,
            pub next_page_token: Option<String>,
        }
    };
}
list_response!(ListGroupsResponse, Group);
list_response!(ListUsersResponse, User);
list_response!(ListMarkingsResponse, Marking);
list_response!(ListMarkingCategoriesResponse, MarkingCategory);
list_response!(ListGroupMembersResponse, GroupMember);
list_response!(ListGroupMembershipsResponse, GroupMembership);
list_response!(ListMarkingMembersResponse, MarkingMember);
list_response!(ListMarkingRoleAssignmentsResponse, MarkingRoleAssignment);
list_response!(ListHostsResponse, Host);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCurrentGroupsResponse {
    pub data: Vec<Group>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAuthenticationProvidersResponse {
    pub data: Vec<AuthenticationProvider>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEnrollmentRoleAssignmentsResponse {
    pub data: Vec<EnrollmentRoleAssignment>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrganizationGuestMembersResponse {
    pub data: Vec<OrganizationGuestMember>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrganizationRoleAssignmentsResponse {
    pub data: Vec<OrganizationRoleAssignment>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAvailableOrganizationRolesResponse {
    pub data: Vec<Role>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupsBatchResponse {
    pub data: HashMap<GroupId, Group>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUsersBatchResponse {
    pub data: HashMap<UserId, User>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarkingsBatchResponse {
    pub data: HashMap<MarkingId, Marking>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRolesBatchResponse {
    pub data: HashMap<RoleId, Role>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserMarkingsResponse {
    pub view: Vec<MarkingId>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseClassificationsResponse {
    pub parsed: HashMap<String, Vec<MarkingId>>,
    pub errors: HashMap<String, String>,
}
pub type SearchGroupsResponse = ListGroupsResponse;
pub type SearchUsersResponse = ListUsersResponse;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleAssignmentUpdate {
    pub role_id: RoleId,
    pub principal_id: PrincipalId,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupRequest {
    pub name: String,
    pub organizations: Vec<OrganizationRid>,
    pub description: Option<String>,
    pub attributes: Attributes,
}
pub type ReplaceGroupRequest = CreateGroupRequest;
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSearchFilter {
    #[serde(rename = "type")]
    pub filter_type: String,
    pub value: String,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchGroupsRequest {
    pub r#where: GroupSearchFilter,
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchFilter {
    #[serde(rename = "type")]
    pub filter_type: String,
    pub value: String,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchUsersRequest {
    pub r#where: UserSearchFilter,
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupsBatchRequestElement {
    pub group_id: GroupId,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUsersBatchRequestElement {
    pub user_id: UserId,
    pub status: Option<UserStatus>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarkingsBatchRequestElement {
    pub marking_id: MarkingId,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRolesBatchRequestElement {
    pub role_id: RoleId,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGroupMembersRequest {
    pub principal_ids: Vec<PrincipalId>,
    pub expiration: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrincipalIdsRequest {
    pub principal_ids: Vec<PrincipalId>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleAssignmentsRequest<T> {
    pub role_assignments: Vec<T>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceGroupMembershipExpirationPolicyRequest {
    pub maximum_duration: Option<u64>,
    pub maximum_value: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceProviderInfoRequest {
    pub provider_id: String,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMarkingRequest {
    pub initial_role_assignments: Vec<MarkingRoleUpdate>,
    pub initial_members: Vec<PrincipalId>,
    pub name: String,
    pub description: Option<String>,
    pub category_id: MarkingCategoryId,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceMarkingRequest {
    pub name: String,
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseClassificationsRequest {
    pub classification_strings: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMarkingCategoryRequest {
    pub initial_permissions: MarkingCategoryPermissions,
    pub name: String,
    pub description: String,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceMarkingCategoryRequest {
    pub name: String,
    pub description: String,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationRequest {
    pub administrators: Vec<PrincipalId>,
    pub enrollment_rid: EnrollmentRid,
    pub name: String,
    pub host: Option<String>,
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOrganizationRequest {
    pub name: String,
    pub host: Option<String>,
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreregisterGroupRequest {
    pub name: String,
    pub organizations: Vec<OrganizationRid>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreregisterUserRequest {
    pub username: String,
    pub organization: OrganizationRid,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email: Option<String>,
    pub attributes: Option<Attributes>,
}
