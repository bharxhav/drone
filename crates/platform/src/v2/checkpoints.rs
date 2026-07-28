pub mod errors;
pub mod models;

mod record;
pub use record::Records;

use crate::transport::Transport;

/// Checkpoints namespace handle.
#[derive(Debug)]
pub struct Checkpoints<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> Checkpoints<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn record(&self) -> Records<'_> {
        Records::new(self.transport)
    }
    pub fn records(&self) -> Records<'_> {
        Records::new(self.transport)
    }
}
