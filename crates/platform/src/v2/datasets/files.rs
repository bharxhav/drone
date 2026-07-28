use crate::error::Result;
use crate::transport::Transport;
use bytes::Bytes;
use reqwest::Method;
use reqwest::header::{CONTENT_TYPE, HeaderValue};

use super::models::*;

#[derive(Debug)]
pub struct Files<'c> {
    transport: &'c Transport,
}

impl<'c> Files<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    fn range_query<'a>(
        branch_name: Option<&'a str>,
        end_transaction_rid: Option<&'a str>,
        start_transaction_rid: Option<&'a str>,
    ) -> Vec<(&'static str, &'a str)> {
        let mut query = Vec::new();
        if let Some(v) = branch_name {
            query.push(("branchName", v));
        }
        if let Some(v) = end_transaction_rid {
            query.push(("endTransactionRid", v));
        }
        if let Some(v) = start_transaction_rid {
            query.push(("startTransactionRid", v));
        }
        query
    }

    pub async fn content(
        &self,
        dataset_rid: &str,
        file_path: &str,
        branch_name: Option<&str>,
        end_transaction_rid: Option<&str>,
        start_transaction_rid: Option<&str>,
    ) -> Result<Bytes> {
        let path = format!("v2/datasets/{dataset_rid}/files/{file_path}/content");
        self.transport
            .send_binary(
                Method::GET,
                &path,
                &Self::range_query(branch_name, end_transaction_rid, start_transaction_rid),
            )
            .await
    }

    pub async fn delete(
        &self,
        dataset_rid: &str,
        file_path: &str,
        branch_name: Option<&str>,
        transaction_rid: Option<&str>,
    ) -> Result<()> {
        let path = format!("v2/datasets/{dataset_rid}/files/{file_path}");
        let mut query = Vec::new();
        if let Some(v) = branch_name {
            query.push(("branchName", v));
        }
        if let Some(v) = transaction_rid {
            query.push(("transactionRid", v));
        }
        self.transport
            .send_no_content(Method::DELETE, &path, &query, None)
            .await
    }

    pub async fn get(
        &self,
        dataset_rid: &str,
        file_path: &str,
        branch_name: Option<&str>,
        end_transaction_rid: Option<&str>,
        start_transaction_rid: Option<&str>,
    ) -> Result<File> {
        let path = format!("v2/datasets/{dataset_rid}/files/{file_path}");
        self.transport
            .send_json(
                Method::GET,
                &path,
                &Self::range_query(branch_name, end_transaction_rid, start_transaction_rid),
                None,
            )
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        dataset_rid: &str,
        branch_name: Option<&str>,
        end_transaction_rid: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
        path_prefix: Option<&str>,
        start_transaction_rid: Option<&str>,
    ) -> Result<ListFilesResponse> {
        let path = format!("v2/datasets/{dataset_rid}/files");
        let page_size = page_size.map(|v| v.to_string());
        let mut query = Self::range_query(branch_name, end_transaction_rid, start_transaction_rid);
        if let Some(v) = page_size.as_deref() {
            query.push(("pageSize", v));
        }
        if let Some(v) = page_token {
            query.push(("pageToken", v));
        }
        if let Some(v) = path_prefix {
            query.push(("pathPrefix", v));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn upload(
        &self,
        dataset_rid: &str,
        file_path: &str,
        body: impl Into<Bytes>,
        branch_name: Option<&str>,
        transaction_rid: Option<&str>,
        transaction_type: Option<TransactionType>,
    ) -> Result<File> {
        let path = format!("v2/datasets/{dataset_rid}/files/{file_path}/upload");
        let mut query = Vec::new();
        if let Some(v) = branch_name {
            query.push(("branchName", v));
        }
        if let Some(v) = transaction_rid {
            query.push(("transactionRid", v));
        }
        if let Some(v) = transaction_type.as_ref() {
            query.push(("transactionType", v.as_str()));
        }
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        let response = self
            .transport
            .send(
                Method::POST,
                &path,
                &query,
                headers,
                Some(crate::transport::RequestBody::Bytes(body.into())),
            )
            .await?;
        Ok(response.json().await?)
    }
}
