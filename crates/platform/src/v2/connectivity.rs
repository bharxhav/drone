pub mod errors;
pub mod models;

mod connections;
mod file_imports;
mod table_imports;
mod virtual_tables;

pub use connections::Connections;
pub use file_imports::FileImports;
pub use table_imports::TableImports;
pub use virtual_tables::VirtualTables;

use crate::transport::Transport;

/// Connectivity namespace handle (connections, file imports, table imports).
#[derive(Debug)]
pub struct Connectivity<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> Connectivity<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn connections(&self) -> Connections<'_> {
        Connections::new(self.transport)
    }

    pub fn file_imports(&self) -> FileImports<'_> {
        FileImports::new(self.transport)
    }

    pub fn table_imports(&self) -> TableImports<'_> {
        TableImports::new(self.transport)
    }

    pub fn virtual_tables(&self) -> VirtualTables<'_> {
        VirtualTables::new(self.transport)
    }
}
