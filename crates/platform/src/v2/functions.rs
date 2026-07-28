pub mod errors;
pub mod models;
mod resources;

pub use resources::{Executions, Queries, ValueTypes, VersionIds};

use crate::transport::Transport;

/// Functions namespace handle (queries, executions).
#[derive(Debug)]
pub struct Functions<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> Functions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn queries(&self) -> Queries<'_> {
        Queries::new(self.transport)
    }

    pub fn executions(&self) -> Executions<'_> {
        Executions::new(self.transport)
    }

    pub fn value_types(&self) -> ValueTypes<'_> {
        ValueTypes::new(self.transport)
    }

    pub fn version_ids(&self) -> VersionIds<'_> {
        VersionIds::new(self.transport)
    }
}
