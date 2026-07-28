//! Language Models namespace wire types.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type JsonSchema = HashMap<String, Value>;
pub type LanguageModelApiName = String;
pub type LanguageModelRid = String;
pub type OpenAiEmbeddingInput = Vec<String>;
pub type AnthropicDisableParallelToolUse = bool;
pub type AnthropicCacheControl = AnthropicEphemeralCacheControl;
pub type AnthropicCompletionCitation = AnthropicCharacterLocationCitation;
pub type AnthropicImageSource = AnthropicImageBase64Source;
pub type AnthropicOutputFormat = AnthropicJsonSchemaOutputFormat;
pub type AnthropicSystemMessage = AnthropicText;
pub type AnthropicTool = AnthropicCustomTool;
pub type AnthropicToolResultContent = AnthropicText;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicAnyToolChoice {
    pub disable_parallel_tool_use: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicAutoToolChoice {
    pub disable_parallel_tool_use: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicNoneToolChoice {}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicToolToolChoice {
    pub name: String,
    pub disable_parallel_tool_use: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicToolChoice {
    #[serde(rename = "auto")]
    Auto(AnthropicAutoToolChoice),
    #[serde(rename = "none")]
    None(AnthropicNoneToolChoice),
    #[serde(rename = "any")]
    Any(AnthropicAnyToolChoice),
    #[serde(rename = "tool")]
    Tool(AnthropicToolToolChoice),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicBase64PdfDocumentSource {
    pub data: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicTextDocumentSource {
    pub data: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicUrlDocumentSource {
    pub url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicDocumentSource {
    #[serde(rename = "pdf")]
    Pdf(AnthropicBase64PdfDocumentSource),
    #[serde(rename = "text")]
    Text(AnthropicTextDocumentSource),
    #[serde(rename = "url")]
    Url(AnthropicUrlDocumentSource),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicCharacterLocationCitation {
    pub cited_text: String,
    pub document_index: i64,
    pub document_title: Option<String>,
    pub start_char_index: i64,
    pub end_char_index: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicEphemeralCacheControl {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicDocumentCitations {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicDocument {
    pub source: AnthropicDocumentSource,
    pub cache_control: Option<AnthropicCacheControl>,
    pub citations: Option<AnthropicDocumentCitations>,
    pub context: Option<String>,
    pub title: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicImageBase64Source {
    pub data: String,
    pub media_type: AnthropicMediaType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicImage {
    pub source: AnthropicImageSource,
    pub cache_control: Option<AnthropicCacheControl>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicText {
    pub text: String,
    pub citations: Option<Vec<AnthropicCompletionCitation>>,
    pub cache_control: Option<AnthropicCacheControl>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicThinking {
    pub signature: String,
    pub thinking: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicRedactedThinking {
    pub data: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicToolUse {
    pub id: String,
    pub input: Value,
    pub name: String,
    pub cache_control: Option<AnthropicCacheControl>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicToolResult {
    pub tool_use_id: String,
    pub content: Option<Vec<AnthropicToolResultContent>>,
    pub is_error: Option<bool>,
    pub cache_control: Option<AnthropicCacheControl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicMessageContent {
    #[serde(rename = "image")]
    Image(AnthropicImage),
    #[serde(rename = "toolUse")]
    ToolUse(AnthropicToolUse),
    #[serde(rename = "document")]
    Document(AnthropicDocument),
    #[serde(rename = "text")]
    Text(AnthropicText),
    #[serde(rename = "toolResult")]
    ToolResult(AnthropicToolResult),
    #[serde(rename = "thinking")]
    Thinking(AnthropicThinking),
    #[serde(rename = "redactedThinking")]
    RedactedThinking(AnthropicRedactedThinking),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AnthropicMessageRole {
    User,
    Assistant,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnthropicMediaType {
    ImageJpeg,
    ImagePng,
    ImageGif,
    ImageWebp,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AnthropicEffort {
    Low,
    Medium,
    High,
    Max,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicDisabledThinking {}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicEnabledThinking {
    pub budget_tokens: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicThinkingConfig {
    #[serde(rename = "disabled")]
    Disabled(AnthropicDisabledThinking),
    #[serde(rename = "enabled")]
    Enabled(AnthropicEnabledThinking),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicJsonSchemaOutputFormat {
    pub schema: JsonSchema,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicOutputConfig {
    pub format: Option<AnthropicOutputFormat>,
    pub effort: Option<AnthropicEffort>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicCustomTool {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "inputSchema")]
    pub input_schema: JsonSchema,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicMessage {
    pub content: Vec<AnthropicMessageContent>,
    pub role: AnthropicMessageRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicMessagesRequest {
    pub messages: Vec<AnthropicMessage>,
    pub max_tokens: i64,
    pub stop_sequences: Option<Vec<String>>,
    pub system: Option<Vec<AnthropicSystemMessage>>,
    pub temperature: Option<f64>,
    pub thinking: Option<AnthropicThinkingConfig>,
    pub tool_choice: Option<AnthropicToolChoice>,
    pub tools: Option<Vec<AnthropicTool>>,
    pub top_k: Option<i64>,
    pub top_p: Option<f64>,
    pub output_config: Option<AnthropicOutputConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicCompletionRedactedThinking {
    pub data: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicCompletionText {
    pub text: String,
    pub citations: Option<Vec<AnthropicCompletionCitation>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicCompletionThinking {
    pub signature: String,
    pub thinking: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicCompletionToolUse {
    pub id: String,
    pub input: Value,
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicCompletionContent {
    #[serde(rename = "toolUse")]
    ToolUse(AnthropicCompletionToolUse),
    #[serde(rename = "text")]
    Text(AnthropicCompletionText),
    #[serde(rename = "thinking")]
    Thinking(AnthropicCompletionThinking),
    #[serde(rename = "redactedThinking")]
    RedactedThinking(AnthropicCompletionRedactedThinking),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicTokenUsage {
    pub cache_creation_input_tokens: Option<i64>,
    pub cache_read_input_tokens: Option<i64>,
    pub input_tokens: i64,
    pub output_tokens: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicMessagesResponse {
    pub content: Vec<AnthropicCompletionContent>,
    pub id: String,
    pub model: String,
    pub role: AnthropicMessageRole,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: AnthropicTokenUsage,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnthropicModel {
    pub model_id: LanguageModelApiName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OpenAiEncodingFormat {
    Float,
    Base64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAiEmbeddingsRequest {
    pub input: OpenAiEmbeddingInput,
    pub dimensions: Option<i64>,
    pub encoding_format: Option<OpenAiEncodingFormat>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAiEmbeddingTokenUsage {
    pub prompt_tokens: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiEmbeddingsResponse {
    pub data: Vec<Vec<f64>>,
    pub model: String,
    pub usage: OpenAiEmbeddingTokenUsage,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAiModel {
    pub model_id: LanguageModelApiName,
}
