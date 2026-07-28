use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct LinkedObjects<'c> {
    transport: &'c Transport,
}

impl<'c> LinkedObjects<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_linked_object(
        &self,
        ontology: &str,
        object_type: &str,
        primary_key: &str,
        link_type: &str,
        linked_primary_key: &str,
        options: ObjectQuery<'_>,
    ) -> Result<OntologyObject> {
        let exclude = options.exclude_rid.map(|v| v.to_string());
        let select = options.select.as_ref().map(|v| v.join(","));
        let mut query = Vec::new();
        if let Some(v) = options.common.branch {
            query.push(("branch", v));
        }
        if let Some(v) = exclude.as_deref() {
            query.push(("excludeRid", v));
        }
        if let Some(v) = options.common.sdk_package_rid {
            query.push(("sdkPackageRid", v));
        }
        if let Some(v) = options.common.sdk_version {
            query.push(("sdkVersion", v));
        }
        if let Some(v) = select.as_deref() {
            query.push(("select", v));
        }
        let path = format!(
            "v2/ontologies/{ontology}/objects/{object_type}/{primary_key}/links/{link_type}/{linked_primary_key}"
        );
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn list_linked_objects(
        &self,
        ontology: &str,
        object_type: &str,
        primary_key: &str,
        link_type: &str,
        options: ListObjectsQuery<'_>,
    ) -> Result<ObjectsPage> {
        let exclude = options.object.exclude_rid.map(|v| v.to_string());
        let page_size = options.page_size.map(|v| v.to_string());
        let snapshot = options.snapshot.map(|v| v.to_string());
        let select = options.object.select.as_ref().map(|v| v.join(","));
        let mut query = Vec::new();
        if let Some(v) = options.object.common.branch {
            query.push(("branch", v));
        }
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
        if let Some(v) = options.object.common.sdk_package_rid {
            query.push(("sdkPackageRid", v));
        }
        if let Some(v) = options.object.common.sdk_version {
            query.push(("sdkVersion", v));
        }
        if let Some(v) = select.as_deref() {
            query.push(("select", v));
        }
        if let Some(v) = snapshot.as_deref() {
            query.push(("snapshot", v));
        }
        let path = format!(
            "v2/ontologies/{ontology}/objects/{object_type}/{primary_key}/links/{link_type}"
        );
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }
}
