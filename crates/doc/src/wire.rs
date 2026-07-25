use std::collections::HashMap;

use serde::Deserialize;

pub(crate) enum ExtractedPage {
    Generic(GenericPage),
    Specification(SpecPage),
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
