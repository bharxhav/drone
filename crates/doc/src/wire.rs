use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;
use serde_json::Value;

use crate::{Documentation, Error, NavItem, Page, Scope, domain::Domain, scope::Path};

pub(crate) enum ExtractedPage {
    Home(HomePage),
    Generic(GenericPage),
    Specification(Box<SpecPage>),
}

impl ExtractedPage {
    fn page(&self) -> Option<Documentation> {
        match self {
            Self::Home(_) => None,
            Self::Generic(page) => Some(Documentation::Page(Arc::new(page.response()))),
            Self::Specification(page) => Some(Documentation::Page(Arc::new(page.response()))),
        }
    }

    fn redirected(&self) -> bool {
        match self {
            Self::Home(_) => false,
            Self::Generic(page) => page.should_skip_pagefind_index,
            Self::Specification(page) => page.should_skip_pagefind_index,
        }
    }

    fn index(&self) -> Result<Vec<NavItem>, Error> {
        match self {
            Self::Home(page) => page.index(),
            Self::Generic(page) => page
                .sidebar_nav_props
                .items
                .clone()
                .into_iter()
                .map(TryInto::try_into)
                .collect(),
            Self::Specification(page) => page
                .sidebar_nav_items
                .clone()
                .into_iter()
                .map(TryInto::try_into)
                .collect(),
        }
    }

    #[allow(dead_code)]
    fn delivered_scope(&self) -> Option<Scope> {
        match self {
            Self::Home(_) => None,
            Self::Generic(page) => Some(page.scope()),
            Self::Specification(page) => Some(page.scope()),
        }
    }
}

impl ExtractedPage {
    /// Parse pageProps into the appropriate wire type based on domain and scope.
    pub fn parse(domain: Domain, scope: &[String], page_props: &Value) -> Result<Self, Error> {
        match domain {
            Domain::Product if scope.is_empty() => {
                let home: HomePage = serde_json::from_value(page_props.clone())?;
                Ok(Self::Home(home))
            }
            Domain::Platform => {
                let spec: SpecPage = serde_json::from_value(page_props.clone())?;
                Ok(Self::Specification(Box::new(spec)))
            }
            Domain::Product | Domain::Updates => {
                let generic: GenericPage = serde_json::from_value(page_props.clone())?;
                Ok(Self::Generic(generic))
            }
        }
    }

    /// Returns Index if Redirected, else a Page.
    pub fn doc(&self) -> Result<Option<Documentation>, Error> {
        if matches!(self, Self::Home(_)) || self.redirected() {
            Ok(Some(Documentation::Index(self.index()?)))
        } else {
            Ok(self.page())
        }
    }

    /// Returns the post redirection Page.
    #[allow(dead_code)]
    pub fn redirected_doc(&self) -> Option<(Scope, Documentation)> {
        self.redirected()
            .then(|| Some((self.delivered_scope()?, self.page()?)))
            .flatten()
    }
}

/// Deserialization target for the product domain root (`/docs/foundry/`).
/// Extracted from `props.pageProps`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HomePage {
    pub docs_homepage_config: HomeConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HomeConfig {
    pub reference_cells: Vec<ReferenceCell>,
    pub product_reference_cells: Vec<ReferenceCell>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReferenceCell {
    pub header_link: HomeLink,
    #[serde(default)]
    pub links: Vec<HomeLink>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct HomeLink {
    pub text: String,
    pub url: String,
}

impl HomePage {
    fn index(&self) -> Result<Vec<NavItem>, Error> {
        self.docs_homepage_config
            .reference_cells
            .iter()
            .chain(&self.docs_homepage_config.product_reference_cells)
            .map(|cell| {
                Ok(NavItem::Section {
                    id: scope_from_url(&cell.header_link.url)?.to_string(),
                    title: cell.header_link.text.clone(),
                    items: cell
                        .links
                        .iter()
                        .map(HomeLink::nav_item)
                        .collect::<Result<_, _>>()?,
                })
            })
            .collect()
    }
}

impl HomeLink {
    fn nav_item(&self) -> Result<NavItem, Error> {
        let scope = scope_from_url(&self.url)?;
        Ok(NavItem::PageLink {
            page_id: scope.tail().last().cloned().unwrap_or_default(),
            context: scope.tail().first().cloned().unwrap_or_default(),
            text: self.text.clone(),
            scope,
        })
    }
}

/// Deserialization target for standard product/updates pages
/// Extracted from `props.pageProps`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenericPage {
    pub context: String,
    pub page_id: String,
    pub markdown: String,
    #[serde(default)]
    pub is_preview: bool,
    #[serde(default)]
    pub page_neighbours: Option<Neighbours>,
    #[serde(default)]
    pub should_skip_pagefind_index: bool,
    pub sidebar_nav_props: SidebarNav,
}

impl GenericPage {
    fn scope(&self) -> Scope {
        if self.context == "announcements" {
            Scope::new(["updates", self.page_id.as_str()])
        } else {
            Scope::new(["product", self.context.as_str(), self.page_id.as_str()])
        }
    }

    fn response(&self) -> Page {
        Page {
            scope: self.scope(),
            content: self.markdown.clone(),
            images: HashMap::new(),
            preview: self.is_preview,
            prev: self.page_neighbours.as_ref().and_then(|neighbours| {
                neighbours
                    .previous_page
                    .as_ref()
                    .and_then(|link| scope_from_url(&link.url).ok())
            }),
            next: self.page_neighbours.as_ref().and_then(|neighbours| {
                neighbours
                    .next_page
                    .as_ref()
                    .and_then(|link| scope_from_url(&link.url).ok())
            }),
        }
    }
}

/// Deserialization target for API v2 pages
/// Extracted from `props.pageProps`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SpecPage {
    pub page: ApiPageContent,
    pub category_id: String,
    pub section_id: String,
    pub page_id: String,
    #[serde(default)]
    pub should_skip_pagefind_index: bool,
    pub sidebar_nav_items: Vec<WireNavItem>,
}

impl SpecPage {
    fn scope(&self) -> Scope {
        Scope::new([
            "platform",
            self.category_id.as_str(),
            self.section_id.as_str(),
            self.page_id.as_str(),
        ])
    }

    fn response(&self) -> Page {
        Page {
            scope: self.scope(),
            content: self.page.markdown(),
            images: HashMap::new(),
            preview: false,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiPageContent {
    pub title: String,
    pub content: ApiContent,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub(crate) enum ApiContent {
    Markdown { markdown: String },
    Endpoint { endpoint: Box<ApiEndpoint> },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiEndpoint {
    pub path: String,
    pub operation_type: ApiOperation,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub scopes: Vec<ApiScope>,
    #[serde(default)]
    pub path_parameters: Vec<ApiParameter>,
    #[serde(default)]
    pub query_parameters: Vec<ApiParameter>,
    pub request: Option<ApiBody>,
    pub response: Option<ApiResponse>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiOperation {
    #[serde(rename = "type")]
    pub method: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiScope {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiParameter {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiBody {
    pub body: ApiParameter,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiResponse {
    pub body: ApiParameter,
}

impl ApiPageContent {
    fn markdown(&self) -> String {
        match &self.content {
            ApiContent::Markdown { markdown } => markdown.clone(),
            ApiContent::Endpoint { endpoint } => endpoint.markdown(&self.title),
        }
    }
}

impl ApiEndpoint {
    fn markdown(&self, title: &str) -> String {
        let mut output = format!(
            "# {title}\n\n`{} {}`\n\n{}",
            self.operation_type.method.to_uppercase(),
            self.path,
            self.description.trim()
        );

        if !self.scopes.is_empty() {
            output.push_str("\n\n## OAuth scopes\n\n");
            for scope in &self.scopes {
                output.push_str(&format!("- `{}`\n", scope.name));
            }
        }
        write_parameters(&mut output, "Path parameters", &self.path_parameters);
        write_parameters(&mut output, "Query parameters", &self.query_parameters);
        if let Some(request) = &self.request {
            write_parameters(&mut output, "Request", std::slice::from_ref(&request.body));
        }
        if let Some(response) = &self.response {
            write_parameters(
                &mut output,
                "Response",
                std::slice::from_ref(&response.body),
            );
        }
        output
    }
}

fn write_parameters(output: &mut String, title: &str, parameters: &[ApiParameter]) {
    if parameters.is_empty() {
        return;
    }
    output.push_str(&format!("\n\n## {title}\n\n"));
    for parameter in parameters {
        let required = if parameter.required { " required" } else { "" };
        output.push_str(&format!("### `{}`{required}\n", parameter.name));
        if let Some(description) = &parameter.description {
            output.push_str(&format!("\n{}\n", description.trim()));
        }
    }
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

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct SidebarNav {
    pub items: Vec<WireNavItem>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum WireNavItem {
    #[serde(rename = "header")]
    Header {
        text: String,
        #[serde(default)]
        minimal: bool,
    },
    #[serde(rename = "section")]
    Section {
        id: String,
        title: String,
        items: Vec<Self>,
    },
    #[serde(rename = "pageGroup")]
    PageGroup { title: String, pages: Vec<Self> },
    #[serde(rename = "pageLink")]
    PageLink {
        #[serde(rename = "pageId")]
        page_id: String,
        context: String,
        link: WireLink,
    },
    #[serde(rename = "divider")]
    Divider,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct WireLink {
    pub text: String,
    pub url: String,
}

impl TryFrom<WireNavItem> for NavItem {
    type Error = Error;

    fn try_from(item: WireNavItem) -> Result<Self, Self::Error> {
        Ok(match item {
            WireNavItem::Header { text, minimal } => Self::Header { text, minimal },
            WireNavItem::Section { id, title, items } => Self::Section {
                id,
                title,
                items: items
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            },
            WireNavItem::PageGroup { title, pages } => Self::PageGroup {
                title,
                pages: pages
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            },
            WireNavItem::PageLink {
                page_id,
                context,
                link,
            } => Self::PageLink {
                page_id,
                context,
                text: link.text,
                scope: scope_from_url(&link.url)?,
            },
            WireNavItem::Divider => Self::Divider,
        })
    }
}

fn scope_from_url(url: &str) -> Result<Scope, Error> {
    Scope::try_from(Path::new(url)).map_err(|path| Error::NotFound(path.to_string()))
}
