use clap::{Command, Subcommand};

pub fn command(name: String) -> Command {
    Verb::augment_subcommands(Command::new(name))
}

#[derive(Subcommand)]
pub enum Verb {
    List,
    Search,
    Save,
    Alias,
    Info,
}
