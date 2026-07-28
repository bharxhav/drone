use reqwest::Method;

use crate::{error::Result, transport::Transport};

use super::models::*;

#[derive(Debug)]
pub struct DatasetResource<'c> {
    transport: &'c Transport,
}

impl<'c> DatasetResource<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        name: &str,
        parent_folder_rid: &str,
        schema: serde_json::Value,
        branch_name: Option<&str>,
        compressed: Option<bool>,
        partitions_count: Option<i32>,
        preview: Option<bool>,
        stream_type: Option<StreamType>,
    ) -> Result<Dataset> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(CreateStreamingDatasetRequest {
            name: name.to_owned(),
            parent_folder_rid: parent_folder_rid.to_owned(),
            schema,
            branch_name: branch_name.map(str::to_owned),
            partitions_count,
            stream_type,
            compressed,
        })?;
        self.transport
            .send_json(
                Method::POST,
                "v2/streams/datasets/create",
                &query,
                Some(&body),
            )
            .await
    }
}
