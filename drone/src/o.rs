use clap::Args;
use sysexits::ExitCode;

/// Work with Foundry ontologies.
#[derive(Args)]
pub struct O {}

impl O {
    pub fn run(self) -> ExitCode {
        ExitCode::Unavailable
    }
}
