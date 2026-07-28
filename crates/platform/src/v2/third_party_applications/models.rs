//! Third Party Applications namespace wire types.

use serde::{Deserialize, Serialize};
pub type Subdomain = String;
pub type ThirdPartyApplicationRid = String;
pub type VersionVersion = String;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployWebsiteRequest {
    pub version: VersionVersion,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThirdPartyApplication {
    pub rid: ThirdPartyApplicationRid,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub version: VersionVersion,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Website {
    pub deployed_version: Option<VersionVersion>,
    pub subdomains: Vec<Subdomain>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListVersionsResponse {
    pub data: Vec<Version>,
    pub next_page_token: Option<String>,
}
