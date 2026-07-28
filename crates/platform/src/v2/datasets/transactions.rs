use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct Transactions<'c> {
    transport: &'c Transport,
}

impl<'c> Transactions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    async fn action(
        &self,
        dataset_rid: &str,
        transaction_rid: &str,
        action: &str,
    ) -> Result<Transaction> {
        let path = format!("v2/datasets/{dataset_rid}/transactions/{transaction_rid}/{action}");
        self.transport
            .send_json(Method::POST, &path, &[], None)
            .await
    }

    pub async fn abort(&self, dataset_rid: &str, transaction_rid: &str) -> Result<Transaction> {
        self.action(dataset_rid, transaction_rid, "abort").await
    }
    pub async fn commit(&self, dataset_rid: &str, transaction_rid: &str) -> Result<Transaction> {
        self.action(dataset_rid, transaction_rid, "commit").await
    }

    pub async fn create(
        &self,
        dataset_rid: &str,
        transaction_type: TransactionType,
        branch_name: Option<&str>,
    ) -> Result<Transaction> {
        let path = format!("v2/datasets/{dataset_rid}/transactions");
        let body = serde_json::to_value(CreateTransactionRequest { transaction_type })?;
        let mut query = Vec::new();
        if let Some(v) = branch_name {
            query.push(("branchName", v));
        }
        self.transport
            .send_json(Method::POST, &path, &query, Some(&body))
            .await
    }

    pub async fn get(&self, dataset_rid: &str, transaction_rid: &str) -> Result<Transaction> {
        let path = format!("v2/datasets/{dataset_rid}/transactions/{transaction_rid}");
        self.transport
            .send_json(Method::GET, &path, &[], None)
            .await
    }

    pub async fn build(
        &self,
        dataset_rid: &str,
        transaction_rid: &str,
        preview: Option<bool>,
    ) -> Result<Option<BuildRid>> {
        self.link(dataset_rid, transaction_rid, "build", preview)
            .await
    }

    pub async fn job(
        &self,
        dataset_rid: &str,
        transaction_rid: &str,
        preview: Option<bool>,
    ) -> Result<Option<JobRid>> {
        self.link(dataset_rid, transaction_rid, "job", preview)
            .await
    }

    async fn link<T: serde::de::DeserializeOwned>(
        &self,
        dataset_rid: &str,
        transaction_rid: &str,
        kind: &str,
        preview: Option<bool>,
    ) -> Result<Option<T>> {
        let path = format!("v2/datasets/{dataset_rid}/transactions/{transaction_rid}/{kind}");
        let preview = preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }
}
