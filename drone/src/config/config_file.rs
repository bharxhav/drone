use std::collections::HashMap;

use serde::{Deserialize, Deserializer, de::Error};
use url::Url;

use super::{config_args::ConfigArgs, resolved_config::ResolvedConfig};

/// Drone's configuration file.
#[derive(Debug)]
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
    #[serde(deserialize_with = "url")]
    pub base_url: Url,
    /// Token captured from the environment variable named by `token_env`.
    #[serde(rename = "token_env", deserialize_with = "token")]
    pub token: Option<String>,
    /// Configured ontology aliases.
    #[serde(default)]
    pub ontologies: FoundryResource,
    /// Configured filesystem space aliases.
    #[serde(default)]
    pub spaces: FoundryResource,
}

/// Named aliases for one kind of Foundry resource.
#[derive(Debug, Default)]
pub struct FoundryResource {
    /// Alias used when its command-line selector is omitted.
    pub default: Option<String>,
    /// Stable Foundry identifiers keyed by local aliases.
    pub items: HashMap<String, String>,
}
