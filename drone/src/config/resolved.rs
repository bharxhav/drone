use std::{fs, path::Path};

use url::Url;

use super::file::{ConfigFile, FoundryDeployment};
use crate::error::Error;

/// Foundry configuration resolved for one deployment.
#[derive(Debug)]
pub struct DeploymentConfig {
    pub name: String,
    pub uri: Url,
    pub token: Option<String>,
}

impl DeploymentConfig {
    pub fn resolve(path: &Path, selected: Option<String>) -> Result<Self, Error> {
        let content = fs::read_to_string(path).map_err(|error| Error::Config {
            message: format!("could not read configuration: {error}"),
            help: format!("check that {} exists and is readable", path.display()),
        })?;

        let mut file: ConfigFile = toml::from_str(&content).map_err(|error| Error::Config {
            message: format!("invalid configuration: {error}"),
            help: format!("fix the TOML in {}", path.display()),
        })?;

        let deployment = match selected {
            Some(name) => name,
            None => {
                let mut defaults = file
                    .deployments
                    .iter()
                    .filter(|(_, deployment)| deployment.default == Some(true))
                    .map(|(name, _)| name);
                let default = defaults.next().ok_or_else(|| Error::Config {
                    message: "no deployment selected".into(),
                    help: format!(
                        "pass --deployment <name> or set default = true on one deployment in {}",
                        path.display()
                    ),
                })?;
                if defaults.next().is_some() {
                    return Err(Error::Config {
                        message: "multiple default deployments are configured".into(),
                        help: format!(
                            "keep default = true on only one deployment in {}",
                            path.display()
                        ),
                    });
                }
                default.clone()
            }
        };

        let FoundryDeployment {
            base_url,
            token_env,
            default: _,
        } = file
            .deployments
            .remove(&deployment)
            .ok_or_else(|| Error::Config {
                message: format!("`{deployment}` is not a configured deployment"),
                help: format!(
                    "configure [deployments.{deployment}] in {} or select another deployment",
                    path.display()
                ),
            })?;

        let uri = base_url.parse().map_err(|error| Error::Config {
            message: format!("deployment `{deployment}` has an invalid base URL: {error}"),
            help: format!(
                "set deployments.{deployment}.base_url to an absolute URL in {}",
                path.display()
            ),
        })?;

        Ok(Self {
            name: deployment,
            uri,
            token: std::env::var(token_env).ok(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    #[test]
    fn resolves_default_and_environment_token() {
        let variable = format!("DRONE_TEST_TOKEN_{}", std::process::id());
        // SAFETY: this test uses a process-unique variable and does not spawn threads.
        unsafe { std::env::set_var(&variable, "secret") };
        let path = config_file(&format!(
            r#"
            [deployments.us]
            base_url = "https://example.com/"
            token_env = "{variable}"
            default = true
            "#
        ));

        let config = DeploymentConfig::resolve(&path, None).unwrap();

        assert_eq!(config.name, "us");
        assert_eq!(config.uri.as_str(), "https://example.com/");
        assert_eq!(config.token.as_deref(), Some("secret"));
        fs::remove_file(path).unwrap();
        // SAFETY: this removes the process-unique variable set by this test.
        unsafe { std::env::remove_var(variable) };
    }

    #[test]
    fn rejects_multiple_defaults() {
        let path = config_file(
            r#"
            [deployments.us]
            base_url = "https://example.com/"
            token_env = "MISSING_TOKEN"
            default = true

            [deployments.eu]
            base_url = "https://eu.example.com/"
            token_env = "MISSING_TOKEN"
            default = true
            "#,
        );

        assert!(DeploymentConfig::resolve(&path, None).is_err());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn explicit_deployment_overrides_default() {
        let path = config_file(
            r#"
            [deployments.us]
            base_url = "https://us.example.com/"
            token_env = "MISSING_TOKEN"
            default = true

            [deployments.eu]
            base_url = "https://eu.example.com/"
            token_env = "MISSING_TOKEN"
            "#,
        );

        let config = DeploymentConfig::resolve(&path, Some("eu".into())).unwrap();

        assert_eq!(config.name, "eu");
        assert_eq!(config.uri.as_str(), "https://eu.example.com/");
        fs::remove_file(path).unwrap();
    }

    fn config_file(content: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "drone-config-test-{}-{}.toml",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::write(&path, content).unwrap();
        path
    }
}
