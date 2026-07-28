use super::models::*;
use crate::{
    error::Result,
    transport::{RequestBody, Transport},
};
use bytes::Bytes;
use reqwest::{
    Method,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};

#[derive(Debug)]
pub struct StreamResource<'c> {
    transport: &'c Transport,
}

impl<'c> StreamResource<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        dataset_rid: &str,
        branch_name: &str,
        schema: CreateStreamRequestStreamSchema,
        compressed: Option<bool>,
        partitions_count: Option<i32>,
        preview: Option<bool>,
        stream_type: Option<StreamType>,
    ) -> Result<Stream> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(CreateStreamRequest {
            schema,
            partitions_count,
            stream_type,
            branch_name: branch_name.to_owned(),
            compressed,
        })?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/streams/datasets/{dataset_rid}/streams"),
                &query,
                Some(&body),
            )
            .await
    }

    pub async fn get(&self, dataset_rid: &str, stream_branch_name: &str) -> Result<Stream> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/streams/datasets/{dataset_rid}/streams/{stream_branch_name}"),
                &[],
                None,
            )
            .await
    }

    pub async fn get_end_offsets(
        &self,
        dataset_rid: &str,
        stream_branch_name: &str,
        preview: Option<bool>,
        view_rid: Option<&str>,
    ) -> Result<GetEndOffsetsResponse> {
        let preview = preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        if let Some(v) = view_rid {
            query.push(("viewRid", v));
        }
        self.transport.send_json(Method::GET, &format!("v2/highScale/streams/datasets/{dataset_rid}/streams/{stream_branch_name}/getEndOffsets"), &query, None).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_records(
        &self,
        dataset_rid: &str,
        stream_branch_name: &str,
        limit: i32,
        partition_id: &str,
        preview: Option<bool>,
        start_offset: Option<i64>,
        view_rid: Option<&str>,
    ) -> Result<GetRecordsResponse> {
        let limit = limit.to_string();
        let preview = preview.map(|v| v.to_string());
        let start_offset = start_offset.map(|v| v.to_string());
        let mut query = vec![("limit", limit.as_str()), ("partitionId", partition_id)];
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        if let Some(v) = start_offset.as_deref() {
            query.push(("startOffset", v));
        }
        if let Some(v) = view_rid {
            query.push(("viewRid", v));
        }
        self.transport.send_json(Method::GET, &format!("v2/highScale/streams/datasets/{dataset_rid}/streams/{stream_branch_name}/getRecords"), &query, None).await
    }

    pub async fn publish_record(
        &self,
        dataset_rid: &str,
        stream_branch_name: &str,
        record: Record,
        view_rid: Option<&str>,
    ) -> Result<()> {
        let body = serde_json::to_value(PublishRecordToStreamRequest {
            record,
            view_rid: view_rid.map(str::to_owned),
        })?;
        self.transport.send_no_content(Method::POST, &format!("v2/highScale/streams/datasets/{dataset_rid}/streams/{stream_branch_name}/publishRecord"), &[], Some(&body)).await
    }

    pub async fn publish_binary_record(
        &self,
        dataset_rid: &str,
        stream_branch_name: &str,
        body: impl Into<Bytes>,
        view_rid: Option<&str>,
    ) -> Result<()> {
        let query = view_rid
            .map(|value| vec![("viewRid", value)])
            .unwrap_or_default();
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        self.transport
            .send(
                Method::POST,
                &format!("v2/highScale/streams/datasets/{dataset_rid}/streams/{stream_branch_name}/publishBinaryRecord"),
                &query,
                headers,
                Some(RequestBody::Bytes(body.into())),
            )
            .await
            .map(|_| ())
    }

    pub async fn publish_records(
        &self,
        dataset_rid: &str,
        stream_branch_name: &str,
        records: Vec<Record>,
        view_rid: Option<&str>,
    ) -> Result<()> {
        let body = serde_json::to_value(PublishRecordsToStreamRequest {
            records,
            view_rid: view_rid.map(str::to_owned),
        })?;
        self.transport.send_no_content(Method::POST, &format!("v2/highScale/streams/datasets/{dataset_rid}/streams/{stream_branch_name}/publishRecords"), &[], Some(&body)).await
    }

    pub async fn reset(
        &self,
        dataset_rid: &str,
        stream_branch_name: &str,
        request: ResetStreamRequest,
        preview: Option<bool>,
    ) -> Result<Stream> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/streams/datasets/{dataset_rid}/streams/{stream_branch_name}/reset"),
                &query,
                Some(&body),
            )
            .await
    }
}
