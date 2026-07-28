use reqwest::Method;

use super::models::*;
use crate::{
    error::Result,
    transport::{RequestBody, Transport},
};
use bytes::Bytes;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

#[derive(Debug)]
pub struct Connections<'c> {
    transport: &'c Transport,
}

impl<'c> Connections<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn create(&self, request: CreateConnectionRequest) -> Result<Connection> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/connectivity/connections",
                &[],
                Some(&body),
            )
            .await
    }

    pub async fn get(&self, connection_rid: &str) -> Result<Connection> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/connectivity/connections/{connection_rid}"),
                &[],
                None,
            )
            .await
    }

    pub async fn get_configuration(&self, connection_rid: &str) -> Result<ConnectionConfiguration> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/connectivity/connections/{connection_rid}/getConfiguration"),
                &[],
                None,
            )
            .await
    }

    pub async fn get_configuration_batch(
        &self,
        request: Vec<GetConfigurationConnectionsBatchRequestElement>,
    ) -> Result<GetConfigurationConnectionsBatchResponse> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/connectivity/connections/getConfigurationBatch",
                &[],
                Some(&body),
            )
            .await
    }

    pub async fn update_export_settings(
        &self,
        connection_rid: &str,
        export_settings: ConnectionExportSettings,
    ) -> Result<()> {
        let body =
            serde_json::to_value(UpdateExportSettingsForConnectionRequest { export_settings })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/connectivity/connections/{connection_rid}/updateExportSettings"),
                &[],
                Some(&body),
            )
            .await
    }

    pub async fn update_secrets(
        &self,
        connection_rid: &str,
        secrets: std::collections::HashMap<SecretName, PlaintextValue>,
    ) -> Result<()> {
        let body = serde_json::to_value(UpdateSecretsForConnectionRequest { secrets })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/connectivity/connections/{connection_rid}/updateSecrets"),
                &[],
                Some(&body),
            )
            .await
    }

    pub async fn upload_custom_jdbc_drivers(
        &self,
        connection_rid: &str,
        body: impl Into<Bytes>,
        file_name: &str,
    ) -> Result<Connection> {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        let response = self
            .transport
            .send(
                Method::POST,
                &format!("v2/connectivity/connections/{connection_rid}/uploadCustomJdbcDrivers"),
                &[("fileName", file_name)],
                headers,
                Some(RequestBody::Bytes(body.into())),
            )
            .await?;
        Ok(response.json().await?)
    }
}
