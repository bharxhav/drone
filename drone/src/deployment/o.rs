use clap::Args;
use sysexits::ExitCode;

use crate::config::resolved_config::ResolvedConfig;

/// Work with Foundry ontologies.
#[derive(Args)]
pub struct O {}

impl O {
    pub fn run(self, config: &ResolvedConfig) -> ExitCode {
        let _ = (&config.url, &config.token, &config.ontology);
        ExitCode::Unavailable
    }
}
