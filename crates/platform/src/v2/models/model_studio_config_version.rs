use super::types::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
#[derive(Debug)]
pub struct ModelStudioConfigVersions<'c> {
    transport: &'c Transport,
}
impl<'c> ModelStudioConfigVersions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(
        &self,
        r: &str,
        request: CreateModelStudioConfigVersionRequest,
        preview: Option<&str>,
    ) -> Result<ModelStudioConfigVersion> {
        let b = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/models/modelStudios/{r}/configVersions"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
    pub async fn get(
        &self,
        r: &str,
        v: i32,
        preview: Option<&str>,
    ) -> Result<ModelStudioConfigVersion> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/models/modelStudios/{r}/configVersions/{v}"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                None,
            )
            .await
    }
    pub async fn latest(
        &self,
        r: &str,
        preview: Option<&str>,
    ) -> Result<Option<ModelStudioConfigVersion>> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/models/modelStudios/{r}/configVersions/latest"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                None,
            )
            .await
    }
    pub async fn list(
        &self,
        r: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<&str>,
    ) -> Result<ListModelStudioConfigVersionsResponse> {
        let p = page_size.map(|v| v.to_string());
        let mut q = vec![];
        if let Some(ref v) = p {
            q.push(("pageSize", v.as_str()))
        }
        if let Some(v) = page_token {
            q.push(("pageToken", v))
        }
        if let Some(v) = preview {
            q.push(("preview", v))
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/models/modelStudios/{r}/configVersions"),
                &q,
                None,
            )
            .await
    }
}
