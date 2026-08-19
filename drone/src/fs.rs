use clap::Args;
use sysexits::ExitCode;

/// Work with Foundry filesystem resources.
#[derive(Args)]
pub struct Fs {}

impl Fs {
    pub fn run(self) -> ExitCode {
        ExitCode::Unavailable
    }
}
