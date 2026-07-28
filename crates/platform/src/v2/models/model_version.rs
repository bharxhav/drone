use super::types::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
#[derive(Debug)]
pub struct ModelVersions<'c> {
    transport: &'c Transport,
}
impl<'c> ModelVersions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(
        &self,
        m: &str,
        r: CreateModelVersionRequest,
        preview: Option<&str>,
    ) -> Result<ModelVersion> {
        let b = serde_json::to_value(r)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/models/{m}/versions"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
    pub async fn get(&self, m: &str, v: &str, preview: Option<&str>) -> Result<ModelVersion> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/models/{m}/versions/{v}"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                None,
            )
            .await
    }
    pub async fn list(
        &self,
        m: &str,
        branch: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<&str>,
    ) -> Result<ListModelVersionsResponse> {
        let ps = page_size.map(|v| v.to_string());
        let mut q = vec![];
        if let Some(v) = branch {
            q.push(("branch", v))
        }
        if let Some(ref v) = ps {
            q.push(("pageSize", v.as_str()))
        }
        if let Some(v) = page_token {
            q.push(("pageToken", v))
        }
        if let Some(v) = preview {
            q.push(("preview", v))
        }
        self.transport
            .send_json(Method::GET, &format!("v2/models/{m}/versions"), &q, None)
            .await
    }
}
