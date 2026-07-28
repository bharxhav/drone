use super::models::*;
use crate::{
    error::Result,
    transport::{RequestBody, Transport},
};
use bytes::Bytes;
use reqwest::{
    Method,
    header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue},
};
#[derive(Debug)]
pub struct MediaSetResources<'c> {
    transport: &'c Transport,
}

#[derive(Debug, Clone, Default)]
pub struct ClearOptions<'a> {
    pub branch_name: Option<&'a str>,
    pub branch_rid: Option<&'a str>,
    pub transaction_id: Option<&'a str>,
    pub view_rid: Option<&'a str>,
    pub preview: Option<&'a str>,
}

#[derive(Debug, Clone, Default)]
pub struct UploadOptions<'a> {
    pub branch_name: Option<&'a str>,
    pub branch_rid: Option<&'a str>,
    pub media_item_path: Option<&'a str>,
    pub media_item_rid: Option<&'a str>,
    pub preview: Option<&'a str>,
    pub transaction_id: Option<&'a str>,
    pub view_rid: Option<&'a str>,
}
impl<'c> MediaSetResources<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    fn preview(v: Option<&str>) -> Vec<(&str, &str)> {
        v.map(|x| vec![("preview", x)]).unwrap_or_default()
    }
    pub async fn abort(&self, m: &str, t: &str, preview: Option<&str>) -> Result<()> {
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/mediasets/{m}/transactions/{t}/abort"),
                &Self::preview(preview),
                None,
            )
            .await
    }
    pub async fn commit(&self, m: &str, t: &str, preview: Option<&str>) -> Result<()> {
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/mediasets/{m}/transactions/{t}/commit"),
                &Self::preview(preview),
                None,
            )
            .await
    }
    pub async fn create_transaction(
        &self,
        m: &str,
        branch_name: Option<&str>,
        preview: Option<&str>,
    ) -> Result<TransactionId> {
        let mut q = vec![];
        if let Some(v) = branch_name {
            q.push(("branchName", v))
        }
        if let Some(v) = preview {
            q.push(("preview", v))
        }
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/mediasets/{m}/transactions"),
                &q,
                None,
            )
            .await
    }
    pub async fn get(&self, m: &str, preview: Option<&str>) -> Result<GetMediaSetResponse> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/mediasets/{m}"),
                &Self::preview(preview),
                None,
            )
            .await
    }
    pub async fn clear(&self, m: &str, path: &str, options: ClearOptions<'_>) -> Result<()> {
        let mut q = vec![("mediaItemPath", path)];
        for (k, v) in [
            ("branchName", options.branch_name),
            ("branchRid", options.branch_rid),
            ("preview", options.preview),
            ("transactionId", options.transaction_id),
            ("viewRid", options.view_rid),
        ] {
            if let Some(v) = v {
                q.push((k, v))
            }
        }
        self.transport
            .send_no_content(
                Method::DELETE,
                &format!("v2/mediasets/{m}/items/clearAtPath"),
                &q,
                None,
            )
            .await
    }
    pub async fn get_rid_by_path(
        &self,
        m: &str,
        path: &str,
        branch_name: Option<&str>,
        branch_rid: Option<&str>,
        view_rid: Option<&str>,
        preview: Option<&str>,
    ) -> Result<GetMediaItemRidByPathResponse> {
        let mut q = vec![("mediaItemPath", path)];
        for (k, v) in [
            ("branchName", branch_name),
            ("branchRid", branch_rid),
            ("preview", preview),
            ("viewRid", view_rid),
        ] {
            if let Some(v) = v {
                q.push((k, v))
            }
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/mediasets/{m}/items/getRidByPath"),
                &q,
                None,
            )
            .await
    }
    pub async fn register(
        &self,
        m: &str,
        r: RegisterMediaItemRequest,
        branch_name: Option<&str>,
        transaction_id: Option<&str>,
        view_rid: Option<&str>,
        preview: Option<&str>,
    ) -> Result<RegisterMediaItemResponse> {
        let mut q = vec![];
        for (k, v) in [
            ("branchName", branch_name),
            ("preview", preview),
            ("transactionId", transaction_id),
            ("viewRid", view_rid),
        ] {
            if let Some(v) = v {
                q.push((k, v))
            }
        }
        let b = serde_json::to_value(r)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/mediasets/{m}/items/register"),
                &q,
                Some(&b),
            )
            .await
    }
    pub async fn calculate(
        &self,
        m: &str,
        i: &str,
        preview: Option<&str>,
    ) -> Result<TrackedTransformationResponse> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/mediasets/{m}/items/{i}/transform/imagery/thumbnail/calculate"),
                &Self::preview(preview),
                None,
            )
            .await
    }
    pub async fn info(
        &self,
        m: &str,
        i: &str,
        preview: Option<&str>,
    ) -> Result<GetMediaItemInfoResponse> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/mediasets/{m}/items/{i}"),
                &Self::preview(preview),
                None,
            )
            .await
    }
    pub async fn metadata(
        &self,
        m: &str,
        i: &str,
        preview: Option<&str>,
    ) -> Result<MediaItemMetadata> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/mediasets/{m}/items/{i}/metadata"),
                &Self::preview(preview),
                None,
            )
            .await
    }
    pub async fn read(&self, m: &str, i: &str, preview: Option<&str>) -> Result<Bytes> {
        self.transport
            .send_binary(
                Method::GET,
                &format!("v2/mediasets/{m}/items/{i}/content"),
                &Self::preview(preview),
            )
            .await
    }
    pub async fn read_original(&self, m: &str, i: &str, preview: Option<&str>) -> Result<Bytes> {
        self.transport
            .send_binary(
                Method::GET,
                &format!("v2/mediasets/{m}/items/{i}/original"),
                &Self::preview(preview),
            )
            .await
    }
    pub async fn retrieve(&self, m: &str, i: &str, preview: Option<&str>) -> Result<Bytes> {
        self.transport
            .send_binary(
                Method::GET,
                &format!("v2/mediasets/{m}/items/{i}/transform/imagery/thumbnail/retrieve"),
                &Self::preview(preview),
            )
            .await
    }
    pub async fn transform(
        &self,
        m: &str,
        i: &str,
        r: TransformMediaItemRequest,
        preview: Option<&str>,
    ) -> Result<TransformMediaItemResponse> {
        let b = serde_json::to_value(r)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/mediasets/{m}/items/{i}/transform"),
                &Self::preview(preview),
                Some(&b),
            )
            .await
    }
    pub async fn get_status(
        &self,
        m: &str,
        i: &str,
        j: &str,
        preview: Option<&str>,
    ) -> Result<GetTransformationJobStatusResponse> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/mediasets/{m}/items/{i}/transformationJobs/{j}"),
                &Self::preview(preview),
                None,
            )
            .await
    }
    pub async fn get_result(
        &self,
        m: &str,
        i: &str,
        j: &str,
        preview: Option<&str>,
    ) -> Result<Bytes> {
        self.transport
            .send_binary(
                Method::GET,
                &format!("v2/mediasets/{m}/items/{i}/transformationJobs/{j}/result"),
                &Self::preview(preview),
            )
            .await
    }

    pub async fn upload(
        &self,
        media_set_rid: &str,
        body: impl Into<Bytes>,
        options: UploadOptions<'_>,
    ) -> Result<PutMediaItemResponse> {
        let mut query = Vec::new();
        for (name, value) in [
            ("branchName", options.branch_name),
            ("branchRid", options.branch_rid),
            ("mediaItemPath", options.media_item_path),
            ("mediaItemRid", options.media_item_rid),
            ("preview", options.preview),
            ("transactionId", options.transaction_id),
            ("viewRid", options.view_rid),
        ] {
            if let Some(value) = value {
                query.push((name, value));
            }
        }
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        let response = self
            .transport
            .send(
                Method::POST,
                &format!("v2/mediasets/{media_set_rid}/items"),
                &query,
                headers,
                Some(RequestBody::Bytes(body.into())),
            )
            .await?;
        Ok(response.json().await?)
    }

    pub async fn upload_media(
        &self,
        body: impl Into<Bytes>,
        filename: &str,
        media_item_rid: Option<&str>,
        preview: Option<&str>,
        attribution: Option<&str>,
    ) -> Result<MediaReference> {
        let mut query = vec![("filename", filename)];
        if let Some(value) = media_item_rid {
            query.push(("mediaItemRid", value));
        }
        if let Some(value) = preview {
            query.push(("preview", value));
        }
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        if let Some(value) = attribution {
            headers.insert(
                HeaderName::from_static("attribution"),
                HeaderValue::from_str(value)
                    .map_err(|error| crate::error::Error::Config(error.to_string()))?,
            );
        }
        let response = self
            .transport
            .send(
                Method::PUT,
                "v2/mediasets/media/upload",
                &query,
                headers,
                Some(RequestBody::Bytes(body.into())),
            )
            .await?;
        Ok(response.json().await?)
    }
}
