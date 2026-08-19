use clap::Args;
use sysexits::ExitCode;

use crate::config::resolved_config::AppConfig;

/// Work with Foundry ontologies.
#[derive(Args)]
pub struct O {}

impl O {
    pub fn run(self, config: &AppConfig) -> ExitCode {
        let _ = (&config.url, &config.token, &config.ontology);
        ExitCode::Unavailable
    }
}
