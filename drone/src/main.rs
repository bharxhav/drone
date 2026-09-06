mod config;
mod consts;
mod dronfig;
mod error;
mod verb;

use std::collections::BTreeSet;

use clap::{ArgGroup, CommandFactory, FromArgMatches, Parser, Subcommand};
use etcetera::{AppStrategy, AppStrategyArgs, app_strategy::choose_native_strategy};
use sysexits::ExitCode;

use crate::{
    config::resolved::DeploymentConfig,
    consts::{APP_AUTHOR, APP_NAME, APP_TOP_LEVEL_DOMAIN, LOGO},
    dronfig::resolved::Dronfig,
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
    /// Foundry deployment name.
    #[arg(long, global = true)]
    deployment: Option<String>,

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
    // Locate and deserialize the platform-native configuration file.
    let strategy = choose_native_strategy(AppStrategyArgs {
        top_level_domain: APP_TOP_LEVEL_DOMAIN.into(),
        author: APP_AUTHOR.into(),
        app_name: APP_NAME.into(),
    })
    .map_err(|error| Error::Config {
        message: format!("could not locate the configuration directory: {error}"),
        help: "check that the platform configuration directory is available".into(),
    })?;

    let initial_matches = Cli::command().ignore_errors(true).get_matches();
    let deployment = initial_matches.get_one::<String>("deployment").cloned();

    let config_dir = strategy.config_dir();
    let cwd = std::env::current_dir()?;

    // Resolve Configs
    let config = DeploymentConfig::resolve(&config_dir.join("config.toml"), deployment)?;
    let (dronfig, dronfig_errors) = Dronfig::resolve(&config_dir, &cwd, false, true)?;

    // Print warnings
    for error in dronfig_errors {
        eprintln!("{:?}", error.warning());
    }

    // Construct synthetic command space
    let mut command = Cli::command();
    let resource_names = dronfig
        .resources
        .iter()
        .filter(|resource| {
            resource
                .deployment
                .as_ref()
                .is_none_or(|deployment| deployment == &config.name)
        })
        .flat_map(|resource| [resource.name.clone(), resource.kind.as_ref().into()])
        .collect::<BTreeSet<_>>();
    for resource in resource_names {
        command = command.subcommand(verb::command(resource));
    }

    let matches = command.try_get_matches()?;
    let cli = Cli::from_arg_matches(&matches)?;
    let _ = (&cli.deployment, cli.json, cli.toon);

    // Dispatch with one fully resolved configuration.
    Ok(match cli.command {
        Some(Command::Man) => {
            let _ = (&config, &dronfig);
            ExitCode::Unavailable
        }
        None => {
            let _ = (&config.uri, &config.token);
            let _ = &dronfig.resources;
            Cli::command().print_help().expect("failed to print help");
            println!();
            ExitCode::Ok
        }
    })
}
