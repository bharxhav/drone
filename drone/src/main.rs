mod config;
mod consts;
mod deployment;

use clap::{ArgGroup, CommandFactory, Parser, Subcommand};
use sysexits::ExitCode;

use crate::consts::LOGO;

const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("DRONE_RELEASE_DATE"),
    ")"
);

#[derive(Parser)]
#[command(
    version = VERSION,
    about = "CLI for Palantir Foundry",
    before_help = LOGO,
    group = ArgGroup::new("output").args(["json", "toon"]).multiple(false)
)]
struct Cli {
    /// Emit JSON output.
    #[arg(long, global = true)]
    json: bool,

    /// Emit TOON output.
    #[arg(long, global = true)]
    toon: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Return the Palantir Foundry documentation.
    Man,

    #[command(flatten)]
    Deployment(deployment::Command),
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let _ = (cli.json, cli.toon);

    match cli.command {
        Some(Command::Man) => ExitCode::Unavailable,
        Some(Command::Deployment(command)) => command.run(),
        None => {
            Cli::command().print_help().expect("failed to print help");
            println!();
            ExitCode::Ok
        }
    }
}
