use std::todo;

use serde::Deserialize;
use serde_json::Value;

use crate::{Documentation, Error, Scope, domain::Domain};

pub(crate) enum ExtractedPage {
    Generic(GenericPage),
    Specification(SpecPage),
}

impl ExtractedPage {
    /// Parse pageProps into the appropriate wire type based on domain.
    pub fn parse(domain: Domain, page_props: &Value) -> Result<Self, Error> {
        match domain {
            Domain::Platform => {
                let spec: SpecPage = serde_json::from_value(page_props.clone())?;
                Ok(Self::Specification(spec))
            }
            Domain::Product | Domain::Updates => {
                let generic: GenericPage = serde_json::from_value(page_props.clone())?;
                Ok(Self::Generic(generic))
            }
        }
    }

    pub fn doc(&self) -> Option<Documentation> {
        todo!()
    }

    pub fn redirected_doc(&self) -> Option<(Scope, Documentation)> {
        todo!()
    }
}

/// Deserialization target for standard product/updates pages
/// Extracted from `props.pageProps`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenericPage {
    pub markdown: String,
    #[serde(default)]
    pub is_preview: bool,
    #[serde(default)]
    pub page_neighbours: Option<Neighbours>,
}

/// Deserialization target for API v2 pages
/// Extracted from `props.pageProps`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SpecPage {
    pub page: ApiPageContent,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiPageContent {
    pub id: String,
    pub title: String,
    pub content: ApiContent,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiContent {
    pub markdown: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Neighbours {
    pub previous_page: Option<NeighbourLink>,
    pub next_page: Option<NeighbourLink>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct NeighbourLink {
    pub url: String,
}
