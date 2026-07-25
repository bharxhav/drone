//! URL representations used by the documentation client.
//!
//! Palantir's documentation site is rooted at `https://www.palantir.com/docs/`. Routes in
//! its payloads, such as `/foundry/notepad/aip-features/`, are relative to that root. The
//! same location therefore has three equivalent representations:
//!
//! ```text
//! Scope  product/notepad/aip-features
//! Path   foundry/notepad/aip-features
//! Route  https://www.palantir.com/docs/foundry/notepad/aip-features/
//! ```
//!
//! `Scope` is the public, domain-oriented input. `Path` is Palantir's payload form, and
//! `Route` is the fully resolved URL used for HTTP requests.

use crate::domain::Domain;

/// URL with `https://www.palantir.com/docs/`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct Route(String);

impl Route {
    pub(super) fn new(path: impl AsRef<str>) -> Self {
        Self(format!(
            "{}{}",
            Path::base_url(),
            path.as_ref().trim_start_matches('/')
        ))
    }
}

impl AsRef<str> for Route {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Path> for Route {
    fn from(path: Path) -> Self {
        Self::new(path.to_string())
    }
}

impl From<Scope> for Route {
    fn from(scope: Scope) -> Self {
        Path::from(scope).into()
    }
}

/// A path relative to `https://www.palantir.com/docs/`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct Path(Vec<String>);

impl Path {
    const fn base_url() -> &'static str {
        "https://www.palantir.com/docs/"
    }

    fn new(path: &str) -> Self {
        Self(
            path.split('/')
                .filter(|segment| !segment.is_empty())
                .map(str::to_owned)
                .collect(),
        )
    }

    fn as_segments(&self) -> &[String] {
        &self.0
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0.join("/"))
    }
}

impl TryFrom<Route> for Path {
    type Error = Route;

    fn try_from(route: Route) -> Result<Self, Self::Error> {
        route
            .0
            .strip_prefix(Self::base_url())
            .map(Self::new)
            .ok_or(route)
    }
}

impl From<Scope> for Path {
    fn from(scope: Scope) -> Self {
        let Some(domain) = scope.domain() else {
            return Self::new(&scope.to_string());
        };
        let mut segments = domain
            .path()
            .iter()
            .map(|segment| (*segment).to_owned())
            .collect::<Vec<_>>();
        segments.extend(scope.into_tail());
        Self(segments)
    }
}

/// A client path beginning with `product`, `platform`, or `updates`.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Scope(Vec<String>);

impl Scope {
    pub fn new(segments: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        Self(
            segments
                .into_iter()
                .flat_map(|segment| {
                    segment
                        .as_ref()
                        .split('/')
                        .filter(|segment| !segment.is_empty())
                        .map(str::to_owned)
                        .collect::<Vec<_>>()
                })
                .collect(),
        )
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub(super) fn domain(&self) -> Option<Domain> {
        self.domain_name().and_then(Domain::from_scope)
    }

    pub(super) fn domain_name(&self) -> Option<&str> {
        self.0.first().map(String::as_str)
    }

    pub(super) fn tail(&self) -> &[String] {
        self.0.get(1..).unwrap_or_default()
    }

    fn into_tail(self) -> impl Iterator<Item = String> {
        self.0.into_iter().skip(1)
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0.join("/"))
    }
}

impl TryFrom<Path> for Scope {
    type Error = Path;

    fn try_from(path: Path) -> Result<Self, Self::Error> {
        let mut domains = Domain::ALL;
        domains.sort_by_key(|domain| std::cmp::Reverse(domain.path().len()));
        for domain in domains {
            let prefix = domain.path();
            let segments = path.as_segments();
            if segments.len() >= prefix.len()
                && segments
                    .iter()
                    .zip(prefix)
                    .all(|(segment, prefix)| segment == prefix)
            {
                let mut segments = vec![domain.name().to_owned()];
                segments.extend_from_slice(&path.as_segments()[prefix.len()..]);
                return Ok(Self::new(segments));
            }
        }

        Err(path)
    }
}

impl TryFrom<Route> for Scope {
    type Error = Route;

    fn try_from(route: Route) -> Result<Self, Self::Error> {
        let path = Path::try_from(route.clone()).map_err(|_| route.clone())?;
        Self::try_from(path).map_err(|_| route)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_scope_is_root() {
        let scope = Scope::new([] as [&str; 0]);

        assert!(scope.is_root());
        assert_eq!(scope.to_string(), "");
        assert_eq!(scope.domain_name(), None);
        assert_eq!(scope.domain(), None);
        assert!(scope.tail().is_empty());
    }

    #[test]
    fn scope_preserves_segments() {
        let scope = Scope::new(["product", "notepad", "aip-features"]);

        assert!(!scope.is_root());
        assert_eq!(scope.to_string(), "product/notepad/aip-features");
        assert_eq!(scope.domain_name(), Some("product"));
        assert_eq!(scope.domain(), Some(Domain::Product));
        assert_eq!(scope.tail(), ["notepad", "aip-features"]);
    }

    #[test]
    fn scope_normalizes_slashes_and_empty_segments() {
        let scope = Scope::new(["/platform/", "", "//ontology/objects//"]);

        assert_eq!(scope.to_string(), "platform/ontology/objects");
        assert_eq!(scope.domain(), Some(Domain::Platform));
        assert_eq!(scope.tail(), ["ontology", "objects"]);
    }

    #[test]
    fn scope_keeps_unknown_domain_for_client_validation() {
        let scope = Scope::new(["unknown", "page"]);

        assert_eq!(scope.domain_name(), Some("unknown"));
        assert_eq!(scope.domain(), None);
        assert_eq!(scope.tail(), ["page"]);
    }

    #[test]
    fn empty_path_is_empty() {
        let path = Path::new("");

        assert!(path.as_segments().is_empty());
        assert_eq!(path.to_string(), "");
    }

    #[test]
    fn path_normalizes_slashes() {
        let path = Path::new("//docs//resources/foundry/notepad/aip-feature.png//");

        assert_eq!(
            path.as_segments(),
            ["docs", "resources", "foundry", "notepad", "aip-feature.png"]
        );
        assert_eq!(
            path.to_string(),
            "docs/resources/foundry/notepad/aip-feature.png"
        );
    }

    #[test]
    fn route_resolves_payload_path_against_docs_base() {
        let route = Route::new("/foundry/notepad/aip-features/");

        assert_eq!(
            route.as_ref(),
            "https://www.palantir.com/docs/foundry/notepad/aip-features/"
        );
    }

    #[test]
    fn path_converts_to_route_for_resource_fetching() {
        let path = Path::new("resources/foundry/notepad/aip-feature.png");
        let route = Route::from(path);

        assert_eq!(
            route.as_ref(),
            "https://www.palantir.com/docs/resources/foundry/notepad/aip-feature.png"
        );
    }

    #[test]
    fn product_scope_converts_to_path_and_route() {
        let scope = Scope::new(["product", "notepad", "aip-features"]);

        let path = Path::from(scope.clone());
        let route = Route::from(scope);

        assert_eq!(path.as_segments(), ["foundry", "notepad", "aip-features"]);
        assert_eq!(
            route.as_ref(),
            "https://www.palantir.com/docs/foundry/notepad/aip-features"
        );
    }

    #[test]
    fn platform_scope_adds_api_v2_prefix() {
        let scope = Scope::new(["platform", "general", "overview", "introduction"]);

        let path = Path::from(scope.clone());
        let route = Route::from(scope);

        assert_eq!(
            path.as_segments(),
            [
                "foundry",
                "api",
                "v2",
                "general",
                "overview",
                "introduction"
            ]
        );
        assert_eq!(
            route.as_ref(),
            "https://www.palantir.com/docs/foundry/api/v2/general/overview/introduction"
        );
    }

    #[test]
    fn updates_scope_adds_announcements_prefix() {
        let scope = Scope::new(["updates", "2026-07"]);

        let path = Path::from(scope.clone());
        let route = Route::from(scope);

        assert_eq!(path.as_segments(), ["foundry", "announcements", "2026-07"]);
        assert_eq!(
            route.as_ref(),
            "https://www.palantir.com/docs/foundry/announcements/2026-07"
        );
    }

    #[test]
    fn domain_root_scopes_convert_to_domain_roots() {
        let cases = [
            ("product", "foundry"),
            ("platform", "foundry/api/v2"),
            ("updates", "foundry/announcements"),
        ];

        for (scope, expected_path) in cases {
            let path = Path::from(Scope::new([scope]));

            assert_eq!(path.to_string(), expected_path);
        }
    }

    #[test]
    fn route_converts_to_path() {
        let route = Route::new("foundry/notepad/aip-features/");
        let path = Path::try_from(route).unwrap();

        assert_eq!(path.to_string(), "foundry/notepad/aip-features");
    }

    #[test]
    fn product_path_converts_to_scope() {
        let scope = Scope::try_from(Path::new("foundry/notepad/aip-features")).unwrap();

        assert_eq!(scope.to_string(), "product/notepad/aip-features");
    }

    #[test]
    fn platform_path_uses_longest_domain_prefix() {
        let scope =
            Scope::try_from(Path::new("foundry/api/v2/general/overview/introduction")).unwrap();

        assert_eq!(scope.to_string(), "platform/general/overview/introduction");
    }

    #[test]
    fn updates_path_uses_longest_domain_prefix() {
        let scope = Scope::try_from(Path::new("foundry/announcements/2026-07")).unwrap();

        assert_eq!(scope.to_string(), "updates/2026-07");
    }

    #[test]
    fn fixture_double_trailing_slash_is_normalized() {
        let route = Route::new("/foundry/api/general/overview/introduction//");
        let path = Path::try_from(route).unwrap();

        assert_eq!(
            path.to_string(),
            "foundry/api/general/overview/introduction"
        );
    }

    #[test]
    fn route_round_trips_through_path_and_scope() {
        let route = Route::new("foundry/announcements/release-notes/");

        let scope = Scope::try_from(route.clone()).unwrap();
        let round_trip = Route::from(scope);

        assert_eq!(
            round_trip.as_ref(),
            "https://www.palantir.com/docs/foundry/announcements/release-notes"
        );
    }

    #[test]
    fn resource_path_converts_to_route_but_not_scope() {
        let path = Path::new("resources/foundry/notepad/aip-feature.png");

        assert_eq!(
            Route::from(path.clone()).as_ref(),
            "https://www.palantir.com/docs/resources/foundry/notepad/aip-feature.png"
        );
        assert_eq!(Scope::try_from(path.clone()), Err(path));
    }

    #[test]
    fn docs_prefixed_markdown_resource_is_not_a_scope() {
        let path = Path::new("/docs/resources/foundry/getting-started/Get-Started.svg");

        assert_eq!(
            path.to_string(),
            "docs/resources/foundry/getting-started/Get-Started.svg"
        );
        assert_eq!(Scope::try_from(path.clone()), Err(path));
    }

    #[test]
    fn path_rejects_route_outside_exact_docs_base() {
        let wrong_host = Route("https://example.com/docs/foundry/example".into());
        let wrong_scheme = Route("http://www.palantir.com/docs/foundry/example".into());
        let missing_docs = Route("https://www.palantir.com/foundry/example".into());

        assert!(Path::try_from(wrong_host).is_err());
        assert!(Path::try_from(wrong_scheme).is_err());
        assert!(Path::try_from(missing_docs).is_err());
    }

    #[test]
    fn path_rejects_similar_but_inexact_docs_base() {
        let route = Route("https://www.palantir.com/docs-malicious/foundry/example".into());

        assert!(Path::try_from(route).is_err());
    }

    #[test]
    fn non_foundry_path_cannot_become_scope() {
        let path = Path::new("apollo/overview");

        assert_eq!(Scope::try_from(path.clone()), Err(path));
    }

    #[test]
    fn non_foundry_route_cannot_become_scope() {
        let route = Route::new("apollo/overview");

        assert_eq!(Scope::try_from(route.clone()), Err(route));
    }
}
