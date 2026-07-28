use crate::{error::Result, transport::Transport};
use reqwest::Method;

use super::{ResourceRoles, ResourceTags, models::*};

#[derive(Debug)]
pub struct Resources<'c> {
    transport: &'c Transport,
}

impl<'c> Resources<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub fn roles(&self) -> ResourceRoles<'_> {
        ResourceRoles::new(self.transport)
    }
    pub fn tags(&self) -> ResourceTags<'_> {
        ResourceTags::new(self.transport)
    }

    pub async fn get(&self, resource_rid: &str) -> Result<Resource> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/filesystem/resources/{resource_rid}"),
                &[],
                None,
            )
            .await
    }
    pub async fn delete(&self, resource_rid: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::DELETE,
                &format!("v2/filesystem/resources/{resource_rid}"),
                &[],
                None,
            )
            .await
    }
    pub async fn restore(&self, resource_rid: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/resources/{resource_rid}/restore"),
                &[],
                None,
            )
            .await
    }
    pub async fn permanently_delete(&self, resource_rid: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/resources/{resource_rid}/permanentlyDelete"),
                &[],
                None,
            )
            .await
    }
    pub async fn get_batch(
        &self,
        request: &[GetResourcesBatchRequestElement],
    ) -> Result<GetResourcesBatchResponse> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/filesystem/resources/getBatch",
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn get_by_path(&self, path: &str) -> Result<Resource> {
        self.transport
            .send_json(
                Method::GET,
                "v2/filesystem/resources/getByPath",
                &[("path", path)],
                None,
            )
            .await
    }
    pub async fn get_by_path_batch(
        &self,
        request: &[GetByPathResourcesBatchRequestElement],
    ) -> Result<GetByPathResourcesBatchResponse> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/filesystem/resources/getByPathBatch",
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn add_markings(
        &self,
        resource_rid: &str,
        marking_ids: Vec<MarkingId>,
    ) -> Result<()> {
        let body = serde_json::to_value(AddMarkingsRequest { marking_ids })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/resources/{resource_rid}/addMarkings"),
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn remove_markings(
        &self,
        resource_rid: &str,
        marking_ids: Vec<MarkingId>,
    ) -> Result<()> {
        let body = serde_json::to_value(RemoveMarkingsRequest { marking_ids })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/resources/{resource_rid}/removeMarkings"),
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn get_access_requirements(&self, resource_rid: &str) -> Result<AccessRequirements> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/filesystem/resources/{resource_rid}/getAccessRequirements"),
                &[],
                None,
            )
            .await
    }
    pub async fn markings(
        &self,
        resource_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListMarkingsOfResourceResponse> {
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
                &format!("v2/filesystem/resources/{resource_rid}/markings"),
                &query,
                None,
            )
            .await
    }
}
