use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;

#[derive(Debug)]
pub struct Subscribers<'c> {
    transport: &'c Transport,
}

impl<'c> Subscribers<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    fn path(
        dataset_rid: &str,
        branch: &str,
        id: &str,
        operation: &str,
        high_scale: bool,
    ) -> String {
        let prefix = if high_scale { "v2/highScale" } else { "v2" };
        format!(
            "{prefix}/streams/datasets/{dataset_rid}/streams/{branch}/subscribers/{id}{operation}"
        )
    }

    pub async fn commit_offsets(
        &self,
        dataset_rid: &str,
        branch: &str,
        id: &str,
        offsets: PartitionOffsets,
        preview: Option<bool>,
        view_rid: Option<&str>,
    ) -> Result<PartitionOffsets> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(CommitSubscriberOffsetsRequest {
            view_rid: view_rid.map(str::to_owned),
            offsets,
        })?;
        self.transport
            .send_json(
                Method::POST,
                &Self::path(dataset_rid, branch, id, "/commitOffsets", true),
                &query,
                Some(&body),
            )
            .await
    }

    pub async fn create(
        &self,
        dataset_rid: &str,
        branch: &str,
        subscriber_id: &str,
        preview: Option<bool>,
        read_position: Option<ReadPosition>,
    ) -> Result<Subscriber> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(CreateSubscriberRequest {
            subscriber_id: subscriber_id.to_owned(),
            read_position,
        })?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/streams/datasets/{dataset_rid}/streams/{branch}/subscribers"),
                &query,
                Some(&body),
            )
            .await
    }

    pub async fn delete(
        &self,
        dataset_rid: &str,
        branch: &str,
        id: &str,
        preview: Option<bool>,
    ) -> Result<()> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_no_content(
                Method::DELETE,
                &Self::path(dataset_rid, branch, id, "", false),
                &query,
                None,
            )
            .await
    }

    pub async fn get_read_position(
        &self,
        dataset_rid: &str,
        branch: &str,
        id: &str,
        preview: Option<bool>,
        view_rid: Option<&str>,
    ) -> Result<PartitionOffsets> {
        let preview = preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        if let Some(v) = view_rid {
            query.push(("viewRid", v));
        }
        self.transport
            .send_json(
                Method::GET,
                &Self::path(dataset_rid, branch, id, "/getReadPosition", true),
                &query,
                None,
            )
            .await
    }

    pub async fn read_records(
        &self,
        dataset_rid: &str,
        branch: &str,
        id: &str,
        request: ReadRecordsFromSubscriberRequest,
        preview: Option<bool>,
    ) -> Result<ReadSubscriberRecordsResponse> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &Self::path(dataset_rid, branch, id, "/readRecords", true),
                &query,
                Some(&body),
            )
            .await
    }

    pub async fn reset_offsets(
        &self,
        dataset_rid: &str,
        branch: &str,
        id: &str,
        position: ReadPosition,
        preview: Option<bool>,
    ) -> Result<PartitionOffsets> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(ResetSubscriberOffsetsRequest { position })?;
        self.transport
            .send_json(
                Method::POST,
                &Self::path(dataset_rid, branch, id, "/resetOffsets", true),
                &query,
                Some(&body),
            )
            .await
    }
}
