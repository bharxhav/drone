use super::types::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
#[derive(Debug)]
pub struct ModelStudioRuns<'c> {
    transport: &'c Transport,
}
impl<'c> ModelStudioRuns<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn list(
        &self,
        r: &str,
        config_version: Option<i32>,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<&str>,
    ) -> Result<ListModelStudioRunsResponse> {
        let c = config_version.map(|v| v.to_string());
        let p = page_size.map(|v| v.to_string());
        let mut q = vec![];
        if let Some(ref v) = c {
            q.push(("configVersion", v.as_str()))
        }
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
                &format!("v2/models/modelStudios/{r}/runs"),
                &q,
                None,
            )
            .await
    }
}
