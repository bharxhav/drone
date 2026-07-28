pub mod errors;
pub mod models;

mod builds;
mod jobs;
mod schedule_versions;
mod schedules;

pub use builds::Builds;
pub use jobs::Jobs;
pub use schedule_versions::ScheduleVersions;
pub use schedules::Schedules;

use crate::transport::Transport;

/// Orchestration namespace handle (schedules, builds, jobs).
#[derive(Debug)]
pub struct Orchestration<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> Orchestration<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn builds(&self) -> Builds<'_> {
        Builds::new(self.transport)
    }

    pub fn jobs(&self) -> Jobs<'_> {
        Jobs::new(self.transport)
    }

    pub fn schedules(&self) -> Schedules<'_> {
        Schedules::new(self.transport)
    }

    pub fn schedule_versions(&self) -> ScheduleVersions<'_> {
        ScheduleVersions::new(self.transport)
    }
}
