use std::ops::Deref;

/// A path relative to `https://www.palantir.com/`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct Route(Vec<String>);

impl Route {
    pub const fn base_url() -> &'static str {
        "https://www.palantir.com/"
    }
}

/// A client path beginning with `product`, `platform`, or `updates`.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Scope(Vec<String>);

impl Scope {
    pub const fn base_url() -> &'static str {
        "https://www.palantir.com/docs/foundry/"
    }
}

macro_rules! impl_path {
    ($type:ty) => {
        impl Deref for $type {
            type Target = [String];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<Vec<String>> for $type {
            fn from(segments: Vec<String>) -> Self {
                Self(segments)
            }
        }
    };
}

impl_path!(Route);
impl_path!(Scope);
