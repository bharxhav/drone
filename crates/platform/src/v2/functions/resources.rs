use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

fn boolean(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

#[derive(Debug)]
pub struct Queries<'c> {
    transport: &'c Transport,
}
impl<'c> Queries<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn get(
        &self,
        api_name: &str,
        version: Option<&str>,
        preview: Option<bool>,
    ) -> Result<Query> {
        let path = format!("v2/functions/queries/{api_name}");
        let mut query = Vec::new();
        if let Some(value) = version {
            query.push(("version", value));
        }
        if let Some(value) = preview {
            query.push(("preview", boolean(value)));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn get_by_rid(
        &self,
        rid: &str,
        include_prerelease: Option<bool>,
        version: Option<&str>,
        preview: Option<bool>,
    ) -> Result<Query> {
        let mut query = vec![("rid", rid)];
        if let Some(value) = include_prerelease {
            query.push(("includePrerelease", boolean(value)));
        }
        if let Some(value) = version {
            query.push(("version", value));
        }
        if let Some(value) = preview {
            query.push(("preview", boolean(value)));
        }
        self.transport
            .send_json(Method::GET, "v2/functions/queries/getByRid", &query, None)
            .await
    }

    pub async fn get_by_rid_batch(
        &self,
        requests: &[GetByRidQueriesBatchRequestElement],
        preview: Option<bool>,
    ) -> Result<GetByRidQueriesBatchResponse> {
        let body = serde_json::to_value(requests)?;
        let mut query = Vec::new();
        if let Some(value) = preview {
            query.push(("preview", boolean(value)));
        }
        self.transport
            .send_json(
                Method::POST,
                "v2/functions/queries/getByRidBatch",
                &query,
                Some(&body),
            )
            .await
    }

    pub async fn execute(
        &self,
        api_name: &str,
        request: &ExecuteQueryRequest,
        transaction_id: Option<&str>,
        preview: Option<bool>,
    ) -> Result<ExecuteQueryResponse> {
        let path = format!("v2/functions/queries/{api_name}/execute");
        let body = serde_json::to_value(request)?;
        let mut query = Vec::new();
        if let Some(value) = transaction_id {
            query.push(("transactionId", value));
        }
        if let Some(value) = preview {
            query.push(("preview", boolean(value)));
        }
        self.transport
            .send_json(Method::POST, &path, &query, Some(&body))
            .await
    }

    pub async fn execute_async(
        &self,
        api_name: &str,
        request: &ExecuteAsyncQueryRequest,
        transaction_id: Option<&str>,
        preview: Option<bool>,
    ) -> Result<ExecuteQueryAsyncResponse> {
        let path = format!("v2/functions/queries/{api_name}/executeAsync");
        let body = serde_json::to_value(request)?;
        let mut query = Vec::new();
        if let Some(value) = transaction_id {
            query.push(("transactionId", value));
        }
        if let Some(value) = preview {
            query.push(("preview", boolean(value)));
        }
        self.transport
            .send_json(Method::POST, &path, &query, Some(&body))
            .await
    }
}

#[derive(Debug)]
pub struct Executions<'c> {
    transport: &'c Transport,
}
impl<'c> Executions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn cancel(
        &self,
        execution_id: &str,
        preview: Option<bool>,
    ) -> Result<CancelExecutionResponse> {
        let path = format!("v2/functions/executions/{execution_id}/cancel");
        let mut query = Vec::new();
        if let Some(value) = preview {
            query.push(("preview", boolean(value)));
        }
        self.transport
            .send_json(Method::POST, &path, &query, None)
            .await
    }
    pub async fn get_result(
        &self,
        execution_id: &str,
        timeout: Option<u32>,
        preview: Option<bool>,
    ) -> Result<GetExecutionResultResponse> {
        let path = format!("v2/functions/executions/{execution_id}/getResult");
        let body = serde_json::to_value(GetResultExecutionRequest { timeout })?;
        let mut query = Vec::new();
        if let Some(value) = preview {
            query.push(("preview", boolean(value)));
        }
        self.transport
            .send_json(Method::POST, &path, &query, Some(&body))
            .await
    }
}

#[derive(Debug)]
pub struct ValueTypes<'c> {
    transport: &'c Transport,
}
impl<'c> ValueTypes<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(&self, rid: &str, preview: Option<bool>) -> Result<ValueType> {
        let path = format!("v2/functions/valueTypes/{rid}");
        let mut query = Vec::new();
        if let Some(value) = preview {
            query.push(("preview", boolean(value)));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }
}

#[derive(Debug)]
pub struct VersionIds<'c> {
    transport: &'c Transport,
}
impl<'c> VersionIds<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(
        &self,
        value_type_rid: &str,
        version_id: &str,
        preview: Option<bool>,
    ) -> Result<VersionId> {
        let path = format!("v2/functions/valueTypes/{value_type_rid}/versionIds/{version_id}");
        let mut query = Vec::new();
        if let Some(value) = preview {
            query.push(("preview", boolean(value)));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }
}
