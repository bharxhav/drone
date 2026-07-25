//! `Scope  product/notepad/aip-features`
//! `Path   docs/foundry/notepad/aip-features`
//! `Route  https://www.palantir.com/docs/foundry/notepad/aip-features/`

use std::ops::Deref;

use crate::domain::Domain;

/// Url with `https://www.palantir.com/`.
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
        Self::new(path.join("/"))
    }
}

impl From<Scope> for Route {
    fn from(scope: Scope) -> Self {
        Path::from(scope).into()
    }
}

/// A path relative to `https://www.palantir.com/`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct Path(Vec<String>);

impl Path {
    pub const fn base_url() -> &'static str {
        "https://www.palantir.com/"
    }

    fn from_segments(path: &str) -> Self {
        Self(
            path.split('/')
                .filter(|segment| !segment.is_empty())
                .map(str::to_owned)
                .collect(),
        )
    }
}

impl From<Route> for Path {
    fn from(route: Route) -> Self {
        Self::from_segments(route.0.trim_start_matches(Self::base_url()))
    }
}

impl From<Scope> for Path {
    fn from(scope: Scope) -> Self {
        Route::from(scope).into()
    }
}

impl Deref for Path {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<String>> for Path {
    fn from(segments: Vec<String>) -> Self {
        Self(segments)
    }
}

/// A client path beginning with `product`, `platform`, or `updates`.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Scope(Vec<String>);

impl Scope {
    pub const fn base_url() -> &'static str {
        "https://www.palantir.com/docs/foundry/"
    }

    pub(super) fn domain(&self) -> Option<Domain> {
        self.0
            .first()
            .and_then(|segment| Domain::from_scope(segment))
    }
}

impl TryFrom<Path> for Scope {
    type Error = Path;

    fn try_from(path: Path) -> Result<Self, Self::Error> {
        for domain in Domain::ALL {
            let prefix: Vec<String> = domain
                .path()
                .iter()
                .map(|segment| (*segment).into())
                .collect();
            if let Some(scope) = path.0.strip_prefix(prefix.as_slice()) {
                let mut segments = vec![domain.name().to_owned()];
                segments.extend_from_slice(scope);
                return Ok(Self(segments));
            }
        }

        Err(path)
    }
}

impl TryFrom<Route> for Scope {
    type Error = Route;

    fn try_from(route: Route) -> Result<Self, Self::Error> {
        let path = Path::from(route.clone());
        Self::try_from(path).map_err(|_| route)
    }
}

impl Deref for Scope {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<String>> for Scope {
    fn from(segments: Vec<String>) -> Self {
        Self(segments)
    }
}
