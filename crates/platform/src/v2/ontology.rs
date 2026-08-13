use serde::Deserialize;

use super::Client;
use crate::{FoundryError, Rid};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ontology {
    pub api_name: String,
    pub display_name: String,
    pub description: String,
    pub rid: Rid,
}

impl Client {
    pub async fn list_ontologies(&self) -> Result<Vec<Ontology>, FoundryError> {
        #[derive(Deserialize)]
        struct Response {
            data: Vec<Ontology>,
        }

        let url = self.hostname.join("api/v2/ontologies")?;
        let response: Response = self
            .http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response.data)
    }

    /// Fetches a single ontology by its API name or RID.
    pub async fn get_ontology(&self, ontology: impl AsRef<str>) -> Result<Ontology, FoundryError> {
        let url = self
            .hostname
            .join("api/v2/ontologies/")?
            .join(ontology.as_ref())?;
        self.http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }
}
