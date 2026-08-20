use std::collections::HashMap;

use serde::Deserialize;

/// Drone's configuration file.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigFile {
    /// Foundry deployments keyed by their local names.
    pub deployments: HashMap<String, FoundryDeployment>,
}

/// Connection and convenience settings for one Foundry deployment.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FoundryDeployment {
    /// Foundry deployment base URL.
    pub base_url: String,
    /// Environment variable containing the Foundry token.
    pub token_env: String,
    /// Select this deployment when no declaration specifies one.
    pub default: Option<bool>,
}
