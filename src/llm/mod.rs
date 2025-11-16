//! LLM abstraction and implementations

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::Result;

pub mod watsonx;
pub mod prompts;
pub mod openai;
pub mod claude;

pub use watsonx::WatsonxLLM;
pub use prompts::{PromptTemplate, PromptManager};
pub use openai::OpenAILLM;
pub use claude::ClaudeLLM;

/// LLM generation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParams {
    /// Maximum tokens to generate
    pub max_tokens: Option<usize>,

    /// Temperature for sampling (0-1)
    pub temperature: Option<f32>,

    /// Top-p for nucleus sampling
    pub top_p: Option<f32>,

    /// Top-k for top-k sampling
    pub top_k: Option<usize>,

    /// Stop sequences
    pub stop_sequences: Option<Vec<String>>,
}

impl Default for GenerationParams {
    fn default() -> Self {
        Self {
            max_tokens: Some(1024),
            temperature: Some(0.7),
            top_p: Some(0.9),
            top_k: None,
            stop_sequences: None,
        }
    }
}

/// Base trait for LLM implementations
#[async_trait]
pub trait LlmBase: Send + Sync {
    /// Generate text from prompt
    async fn generate(
        &self,
        prompt: &str,
        params: Option<GenerationParams>,
    ) -> Result<String>;

    /// Generate text with streaming
    async fn generate_stream(
        &self,
        prompt: &str,
        params: Option<GenerationParams>,
    ) -> Result<String>;

    /// Get model name
    fn model_name(&self) -> &str;
}
