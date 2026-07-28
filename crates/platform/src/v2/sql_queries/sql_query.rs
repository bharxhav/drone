use super::models::*;
use crate::{error::Result, transport::Transport};
use bytes::Bytes;
use reqwest::Method;

#[derive(Debug)]
pub struct SqlQueryResource<'c> {
    transport: &'c Transport,
}

impl<'c> SqlQueryResource<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn cancel(&self, sql_query_id: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/sqlQueries/{sql_query_id}/cancel"),
                &[],
                None,
            )
            .await
    }

    pub async fn execute(
        &self,
        query: &str,
        fallback_branch_ids: Option<Vec<String>>,
        serialization_format: Option<SerializationFormat>,
    ) -> Result<QueryStatus> {
        let body = serde_json::to_value(ExecuteSqlQueryRequest {
            query: query.to_owned(),
            fallback_branch_ids,
            serialization_format,
        })?;
        self.transport
            .send_json(Method::POST, "v2/sqlQueries/execute", &[], Some(&body))
            .await
    }

    pub async fn execute_ontology(
        &self,
        request: ExecuteOntologySqlQueryRequest,
        preview: Option<bool>,
    ) -> Result<Bytes> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        let body = serde_json::to_value(request)?;
        self.transport
            .send_binary_with(
                Method::POST,
                "v2/sqlQueries/executeOntology",
                &query,
                reqwest::header::HeaderMap::new(),
                Some(crate::transport::RequestBody::Json(body)),
            )
            .await
    }

    pub async fn get_results(&self, sql_query_id: &str) -> Result<Bytes> {
        self.transport
            .send_binary(
                Method::GET,
                &format!("v2/sqlQueries/{sql_query_id}/getResults"),
                &[],
            )
            .await
    }

    pub async fn get_status(&self, sql_query_id: &str) -> Result<QueryStatus> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/sqlQueries/{sql_query_id}/getStatus"),
                &[],
                None,
            )
            .await
    }
}
