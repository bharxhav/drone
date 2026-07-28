use crate::{error::Result, transport::Transport};
use reqwest::Method;

use super::{ProjectResourceReferences, models::*};

#[derive(Debug)]
pub struct Projects<'c> {
    transport: &'c Transport,
}

impl<'c> Projects<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub fn references(&self) -> ProjectResourceReferences<'_> {
        ProjectResourceReferences::new(self.transport)
    }

    pub async fn get(&self, project_rid: &str) -> Result<Project> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/filesystem/projects/{project_rid}"),
                &[],
                None,
            )
            .await
    }
    pub async fn replace(
        &self,
        project_rid: &str,
        request: &ReplaceProjectRequest,
        preview: Option<bool>,
    ) -> Result<Project> {
        let body = serde_json::to_value(request)?;
        let pv = preview.map(|v| v.to_string());
        let query = pv
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/filesystem/projects/{project_rid}"),
                &query,
                Some(&body),
            )
            .await
    }
    pub async fn create(&self, request: &CreateProjectRequest) -> Result<Project> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/filesystem/projects/create",
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn create_from_template(
        &self,
        request: &CreateProjectFromTemplateRequest,
    ) -> Result<Project> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/filesystem/projects/createFromTemplate",
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn add_organizations(
        &self,
        project_rid: &str,
        organization_rids: Vec<OrganizationRid>,
    ) -> Result<()> {
        let body = serde_json::to_value(AddOrganizationsRequest { organization_rids })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/projects/{project_rid}/addOrganizations"),
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn remove_organizations(
        &self,
        project_rid: &str,
        organization_rids: Vec<OrganizationRid>,
    ) -> Result<()> {
        let body = serde_json::to_value(RemoveOrganizationsRequest { organization_rids })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/projects/{project_rid}/removeOrganizations"),
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn organizations(
        &self,
        project_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListOrganizationsOfProjectResponse> {
        let size = page_size.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = size.as_deref() {
            query.push(("pageSize", v));
        }
        if let Some(v) = page_token {
            query.push(("pageToken", v));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/filesystem/projects/{project_rid}/organizations"),
                &query,
                None,
            )
            .await
    }
}
