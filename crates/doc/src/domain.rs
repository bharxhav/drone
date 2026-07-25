use serde_json::Value;

use crate::{Documentation, Error, Scope, scope::Route};

pub(super) struct PalantirPageData {
    pub domain: Domain,
    pub scope: Scope,
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

    pub const fn path(self) -> &'static [&'static str] {
        match self {
            Self::Product => &["foundry"],
            Self::Platform => &["foundry", "api", "v2"],
            Self::Updates => &["foundry", "announcements"],
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
        let scope = Scope::new(std::iter::once(self.name()).chain(scope.iter().map(AsRef::as_ref)));
        let route = Route::from(scope.clone());

        let html = http
            .get(route.as_ref())
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
            scope,
            content,
        };

        Documentation::try_from(page_data)
    }
}
