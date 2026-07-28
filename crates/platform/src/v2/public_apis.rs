pub mod errors;
pub mod models;
pub mod resources;

use crate::transport::Transport;
pub use resources::{ApiDefinitions, OpenApiDefinitions};

/// Public APIs namespace handle.
#[derive(Debug)]
pub struct PublicApis<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> PublicApis<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn api_definitions(&self) -> ApiDefinitions<'c> {
        ApiDefinitions::new(self.transport)
    }
    pub fn open_api_definitions(&self) -> OpenApiDefinitions<'c> {
        OpenApiDefinitions::new(self.transport)
    }
}
