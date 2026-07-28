use reqwest::Method;

use super::models::*;
use crate::{error::Result, transport::Transport};

#[derive(Debug)]
pub struct Jobs<'c> {
    transport: &'c Transport,
}

impl<'c> Jobs<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn get(&self, job_rid: &str, preview: Option<bool>) -> Result<Job> {
        let preview_value = preview.map(|value| value.to_string());
        let query = preview_value
            .as_deref()
            .map(|value| vec![("preview", value)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/orchestration/jobs/{job_rid}"),
                &query,
                None,
            )
            .await
    }

    pub async fn get_batch(
        &self,
        request: Vec<GetJobsBatchRequestElement>,
        preview: Option<bool>,
    ) -> Result<GetJobsBatchResponse> {
        let preview_value = preview.map(|value| value.to_string());
        let query = preview_value
            .as_deref()
            .map(|value| vec![("preview", value)])
            .unwrap_or_default();
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/orchestration/jobs/getBatch",
                &query,
                Some(&body),
            )
            .await
    }
}
