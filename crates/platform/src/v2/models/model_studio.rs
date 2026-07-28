use super::types::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
#[derive(Debug)]
pub struct ModelStudios<'c> {
    transport: &'c Transport,
}
impl<'c> ModelStudios<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(
        &self,
        name: &str,
        parent_folder_rid: &str,
        preview: Option<&str>,
    ) -> Result<ModelStudio> {
        let b = serde_json::to_value(CreateModelStudioRequest {
            name: name.into(),
            parent_folder_rid: parent_folder_rid.into(),
        })?;
        self.transport
            .send_json(
                Method::POST,
                "v2/models/modelStudios",
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
    pub async fn get(&self, r: &str, preview: Option<&str>) -> Result<ModelStudio> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/models/modelStudios/{r}"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                None,
            )
            .await
    }
    pub async fn launch(&self, r: &str, preview: Option<&str>) -> Result<ModelStudioRun> {
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/models/modelStudios/{r}/launch"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                None,
            )
            .await
    }
}
