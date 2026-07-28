use crate::{error::Result, transport::Transport};
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct ResourceTags<'c> {
    transport: &'c Transport,
}

impl<'c> ResourceTags<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn list(
        &self,
        resource_rid: &str,
        preview: Option<bool>,
    ) -> Result<ListResourceTagsResponse> {
        let pv = preview.map(|v| v.to_string());
        let query = pv
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/filesystem/resources/{resource_rid}/tags"),
                &query,
                None,
            )
            .await
    }
    pub async fn add(
        &self,
        resource_rid: &str,
        tag_rids: Vec<TagRid>,
        preview: Option<bool>,
    ) -> Result<()> {
        let body = serde_json::to_value(AddResourceTagsRequest { tag_rids })?;
        let pv = preview.map(|v| v.to_string());
        let query = pv
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/resources/{resource_rid}/tags/add"),
                &query,
                Some(&body),
            )
            .await
    }
    pub async fn remove(
        &self,
        resource_rid: &str,
        tag_rids: Vec<TagRid>,
        preview: Option<bool>,
    ) -> Result<()> {
        let body = serde_json::to_value(RemoveResourceTagsRequest { tag_rids })?;
        let pv = preview.map(|v| v.to_string());
        let query = pv
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/resources/{resource_rid}/tags/remove"),
                &query,
                Some(&body),
            )
            .await
    }
}
