use super::types::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
#[derive(Debug)]
pub struct ModelResources<'c> {
    transport: &'c Transport,
}
impl<'c> ModelResources<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(
        &self,
        name: &str,
        parent_folder_rid: &str,
        preview: Option<&str>,
    ) -> Result<Model> {
        let b = serde_json::to_value(CreateModelRequest {
            name: name.into(),
            parent_folder_rid: parent_folder_rid.into(),
        })?;
        self.transport
            .send_json(
                Method::POST,
                "v2/models",
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
    pub async fn get(&self, rid: &str, preview: Option<&str>) -> Result<Model> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/models/{rid}"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                None,
            )
            .await
    }
    pub async fn promote_version(
        &self,
        rid: &str,
        request: PromoteVersionModelRequest,
        preview: Option<&str>,
    ) -> Result<ModelVersion> {
        let b = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/models/{rid}/promoteVersion"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
}
