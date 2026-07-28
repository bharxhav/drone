pub mod errors;
pub mod models;
pub mod resources;

use crate::transport::Transport;
pub use resources::{AnthropicModels, OpenAiModels};

/// Language Models namespace handle (proxy, completions).
#[derive(Debug)]
pub struct LanguageModels<'c> {
    pub(crate) transport: &'c Transport,
}

impl<'c> LanguageModels<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub fn anthropic_models(&self) -> AnthropicModels<'c> {
        AnthropicModels::new(self.transport)
    }

    pub fn open_ai_models(&self) -> OpenAiModels<'c> {
        OpenAiModels::new(self.transport)
    }
}
