/// Authentication credentials for the Foundry API.
#[derive(Debug, Clone)]
pub enum Auth {
    /// Static bearer token.
    Token(String),
}

impl Auth {
    pub fn token(token: impl Into<String>) -> Self {
        Self::Token(token.into())
    }

    pub(crate) fn header_value(&self) -> String {
        match self {
            Self::Token(t) => format!("Bearer {t}"),
        }
    }
}
