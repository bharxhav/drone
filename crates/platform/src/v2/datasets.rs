pub mod errors;
pub mod models;

mod branches;
#[path = "datasets/datasets.rs"]
mod dataset_resource;
mod files;
mod transactions;
mod views;

pub use branches::Branches;
pub use dataset_resource::DatasetResource;
pub use files::Files;
pub use transactions::Transactions;
pub use views::Views;

use crate::transport::Transport;

/// Datasets namespace handle.
#[derive(Debug)]
pub struct Datasets<'c> {
    transport: &'c Transport,
}

impl<'c> Datasets<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn dataset(&self) -> DatasetResource<'_> {
        DatasetResource::new(self.transport)
    }

    pub fn datasets(&self) -> DatasetResource<'_> {
        DatasetResource::new(self.transport)
    }

    pub fn branches(&self) -> Branches<'_> {
        Branches::new(self.transport)
    }

    pub fn files(&self) -> Files<'_> {
        Files::new(self.transport)
    }

    pub fn transactions(&self) -> Transactions<'_> {
        Transactions::new(self.transport)
    }

    pub fn views(&self) -> Views<'_> {
        Views::new(self.transport)
    }
}
