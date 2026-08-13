use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Client;
use super::object_type::{MarkingType, ReleaseStatus, TypeClass, VectorSimilarityFunction};
use crate::{FoundryError, Rid};

/// The maximum number of action type RIDs accepted by a single batch request.
pub const ACTION_TYPE_BATCH_LIMIT: usize = 100;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionType {
    pub api_name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub status: ReleaseStatus,
    pub parameters: HashMap<String, ActionParameter>,
    pub rid: Rid,
    pub operations: Vec<LogicRule>,
    /// Description intended for tool use contexts, such as AI agents.
    pub tool_description: Option<String>,
}

impl ActionType {
    /// Whether this action type is identified by `identifier`, which may be a RID or an API name.
    pub fn matches(&self, identifier: impl AsRef<str>) -> bool {
        let identifier = identifier.as_ref();
        self.rid == identifier || self.api_name == identifier
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionParameter {
    pub display_name: String,
    pub description: Option<String>,
    pub data_type: ActionParameterType,
    pub required: bool,
    pub type_classes: Option<Vec<TypeClass>>,
    pub validation: Option<ActionParameterValidation>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionParameterValidation {
    pub default_validation: ActionParameterValidationBlock,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionParameterValidationBlock {
    /// The allowed-values constraint, retained verbatim.
    pub allowed_values: Option<Value>,
    pub array_size: Option<ParameterArraySize>,
}

/// Bounds on the size of an array-typed parameter.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ParameterArraySize {
    /// Greater than or equal.
    pub gte: Option<i64>,
    /// Less than or equal.
    pub lte: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ActionParameterType {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "geoshape")]
    GeoShape,
    #[serde(rename = "geohash")]
    Geohash,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "mediaReference")]
    MediaReference,
    #[serde(rename = "scenarioReference")]
    ScenarioReference,
    #[serde(rename = "objectType")]
    ObjectTypeReference,
    #[serde(rename = "marking")]
    Marking {
        #[serde(rename = "markingType")]
        marking_type: Option<MarkingType>,
    },
    #[serde(rename = "array")]
    Array {
        #[serde(rename = "subType")]
        sub_type: Box<ActionParameterType>,
    },
    #[serde(rename = "object")]
    Object {
        #[serde(rename = "objectApiName")]
        object_api_name: String,
        #[serde(rename = "objectTypeApiName")]
        object_type_api_name: String,
    },
    #[serde(rename = "objectSet")]
    ObjectSet {
        #[serde(rename = "objectApiName")]
        object_api_name: Option<String>,
        #[serde(rename = "objectTypeApiName")]
        object_type_api_name: Option<String>,
    },
    #[serde(rename = "interfaceObject")]
    InterfaceObject {
        #[serde(rename = "interfaceTypeApiName")]
        interface_type_api_name: Option<String>,
    },
    #[serde(rename = "struct")]
    Struct { fields: Vec<StructField> },
    #[serde(rename = "vector")]
    Vector {
        dimension: u32,
        #[serde(rename = "supportsSearchWith")]
        supports_search_with: Vec<VectorSimilarityFunction>,
        #[serde(rename = "embeddingModel")]
        embedding_model: Option<Value>,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructField {
    pub name: String,
    /// The field's ontology data type, retained verbatim.
    pub field_type: Value,
    pub required: bool,
}

/// An operation an action performs when applied.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum LogicRule {
    #[serde(rename = "createObject", rename_all = "camelCase")]
    CreateObject { object_type_api_name: String },
    #[serde(rename = "modifyObject", rename_all = "camelCase")]
    ModifyObject { object_type_api_name: String },
    #[serde(rename = "deleteObject", rename_all = "camelCase")]
    DeleteObject { object_type_api_name: String },
    #[serde(rename = "createInterfaceObject", rename_all = "camelCase")]
    CreateInterfaceObject { interface_type_api_name: String },
    #[serde(rename = "modifyInterfaceObject", rename_all = "camelCase")]
    ModifyInterfaceObject { interface_type_api_name: String },
    #[serde(rename = "deleteInterfaceObject", rename_all = "camelCase")]
    DeleteInterfaceObject { interface_type_api_name: String },
    #[serde(rename = "createLink")]
    CreateLink(LinkRule),
    #[serde(rename = "deleteLink")]
    DeleteLink(LinkRule),
    /// Applies the edits accumulated on a referenced scenario onto the ontology data context.
    #[serde(rename = "applyScenario")]
    ApplyScenario(Value),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkRule {
    #[serde(rename = "linkTypeApiNameAtoB")]
    pub link_type_api_name_a_to_b: String,
    #[serde(rename = "linkTypeApiNameBtoA")]
    pub link_type_api_name_b_to_a: String,
    pub a_side_object_type_api_name: String,
    pub b_side_object_type_api_name: String,
}

impl Client {
    /// Lists every action type in the ontology, following all pages.
    ///
    /// # Warning
    ///
    /// This can be a large and expensive request. Prefer [`Client::get_action_type`]
    /// unless populating a complete cache.
    pub async fn list_action_types(
        &self,
        ontology: impl AsRef<str>,
    ) -> Result<Vec<ActionType>, FoundryError> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response {
            data: Vec<ActionType>,
            next_page_token: Option<String>,
        }

        let mut url = self.hostname.join("api/v2/ontologies/")?;
        url.path_segments_mut()
            .map_err(|_| url::ParseError::RelativeUrlWithCannotBeABaseBase)?
            .pop_if_empty()
            .extend([ontology.as_ref(), "actionTypes"]);
        let mut action_types = Vec::new();
        let mut page_token = None;

        loop {
            let mut page_url = url.clone();
            if let Some(token) = page_token.as_deref() {
                page_url.query_pairs_mut().append_pair("pageToken", token);
            }

            let response: Response = self
                .http
                .get(page_url)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;
            action_types.extend(response.data);

            let Some(token) = response.next_page_token else {
                return Ok(action_types);
            };
            page_token = Some(token);
        }
    }

    /// Fetches a single action type by its API name.
    pub async fn get_action_type(
        &self,
        ontology: impl AsRef<str>,
        action_type: impl AsRef<str>,
    ) -> Result<ActionType, FoundryError> {
        let mut url = self.hostname.join("api/v2/ontologies/")?;
        url.path_segments_mut()
            .map_err(|_| url::ParseError::RelativeUrlWithCannotBeABaseBase)?
            .pop_if_empty()
            .extend([ontology.as_ref(), "actionTypes", action_type.as_ref()]);
        self.http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Fetches a single action type by its RID.
    pub async fn get_action_type_by_rid(
        &self,
        ontology: impl AsRef<str>,
        action_type: &Rid,
    ) -> Result<ActionType, FoundryError> {
        let mut url = self.hostname.join("api/v2/ontologies/")?;
        url.path_segments_mut()
            .map_err(|_| url::ParseError::RelativeUrlWithCannotBeABaseBase)?
            .pop_if_empty()
            .extend([
                ontology.as_ref(),
                "actionTypes",
                "byRid",
                action_type.as_str(),
            ]);
        self.http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Resolves a mixed list of action type RIDs and API names, in input order.
    ///
    /// RIDs are resolved in batches of [`ACTION_TYPE_BATCH_LIMIT`], while API names are
    /// fetched individually. A RID that does not exist, or that the token cannot read, is
    /// omitted by the server rather than reported as an error; an unknown API name fails
    /// the whole call.
    pub async fn get_action_types<I>(
        &self,
        ontology: impl AsRef<str>,
        action_types: impl IntoIterator<Item = I>,
    ) -> Result<Vec<ActionType>, FoundryError>
    where
        I: AsRef<str>,
    {
        let ontology = ontology.as_ref();
        let identifiers: Vec<I> = action_types.into_iter().collect();
        let (rids, api_names): (Vec<&str>, Vec<&str>) = identifiers
            .iter()
            .map(AsRef::as_ref)
            .partition(|identifier| identifier.parse::<Rid>().is_ok());

        let mut resolved = Vec::with_capacity(identifiers.len());
        for batch in rids.chunks(ACTION_TYPE_BATCH_LIMIT) {
            resolved.extend(self.get_action_types_by_rid(ontology, batch).await?);
        }
        for api_name in api_names {
            resolved.push(self.get_action_type(ontology, api_name).await?);
        }

        let mut ordered = Vec::with_capacity(resolved.len());
        for identifier in identifiers.iter().map(AsRef::as_ref) {
            if let Some(position) = resolved
                .iter()
                .position(|action_type| action_type.matches(identifier))
            {
                ordered.push(resolved.swap_remove(position));
            }
        }

        Ok(ordered)
    }

    /// Fetches action types by RID in a single batch request.
    ///
    /// A RID that does not exist, or that the token cannot read, is omitted from the
    /// response rather than reported as an error, so the result may be shorter than
    /// `rids`. Duplicate RIDs are echoed back once per occurrence.
    ///
    /// Callers must pass no more than [`ACTION_TYPE_BATCH_LIMIT`] RIDs, which the server
    /// enforces. This endpoint is also gated behind Foundry's preview flag, so its
    /// behaviour may change.
    async fn get_action_types_by_rid(
        &self,
        ontology: &str,
        rids: &[&str],
    ) -> Result<Vec<ActionType>, FoundryError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request<'a> {
            action_type_rid: &'a str,
        }

        #[derive(Deserialize)]
        struct Response {
            data: Vec<ActionType>,
        }

        let mut url = self.hostname.join("api/v2/ontologies/")?;
        url.path_segments_mut()
            .map_err(|_| url::ParseError::RelativeUrlWithCannotBeABaseBase)?
            .pop_if_empty()
            .extend([ontology, "actionTypes", "getByRidBatch"]);
        // This endpoint is only available as a preview feature.
        url.query_pairs_mut().append_pair("preview", "true");

        let requests: Vec<Request<'_>> = rids
            .iter()
            .map(|rid| Request {
                action_type_rid: rid,
            })
            .collect();
        let response: Response = self
            .http
            .post(url)
            .json(&serde_json::json!({ "requests": requests }))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response.data)
    }
}
