use clap::Subcommand;
use sysexits::ExitCode;

use crate::config::resolved_config::ResolvedConfig;

pub mod fs;
pub mod o;

#[derive(Subcommand)]
/// Commands backed by a configured Foundry deployment.
pub enum DeploymentCommand {
    /// Work with Foundry ontologies.
    O(o::O),

    /// Work with Foundry filesystem resources.
    Fs(fs::Fs),
}

impl DeploymentCommand {
    /// Loads configuration, selects a deployment, and dispatches the command.
    pub fn run(self, config: &ResolvedConfig) -> ExitCode {
        match self {
            Self::O(command) => command.run(config),
            Self::Fs(command) => command.run(config),
        }
    }
}
