use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct OntologyResource<'c> {
    transport: &'c Transport,
}

impl<'c> OntologyResource<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn get(&self, ontology: &str) -> Result<Ontology> {
        self.transport
            .send_json(Method::GET, &format!("v2/ontologies/{ontology}"), &[], None)
            .await
    }

    pub async fn list(&self) -> Result<ListOntologiesResponse> {
        self.transport
            .send_json(Method::GET, "v2/ontologies", &[], None)
            .await
    }

    pub async fn get_full_metadata(
        &self,
        ontology: &str,
        branch: Option<&str>,
        preview: Option<bool>,
    ) -> Result<OntologyFullMetadata> {
        let preview = preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = branch {
            query.push(("branch", v));
        }
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/ontologies/{ontology}/fullMetadata"),
                &query,
                None,
            )
            .await
    }

    pub async fn load_metadata(
        &self,
        ontology: &str,
        request: LoadOntologyMetadataRequest,
        branch: Option<&str>,
        preview: Option<bool>,
    ) -> Result<OntologyFullMetadata> {
        let preview = preview.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = branch {
            query.push(("branch", v));
        }
        if let Some(v) = preview.as_deref() {
            query.push(("preview", v));
        }
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/ontologies/{ontology}/metadata"),
                &query,
                Some(&body),
            )
            .await
    }
}
