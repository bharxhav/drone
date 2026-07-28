use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;

#[derive(Debug)]
pub struct Records<'c> {
    transport: &'c Transport,
}

impl<'c> Records<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn get(&self, record_rid: &str, preview: Option<bool>) -> Result<Record> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/checkpoints/records/{record_rid}"),
                &query,
                None,
            )
            .await
    }

    pub async fn get_batch(
        &self,
        body: &[GetRecordsBatchRequestElement],
        preview: Option<bool>,
    ) -> Result<GetRecordsBatchResponse> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(body)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/checkpoints/records/getBatch",
                &query,
                Some(&body),
            )
            .await
    }

    pub async fn search(
        &self,
        r#where: SearchCheckpointRecordsRequest,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<bool>,
        sort_direction: Option<SortDirection>,
    ) -> Result<SearchCheckpointRecordsResponse> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(SearchRecordsRequest {
            r#where,
            page_token: page_token.map(str::to_owned),
            page_size,
            sort_direction,
        })?;
        self.transport
            .send_json(
                Method::POST,
                "v2/checkpoints/records/search",
                &query,
                Some(&body),
            )
            .await
    }
}
