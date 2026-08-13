use serde::Deserialize;

use super::Client;
use crate::{FoundryError, Rid};

pub mod object_type;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ontology {
    pub api_name: String,
    pub display_name: String,
    pub description: String,
    pub rid: Rid,
}

impl Ontology {
    /// Whether this ontology is identified by `identifier`, which may be a RID or an API name.
    pub fn matches(&self, identifier: impl AsRef<str>) -> bool {
        let identifier = identifier.as_ref();
        self.rid == identifier || self.api_name == identifier
    }
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
        let mut url = self.hostname.join("api/v2/ontologies/")?;
        url.path_segments_mut()
            .map_err(|_| url::ParseError::RelativeUrlWithCannotBeABaseBase)?
            .pop_if_empty()
            .push(ontology.as_ref());
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
