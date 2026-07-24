mod error;
mod platform;
mod product;
mod response;
mod updates;

use std::borrow::Cow;

pub use error::Error;
pub use response::Documentation;

const DOCS_HOME: &str = "https://www.palantir.com/docs/foundry/";

pub struct Client {
    http: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    pub async fn get<S>(&self, domain: Domain, scope: &[S]) -> Result<Documentation, Error>
    where
        S: AsRef<str>,
    {
        let scope: Vec<&str> = scope.iter().map(AsRef::as_ref).collect();

        match domain {
            Domain::Product => product::get(self, &scope).await,
            Domain::Platform => platform::get(self, &scope).await,
            Domain::Updates => updates::get(self, &scope).await,
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

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
