use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;

fn preview(value: Option<bool>) -> Vec<(&'static str, &'static str)> {
    value
        .map(|v| vec![("preview", if v { "true" } else { "false" })])
        .unwrap_or_default()
}

#[derive(Debug)]
pub struct AnthropicModels<'c> {
    transport: &'c Transport,
}
impl<'c> AnthropicModels<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn messages(
        &self,
        model_id: &str,
        request: AnthropicMessagesRequest,
        preview_mode: Option<bool>,
    ) -> Result<AnthropicMessagesResponse> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/languageModels/anthropic/{model_id}/messages"),
                &preview(preview_mode),
                Some(&body),
            )
            .await
    }
}

#[derive(Debug)]
pub struct OpenAiModels<'c> {
    transport: &'c Transport,
}
impl<'c> OpenAiModels<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn embeddings(
        &self,
        model_id: &str,
        request: OpenAiEmbeddingsRequest,
        preview_mode: Option<bool>,
    ) -> Result<OpenAiEmbeddingsResponse> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/languageModels/openAi/{model_id}/embeddings"),
                &preview(preview_mode),
                Some(&body),
            )
            .await
    }
}
