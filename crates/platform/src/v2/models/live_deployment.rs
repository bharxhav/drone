use super::types::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;
#[derive(Debug)]
pub struct LiveDeployments<'c> {
    transport: &'c Transport,
}
impl<'c> LiveDeployments<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(
        &self,
        r: CreateLiveDeploymentRequest,
        preview: Option<&str>,
    ) -> Result<LiveDeployment> {
        let b = serde_json::to_value(r)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/models/liveDeployments",
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
    pub async fn get(&self, r: &str, preview: Option<&str>) -> Result<LiveDeployment> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/models/liveDeployments/{r}"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                None,
            )
            .await
    }
    pub async fn list(
        &self,
        m: &str,
        branch: Option<&str>,
        preview: Option<&str>,
    ) -> Result<ListLiveDeploymentsResponse> {
        let mut q = vec![("modelRid", m)];
        if let Some(v) = branch {
            q.push(("branch", v))
        }
        if let Some(v) = preview {
            q.push(("preview", v))
        }
        self.transport
            .send_json(Method::GET, "v2/models/liveDeployments", &q, None)
            .await
    }
    pub async fn replace(
        &self,
        r: &str,
        request: ReplaceLiveDeploymentRequest,
        preview: Option<&str>,
    ) -> Result<LiveDeployment> {
        let b = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/models/liveDeployments/{r}"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
    /// Does not expose attribution/trace headers because Transport cannot set per-request headers.
    pub async fn transform_json(
        &self,
        r: &str,
        input: HashMap<String, Value>,
        preview: Option<&str>,
    ) -> Result<TransformLiveDeploymentResponse> {
        let b = serde_json::to_value(TransformJsonLiveDeploymentRequest { input })?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/models/liveDeployments/{r}/transformJson"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
}
