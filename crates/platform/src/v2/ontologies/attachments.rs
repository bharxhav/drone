use crate::error::{ApiError, Error, Result};
use crate::transport::Transport;
use reqwest::Method;
use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};

use super::models::Attachment;

#[derive(Debug)]
pub struct Attachments<'c> {
    transport: &'c Transport,
}

impl<'c> Attachments<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn get(&self, attachment_rid: &str) -> Result<Attachment> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/ontologies/attachments/{attachment_rid}"),
                &[],
                None,
            )
            .await
    }

    pub async fn read(&self, attachment_rid: &str) -> Result<bytes::Bytes> {
        self.transport
            .send_binary(
                Method::GET,
                &format!("v2/ontologies/attachments/{attachment_rid}/content"),
                &[],
            )
            .await
    }

    pub async fn upload(
        &self,
        body: bytes::Bytes,
        content_type: &str,
        filename: &str,
    ) -> Result<Attachment> {
        self.upload_to(
            "v2/ontologies/attachments/upload",
            body,
            content_type,
            filename,
            None,
        )
        .await
    }

    pub async fn upload_with_rid(
        &self,
        attachment_rid: &str,
        body: bytes::Bytes,
        content_type: &str,
        filename: &str,
        preview: Option<bool>,
    ) -> Result<Attachment> {
        self.upload_to(
            &format!("v2/ontologies/attachments/upload/{attachment_rid}"),
            body,
            content_type,
            filename,
            preview,
        )
        .await
    }

    async fn upload_to(
        &self,
        path: &str,
        body: bytes::Bytes,
        content_type: &str,
        filename: &str,
        preview: Option<bool>,
    ) -> Result<Attachment> {
        let url = self
            .transport
            .base_url
            .join(path)
            .map_err(|e| Error::Config(e.to_string()))?;
        let length = body.len();
        let preview = preview.map(|v| v.to_string());
        let mut query = vec![("filename", filename)];
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        let response = self
            .transport
            .http
            .request(Method::POST, url)
            .header(AUTHORIZATION, self.transport.auth.header_value())
            .header(CONTENT_LENGTH, length)
            .header(CONTENT_TYPE, content_type)
            .query(&query)
            .body(body)
            .send()
            .await?;
        let status = response.status();
        let text = response.text().await?;
        if status.is_success() {
            Ok(serde_json::from_str(&text)?)
        } else {
            let json = serde_json::from_str::<serde_json::Value>(&text).unwrap_or_default();
            Err(Error::Api(Box::new(ApiError {
                status,
                error_name: json
                    .get("errorName")
                    .and_then(|v| v.as_str())
                    .map(str::to_owned),
                error_code: json
                    .get("errorCode")
                    .and_then(|v| v.as_str())
                    .map(str::to_owned),
                error_instance_id: json
                    .get("errorInstanceId")
                    .and_then(|v| v.as_str())
                    .map(str::to_owned),
                error_description: json
                    .get("errorDescription")
                    .and_then(|v| v.as_str())
                    .map(str::to_owned)
                    .or(Some(text)),
                parameters: json.get("parameters").cloned().unwrap_or_default(),
            })))
        }
    }
}
