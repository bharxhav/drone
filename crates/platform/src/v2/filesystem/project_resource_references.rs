use crate::{error::Result, transport::Transport};
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct ProjectResourceReferences<'c> {
    transport: &'c Transport,
}

impl<'c> ProjectResourceReferences<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn list(
        &self,
        project_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
        reference_type: Option<ProjectResourceReferenceType>,
    ) -> Result<ListProjectResourceReferencesResponse> {
        let size = page_size.map(|v| v.to_string());
        let reference_type = reference_type.map(|v| {
            serde_json::to_value(v)
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned()
        });
        let mut query = Vec::new();
        if let Some(v) = size.as_deref() {
            query.push(("pageSize", v));
        }
        if let Some(v) = page_token {
            query.push(("pageToken", v));
        }
        if let Some(v) = reference_type.as_deref() {
            query.push(("referenceType", v));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/filesystem/projects/{project_rid}/references"),
                &query,
                None,
            )
            .await
    }
    pub async fn add(
        &self,
        project_rid: &str,
        resources: Vec<AddResourceReferenceRequest>,
    ) -> Result<()> {
        let body = serde_json::to_value(AddProjectResourceReferencesRequest { resources })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/projects/{project_rid}/references/add"),
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn remove(&self, project_rid: &str, resources: Vec<Rid>) -> Result<()> {
        let body = serde_json::to_value(RemoveProjectResourceReferencesRequest { resources })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/projects/{project_rid}/references/remove"),
                &[],
                Some(&body),
            )
            .await
    }
}
