mod config;
mod consts;
mod deployment;
mod error;

use clap::{ArgGroup, CommandFactory, Parser, Subcommand};
use etcetera::{AppStrategy, AppStrategyArgs, app_strategy::choose_native_strategy};
use sysexits::ExitCode;

use crate::{
    config::{config_args::ConfigArgs, config_file::ConfigFile, resolved_config::AppConfig},
    consts::{APP_AUTHOR, APP_NAME, APP_TOP_LEVEL_DOMAIN, LOGO},
    error::Error,
};

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
    #[command(flatten)]
    config: ConfigArgs,

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
    Deployment(deployment::DeploymentCommand),
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            let exit_code = error.exit_code();
            eprintln!("{:?}", miette::Report::new(error));
            exit_code
        }
    }
}

fn run() -> Result<ExitCode, Error> {
    let cli = Cli::parse();

    // Locate and deserialize the platform-native configuration file.
    let strategy = choose_native_strategy(AppStrategyArgs {
        top_level_domain: APP_TOP_LEVEL_DOMAIN.into(),
        author: APP_AUTHOR.into(),
        app_name: APP_NAME.into(),
    })
    .map_err(|error| Error::Config(error.to_string()))?;

    let content = std::fs::read_to_string(strategy.in_config_dir("config.toml"))?;
    let config = AppConfig::new(toml::from_str::<ConfigFile>(&content)?, cli.config)?;
    let _ = (cli.json, cli.toon);

    // Dispatch with one fully resolved configuration.
    Ok(match cli.command {
        Some(Command::Man) => {
            let _ = &config;
            ExitCode::Unavailable
        }
        Some(Command::Deployment(command)) => command.run(&config),
        None => {
            Cli::command().print_help().expect("failed to print help");
            println!();
            ExitCode::Ok
        }
    })
}
