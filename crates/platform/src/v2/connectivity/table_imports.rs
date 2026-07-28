use reqwest::Method;

use super::models::*;
use crate::{error::Result, transport::Transport};

#[derive(Debug)]
pub struct TableImports<'c> {
    transport: &'c Transport,
}

impl<'c> TableImports<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn create(
        &self,
        connection_rid: &str,
        request: CreateTableImportRequest,
    ) -> Result<TableImport> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/connectivity/connections/{connection_rid}/tableImports"),
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn delete(&self, connection_rid: &str, table_import_rid: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::DELETE,
                &format!(
                    "v2/connectivity/connections/{connection_rid}/tableImports/{table_import_rid}"
                ),
                &[],
                None,
            )
            .await
    }
    pub async fn execute(&self, connection_rid: &str, table_import_rid: &str) -> Result<BuildRid> {
        self.transport.send_json(Method::POST, &format!("v2/connectivity/connections/{connection_rid}/tableImports/{table_import_rid}/execute"), &[], None).await
    }
    pub async fn get(&self, connection_rid: &str, table_import_rid: &str) -> Result<TableImport> {
        self.transport
            .send_json(
                Method::GET,
                &format!(
                    "v2/connectivity/connections/{connection_rid}/tableImports/{table_import_rid}"
                ),
                &[],
                None,
            )
            .await
    }
    pub async fn list(
        &self,
        connection_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListTableImportsResponse> {
        let size = page_size.map(|value| value.to_string());
        let mut query = Vec::new();
        if let Some(value) = size.as_deref() {
            query.push(("pageSize", value));
        }
        if let Some(value) = page_token {
            query.push(("pageToken", value));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/connectivity/connections/{connection_rid}/tableImports"),
                &query,
                None,
            )
            .await
    }
    pub async fn replace(
        &self,
        connection_rid: &str,
        table_import_rid: &str,
        request: ReplaceTableImportRequest,
    ) -> Result<TableImport> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::PUT,
                &format!(
                    "v2/connectivity/connections/{connection_rid}/tableImports/{table_import_rid}"
                ),
                &[],
                Some(&body),
            )
            .await
    }
}
