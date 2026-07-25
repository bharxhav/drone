use std::todo;

use crate::{Domain, Error, domain::PalantirPageProps};

#[derive(Debug)]
pub enum Documentation {
    Index(Vec<NavItem>),
    Page(Page),
}

#[derive(Debug)]
pub struct NavItem {}

#[derive(Debug)]
pub struct Page {}

impl TryFrom<(Domain, PalantirPageProps)> for Documentation {
    type Error = Error;

    fn try_from((domain, page_props): (Domain, PalantirPageProps)) -> Result<Self, Self::Error> {
        todo!()
    }
}
