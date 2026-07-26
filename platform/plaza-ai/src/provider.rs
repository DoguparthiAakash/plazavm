//! AI provider trait and built-in provider stubs.

use async_trait::async_trait;
use plaza_foundation::core::PlazaResult;

/// Contract for AI provider backends (OpenAI, Ollama, Anthropic, etc.).
#[async_trait]
pub trait AiProvider: Send + Sync {
    fn id(&self) -> &str;
    fn display_name(&self) -> &str;
    async fn is_available(&self) -> bool;
    async fn complete(&self, prompt: &str) -> PlazaResult<String>;
}

/// Stub for local Ollama provider.
#[allow(dead_code)]
pub struct OllamaProvider {
    endpoint: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(endpoint: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            model: model.into(),
        }
    }
}

#[async_trait]
impl AiProvider for OllamaProvider {
    fn id(&self) -> &str {
        "ollama"
    }

    fn display_name(&self) -> &str {
        "Ollama (Local LLM)"
    }

    async fn is_available(&self) -> bool {
        // Phase 2 check endpoint ping
        false
    }

    async fn complete(&self, _prompt: &str) -> PlazaResult<String> {
        Ok("Ollama provider stub response".into())
    }
}

/// Stub for OpenAI API provider.
#[allow(dead_code)]
pub struct OpenAIProvider {
    api_key: Option<String>,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: Option<String>, model: impl Into<String>) -> Self {
        Self {
            api_key,
            model: model.into(),
        }
    }
}

#[async_trait]
impl AiProvider for OpenAIProvider {
    fn id(&self) -> &str {
        "openai"
    }

    fn display_name(&self) -> &str {
        "OpenAI API"
    }

    async fn is_available(&self) -> bool {
        self.api_key.is_some()
    }

    async fn complete(&self, _prompt: &str) -> PlazaResult<String> {
        Ok("OpenAI provider stub response".into())
    }
}

