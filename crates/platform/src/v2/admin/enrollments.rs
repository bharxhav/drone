use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
fn json<T: serde::Serialize>(v: T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(v)?)
}
#[derive(Debug)]
pub struct Enrollments<'c> {
    transport: &'c Transport,
}
impl<'c> Enrollments<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(&self, id: &str, preview_value: Option<bool>) -> Result<Enrollment> {
        let preview = preview_value.map(|value| value.to_string());
        let q = preview
            .as_ref()
            .map(|value| vec![("preview", value.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(Method::GET, &format!("v2/admin/enrollments/{id}"), &q, None)
            .await
    }
    pub async fn get_current(&self, preview_value: Option<bool>) -> Result<Enrollment> {
        let preview = preview_value.map(|value| value.to_string());
        let q = preview
            .as_ref()
            .map(|value| vec![("preview", value.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(Method::GET, "v2/admin/enrollments/getCurrent", &q, None)
            .await
    }
}
#[derive(Debug)]
pub struct EnrollmentRoleAssignments<'c> {
    transport: &'c Transport,
}
impl<'c> EnrollmentRoleAssignments<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn add(
        &self,
        id: &str,
        role_assignments: Vec<RoleAssignmentUpdate>,
        preview_value: Option<bool>,
    ) -> Result<()> {
        let b = json(RoleAssignmentsRequest { role_assignments })?;
        let preview = preview_value.map(|value| value.to_string());
        let q = preview
            .as_ref()
            .map(|value| vec![("preview", value.as_str())])
            .unwrap_or_default();
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/enrollments/{id}/roleAssignments/add"),
                &q,
                Some(&b),
            )
            .await
    }
    pub async fn list(
        &self,
        id: &str,
        preview_value: Option<bool>,
    ) -> Result<ListEnrollmentRoleAssignmentsResponse> {
        let preview = preview_value.map(|value| value.to_string());
        let q = preview
            .as_ref()
            .map(|value| vec![("preview", value.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/enrollments/{id}/roleAssignments"),
                &q,
                None,
            )
            .await
    }
    pub async fn remove(
        &self,
        id: &str,
        role_assignments: Vec<RoleAssignmentUpdate>,
        preview_value: Option<bool>,
    ) -> Result<()> {
        let b = json(RoleAssignmentsRequest { role_assignments })?;
        let preview = preview_value.map(|value| value.to_string());
        let q = preview
            .as_ref()
            .map(|value| vec![("preview", value.as_str())])
            .unwrap_or_default();
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/enrollments/{id}/roleAssignments/remove"),
                &q,
                Some(&b),
            )
            .await
    }
}
#[derive(Debug)]
pub struct Hosts<'c> {
    transport: &'c Transport,
}
impl<'c> Hosts<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn list(
        &self,
        id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview_value: Option<bool>,
    ) -> Result<ListHostsResponse> {
        let s = page_size.map(|v| v.to_string());
        let p = preview_value.map(|v| v.to_string());
        let mut q = Vec::new();
        if let Some(ref v) = s {
            q.push(("pageSize", v.as_str()))
        }
        if let Some(v) = page_token {
            q.push(("pageToken", v))
        }
        if let Some(ref v) = p {
            q.push(("preview", v.as_str()))
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/enrollments/{id}/hosts"),
                &q,
                None,
            )
            .await
    }
}
