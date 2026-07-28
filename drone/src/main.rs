mod consts;

use clap::{CommandFactory, Parser, Subcommand};
use sysexits::ExitCode;

use crate::consts::LOGO;

const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("DRONE_RELEASE_DATE"),
    ")"
);

#[derive(Parser)]
#[command(version = VERSION, about = "CLI for Palantir Foundry", before_help = LOGO)]
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
        Some(Command::Man) => ExitCode::Unavailable,
        None => {
            Cli::command().print_help().expect("failed to print help");
            println!();
            ExitCode::Ok
        }
    }
}
