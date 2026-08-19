use url::Url;

use super::{
    config_args::ConfigArgs,
    config_file::{ConfigFile, FoundryResource},
};
use crate::error::Error;

/// Configuration resolved for one invocation.
#[derive(Debug)]
pub struct AppConfig {
    pub url: Url,
    pub token: Option<String>,
    pub ontology: String,
    pub space: String,
}

impl AppConfig {
    /// Resolves persisted configuration and CLI selectors for one invocation.
    pub fn new(file: ConfigFile, args: ConfigArgs) -> Result<Self, Error> {
        let deployment_name = args
            .deployment
            .or(file.default)
            .ok_or_else(|| Error::Config("no deployment selected".into()))?;
        let deployment = file.deployments.get(&deployment_name).ok_or_else(|| {
            Error::Config(format!("deployment `{deployment_name}` is not configured"))
        })?;

        Ok(Self {
            url: deployment.base_url.clone(),
            token: deployment.token.clone(),
            ontology: resolve_resources(&deployment.ontologies, args.ontology, "ontology")?,
            space: resolve_resources(&deployment.spaces, args.space, "space")?,
        })
    }
}

fn resolve_resources(
    resources: &FoundryResource,
    selected: Option<String>,
    kind: &str,
) -> Result<String, Error> {
    let name = selected
        .or_else(|| resources.default.clone())
        .ok_or_else(|| Error::Config(format!("no {kind} selected")))?;
    resources
        .items
        .get(&name)
        .cloned()
        .ok_or_else(|| Error::Config(format!("{kind} `{name}` is not configured")))
}
