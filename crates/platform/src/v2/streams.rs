pub mod errors;
pub mod models;

mod dataset;
mod stream;
mod subscriber;

pub use dataset::DatasetResource;
pub use stream::StreamResource;
pub use subscriber::Subscribers;

use crate::transport::Transport;

/// Streams namespace handle.
#[derive(Debug)]
pub struct Streams<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> Streams<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn dataset(&self) -> DatasetResource<'_> {
        DatasetResource::new(self.transport)
    }

    pub fn datasets(&self) -> DatasetResource<'_> {
        DatasetResource::new(self.transport)
    }

    pub fn stream(&self) -> StreamResource<'_> {
        StreamResource::new(self.transport)
    }

    pub fn subscribers(&self) -> Subscribers<'_> {
        Subscribers::new(self.transport)
    }
}
