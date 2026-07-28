pub mod errors;
pub mod models;

use crate::transport::Transport;
use crate::{error::Result, transport::RequestBody, v2::core::models::PreviewMode};
use models::*;
use reqwest::{Method, header::HeaderMap};

/// Third Party Applications namespace handle (websites, versions).
#[derive(Debug)]
pub struct ThirdPartyApplications<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> ThirdPartyApplications<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub fn applications(&self) -> Applications<'c> {
        Applications {
            transport: self.transport,
        }
    }
}
#[derive(Debug)]
pub struct Applications<'c> {
    transport: &'c Transport,
}
impl<'c> Applications<'c> {
    pub fn website(&self) -> WebsiteResource<'c> {
        WebsiteResource {
            transport: self.transport,
        }
    }
    pub async fn get(
        &self,
        rid: &str,
        preview: Option<PreviewMode>,
    ) -> Result<ThirdPartyApplication> {
        let preview = preview.map(|value| value.to_string());
        let query = preview
            .as_deref()
            .map(|value| vec![("preview", value)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/thirdPartyApplications/{rid}"),
                &query,
                None,
            )
            .await
    }
}
#[derive(Debug)]
pub struct WebsiteResource<'c> {
    transport: &'c Transport,
}
impl<'c> WebsiteResource<'c> {
    pub fn versions(&self) -> Versions<'c> {
        Versions {
            transport: self.transport,
        }
    }
    pub async fn deploy(&self, rid: &str, version: VersionVersion) -> Result<Website> {
        let body = serde_json::to_value(DeployWebsiteRequest { version })?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/thirdPartyApplications/{rid}/website/deploy"),
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn get(&self, rid: &str) -> Result<Website> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/thirdPartyApplications/{rid}/website"),
                &[],
                None,
            )
            .await
    }
    pub async fn undeploy(&self, rid: &str) -> Result<Website> {
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/thirdPartyApplications/{rid}/website/undeploy"),
                &[],
                None,
            )
            .await
    }
}
#[derive(Debug)]
pub struct Versions<'c> {
    transport: &'c Transport,
}
impl<'c> Versions<'c> {
    pub async fn delete(&self, rid: &str, version: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::DELETE,
                &format!("v2/thirdPartyApplications/{rid}/website/versions/{version}"),
                &[],
                None,
            )
            .await
    }
    pub async fn get(&self, rid: &str, version: &str) -> Result<Version> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/thirdPartyApplications/{rid}/website/versions/{version}"),
                &[],
                None,
            )
            .await
    }
    pub async fn list(
        &self,
        rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListVersionsResponse> {
        let page_size = page_size.map(|value| value.to_string());
        let mut query = Vec::new();
        if let Some(value) = page_size.as_deref() {
            query.push(("pageSize", value));
        }
        if let Some(value) = page_token {
            query.push(("pageToken", value));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/thirdPartyApplications/{rid}/website/versions"),
                &query,
                None,
            )
            .await
    }
    pub async fn upload(
        &self,
        rid: &str,
        body: bytes::Bytes,
        version: VersionVersion,
    ) -> Result<Version> {
        let query = [("version", version.as_str())];
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            crate::transport::request::content_type("application/octet-stream")?,
        );
        let response = self
            .transport
            .send(
                Method::POST,
                &format!("v2/thirdPartyApplications/{rid}/website/versions"),
                &query,
                headers,
                Some(RequestBody::Bytes(body)),
            )
            .await?;
        Ok(response.json().await?)
    }
    pub async fn upload_snapshot(
        &self,
        rid: &str,
        body: bytes::Bytes,
        version: VersionVersion,
        preview: Option<PreviewMode>,
        snapshot_identifier: Option<&str>,
    ) -> Result<Version> {
        let preview = preview.map(|value| value.to_string());
        let mut query = vec![("version", version.as_str())];
        if let Some(value) = preview.as_deref() {
            query.push(("preview", value));
        }
        if let Some(value) = snapshot_identifier {
            query.push(("snapshotIdentifier", value));
        }
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            crate::transport::request::content_type("application/octet-stream")?,
        );
        let response = self
            .transport
            .send(
                Method::POST,
                &format!("v2/thirdPartyApplications/{rid}/website/versions/uploadSnapshot"),
                &query,
                headers,
                Some(RequestBody::Bytes(body)),
            )
            .await?;
        Ok(response.json().await?)
    }
}
