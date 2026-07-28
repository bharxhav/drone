use super::types::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
#[derive(Debug)]
pub struct Experiments<'c> {
    transport: &'c Transport,
}
impl<'c> Experiments<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(&self, m: &str, e: &str, preview: Option<&str>) -> Result<Experiment> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/models/{m}/experiments/{e}"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                None,
            )
            .await
    }
    pub async fn search(
        &self,
        m: &str,
        r: SearchExperimentsRequest,
        preview: Option<&str>,
    ) -> Result<SearchExperimentsResponse> {
        let b = serde_json::to_value(r)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/models/{m}/experiments/search"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
                Some(&b),
            )
            .await
    }
}
