use std::io;

use miette::Diagnostic;
use sysexits::ExitCode;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum Error {
    #[error("{0}")]
    #[diagnostic(code(drone::config))]
    Config(String),

    #[error("invalid command")]
    #[diagnostic(code(drone::usage))]
    Clap(#[from] clap::Error),

    #[error("I/O error")]
    #[diagnostic(code(drone::io))]
    Io(#[from] io::Error),

    #[error("invalid configuration")]
    #[diagnostic(code(drone::config::toml))]
    Toml(#[from] toml::de::Error),
}

impl Error {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Config(_) | Self::Toml(_) => ExitCode::Config,
            Self::Clap(_) => ExitCode::Usage,
            Self::Io(_) => ExitCode::IoErr,
        }
    }
}
