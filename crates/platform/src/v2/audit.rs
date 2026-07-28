pub mod errors;
pub mod models;

use crate::error::Result;
use crate::transport::Transport;
use models::*;
use reqwest::Method;

/// Audit namespace handle (log files).
#[derive(Debug)]
pub struct Audit<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> Audit<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub fn organizations(&self) -> Organizations<'c> {
        Organizations {
            transport: self.transport,
        }
    }
}
#[derive(Debug)]
pub struct Organizations<'c> {
    transport: &'c Transport,
}
impl<'c> Organizations<'c> {
    pub fn log_files(&self) -> LogFiles<'c> {
        LogFiles {
            transport: self.transport,
        }
    }
}
#[derive(Debug)]
pub struct LogFiles<'c> {
    transport: &'c Transport,
}
impl<'c> LogFiles<'c> {
    pub async fn content(&self, organization_rid: &str, log_file_id: &str) -> Result<bytes::Bytes> {
        self.transport
            .send_binary(
                Method::GET,
                &format!(
                    "v2/audit/organizations/{organization_rid}/logFiles/{log_file_id}/content"
                ),
                &[],
            )
            .await
    }
    pub async fn list(
        &self,
        organization_rid: &str,
        end_date: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
        start_date: Option<&str>,
    ) -> Result<ListLogFilesResponse> {
        let page_size = page_size.map(|value| value.to_string());
        let mut query = Vec::new();
        if let Some(value) = end_date {
            query.push(("endDate", value));
        }
        if let Some(value) = page_size.as_deref() {
            query.push(("pageSize", value));
        }
        if let Some(value) = page_token {
            query.push(("pageToken", value));
        }
        if let Some(value) = start_date {
            query.push(("startDate", value));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/audit/organizations/{organization_rid}/logFiles"),
                &query,
                None,
            )
            .await
    }
}
