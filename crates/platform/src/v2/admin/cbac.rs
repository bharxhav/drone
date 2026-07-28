use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
#[derive(Debug)]
pub struct CbacBanners<'c> {
    transport: &'c Transport,
}
impl<'c> CbacBanners<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(
        &self,
        display_type: Option<ClassificationBannerDisplayType>,
        marking_ids: Option<&[&str]>,
        preview: Option<bool>,
    ) -> Result<CbacBanner> {
        let p = preview.map(|v| v.to_string());
        let ids = marking_ids.map(|v| v.join(","));
        let mut q = Vec::new();
        if let Some(ref v) = display_type {
            q.push(("displayType", v.as_str()))
        }
        if let Some(ref v) = ids {
            q.push(("markingIds", v.as_str()))
        }
        if let Some(ref v) = p {
            q.push(("preview", v.as_str()))
        }
        self.transport
            .send_json(Method::GET, "v2/admin/cbacBanner", &q, None)
            .await
    }
}
#[derive(Debug)]
pub struct CbacMarkingRestrictions<'c> {
    transport: &'c Transport,
}
impl<'c> CbacMarkingRestrictions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(
        &self,
        marking_ids: Option<&[&str]>,
        preview: Option<bool>,
    ) -> Result<super::models::CbacMarkingRestrictions> {
        let p = preview.map(|v| v.to_string());
        let ids = marking_ids.map(|v| v.join(","));
        let mut q = Vec::new();
        if let Some(ref v) = ids {
            q.push(("markingIds", v.as_str()))
        }
        if let Some(ref v) = p {
            q.push(("preview", v.as_str()))
        }
        self.transport
            .send_json(Method::GET, "v2/admin/cbacMarkingRestrictions", &q, None)
            .await
    }
}
