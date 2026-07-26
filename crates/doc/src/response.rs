use std::{collections::HashMap, sync::Arc};

use bytes::Bytes;
use mime::Mime;

use crate::{Scope, scope::Path};

#[derive(Debug)]
pub enum Documentation {
    Index(Vec<NavItem>),
    Page(Arc<Page>),
}

#[derive(Debug)]
pub enum NavItem {
    Header {
        text: String,
        minimal: bool,
    },
    Section {
        id: String,
        title: String,
        items: Vec<Self>,
    },
    PageGroup {
        title: String,
        pages: Vec<Self>,
    },
    PageLink {
        page_id: String,
        context: String,
        text: String,
        scope: Scope,
    },
    Divider,
}

#[derive(Debug)]
pub struct Page {
    pub scope: Scope,
    pub content: String,
    pub images: HashMap<String, Image>,
    pub preview: bool,
    pub next: Option<Scope>,
    pub prev: Option<Scope>,
}

#[derive(Debug)]
pub struct Image {
    #[allow(dead_code)]
    pub(super) route: Path,
    pub media_type: Mime,
    pub content: Bytes,
}
