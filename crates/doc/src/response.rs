use std::{collections::HashMap, sync::Arc, todo};

use bytes::Bytes;
use mime::Mime;

use crate::{Domain, Error, Scope, domain::PalantirPageData, scope::Path};

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
    // `markdown`
    pub content: String,
    pub images: HashMap<String, Image>,
    pub preview: bool,
    // `pageNeighbors.previousPage.url` into()
    pub next: Option<Scope>,
    // `pageNeighbors.nextPage.url` into()
    pub prev: Option<Scope>,
}

#[derive(Debug)]
pub struct Image {
    pub(super) route: Path,
    pub media_type: Mime,
    pub content: Bytes,
}

impl TryFrom<PalantirPageData> for Documentation {
    type Error = Error;

    fn try_from(page_props: PalantirPageData) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<[Domain; 3]> for Documentation {
    fn from(domains: [Domain; 3]) -> Self {
        todo!()
    }
}
