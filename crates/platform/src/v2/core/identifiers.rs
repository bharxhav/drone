//! Foundry identifier newtypes.

use serde::{Deserialize, Serialize};

macro_rules! newtype_id {
    ($($(#[$meta:meta])* $name:ident),* $(,)?) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
            #[serde(transparent)]
            pub struct $name(pub String);

            impl $name {
                pub fn new(s: impl Into<String>) -> Self {
                    Self(s.into())
                }

                pub fn as_str(&self) -> &str {
                    &self.0
                }
            }

            impl AsRef<str> for $name {
                fn as_ref(&self) -> &str {
                    &self.0
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(&self.0)
                }
            }
        )*
    };
}

newtype_id! {
    /// Resource Identifier (RID).
    Rid,
    /// User ID (principal).
    UserId,
    /// Group ID.
    GroupId,
    /// Dataset RID.
    DatasetRid,
    /// Branch ID within a dataset.
    BranchId,
    /// Transaction RID within a dataset.
    TransactionRid,
    /// Folder RID.
    FolderRid,
    /// Project RID.
    ProjectRid,
    /// Ontology RID.
    OntologyRid,
    /// Object type API name.
    ObjectTypeApiName,
    /// Link type API name.
    LinkTypeApiName,
    /// Action type API name.
    ActionTypeApiName,
    /// Query type API name.
    QueryTypeApiName,
    /// Agent RID.
    AgentRid,
    /// Session RID.
    SessionRid,
    /// Schedule RID.
    ScheduleRid,
    /// Build RID.
    BuildRid,
    /// Connection RID.
    ConnectionRid,
    /// Media set RID.
    MediaSetRid,
    /// Model RID.
    ModelRid,
    /// Stream RID.
    StreamRid,
    /// Website RID.
    WebsiteRid,
}
