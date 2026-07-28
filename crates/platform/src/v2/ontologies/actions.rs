use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

#[derive(Debug, Clone, Default)]
pub struct ActionQuery<'a> {
    pub common: CommonQuery<'a>,
    pub scenario_rid: Option<&'a str>,
    pub transaction_id: Option<&'a str>,
    pub preview: Option<bool>,
}

#[derive(Debug)]
pub struct Actions<'c> {
    transport: &'c Transport,
}

impl<'c> Actions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn apply(
        &self,
        ontology: &str,
        action: &str,
        request: ApplyActionRequest,
        options: ActionQuery<'_>,
    ) -> Result<ApplyActionResponse> {
        self.send(ontology, action, "apply", request, options).await
    }

    pub async fn apply_with_overrides(
        &self,
        ontology: &str,
        action: &str,
        request: ApplyActionWithOverridesRequest,
        options: ActionQuery<'_>,
    ) -> Result<ApplyActionResponse> {
        self.send(ontology, action, "applyWithOverrides", request, options)
            .await
    }

    pub async fn apply_batch(
        &self,
        ontology: &str,
        action: &str,
        request: BatchApplyActionRequest<BatchApplyActionRequestItem>,
        options: ActionQuery<'_>,
    ) -> Result<BatchApplyActionResponse> {
        self.send(ontology, action, "applyBatch", request, options)
            .await
    }

    pub async fn apply_batch_with_overrides(
        &self,
        ontology: &str,
        action: &str,
        request: BatchApplyActionRequest<BatchApplyActionRequestItemWithOverrides>,
        options: ActionQuery<'_>,
    ) -> Result<BatchApplyActionResponse> {
        self.send(
            ontology,
            action,
            "applyBatchWithOverrides",
            request,
            options,
        )
        .await
    }

    async fn send<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        ontology: &str,
        action: &str,
        operation: &str,
        request: T,
        options: ActionQuery<'_>,
    ) -> Result<R> {
        let preview = options.preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = options.common.branch {
            query.push(("branch", v));
        }
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
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
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/ontologies/{ontology}/actions/{action}/{operation}"),
                &query,
                Some(&body),
            )
            .await
    }
}
