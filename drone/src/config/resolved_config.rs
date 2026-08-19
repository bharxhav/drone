use url::Url;

/// Configuration resolved for one invocation.
#[derive(Debug)]
pub struct ResolvedConfig {
    pub url: Url,
    pub token: Option<String>,
    pub ontology: String,
    pub space: String,
}
