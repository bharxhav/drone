use super::types::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
#[derive(Debug)]
pub struct ModelFunctions<'c> {
    transport: &'c Transport,
}
impl<'c> ModelFunctions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(
        &self,
        m: &str,
        r: CreateModelFunctionRequest,
        preview: Option<&str>,
    ) -> Result<ModelFunction> {
        let b = serde_json::to_value(r)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/models/{m}/function"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
    pub async fn get(&self, m: &str, preview: Option<&str>) -> Result<ModelFunction> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/models/{m}/function"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                None,
            )
            .await
    }
    pub async fn replace(
        &self,
        m: &str,
        r: ReplaceModelFunctionRequest,
        preview: Option<&str>,
    ) -> Result<ModelFunction> {
        let b = serde_json::to_value(r)?;
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/models/{m}/function"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
}
