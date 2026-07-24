mod consts;

use clap::{CommandFactory, Parser, Subcommand};
use sysexits::ExitCode;

use crate::consts::LOGO;

#[derive(Parser)]
#[command(version, about = "CLI for Palantir Foundry", before_help = LOGO)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Return the Palantir Foundry documentation.
    Man,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Man) => ExitCode::Ok,
        None => {
            Cli::command().print_help().expect("failed to print help");
            println!();
            ExitCode::Ok
        }
    }
}
