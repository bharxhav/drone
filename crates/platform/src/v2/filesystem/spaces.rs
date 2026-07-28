use crate::{error::Result, transport::Transport};
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct Spaces<'c> {
    transport: &'c Transport,
}

impl<'c> Spaces<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(
        &self,
        request: &CreateSpaceRequest,
        preview: Option<bool>,
    ) -> Result<Space> {
        let body = serde_json::to_value(request)?;
        let pv = preview.map(|v| v.to_string());
        let query = pv
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(Method::POST, "v2/filesystem/spaces", &query, Some(&body))
            .await
    }
    pub async fn delete(&self, space_rid: &str, preview: Option<bool>) -> Result<()> {
        let pv = preview.map(|v| v.to_string());
        let query = pv
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_no_content(
                Method::DELETE,
                &format!("v2/filesystem/spaces/{space_rid}"),
                &query,
                None,
            )
            .await
    }
    pub async fn get(&self, space_rid: &str, preview: Option<bool>) -> Result<Space> {
        let pv = preview.map(|v| v.to_string());
        let query = pv
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/filesystem/spaces/{space_rid}"),
                &query,
                None,
            )
            .await
    }
    pub async fn list(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListSpacesResponse> {
        let size = page_size.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = size.as_deref() {
            query.push(("pageSize", v));
        }
        if let Some(v) = page_token {
            query.push(("pageToken", v));
        }
        self.transport
            .send_json(Method::GET, "v2/filesystem/spaces", &query, None)
            .await
    }
    pub async fn replace(
        &self,
        space_rid: &str,
        request: &ReplaceSpaceRequest,
        preview: Option<bool>,
    ) -> Result<Space> {
        let body = serde_json::to_value(request)?;
        let pv = preview.map(|v| v.to_string());
        let query = pv
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/filesystem/spaces/{space_rid}"),
                &query,
                Some(&body),
            )
            .await
    }
}
