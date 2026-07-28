pub mod errors;
pub mod models;

mod folders;
mod project_resource_references;
mod projects;
mod resource_roles;
mod resource_tags;
mod resources;
mod spaces;

pub use folders::Folders;
pub use project_resource_references::ProjectResourceReferences;
pub use projects::Projects;
pub use resource_roles::ResourceRoles;
pub use resource_tags::ResourceTags;
pub use resources::Resources;
pub use spaces::Spaces;

use crate::transport::Transport;

/// Filesystem namespace handle.
#[derive(Debug)]
pub struct Filesystem<'c> {
    transport: &'c Transport,
}

impl<'c> Filesystem<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn folders(&self) -> Folders<'_> {
        Folders::new(self.transport)
    }

    pub fn projects(&self) -> Projects<'_> {
        Projects::new(self.transport)
    }

    pub fn project_resource_references(&self) -> ProjectResourceReferences<'_> {
        ProjectResourceReferences::new(self.transport)
    }

    pub fn resources(&self) -> Resources<'_> {
        Resources::new(self.transport)
    }

    pub fn resource_roles(&self) -> ResourceRoles<'_> {
        ResourceRoles::new(self.transport)
    }

    pub fn resource_tags(&self) -> ResourceTags<'_> {
        ResourceTags::new(self.transport)
    }

    pub fn spaces(&self) -> Spaces<'_> {
        Spaces::new(self.transport)
    }
}
