use std::borrow::Cow;

use serde::Deserialize;

use crate::{Client, Documentation, Error, Item};

pub(crate) mod platform;
pub(crate) mod product;
pub(crate) mod updates;

const DOCS_HOME: &str = "https://www.palantir.com/docs/foundry/";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Domain {
    Product,
    Platform,
    Updates,
}

impl Domain {
    pub const ALL: [Self; 3] = [Self::Product, Self::Platform, Self::Updates];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Product => "product",
            Self::Platform => "platform",
            Self::Updates => "updates",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Product => "Palantir Foundry products' documentation",
            Self::Platform => "Palantir Foundry platform-as-API reference",
            Self::Updates => "Palantir Foundry platform updates",
        }
    }

    pub fn route(self) -> Cow<'static, str> {
        match self {
            Self::Product => DOCS_HOME.into(),
            Self::Platform => format!("{DOCS_HOME}api/v2/").into(),
            Self::Updates => format!("{DOCS_HOME}announcements/").into(),
        }
    }
}
