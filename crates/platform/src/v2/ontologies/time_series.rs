use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct TimeSeries<'c> {
    transport: &'c Transport,
}

impl<'c> TimeSeries<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn get_latest_value(
        &self,
        ontology: &str,
        object_type: &str,
        primary_key: &str,
        property_name: &str,
        common: CommonQuery<'_>,
    ) -> Result<Option<TimeseriesEntry>> {
        let query = common_params(&common);
        let path = format!(
            "v2/ontologies/{ontology}/objects/{object_type}/{primary_key}/timeseries/{property_name}/latestValue"
        );
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn stream_values(
        &self,
        ontology: &str,
        object_type: &str,
        primary_key: &str,
        property: &str,
        request: StreamTimeSeriesValuesRequest,
        common: CommonQuery<'_>,
    ) -> Result<bytes::Bytes> {
        let query = common_params(&common);
        let body = serde_json::to_value(request)?;
        let path = format!(
            "v2/ontologies/{ontology}/objects/{object_type}/{primary_key}/timeseries/{property}/streamValues"
        );
        let url = self
            .transport
            .base_url
            .join(&path)
            .map_err(|e| crate::error::Error::Config(e.to_string()))?;
        let response = self
            .transport
            .http
            .request(Method::POST, url)
            .header(
                reqwest::header::AUTHORIZATION,
                self.transport.auth.header_value(),
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .query(&query)
            .json(&body)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.bytes().await?)
        } else {
            // Current transport cannot expose its error parser for binary POST responses.
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(crate::error::Error::Api(Box::new(crate::error::ApiError {
                status,
                error_name: None,
                error_code: None,
                error_instance_id: None,
                error_description: Some(text),
                parameters: serde_json::Value::Null,
            })))
        }
    }
}

fn common_params<'a>(common: &'a CommonQuery<'a>) -> Vec<(&'static str, &'a str)> {
    let mut query = Vec::new();
    if let Some(v) = common.branch {
        query.push(("branch", v));
    }
    if let Some(v) = common.sdk_package_rid {
        query.push(("sdkPackageRid", v));
    }
    if let Some(v) = common.sdk_version {
        query.push(("sdkVersion", v));
    }
    query
}
