use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
#[derive(Debug)]
pub struct Roles<'c> {
    transport: &'c Transport,
}
impl<'c> Roles<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(&self, id: &str, preview: Option<bool>) -> Result<Role> {
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(Method::GET, &format!("v2/admin/roles/{id}"), &q, None)
            .await
    }
    pub async fn get_batch(
        &self,
        r: Vec<GetRolesBatchRequestElement>,
        preview: Option<bool>,
    ) -> Result<GetRolesBatchResponse> {
        let b = serde_json::to_value(r)?;
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(Method::POST, "v2/admin/roles/getBatch", &q, Some(&b))
            .await
    }
}
