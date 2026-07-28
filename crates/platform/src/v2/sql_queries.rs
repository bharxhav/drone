pub mod errors;
pub mod models;

mod sql_query;
pub use sql_query::SqlQueryResource;

use crate::transport::Transport;

/// SQL Queries namespace handle.
#[derive(Debug)]
pub struct SqlQueries<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> SqlQueries<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn sql_query(&self) -> SqlQueryResource<'_> {
        SqlQueryResource::new(self.transport)
    }
    pub fn sql_queries(&self) -> SqlQueryResource<'_> {
        SqlQueryResource::new(self.transport)
    }
}
