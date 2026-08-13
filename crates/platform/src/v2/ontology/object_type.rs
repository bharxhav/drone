use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Client;
use crate::{FoundryError, Rid};

/// The maximum number of object type RIDs accepted by a single batch request.
pub const OBJECT_TYPE_BATCH_LIMIT: usize = 100;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectType {
    pub api_name: String,
    pub display_name: String,
    pub status: ReleaseStatus,
    pub description: Option<String>,
    pub plural_display_name: String,
    pub icon: Icon,
    pub primary_key: String,
    pub properties: HashMap<String, Property>,
    pub rid: Rid,
    pub title_property: String,
    pub visibility: Option<ObjectTypeVisibility>,
    pub aliases: Option<Vec<String>>,
    pub datasources: Option<Vec<ObjectTypeDatasource>>,
}

impl ObjectType {
    /// Whether this object type is identified by `identifier`, which may be a RID or an API name.
    pub fn matches(&self, identifier: impl AsRef<str>) -> bool {
        let identifier = identifier.as_ref();
        self.rid == identifier || self.api_name == identifier
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReleaseStatus {
    Active,
    Endorsed,
    Experimental,
    Deprecated,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ObjectTypeVisibility {
    Normal,
    Prominent,
    Hidden,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Icon {
    pub color: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub data_type: ObjectPropertyType,
    pub rid: Rid,
    pub status: Option<PropertyStatus>,
    pub visibility: Option<ObjectTypeVisibility>,
    pub value_type_api_name: Option<String>,
    pub value_formatting: Option<Value>,
    pub type_classes: Option<Vec<TypeClass>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum PropertyStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "experimental")]
    Experimental,
    #[serde(rename = "example")]
    Example,
    #[serde(rename = "deprecated")]
    Deprecated {
        #[serde(rename = "message")]
        message: Option<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TypeClass {
    pub kind: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ObjectPropertyType {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "byte")]
    Byte,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "geopoint")]
    GeoPoint,
    #[serde(rename = "geotimeSeriesReference")]
    GeotimeSeriesReference,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "geoshape")]
    GeoShape,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "cipherText")]
    CipherText {
        #[serde(rename = "defaultCipherChannel")]
        default_cipher_channel: Option<Rid>,
    },
    #[serde(rename = "marking")]
    Marking {
        #[serde(rename = "markingType")]
        marking_type: Option<MarkingType>,
    },
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "mediaReference")]
    MediaReference,
    #[serde(rename = "timeseries")]
    Timeseries {
        #[serde(rename = "itemType")]
        item_type: TimeSeriesItemType,
    },
    #[serde(rename = "array")]
    Array {
        #[serde(rename = "subType")]
        sub_type: Box<ObjectPropertyType>,
        reducers: Vec<ArrayReducer>,
    },
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "vector")]
    Vector {
        dimension: u32,
        #[serde(rename = "supportsSearchWith")]
        supports_search_with: Vec<VectorSimilarityFunction>,
        #[serde(rename = "embeddingModel")]
        embedding_model: Option<Value>,
    },
    #[serde(rename = "decimal")]
    Decimal {
        precision: Option<u8>,
        scale: Option<u8>,
    },
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "struct")]
    Struct {
        #[serde(rename = "structFieldTypes")]
        struct_field_types: Vec<StructField>,
        #[serde(rename = "mainValue")]
        main_value: Option<StructMainValue>,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarkingType {
    Cbac,
    Mandatory,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum TimeSeriesItemType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "numericOrNonNumeric")]
    NumericOrNonNumeric,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VectorSimilarityFunctionValue {
    CosineSimilarity,
    DotProduct,
    EuclideanDistance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VectorSimilarityFunction {
    pub value: Option<VectorSimilarityFunctionValue>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ArrayReducer {
    pub direction: ArrayReducerDirection,
    pub field: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArrayReducerDirection {
    AscendingNullsLast,
    DescendingNullsLast,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructField {
    pub api_name: String,
    pub rid: Rid,
    pub data_type: Box<ObjectPropertyType>,
    pub type_classes: Option<Vec<TypeClass>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructMainValue {
    pub main_value_type: Box<ObjectPropertyType>,
    pub fields: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectTypeDatasource {
    pub rid: Rid,
    pub definition: Value,
}

impl Client {
    /// Lists every object type in the ontology, following all pages.
    ///
    /// # Warning
    ///
    /// This can be a large and expensive request. Prefer [`Client::get_object_type`]
    /// unless populating a complete cache.
    pub async fn list_object_types(
        &self,
        ontology: impl AsRef<str>,
    ) -> Result<Vec<ObjectType>, FoundryError> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response {
            data: Vec<ObjectType>,
            next_page_token: Option<String>,
        }

        let mut url = self.hostname.join("api/v2/ontologies/")?;
        url.path_segments_mut()
            .map_err(|_| url::ParseError::RelativeUrlWithCannotBeABaseBase)?
            .pop_if_empty()
            .extend([ontology.as_ref(), "objectTypes"]);
        let mut object_types = Vec::new();
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
            object_types.extend(response.data);

            let Some(token) = response.next_page_token else {
                return Ok(object_types);
            };
            page_token = Some(token);
        }
    }

    /// Fetches a single object type by its API name.
    pub async fn get_object_type(
        &self,
        ontology: impl AsRef<str>,
        object_type: impl AsRef<str>,
    ) -> Result<ObjectType, FoundryError> {
        let mut url = self.hostname.join("api/v2/ontologies/")?;
        url.path_segments_mut()
            .map_err(|_| url::ParseError::RelativeUrlWithCannotBeABaseBase)?
            .pop_if_empty()
            .extend([ontology.as_ref(), "objectTypes", object_type.as_ref()]);
        self.http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Fetches object types by RID in a single batch request.
    ///
    /// A RID that does not exist, or that the token cannot read, is omitted from the
    /// response rather than reported as an error, so the result may be shorter than
    /// `rids`. Duplicate RIDs are echoed back once per occurrence.
    ///
    /// Callers must pass no more than [`OBJECT_TYPE_BATCH_LIMIT`] RIDs, which the server
    /// enforces. This endpoint is also gated behind Foundry's preview flag, so its
    /// behaviour may change.
    async fn get_object_types_by_rid(
        &self,
        ontology: &str,
        rids: &[&str],
    ) -> Result<Vec<ObjectType>, FoundryError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request<'a> {
            object_type_rid: &'a str,
        }

        #[derive(Deserialize)]
        struct Response {
            data: Vec<ObjectType>,
        }

        let mut url = self.hostname.join("api/v2/ontologies/")?;
        url.path_segments_mut()
            .map_err(|_| url::ParseError::RelativeUrlWithCannotBeABaseBase)?
            .pop_if_empty()
            .extend([ontology, "objectTypes", "getByRidBatch"]);
        // This endpoint is only available as a preview feature.
        url.query_pairs_mut().append_pair("preview", "true");

        let requests: Vec<Request<'_>> = rids
            .iter()
            .map(|rid| Request {
                object_type_rid: rid,
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

    /// Resolves a mixed list of object type RIDs and API names, in input order.
    ///
    /// RIDs are resolved in batches of [`OBJECT_TYPE_BATCH_LIMIT`], while API names are
    /// fetched individually. A RID that does not exist, or that the token cannot read, is
    /// omitted by the server rather than reported as an error; an unknown API name fails
    /// the whole call.
    pub async fn get_object_types<I>(
        &self,
        ontology: impl AsRef<str>,
        object_types: impl IntoIterator<Item = I>,
    ) -> Result<Vec<ObjectType>, FoundryError>
    where
        I: AsRef<str>,
    {
        let ontology = ontology.as_ref();
        let identifiers: Vec<I> = object_types.into_iter().collect();
        let (rids, api_names): (Vec<&str>, Vec<&str>) = identifiers
            .iter()
            .map(AsRef::as_ref)
            .partition(|identifier| identifier.parse::<Rid>().is_ok());

        let mut resolved = Vec::with_capacity(identifiers.len());

        for batch in rids.chunks(OBJECT_TYPE_BATCH_LIMIT) {
            resolved.extend(self.get_object_types_by_rid(ontology, batch).await?);
        }

        for api_name in api_names {
            resolved.push(self.get_object_type(ontology, api_name).await?);
        }

        let mut ordered = Vec::with_capacity(resolved.len());
        for identifier in identifiers.iter().map(AsRef::as_ref) {
            if let Some(position) = resolved
                .iter()
                .position(|object_type| object_type.matches(identifier))
            {
                ordered.push(resolved.swap_remove(position));
            }
        }

        Ok(ordered)
    }
}
