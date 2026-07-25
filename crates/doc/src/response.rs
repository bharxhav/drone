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
pub struct NavItem {}

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
    pub(super) route: Path,
    pub media_type: Mime,
    pub content: Bytes,
}
