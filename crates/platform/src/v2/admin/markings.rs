use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
fn json<T: serde::Serialize>(v: T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(v)?)
}
#[derive(Debug)]
pub struct Markings<'c> {
    transport: &'c Transport,
}
impl<'c> Markings<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(&self, r: CreateMarkingRequest) -> Result<Marking> {
        let b = json(r)?;
        self.transport
            .send_json(Method::POST, "v2/admin/markings", &[], Some(&b))
            .await
    }
    pub async fn get(&self, id: &str) -> Result<Marking> {
        self.transport
            .send_json(Method::GET, &format!("v2/admin/markings/{id}"), &[], None)
            .await
    }
    pub async fn get_batch(
        &self,
        r: Vec<GetMarkingsBatchRequestElement>,
    ) -> Result<GetMarkingsBatchResponse> {
        let b = json(r)?;
        self.transport
            .send_json(Method::POST, "v2/admin/markings/getBatch", &[], Some(&b))
            .await
    }
    pub async fn list(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListMarkingsResponse> {
        let s = page_size.map(|v| v.to_string());
        let mut q = Vec::new();
        if let Some(ref v) = s {
            q.push(("pageSize", v.as_str()))
        }
        if let Some(v) = page_token {
            q.push(("pageToken", v))
        }
        self.transport
            .send_json(Method::GET, "v2/admin/markings", &q, None)
            .await
    }
    pub async fn parse_classifications(
        &self,
        classification_strings: Vec<String>,
        preview: Option<bool>,
    ) -> Result<ParseClassificationsResponse> {
        let b = json(ParseClassificationsRequest {
            classification_strings,
        })?;
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::POST,
                "v2/admin/markings/parseClassifications",
                &q,
                Some(&b),
            )
            .await
    }
    pub async fn replace(&self, id: &str, r: ReplaceMarkingRequest) -> Result<Marking> {
        let b = json(r)?;
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/admin/markings/{id}"),
                &[],
                Some(&b),
            )
            .await
    }
}
#[derive(Debug)]
pub struct MarkingMembers<'c> {
    transport: &'c Transport,
}
impl<'c> MarkingMembers<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn add(&self, id: &str, principal_ids: Vec<PrincipalId>) -> Result<()> {
        let b = json(PrincipalIdsRequest { principal_ids })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/markings/{id}/markingMembers/add"),
                &[],
                Some(&b),
            )
            .await
    }
    pub async fn list(
        &self,
        id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListMarkingMembersResponse> {
        let s = page_size.map(|v| v.to_string());
        let mut q = Vec::new();
        if let Some(ref v) = s {
            q.push(("pageSize", v.as_str()))
        }
        if let Some(v) = page_token {
            q.push(("pageToken", v))
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/markings/{id}/markingMembers"),
                &q,
                None,
            )
            .await
    }
    pub async fn remove(&self, id: &str, principal_ids: Vec<PrincipalId>) -> Result<()> {
        let b = json(PrincipalIdsRequest { principal_ids })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/markings/{id}/markingMembers/remove"),
                &[],
                Some(&b),
            )
            .await
    }
}
#[derive(Debug)]
pub struct MarkingRoleAssignments<'c> {
    transport: &'c Transport,
}
impl<'c> MarkingRoleAssignments<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn add(&self, id: &str, role_assignments: Vec<MarkingRoleUpdate>) -> Result<()> {
        let b = json(RoleAssignmentsRequest { role_assignments })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/markings/{id}/roleAssignments/add"),
                &[],
                Some(&b),
            )
            .await
    }
    pub async fn list(
        &self,
        id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListMarkingRoleAssignmentsResponse> {
        let s = page_size.map(|v| v.to_string());
        let mut q = Vec::new();
        if let Some(ref v) = s {
            q.push(("pageSize", v.as_str()))
        }
        if let Some(v) = page_token {
            q.push(("pageToken", v))
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/markings/{id}/roleAssignments"),
                &q,
                None,
            )
            .await
    }
    pub async fn remove(&self, id: &str, role_assignments: Vec<MarkingRoleUpdate>) -> Result<()> {
        let b = json(RoleAssignmentsRequest { role_assignments })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/markings/{id}/roleAssignments/remove"),
                &[],
                Some(&b),
            )
            .await
    }
}
#[derive(Debug)]
pub struct MarkingCategories<'c> {
    transport: &'c Transport,
}
impl<'c> MarkingCategories<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(
        &self,
        r: CreateMarkingCategoryRequest,
        preview: Option<bool>,
    ) -> Result<MarkingCategory> {
        let b = json(r)?;
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(Method::POST, "v2/admin/markingCategories", &q, Some(&b))
            .await
    }
    pub async fn get(&self, id: &str) -> Result<MarkingCategory> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/markingCategories/{id}"),
                &[],
                None,
            )
            .await
    }
    pub async fn list(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListMarkingCategoriesResponse> {
        let s = page_size.map(|v| v.to_string());
        let mut q = Vec::new();
        if let Some(ref v) = s {
            q.push(("pageSize", v.as_str()))
        }
        if let Some(v) = page_token {
            q.push(("pageToken", v))
        }
        self.transport
            .send_json(Method::GET, "v2/admin/markingCategories", &q, None)
            .await
    }
    pub async fn replace(
        &self,
        id: &str,
        r: ReplaceMarkingCategoryRequest,
        preview: Option<bool>,
    ) -> Result<MarkingCategory> {
        let b = json(r)?;
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/admin/markingCategories/{id}"),
                &q,
                Some(&b),
            )
            .await
    }
}
#[derive(Debug)]
pub struct MarkingCategoryPermissions<'c> {
    transport: &'c Transport,
}
impl<'c> MarkingCategoryPermissions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(
        &self,
        id: &str,
        preview: Option<bool>,
    ) -> Result<super::models::MarkingCategoryPermissions> {
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/markingCategories/{id}/permissions"),
                &q,
                None,
            )
            .await
    }
    pub async fn replace(
        &self,
        id: &str,
        r: super::models::MarkingCategoryPermissions,
        preview: Option<bool>,
    ) -> Result<super::models::MarkingCategoryPermissions> {
        let b = json(r)?;
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/admin/markingCategories/{id}/permissions"),
                &q,
                Some(&b),
            )
            .await
    }
}
