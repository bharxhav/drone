use clap::Args;

/// Global selectors used to resolve configuration for an invocation.
#[derive(Args, Debug)]
pub struct ConfigArgs {
    /// Foundry deployment name.
    #[arg(long, global = true)]
    pub deployment: Option<String>,

    /// Configured ontology alias.
    #[arg(long, global = true)]
    pub ontology: Option<String>,

    /// Configured filesystem space alias.
    #[arg(long, global = true)]
    pub space: Option<String>,
}
