use crate::error::Result;
use crate::transport::Transport;
use bytes::Bytes;
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct DatasetResource<'c> {
    transport: &'c Transport,
}

impl<'c> DatasetResource<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn create(&self, name: &str, parent_folder_rid: &str) -> Result<Dataset> {
        let body = serde_json::to_value(CreateDatasetRequest {
            parent_folder_rid: parent_folder_rid.to_owned(),
            name: name.to_owned(),
        })?;
        self.transport
            .send_json(Method::POST, "v2/datasets", &[], Some(&body))
            .await
    }

    pub async fn get(&self, dataset_rid: &str) -> Result<Dataset> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/datasets/{dataset_rid}"),
                &[],
                None,
            )
            .await
    }

    pub async fn get_health_check_reports(
        &self,
        dataset_rid: &str,
        branch_name: Option<&str>,
        preview: Option<bool>,
    ) -> Result<GetHealthCheckReportsResponse> {
        let path = format!("v2/datasets/{dataset_rid}/getHealthCheckReports");
        let preview = preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = branch_name {
            query.push(("branchName", v));
        }
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn get_health_checks(
        &self,
        dataset_rid: &str,
        branch_name: Option<&str>,
        preview: Option<bool>,
    ) -> Result<ListHealthChecksResponse> {
        let path = format!("v2/datasets/{dataset_rid}/getHealthChecks");
        let preview = preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = branch_name {
            query.push(("branchName", v));
        }
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn get_schedules(
        &self,
        dataset_rid: &str,
        branch_name: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListSchedulesResponse> {
        let path = format!("v2/datasets/{dataset_rid}/getSchedules");
        let page_size = page_size.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = branch_name {
            query.push(("branchName", v));
        }
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

    pub async fn get_schema(
        &self,
        dataset_rid: &str,
        branch_name: Option<&str>,
        end_transaction_rid: Option<&str>,
        version_id: Option<&str>,
    ) -> Result<GetDatasetSchemaResponse> {
        let path = format!("v2/datasets/{dataset_rid}/getSchema");
        let mut query = Vec::new();
        if let Some(v) = branch_name {
            query.push(("branchName", v));
        }
        if let Some(v) = end_transaction_rid {
            query.push(("endTransactionRid", v));
        }
        if let Some(v) = version_id {
            query.push(("versionId", v));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn get_schema_batch(
        &self,
        body: &[GetSchemaDatasetsBatchRequestElement],
    ) -> Result<GetSchemaDatasetsBatchResponse> {
        let body = serde_json::to_value(body)?;
        self.transport
            .send_json(Method::POST, "v2/datasets/getSchemaBatch", &[], Some(&body))
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn jobs(
        &self,
        dataset_rid: &str,
        order_by: Vec<GetDatasetJobsSort>,
        branch_name: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<bool>,
        r#where: Option<GetDatasetJobsQuery>,
    ) -> Result<GetJobResponse> {
        let path = format!("v2/datasets/{dataset_rid}/jobs");
        let page_size = page_size.map(|v| v.to_string());
        let preview = preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = branch_name {
            query.push(("branchName", v));
        }
        if let Some(v) = page_size.as_deref() {
            query.push(("pageSize", v));
        }
        if let Some(v) = page_token {
            query.push(("pageToken", v));
        }
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        let body = serde_json::to_value(GetDatasetJobsRequest { r#where, order_by })?;
        self.transport
            .send_json(Method::POST, &path, &query, Some(&body))
            .await
    }

    pub async fn put_schema(
        &self,
        dataset_rid: &str,
        schema: DatasetSchema,
        branch_name: Option<&str>,
        dataframe_reader: Option<DataframeReader>,
        end_transaction_rid: Option<&str>,
    ) -> Result<GetDatasetSchemaResponse> {
        let path = format!("v2/datasets/{dataset_rid}/putSchema");
        let body = serde_json::to_value(PutDatasetSchemaRequest {
            branch_name: branch_name.map(str::to_owned),
            dataframe_reader,
            end_transaction_rid: end_transaction_rid.map(str::to_owned),
            schema_: schema,
        })?;
        self.transport
            .send_json(Method::PUT, &path, &[], Some(&body))
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn read_table(
        &self,
        dataset_rid: &str,
        format: TableExportFormat,
        branch_name: Option<&str>,
        columns: Option<&[String]>,
        end_transaction_rid: Option<&str>,
        row_limit: Option<i32>,
        start_transaction_rid: Option<&str>,
    ) -> Result<Bytes> {
        let path = format!("v2/datasets/{dataset_rid}/readTable");
        let row_limit = row_limit.map(|v| v.to_string());
        let columns = columns.map(|v| v.join(","));
        let mut query = vec![("format", format.as_str())];
        if let Some(v) = branch_name {
            query.push(("branchName", v));
        }
        if let Some(v) = columns.as_deref() {
            query.push(("columns", v));
        }
        if let Some(v) = end_transaction_rid {
            query.push(("endTransactionRid", v));
        }
        if let Some(v) = row_limit.as_deref() {
            query.push(("rowLimit", v));
        }
        if let Some(v) = start_transaction_rid {
            query.push(("startTransactionRid", v));
        }
        self.transport.send_binary(Method::GET, &path, &query).await
    }

    pub async fn transactions(
        &self,
        dataset_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<bool>,
    ) -> Result<ListTransactionsOfDatasetResponse> {
        let path = format!("v2/datasets/{dataset_rid}/transactions");
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
