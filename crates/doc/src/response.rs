use std::todo;

use crate::{Domain, Error, domain::PalantirPageData};

#[derive(Debug)]
pub enum Documentation {
    Index(Vec<NavItem>),
    Page(Page),
}

#[derive(Debug)]
pub struct NavItem {}

#[derive(Debug)]
pub struct Page {}

impl TryFrom<PalantirPageData> for Documentation {
    type Error = Error;

    fn try_from(page_props: PalantirPageData) -> Result<Self, Self::Error> {
        todo!()
    }
}
