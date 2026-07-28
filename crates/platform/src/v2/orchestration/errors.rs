//! Orchestration namespace API errors.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrchestrationApiError {
    pub error_name: OrchestrationErrorName,
    pub error_instance_id: String,
    #[serde(default)]
    pub parameters: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrchestrationErrorName {
    BuildInputsNotFound,
    BuildInputsPermissionDenied,
    BuildNotFound,
    BuildNotRunning,
    BuildTargetsMissingJobSpecs,
    BuildTargetsNotFound,
    BuildTargetsPermissionDenied,
    BuildTargetsResolutionError,
    BuildTargetsUpToDate,
    CancelBuildPermissionDenied,
    CreateBuildPermissionDenied,
    CreateSchedulePermissionDenied,
    DeleteSchedulePermissionDenied,
    DuplicateBuildBranches,
    GetAffectedResourcesSchedulePermissionDenied,
    InvalidAndTrigger,
    InvalidMediaSetTrigger,
    InvalidOrTrigger,
    InvalidScheduleDescription,
    InvalidScheduleName,
    InvalidTimeTrigger,
    JobNotFound,
    MissingBuildTargets,
    MissingConnectingBuildInputs,
    MissingTrigger,
    PauseSchedulePermissionDenied,
    ReplaceSchedulePermissionDenied,
    RunSchedulePermissionDenied,
    ScheduleAlreadyRunning,
    ScheduleNotFound,
    ScheduleTriggerResourcesNotFound,
    ScheduleTriggerResourcesPermissionDenied,
    ScheduleVersionNotFound,
    SearchBuildsPermissionDenied,
    TargetNotSupported,
    UnpauseSchedulePermissionDenied,
}
