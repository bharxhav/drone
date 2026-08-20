use palantir_foundry_types::rid::Rid;
use serde::Deserialize;
use strum::AsRefStr;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DronfigFile {
    pub resources: Vec<FoundryResource>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FoundryResource {
    pub kind: ResourceKind,
    pub name: String,
    pub alias: Option<String>,
    pub default: Option<bool>,
    pub deployment: Option<String>,
    pub rid: Rid,
}

#[derive(AsRefStr, Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ResourceKind {
    Ontology,
    Space,
    ObjectType,
    Project,
}
