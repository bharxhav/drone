use std::borrow::Cow;

use serde_json::Value;

use crate::{Documentation, Error};

const DOCS_HOME: &str = "https://www.palantir.com/docs/foundry/";

pub(super) struct PalantirPageData {
    pub domain: Domain,
    pub scope: Vec<String>,
    pub content: Value,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]

pub(super) enum Domain {
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

    pub fn from_scope(scope: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|domain| domain.name() == scope)
    }

    pub fn route(self) -> Cow<'static, str> {
        match self {
            Self::Product => DOCS_HOME.into(),
            Self::Platform => format!("{DOCS_HOME}api/v2/").into(),
            Self::Updates => format!("{DOCS_HOME}announcements/").into(),
        }
    }

    pub(super) async fn get<S>(
        self,
        http: &reqwest::Client,
        scope: &[S],
    ) -> Result<Documentation, Error>
    where
        S: AsRef<str>,
    {
        let mut route = self.route().into_owned();
        for segment in scope {
            route.push_str(segment.as_ref().trim_matches('/'));
            route.push('/');
        }

        let html = http
            .get(&route)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse("script#__NEXT_DATA__")
            .map_err(|error| Error::Selector(error.to_string()))?;
        let next_data = document
            .select(&selector)
            .next()
            .ok_or(Error::Unavailable)?
            .inner_html();

        let next_data: Value = serde_json::from_str(&next_data)?;
        let content = next_data
            .pointer("/props/pageProps")
            .cloned()
            .ok_or(Error::Unavailable)?;

        let page_data = PalantirPageData {
            domain: self,
            scope: scope
                .iter()
                .map(|segment| segment.as_ref().trim_matches('/').to_owned())
                .collect(),
            content,
        };

        Documentation::try_from(page_data)
    }
}
