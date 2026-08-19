use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use url::Url;

/// Drone's configuration file.
#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    /// Deployment used when `--deployment` is omitted.
    pub default: Option<String>,
    /// Foundry deployments keyed by their local names.
    pub deployments: HashMap<String, FoundryDeployment>,
}

/// Connection and convenience settings for one Foundry deployment.
#[derive(Debug, Deserialize)]
pub struct FoundryDeployment {
    /// Foundry deployment base URL.
    #[serde(deserialize_with = "parse_url")]
    pub base_url: Url,
    /// Token captured from the environment variable named by `token_env`.
    #[serde(rename = "token_env", deserialize_with = "parse_env_token")]
    pub token: Option<String>,
    /// Configured ontology aliases.
    #[serde(default)]
    pub ontologies: FoundryResource,
    /// Configured filesystem space aliases.
    #[serde(default)]
    pub spaces: FoundryResource,
}

/// Named aliases for one kind of Foundry resource.
#[derive(Debug, Default, Deserialize)]
pub struct FoundryResource {
    /// Alias used when its command-line selector is omitted.
    pub default: Option<String>,
    /// Stable Foundry identifiers keyed by local aliases.
    #[serde(default)]
    pub items: HashMap<String, String>,
}

/// Resolves an environment variable once, while the configuration is deserialized.
fn parse_env_token<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let variable = String::deserialize(deserializer)?;
    Ok(std::env::var(variable).ok())
}

/// Parses and validates a URL while the configuration is deserialized.
fn parse_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    value.parse().map_err(serde::de::Error::custom)
}
