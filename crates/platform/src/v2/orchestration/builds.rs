use reqwest::Method;

use super::models::*;
use crate::{error::Result, transport::Transport};

#[derive(Debug)]
pub struct Builds<'c> {
    transport: &'c Transport,
}

impl<'c> Builds<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn cancel(&self, build_rid: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/orchestration/builds/{build_rid}/cancel"),
                &[],
                None,
            )
            .await
    }

    pub async fn create(&self, request: CreateBuildRequest) -> Result<Build> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/orchestration/builds/create",
                &[],
                Some(&body),
            )
            .await
    }

    pub async fn get(&self, build_rid: &str) -> Result<Build> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/orchestration/builds/{build_rid}"),
                &[],
                None,
            )
            .await
    }

    pub async fn get_batch(
        &self,
        request: Vec<GetBuildsBatchRequestElement>,
    ) -> Result<GetBuildsBatchResponse> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/orchestration/builds/getBatch",
                &[],
                Some(&body),
            )
            .await
    }

    pub async fn jobs(
        &self,
        build_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListJobsOfBuildResponse> {
        let page_size_value;
        let mut query = Vec::new();
        if let Some(value) = page_size {
            page_size_value = value.to_string();
            query.push(("pageSize", page_size_value.as_str()));
        }
        if let Some(value) = page_token {
            query.push(("pageToken", value));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/orchestration/builds/{build_rid}/jobs"),
                &query,
                None,
            )
            .await
    }

    pub async fn search(
        &self,
        request: SearchBuildsRequest,
        preview: Option<bool>,
    ) -> Result<SearchBuildsResponse> {
        let preview_value = preview.map(|value| value.to_string());
        let query = preview_value
            .as_deref()
            .map(|value| vec![("preview", value)])
            .unwrap_or_default();
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/orchestration/builds/search",
                &query,
                Some(&body),
            )
            .await
    }
}
