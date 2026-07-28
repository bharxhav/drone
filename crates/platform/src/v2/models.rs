pub mod errors;
#[path = "models/models.rs"]
pub mod types;

mod experiment;
mod experiment_artifact_table;
mod experiment_series;
mod live_deployment;
mod model;
mod model_function;
mod model_studio;
mod model_studio_config_version;
mod model_studio_run;
mod model_studio_trainer;
mod model_version;

pub use experiment::Experiments;
pub use experiment_artifact_table::ExperimentArtifactTables;
pub use experiment_series::ExperimentSeries;
pub use live_deployment::LiveDeployments;
pub use model::ModelResources;
pub use model_function::ModelFunctions;
pub use model_studio::ModelStudios;
pub use model_studio_config_version::ModelStudioConfigVersions;
pub use model_studio_run::ModelStudioRuns;
pub use model_studio_trainer::ModelStudioTrainers;
pub use model_version::ModelVersions;

use crate::transport::Transport;

/// Models namespace handle (models, model versions, experiments, deployments).
#[derive(Debug)]
pub struct Models<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> Models<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn models(&self) -> ModelResources<'_> {
        ModelResources::new(self.transport)
    }
    pub fn experiments(&self) -> Experiments<'_> {
        Experiments::new(self.transport)
    }
    pub fn experiment_series(&self) -> ExperimentSeries<'_> {
        ExperimentSeries::new(self.transport)
    }
    pub fn experiment_artifact_tables(&self) -> ExperimentArtifactTables<'_> {
        ExperimentArtifactTables::new(self.transport)
    }
    pub fn functions(&self) -> ModelFunctions<'_> {
        ModelFunctions::new(self.transport)
    }
    pub fn versions(&self) -> ModelVersions<'_> {
        ModelVersions::new(self.transport)
    }
    pub fn live_deployments(&self) -> LiveDeployments<'_> {
        LiveDeployments::new(self.transport)
    }
    pub fn model_studios(&self) -> ModelStudios<'_> {
        ModelStudios::new(self.transport)
    }
    pub fn model_studio_config_versions(&self) -> ModelStudioConfigVersions<'_> {
        ModelStudioConfigVersions::new(self.transport)
    }
    pub fn model_studio_runs(&self) -> ModelStudioRuns<'_> {
        ModelStudioRuns::new(self.transport)
    }
    pub fn model_studio_trainers(&self) -> ModelStudioTrainers<'_> {
        ModelStudioTrainers::new(self.transport)
    }
}
