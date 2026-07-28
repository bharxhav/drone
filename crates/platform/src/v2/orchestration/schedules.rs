use reqwest::Method;

use super::models::*;
use crate::{error::Result, transport::Transport};

#[derive(Debug)]
pub struct Schedules<'c> {
    transport: &'c Transport,
}

impl<'c> Schedules<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    fn preview(value: Option<bool>) -> Option<String> {
        value.map(|value| value.to_string())
    }

    pub async fn create(
        &self,
        request: CreateScheduleRequest,
        preview: Option<bool>,
    ) -> Result<Schedule> {
        let preview = Self::preview(preview);
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/orchestration/schedules",
                &query,
                Some(&body),
            )
            .await
    }

    pub async fn delete(&self, schedule_rid: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::DELETE,
                &format!("v2/orchestration/schedules/{schedule_rid}"),
                &[],
                None,
            )
            .await
    }

    pub async fn get(&self, schedule_rid: &str, preview: Option<bool>) -> Result<Schedule> {
        let preview = Self::preview(preview);
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/orchestration/schedules/{schedule_rid}"),
                &query,
                None,
            )
            .await
    }

    pub async fn get_affected_resources(
        &self,
        schedule_rid: &str,
        preview: Option<bool>,
    ) -> Result<AffectedResourcesResponse> {
        let preview = Self::preview(preview);
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/orchestration/schedules/{schedule_rid}/getAffectedResources"),
                &query,
                None,
            )
            .await
    }

    pub async fn get_batch(
        &self,
        request: Vec<GetSchedulesBatchRequestElement>,
        preview: Option<bool>,
    ) -> Result<GetSchedulesBatchResponse> {
        let preview = Self::preview(preview);
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/orchestration/schedules/getBatch",
                &query,
                Some(&body),
            )
            .await
    }

    pub async fn pause(&self, schedule_rid: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/orchestration/schedules/{schedule_rid}/pause"),
                &[],
                None,
            )
            .await
    }

    pub async fn replace(
        &self,
        schedule_rid: &str,
        request: ReplaceScheduleRequest,
        preview: Option<bool>,
    ) -> Result<Schedule> {
        let preview = Self::preview(preview);
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/orchestration/schedules/{schedule_rid}"),
                &query,
                Some(&body),
            )
            .await
    }

    pub async fn run(&self, schedule_rid: &str) -> Result<ScheduleRun> {
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/orchestration/schedules/{schedule_rid}/run"),
                &[],
                None,
            )
            .await
    }

    pub async fn runs(
        &self,
        schedule_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListRunsOfScheduleResponse> {
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
                &format!("v2/orchestration/schedules/{schedule_rid}/runs"),
                &query,
                None,
            )
            .await
    }

    pub async fn unpause(&self, schedule_rid: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/orchestration/schedules/{schedule_rid}/unpause"),
                &[],
                None,
            )
            .await
    }
}
