use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;

fn preview(value: Option<bool>) -> Vec<(&'static str, &'static str)> {
    value
        .map(|v| vec![("preview", if v { "true" } else { "false" })])
        .unwrap_or_default()
}

#[derive(Debug)]
pub struct ApiDefinitions<'c> {
    transport: &'c Transport,
}
impl<'c> ApiDefinitions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(
        &self,
        version: IrVersion,
        preview_mode: Option<bool>,
    ) -> Result<ApiDefinition> {
        let version = match version {
            IrVersion::V1 => "v1",
            IrVersion::V2 => "v2",
        };
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/publicApis/apiDefinitions/{version}"),
                &preview(preview_mode),
                None,
            )
            .await
    }
}

#[derive(Debug)]
pub struct OpenApiDefinitions<'c> {
    transport: &'c Transport,
}
impl<'c> OpenApiDefinitions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(
        &self,
        version: ApiVersion,
        preview_mode: Option<bool>,
    ) -> Result<OpenApiDefinition> {
        let version = match version {
            ApiVersion::V1 => "v1",
            ApiVersion::V2 => "v2",
        };
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/publicApis/openApiDefinitions/{version}"),
                &preview(preview_mode),
                None,
            )
            .await
    }
}
