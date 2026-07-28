//! Widgets namespace wire types.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type DevModeSnapshotId = String;
pub type FilePath = String;
pub type OntologySdkPackageRid = String;
pub type OntologySdkVersion = String;
pub type ReleaseVersion = String;
pub type RepositoryRid = String;
pub type RepositoryVersion = String;
pub type WidgetId = String;
pub type WidgetRid = String;
pub type WidgetSetRid = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DevModeStatus {
    Enabled,
    Paused,
    Disabled,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ScriptType {
    Default,
    Module,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevModeSettings {
    pub status: DevModeStatus,
    pub widget_set_settings: HashMap<WidgetSetRid, WidgetSetDevModeSettings>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevModeSettingsV2 {
    pub status: DevModeStatus,
    pub snapshot: Option<DevModeSnapshot>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevModeSnapshot {
    pub snapshot_id: DevModeSnapshotId,
    pub widget_set_settings: HashMap<WidgetSetRid, WidgetSetDevModeSettingsV2>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReleasesResponse {
    pub data: Vec<Release>,
    pub next_page_token: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologySdkInputSpec {
    pub sdk_package_rid: OntologySdkPackageRid,
    pub sdk_version: OntologySdkVersion,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub widget_set_rid: WidgetSetRid,
    pub version: ReleaseVersion,
    pub locator: ReleaseLocator,
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseLocator {
    pub repository_rid: RepositoryRid,
    pub repository_version: RepositoryVersion,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub rid: RepositoryRid,
    pub widget_set_rid: Option<WidgetSetRid>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptEntrypoint {
    pub file_path: FilePath,
    pub script_type: ScriptType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StylesheetEntrypoint {
    pub file_path: FilePath,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetDevModeSettings {
    pub script_entrypoints: Vec<ScriptEntrypoint>,
    pub stylesheet_entrypoints: Vec<StylesheetEntrypoint>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetDevModeSettingsV2 {
    pub name: Option<String>,
    pub description: Option<String>,
    pub script_entrypoints: Vec<ScriptEntrypoint>,
    pub stylesheet_entrypoints: Vec<StylesheetEntrypoint>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSet {
    pub rid: WidgetSetRid,
    pub publish_repository_rid: Option<RepositoryRid>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSetDevModeSettings {
    pub base_href: String,
    pub widget_settings: HashMap<WidgetRid, WidgetDevModeSettings>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSetDevModeSettingsById {
    pub base_href: String,
    pub widget_settings: HashMap<WidgetId, WidgetDevModeSettings>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSetDevModeSettingsV2 {
    pub base_href: String,
    pub input_spec: Option<WidgetSetInputSpec>,
    pub widget_settings: HashMap<WidgetId, WidgetDevModeSettingsV2>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetSetInputSpec {
    pub sdks: Vec<OntologySdkInputSpec>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetWidgetSetDevModeSettingsByIdRequest {
    pub widget_set_rid: WidgetSetRid,
    pub settings: WidgetSetDevModeSettingsById,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetWidgetSetManifestDevModeSettingsV2Request {
    pub widget_set_rid: WidgetSetRid,
    pub manifest: Value,
}
