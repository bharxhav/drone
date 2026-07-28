pub mod errors;
pub mod models;

mod actions;
mod attachments;
mod linked_objects;
mod object_sets;
mod objects;
mod ontology;
mod queries;
mod time_series;

pub use actions::{ActionQuery, Actions};
pub use attachments::Attachments;
pub use linked_objects::LinkedObjects;
pub use object_sets::{ObjectSetQuery, ObjectSets};
pub use objects::Objects;
pub use ontology::OntologyResource;
pub use queries::{ExecuteQueryOptions, Queries};
pub use time_series::TimeSeries;

use crate::transport::Transport;

/// Ontologies namespace handle.
#[derive(Debug)]
pub struct Ontologies<'c> {
    transport: &'c Transport,
}

impl<'c> Ontologies<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn ontology(&self) -> OntologyResource<'_> {
        OntologyResource::new(self.transport)
    }

    pub fn objects(&self) -> Objects<'_> {
        Objects::new(self.transport)
    }

    pub fn object_sets(&self) -> ObjectSets<'_> {
        ObjectSets::new(self.transport)
    }

    pub fn linked_objects(&self) -> LinkedObjects<'_> {
        LinkedObjects::new(self.transport)
    }

    pub fn actions(&self) -> Actions<'_> {
        Actions::new(self.transport)
    }

    pub fn queries(&self) -> Queries<'_> {
        Queries::new(self.transport)
    }

    pub fn attachments(&self) -> Attachments<'_> {
        Attachments::new(self.transport)
    }

    pub fn time_series(&self) -> TimeSeries<'_> {
        TimeSeries::new(self.transport)
    }
}
