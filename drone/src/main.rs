mod error;

use clap::{Parser, Subcommand};
use error::Error;
use sysexits::ExitCode;

#[derive(Debug, Parser)]
#[command(version, about = "A working copy for Palantir Foundry")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Import a Foundry resource into the project.
    Import {
        /// Resource RIDs to import.
        #[arg(required = true, num_args = 1..)]
        rids: Vec<String>,
    },

    /// Refresh local snapshots from Foundry.
    Sync {
        /// Resources to refresh. All resources are refreshed when omitted.
        resources: Vec<String>,
    },

    /// Compare resources and report inconsistencies.
    Compare {
        /// Resources to compare. All resources are compared when omitted.
        resources: Vec<String>,
    },

    /// Draft ordered prompts for AI FDE.
    Draft {
        /// Resources to include. All relevant resources are included when omitted.
        resources: Vec<String>,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => error.report(),
    }
}

fn run() -> Result<ExitCode, Error> {
    let command = Cli::try_parse()?.command;
    let (name, resources) = match command {
        Command::Import { rids } => ("import", rids),
        Command::Sync { resources } => ("sync", resources),
        Command::Compare { resources } => ("compare", resources),
        Command::Draft { resources } => ("draft", resources),
    };

    Err(Error::Unimplemented {
        command: name,
        resources,
    })
}
