use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Client;
use crate::{FoundryError, Rid};

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
}
