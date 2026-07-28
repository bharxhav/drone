use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

fn common_query<'a>(q: &'a CommonQuery<'a>) -> Vec<(&'static str, &'a str)> {
    let mut out = Vec::new();
    if let Some(v) = q.branch {
        out.push(("branch", v));
    }
    if let Some(v) = q.sdk_package_rid {
        out.push(("sdkPackageRid", v));
    }
    if let Some(v) = q.sdk_version {
        out.push(("sdkVersion", v));
    }
    out
}

#[derive(Debug)]
pub struct Objects<'c> {
    transport: &'c Transport,
}

impl<'c> Objects<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn aggregate(
        &self,
        ontology: &str,
        object_type: &str,
        request: AggregateObjectsRequest,
        common: CommonQuery<'_>,
    ) -> Result<AggregateObjectsResponse> {
        let query = common_query(&common);
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/ontologies/{ontology}/objects/{object_type}/aggregate"),
                &query,
                Some(&body),
            )
            .await
    }

    pub async fn count(
        &self,
        ontology: &str,
        object_type: &str,
        common: CommonQuery<'_>,
        preview: Option<bool>,
    ) -> Result<CountObjectsResponse> {
        let preview = preview.map(|v| v.to_string());
        let mut query = common_query(&common);
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/ontologies/{ontology}/objects/{object_type}/count"),
                &query,
                None,
            )
            .await
    }

    pub async fn get(
        &self,
        ontology: &str,
        object_type: &str,
        primary_key: &str,
        options: ObjectQuery<'_>,
    ) -> Result<OntologyObject> {
        let exclude = options.exclude_rid.map(|v| v.to_string());
        let select = options.select.as_ref().map(|v| v.join(","));
        let mut query = common_query(&options.common);
        if let Some(v) = exclude.as_deref() {
            query.push(("excludeRid", v));
        }
        if let Some(v) = select.as_deref() {
            query.push(("select", v));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/ontologies/{ontology}/objects/{object_type}/{primary_key}"),
                &query,
                None,
            )
            .await
    }

    pub async fn list(
        &self,
        ontology: &str,
        object_type: &str,
        options: ListObjectsQuery<'_>,
    ) -> Result<ObjectsPage> {
        let exclude = options.object.exclude_rid.map(|v| v.to_string());
        let page_size = options.page_size.map(|v| v.to_string());
        let snapshot = options.snapshot.map(|v| v.to_string());
        let select = options.object.select.as_ref().map(|v| v.join(","));
        let mut query = common_query(&options.object.common);
        if let Some(v) = exclude.as_deref() {
            query.push(("excludeRid", v));
        }
        if let Some(v) = options.order_by {
            query.push(("orderBy", v));
        }
        if let Some(v) = page_size.as_deref() {
            query.push(("pageSize", v));
        }
        if let Some(v) = options.page_token {
            query.push(("pageToken", v));
        }
        if let Some(v) = select.as_deref() {
            query.push(("select", v));
        }
        if let Some(v) = snapshot.as_deref() {
            query.push(("snapshot", v));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/ontologies/{ontology}/objects/{object_type}"),
                &query,
                None,
            )
            .await
    }

    pub async fn search(
        &self,
        ontology: &str,
        object_type: &str,
        request: SearchObjectsRequest,
        common: CommonQuery<'_>,
        execute_in_memory_only: Option<bool>,
    ) -> Result<SearchObjectsResponse> {
        let execute = execute_in_memory_only.map(|v| v.to_string());
        let mut query = common_query(&common);
        if let Some(v) = execute.as_deref() {
            query.push(("executeInMemoryOnly", v));
        }
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/ontologies/{ontology}/objects/{object_type}/search"),
                &query,
                Some(&body),
            )
            .await
    }
}
