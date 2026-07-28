use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
fn json<T: serde::Serialize>(v: T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(v)?)
}
#[derive(Debug)]
pub struct Organizations<'c> {
    transport: &'c Transport,
}
impl<'c> Organizations<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(
        &self,
        r: CreateOrganizationRequest,
        preview: Option<bool>,
    ) -> Result<Organization> {
        let b = json(r)?;
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(Method::POST, "v2/admin/organizations", &q, Some(&b))
            .await
    }
    pub async fn get(&self, id: &str) -> Result<Organization> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/organizations/{id}"),
                &[],
                None,
            )
            .await
    }
    pub async fn list_available_roles(
        &self,
        id: &str,
    ) -> Result<ListAvailableOrganizationRolesResponse> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/organizations/{id}/listAvailableRoles"),
                &[],
                None,
            )
            .await
    }
    pub async fn replace(&self, id: &str, r: ReplaceOrganizationRequest) -> Result<Organization> {
        let b = json(r)?;
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/admin/organizations/{id}"),
                &[],
                Some(&b),
            )
            .await
    }
}
#[derive(Debug)]
pub struct OrganizationGuestMembers<'c> {
    transport: &'c Transport,
}
impl<'c> OrganizationGuestMembers<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn add(
        &self,
        id: &str,
        principal_ids: Vec<PrincipalId>,
        preview: Option<bool>,
    ) -> Result<()> {
        let b = json(PrincipalIdsRequest { principal_ids })?;
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/organizations/{id}/guestMembers/add"),
                &q,
                Some(&b),
            )
            .await
    }
    pub async fn list(
        &self,
        id: &str,
        preview: Option<bool>,
    ) -> Result<ListOrganizationGuestMembersResponse> {
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/organizations/{id}/guestMembers"),
                &q,
                None,
            )
            .await
    }
    pub async fn remove(
        &self,
        id: &str,
        principal_ids: Vec<PrincipalId>,
        preview: Option<bool>,
    ) -> Result<()> {
        let b = json(PrincipalIdsRequest { principal_ids })?;
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/organizations/{id}/guestMembers/remove"),
                &q,
                Some(&b),
            )
            .await
    }
}
#[derive(Debug)]
pub struct OrganizationRoleAssignments<'c> {
    transport: &'c Transport,
}
impl<'c> OrganizationRoleAssignments<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn add(&self, id: &str, role_assignments: Vec<RoleAssignmentUpdate>) -> Result<()> {
        let b = json(RoleAssignmentsRequest { role_assignments })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/organizations/{id}/roleAssignments/add"),
                &[],
                Some(&b),
            )
            .await
    }
    pub async fn list(&self, id: &str) -> Result<ListOrganizationRoleAssignmentsResponse> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/organizations/{id}/roleAssignments"),
                &[],
                None,
            )
            .await
    }
    pub async fn remove(
        &self,
        id: &str,
        role_assignments: Vec<RoleAssignmentUpdate>,
    ) -> Result<()> {
        let b = json(RoleAssignmentsRequest { role_assignments })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/organizations/{id}/roleAssignments/remove"),
                &[],
                Some(&b),
            )
            .await
    }
}
