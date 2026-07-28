use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

#[derive(Debug, Clone, Default)]
pub struct ObjectSetQuery<'a> {
    pub common: CommonQuery<'a>,
    pub execute_in_memory_only: Option<bool>,
    pub preview: Option<bool>,
    pub scenario_rid: Option<&'a str>,
    pub transaction_id: Option<&'a str>,
}

#[derive(Debug)]
pub struct ObjectSets<'c> {
    transport: &'c Transport,
}

impl<'c> ObjectSets<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn aggregate(
        &self,
        ontology: &str,
        request: AggregateObjectSetRequest,
        options: ObjectSetQuery<'_>,
    ) -> Result<AggregateObjectsResponse> {
        self.post(ontology, "aggregate", request, options).await
    }

    pub async fn create_temporary(
        &self,
        ontology: &str,
        request: CreateTemporaryObjectSetRequest,
        options: ObjectSetQuery<'_>,
    ) -> Result<CreateTemporaryObjectSetResponse> {
        self.post(ontology, "createTemporary", request, options)
            .await
    }

    pub async fn get(
        &self,
        ontology: &str,
        object_set_rid: &str,
        preview: Option<bool>,
    ) -> Result<ObjectSet> {
        let preview = preview.map(|v| v.to_string());
        let query = preview
            .as_deref()
            .map(|v| vec![("preview", v)])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/ontologies/{ontology}/objectSets/{object_set_rid}"),
                &query,
                None,
            )
            .await
    }

    pub async fn load(
        &self,
        ontology: &str,
        request: LoadObjectSetRequest,
        options: ObjectSetQuery<'_>,
    ) -> Result<LoadObjectSetResponse> {
        self.post(ontology, "loadObjects", request, options).await
    }

    pub async fn load_links(
        &self,
        ontology: &str,
        request: LoadObjectSetLinksRequest,
        options: ObjectSetQuery<'_>,
    ) -> Result<LoadObjectSetLinksResponse> {
        self.post(ontology, "loadLinks", request, options).await
    }

    pub async fn load_multiple_object_types(
        &self,
        ontology: &str,
        request: LoadObjectSetRequest,
        options: ObjectSetQuery<'_>,
    ) -> Result<LoadObjectSetResponse> {
        self.post(ontology, "loadObjectsMultipleObjectTypes", request, options)
            .await
    }

    pub async fn load_objects_or_interfaces(
        &self,
        ontology: &str,
        request: LoadObjectSetRequest,
        options: ObjectSetQuery<'_>,
    ) -> Result<LoadObjectSetResponse> {
        self.post(ontology, "loadObjectsOrInterfaces", request, options)
            .await
    }

    async fn post<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        ontology: &str,
        operation: &str,
        request: T,
        options: ObjectSetQuery<'_>,
    ) -> Result<R> {
        let execute = options.execute_in_memory_only.map(|v| v.to_string());
        let preview = options.preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = options.common.branch {
            query.push(("branch", v));
        }
        if let Some(v) = execute.as_deref() {
            query.push(("executeInMemoryOnly", v));
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
                &format!("v2/ontologies/{ontology}/objectSets/{operation}"),
                &query,
                Some(&body),
            )
            .await
    }
}
