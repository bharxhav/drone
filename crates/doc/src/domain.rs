use std::borrow::Cow;

use serde::Deserialize;

use crate::{Client, Documentation, Error};

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

    pub(super) async fn get<S>(self, client: &Client, scope: &[S]) -> Result<Documentation, Error>
    where
        S: AsRef<str>,
    {
        let mut route = self.route().into_owned();
        for segment in scope {
            route.push_str(segment.as_ref().trim_matches('/'));
            route.push('/');
        }

        let html = client
            .http
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

        let next_data: NextData = serde_json::from_str(&next_data)?;

        Documentation::try_from((self, next_data.props.page_props))
    }
}

#[derive(Deserialize)]
struct NextData {
    props: Props,
}

#[derive(Deserialize)]
struct Props {
    #[serde(rename = "pageProps")]
    page_props: PalantirPageProps,
}

#[derive(Deserialize)]
pub(super) struct PalantirPageProps(pub(super) serde_json::Value);
