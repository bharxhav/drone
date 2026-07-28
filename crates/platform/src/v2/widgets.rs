pub mod errors;
pub mod models;
pub mod resources;

use crate::transport::Transport;
pub use resources::{
    DevModeSettingsResource, DevModeSettingsV2Resource, Releases, Repositories, WidgetSets,
};

/// Widgets namespace handle.
#[derive(Debug)]
pub struct Widgets<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> Widgets<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn repositories(&self) -> Repositories<'c> {
        Repositories::new(self.transport)
    }
    pub fn widget_sets(&self) -> WidgetSets<'c> {
        WidgetSets::new(self.transport)
    }
    pub fn releases(&self) -> Releases<'c> {
        Releases::new(self.transport)
    }
    pub fn dev_mode_settings(&self) -> DevModeSettingsResource<'c> {
        DevModeSettingsResource::new(self.transport)
    }
    pub fn dev_mode_settings_v2(&self) -> DevModeSettingsV2Resource<'c> {
        DevModeSettingsV2Resource::new(self.transport)
    }
}
