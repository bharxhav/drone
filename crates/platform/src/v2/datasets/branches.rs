use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct Branches<'c> {
    transport: &'c Transport,
}

impl<'c> Branches<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn create(
        &self,
        dataset_rid: &str,
        name: &str,
        transaction_rid: Option<&str>,
    ) -> Result<Branch> {
        let path = format!("v2/datasets/{dataset_rid}/branches");
        let body = serde_json::to_value(CreateBranchRequest {
            transaction_rid: transaction_rid.map(str::to_owned),
            name: name.to_owned(),
        })?;
        self.transport
            .send_json(Method::POST, &path, &[], Some(&body))
            .await
    }

    pub async fn delete(&self, dataset_rid: &str, branch_name: &str) -> Result<()> {
        let path = format!("v2/datasets/{dataset_rid}/branches/{branch_name}");
        self.transport
            .send_no_content(Method::DELETE, &path, &[], None)
            .await
    }

    pub async fn get(&self, dataset_rid: &str, branch_name: &str) -> Result<Branch> {
        let path = format!("v2/datasets/{dataset_rid}/branches/{branch_name}");
        self.transport
            .send_json(Method::GET, &path, &[], None)
            .await
    }

    pub async fn list(
        &self,
        dataset_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListBranchesResponse> {
        let path = format!("v2/datasets/{dataset_rid}/branches");
        let page_size = page_size.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = page_size.as_deref() {
            query.push(("pageSize", v));
        }
        if let Some(v) = page_token {
            query.push(("pageToken", v));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn transactions(
        &self,
        dataset_rid: &str,
        branch_name: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<bool>,
    ) -> Result<ListTransactionsResponse> {
        let path = format!("v2/datasets/{dataset_rid}/branches/{branch_name}/transactions");
        let page_size = page_size.map(|v| v.to_string());
        let preview = preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = page_size.as_deref() {
            query.push(("pageSize", v));
        }
        if let Some(v) = page_token {
            query.push(("pageToken", v));
        }
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }
}
