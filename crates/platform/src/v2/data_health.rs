pub mod errors;
pub mod models;

use crate::transport::Transport;
use crate::{error::Result, v2::core::models::PreviewMode};
use models::*;
use reqwest::Method;

/// Data Health namespace handle (checks, reports).
#[derive(Debug)]
pub struct DataHealth<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> DataHealth<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn checks(&self) -> Checks<'c> {
        Checks::new(self.transport)
    }
}

#[derive(Debug)]
pub struct Checks<'c> {
    transport: &'c Transport,
}

impl<'c> Checks<'c> {
    fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn check_reports(&self) -> CheckReports<'c> {
        CheckReports::new(self.transport)
    }

    pub async fn create(
        &self,
        request: CreateCheckRequest,
        preview: Option<PreviewMode>,
    ) -> Result<Check> {
        let body = serde_json::to_value(request)?;
        let preview = preview.map(|value| value.to_string());
        let query = preview
            .as_deref()
            .map(|value| vec![("preview", value)])
            .unwrap_or_default();
        self.transport
            .send_json(Method::POST, "v2/dataHealth/checks", &query, Some(&body))
            .await
    }

    pub async fn delete(&self, check_rid: &str, preview: Option<PreviewMode>) -> Result<()> {
        let preview = preview.map(|value| value.to_string());
        let query = preview
            .as_deref()
            .map(|value| vec![("preview", value)])
            .unwrap_or_default();
        self.transport
            .send_no_content(
                Method::DELETE,
                &format!("v2/dataHealth/checks/{check_rid}"),
                &query,
                None,
            )
            .await
    }

    pub async fn get(&self, check_rid: &str, preview: Option<PreviewMode>) -> Result<Check> {
        let preview = preview.map(|value| value.to_string());
        let query = preview
            .as_deref()
            .map(|value| vec![("preview", value)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/dataHealth/checks/{check_rid}"),
                &query,
                None,
            )
            .await
    }

    pub async fn replace(
        &self,
        check_rid: &str,
        request: ReplaceCheckRequest,
        preview: Option<PreviewMode>,
    ) -> Result<Check> {
        let body = serde_json::to_value(request)?;
        let preview = preview.map(|value| value.to_string());
        let query = preview
            .as_deref()
            .map(|value| vec![("preview", value)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/dataHealth/checks/{check_rid}"),
                &query,
                Some(&body),
            )
            .await
    }
}

#[derive(Debug)]
pub struct CheckReports<'c> {
    transport: &'c Transport,
}

impl<'c> CheckReports<'c> {
    fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn get(
        &self,
        check_rid: &str,
        report_rid: &str,
        preview: Option<PreviewMode>,
    ) -> Result<CheckReport> {
        let preview = preview.map(|value| value.to_string());
        let query = preview
            .as_deref()
            .map(|value| vec![("preview", value)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/dataHealth/checks/{check_rid}/checkReports/{report_rid}"),
                &query,
                None,
            )
            .await
    }

    pub async fn get_latest(
        &self,
        check_rid: &str,
        limit: Option<CheckReportLimit>,
        preview: Option<PreviewMode>,
    ) -> Result<GetLatestCheckReportsResponse> {
        let limit = limit.map(|value| value.to_string());
        let preview = preview.map(|value| value.to_string());
        let mut query = Vec::new();
        if let Some(value) = limit.as_deref() {
            query.push(("limit", value));
        }
        if let Some(value) = preview.as_deref() {
            query.push(("preview", value));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/dataHealth/checks/{check_rid}/checkReports/getLatest"),
                &query,
                None,
            )
            .await
    }
}
