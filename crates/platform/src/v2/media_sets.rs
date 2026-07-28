pub mod errors;
pub mod models;

mod media_set;
pub use media_set::MediaSetResources;

use crate::transport::Transport;

/// Media Sets namespace handle.
#[derive(Debug)]
pub struct MediaSets<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> MediaSets<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn media_sets(&self) -> MediaSetResources<'_> {
        MediaSetResources::new(self.transport)
    }
}
