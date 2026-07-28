use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

#[derive(Debug, Clone, Default)]
pub struct ExecuteQueryOptions<'a> {
    pub common: CommonQuery<'a>,
    pub scenario_rid: Option<&'a str>,
    pub transaction_id: Option<&'a str>,
    pub version: Option<&'a str>,
}

#[derive(Debug)]
pub struct Queries<'c> {
    transport: &'c Transport,
}

impl<'c> Queries<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn execute(
        &self,
        ontology: &str,
        query_api_name: &str,
        request: ExecuteQueryRequest,
        options: ExecuteQueryOptions<'_>,
    ) -> Result<ExecuteQueryResponse> {
        let mut query = Vec::new();
        if let Some(v) = options.common.branch {
            query.push(("branch", v));
        }
        if let Some(v) = options.scenario_rid {
            query.push(("scenarioRid", v));
        }
        if let Some(v) = options.common.sdk_package_rid {
            query.push(("sdkPackageRid", v));
        }
        if let Some(v) = options.common.sdk_version {
            query.push(("sdkVersion", v));
        }
        if let Some(v) = options.transaction_id {
            query.push(("transactionId", v));
        }
        if let Some(v) = options.version {
            query.push(("version", v));
        }
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/ontologies/{ontology}/queries/{query_api_name}/execute"),
                &query,
                Some(&body),
            )
            .await
    }
}
