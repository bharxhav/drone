use reqwest::header::CONTENT_TYPE;
use serde_json::Value;

use crate::{
    Documentation, Error, Image, Scope,
    scope::{Path, Route},
    wire::ExtractedPage,
};

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
        let page_props = next_data
            .pointer("/props/pageProps")
            .ok_or(Error::Unavailable)?;

        let extracted = ExtractedPage::parse(self, scope.tail(), page_props)?;

        extracted
            .doc()?
            .ok_or_else(|| Error::NotFound(scope.to_string()))
    }

    #[allow(dead_code)]
    pub(super) async fn resolve_image(
        self,
        http: &reqwest::Client,
        path: Path,
    ) -> Result<Image, Error> {
        let route = Route::from(path.clone());

        let response = http.get(route.as_ref()).send().await?.error_for_status()?;
        let media_type: mime::Mime = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .ok_or(Error::MissingMediaType)?
            .parse::<mime::Mime>()
            .map_err(|error| Error::MediaType(error.to_string()))?;
        let content = response.bytes().await?;

        Ok(Image {
            route: path,
            media_type,
            content,
        })
    }
}
