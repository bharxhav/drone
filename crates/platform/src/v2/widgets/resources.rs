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
use serde_json::Value;

fn query(
    preview: Option<bool>,
    page_size: Option<u32>,
    page_token: Option<&str>,
) -> Vec<(String, String)> {
    let mut q = Vec::new();
    if let Some(v) = page_size {
        q.push(("pageSize".into(), v.to_string()));
    }
    if let Some(v) = page_token {
        q.push(("pageToken".into(), v.into()));
    }
    if let Some(v) = preview {
        q.push(("preview".into(), v.to_string()));
    }
    q
}
fn refs(q: &[(String, String)]) -> Vec<(&str, &str)> {
    q.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect()
}

#[derive(Debug)]
pub struct Repositories<'c> {
    transport: &'c Transport,
}
impl<'c> Repositories<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(&self, rid: &str, preview: Option<bool>) -> Result<Repository> {
        let q = query(preview, None, None);
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/widgets/repositories/{rid}"),
                &refs(&q),
                None,
            )
            .await
    }

    pub async fn publish(
        &self,
        repository_rid: &str,
        body: impl Into<Bytes>,
        repository_version: &str,
        preview: Option<bool>,
    ) -> Result<Release> {
        let q = query(preview, None, None);
        let mut q = refs(&q);
        q.push(("repositoryVersion", repository_version));
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        let response = self
            .transport
            .send(
                Method::POST,
                &format!("v2/widgets/repositories/{repository_rid}/publish"),
                &q,
                headers,
                Some(RequestBody::Bytes(body.into())),
            )
            .await?;
        Ok(response.json().await?)
    }
}
#[derive(Debug)]
pub struct WidgetSets<'c> {
    transport: &'c Transport,
}
impl<'c> WidgetSets<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(&self, rid: &str, preview: Option<bool>) -> Result<WidgetSet> {
        let q = query(preview, None, None);
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/widgets/widgetSets/{rid}"),
                &refs(&q),
                None,
            )
            .await
    }
}
#[derive(Debug)]
pub struct Releases<'c> {
    transport: &'c Transport,
}
impl<'c> Releases<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn delete(
        &self,
        widget_set_rid: &str,
        version: &str,
        preview: Option<bool>,
    ) -> Result<()> {
        let q = query(preview, None, None);
        self.transport
            .send_no_content(
                Method::DELETE,
                &format!("v2/widgets/widgetSets/{widget_set_rid}/releases/{version}"),
                &refs(&q),
                None,
            )
            .await
    }
    pub async fn get(
        &self,
        widget_set_rid: &str,
        version: &str,
        preview: Option<bool>,
    ) -> Result<Release> {
        let q = query(preview, None, None);
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/widgets/widgetSets/{widget_set_rid}/releases/{version}"),
                &refs(&q),
                None,
            )
            .await
    }
    pub async fn list(
        &self,
        widget_set_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<bool>,
    ) -> Result<ListReleasesResponse> {
        let q = query(preview, page_size, page_token);
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/widgets/widgetSets/{widget_set_rid}/releases"),
                &refs(&q),
                None,
            )
            .await
    }
}
#[derive(Debug)]
pub struct DevModeSettingsResource<'c> {
    transport: &'c Transport,
}
impl<'c> DevModeSettingsResource<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn enable(&self, preview: Option<bool>) -> Result<DevModeSettings> {
        let q = query(preview, None, None);
        self.transport
            .send_json(
                Method::POST,
                "v2/widgets/devModeSettings/enable",
                &refs(&q),
                None,
            )
            .await
    }
    pub async fn set_widget_set_by_id(
        &self,
        widget_set_rid: &str,
        settings: WidgetSetDevModeSettingsById,
        preview: Option<bool>,
    ) -> Result<DevModeSettings> {
        let q = query(preview, None, None);
        let body = serde_json::to_value(SetWidgetSetDevModeSettingsByIdRequest {
            widget_set_rid: widget_set_rid.into(),
            settings,
        })?;
        self.transport
            .send_json(
                Method::POST,
                "v2/widgets/devModeSettings/setWidgetSetById",
                &refs(&q),
                Some(&body),
            )
            .await
    }
}
#[derive(Debug)]
pub struct DevModeSettingsV2Resource<'c> {
    transport: &'c Transport,
}
impl<'c> DevModeSettingsV2Resource<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn enable(&self, preview: Option<bool>) -> Result<DevModeSettingsV2> {
        let q = query(preview, None, None);
        self.transport
            .send_json(
                Method::POST,
                "v2/widgets/devModeSettingsV2/enable",
                &refs(&q),
                None,
            )
            .await
    }
    pub async fn set_widget_set_manifest(
        &self,
        widget_set_rid: &str,
        manifest: Value,
        preview: Option<bool>,
    ) -> Result<DevModeSettingsV2> {
        let q = query(preview, None, None);
        let body = serde_json::to_value(SetWidgetSetManifestDevModeSettingsV2Request {
            widget_set_rid: widget_set_rid.into(),
            manifest,
        })?;
        self.transport
            .send_json(
                Method::POST,
                "v2/widgets/devModeSettingsV2/setWidgetSetManifest",
                &refs(&q),
                Some(&body),
            )
            .await
    }
}
