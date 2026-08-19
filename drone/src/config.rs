use std::collections::HashMap;

use serde::{Deserialize, Deserializer, de::Error};
use url::Url;

#[derive(Debug)]
/// Drone's configuration file.
pub struct DroneConfig {
    /// Deployment used when `--deployment` is omitted.
    pub default: Option<String>,
    /// Foundry deployments keyed by their local names.
    pub deployments: HashMap<String, PalantirFoundryDeployment>,
}

#[derive(Debug, Deserialize)]
/// Connection and convenience settings for one Foundry deployment.
pub struct PalantirFoundryDeployment {
    /// Foundry deployment base URL.
    #[serde(deserialize_with = "url")]
    pub base_url: Url,
    /// Token captured from the environment variable named by `token_env`.
    #[serde(rename = "token_env", deserialize_with = "token")]
    pub token: Option<String>,
    /// Configured ontology aliases.
    #[serde(default)]
    pub ontologies: PalantirFoundryResource,
    /// Configured filesystem space aliases.
    #[serde(default)]
    pub spaces: PalantirFoundryResource,
}

#[derive(Debug, Default)]
/// Named aliases for one kind of Foundry resource.
pub struct PalantirFoundryResource {
    /// Alias used when its command-line selector is omitted.
    pub default: Option<String>,
    /// Stable Foundry identifiers keyed by local aliases.
    pub items: HashMap<String, String>,
}
