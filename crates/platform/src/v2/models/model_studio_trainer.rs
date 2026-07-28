use super::types::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
#[derive(Debug)]
pub struct ModelStudioTrainers<'c> {
    transport: &'c Transport,
}
impl<'c> ModelStudioTrainers<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(
        &self,
        id: &str,
        version: Option<&str>,
        preview: Option<&str>,
    ) -> Result<ModelStudioTrainer> {
        let mut q = vec![];
        if let Some(v) = preview {
            q.push(("preview", v))
        }
        if let Some(v) = version {
            q.push(("version", v))
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/models/modelStudioTrainers/{id}"),
                &q,
                None,
            )
            .await
    }
    pub async fn list(&self, preview: Option<&str>) -> Result<ListModelStudioTrainersResponse> {
        self.transport
            .send_json(
                Method::GET,
                "v2/models/modelStudioTrainers",
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                None,
            )
            .await
    }
}
