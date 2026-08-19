use clap::Args;
use sysexits::ExitCode;

use crate::config::resolved_config::ResolvedConfig;

/// Work with Foundry filesystem resources.
#[derive(Args)]
pub struct Fs {}

impl Fs {
    pub fn run(self, config: &ResolvedConfig) -> ExitCode {
        let _ = (&config.url, &config.token, &config.space);
        ExitCode::Unavailable
    }
}
