use reqwest::Method;

use super::models::*;
use crate::{error::Result, transport::Transport};

#[derive(Debug)]
pub struct ScheduleVersions<'c> {
    transport: &'c Transport,
}

impl<'c> ScheduleVersions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    fn preview(value: Option<bool>) -> Option<String> {
        value.map(|value| value.to_string())
    }

    pub async fn get(
        &self,
        schedule_version_rid: &str,
        preview: Option<bool>,
    ) -> Result<ScheduleVersion> {
        let preview = Self::preview(preview);
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/orchestration/scheduleVersions/{schedule_version_rid}"),
                &query,
                None,
            )
            .await
    }

    pub async fn schedule(
        &self,
        schedule_version_rid: &str,
        preview: Option<bool>,
    ) -> Result<Option<Schedule>> {
        let preview = Self::preview(preview);
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/orchestration/scheduleVersions/{schedule_version_rid}/schedule"),
                &query,
                None,
            )
            .await
    }
}
