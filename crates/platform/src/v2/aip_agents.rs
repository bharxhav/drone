pub mod errors;
pub mod models;
mod resources;

pub use resources::{AgentVersions, Agents, Contents, SessionTraces, Sessions};

use crate::transport::Transport;

/// AIP Agents namespace handle (agents, sessions, content).
#[derive(Debug)]
pub struct AipAgents<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> AipAgents<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn agents(&self) -> Agents<'_> {
        Agents::new(self.transport)
    }

    pub fn agent_versions(&self) -> AgentVersions<'_> {
        AgentVersions::new(self.transport)
    }

    pub fn sessions(&self) -> Sessions<'_> {
        Sessions::new(self.transport)
    }

    pub fn contents(&self) -> Contents<'_> {
        Contents::new(self.transport)
    }

    pub fn session_traces(&self) -> SessionTraces<'_> {
        SessionTraces::new(self.transport)
    }
}
