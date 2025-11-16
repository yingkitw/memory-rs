//! Claude (Anthropic) LLM integration

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::Result;
use super::{LlmBase, GenerationParams};

/// Claude LLM provider (Anthropic)
pub struct ClaudeLLM {
    /// API key
    api_key: String,
    /// Model name
    model: String,
    /// HTTP client
    client: reqwest::Client,
    /// API endpoint
    endpoint: String,
    /// API version
    api_version: String,
}

/// Claude message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message role (user, assistant)
    pub role: String,
    /// Message content
    pub content: String,
}

/// Claude message request
#[derive(Debug, Serialize)]
pub struct MessageRequest {
    pub model: String,
    pub max_tokens: usize,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

/// Claude message response
#[derive(Debug, Deserialize)]
pub struct MessageResponse {
    pub id: String,
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: String,
    pub usage: Option<Usage>,
}

/// Content block
#[derive(Debug, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

/// Token usage
#[derive(Debug, Deserialize)]
pub struct Usage {
    pub input_tokens: usize,
    pub output_tokens: usize,
}

impl ClaudeLLM {
    /// Create a new Claude LLM
    pub fn new(api_key: String) -> Self {
        Self::with_model(api_key, "claude-3-opus-20240229".to_string())
    }

    /// Create with specific model
    pub fn with_model(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
            endpoint: "https://api.anthropic.com/v1".to_string(),
            api_version: "2023-06-01".to_string(),
        }
    }

    /// Create with custom endpoint
    pub fn with_endpoint(api_key: String, model: String, endpoint: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
            endpoint,
            api_version: "2023-06-01".to_string(),
        }
    }

    /// Set model
    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }

    /// Get current model
    pub fn get_model(&self) -> &str {
        &self.model
    }

    /// Available Claude models
    pub fn available_models() -> Vec<&'static str> {
        vec![
            "claude-3-opus-20240229",
            "claude-3-sonnet-20240229",
            "claude-3-haiku-20240307",
            "claude-2.1",
            "claude-2",
        ]
    }
}

#[async_trait]
impl LlmBase for ClaudeLLM {
    /// Generate text
    async fn generate(&self, prompt: &str, params: Option<GenerationParams>) -> Result<String> {
        let params = params.unwrap_or_default();
        
        let messages = vec![
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        let request = MessageRequest {
            model: self.model.clone(),
            max_tokens: params.max_tokens.unwrap_or(1024),
            messages,
            temperature: params.temperature,
            top_p: params.top_p,
            system: Some("You are a helpful assistant.".to_string()),
        };

        let url = format!("{}/messages", self.endpoint);
        
        let response = self.client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.api_version)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        let message_response: MessageResponse = response.json().await?;

        let text = message_response
            .content
            .first()
            .and_then(|c| c.text.clone())
            .unwrap_or_default();

        Ok(text)
    }

    /// Generate with streaming (placeholder)
    async fn generate_stream(&self, prompt: &str, params: Option<GenerationParams>) -> Result<String> {
        // For now, use regular generation
        // Full streaming support would require SSE handling
        self.generate(prompt, params).await
    }

    /// Get model name
    fn model_name(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_creation() {
        let llm = ClaudeLLM::new("test-key".to_string());
        assert_eq!(llm.get_model(), "claude-3-opus-20240229");
    }

    #[test]
    fn test_claude_with_model() {
        let llm = ClaudeLLM::with_model(
            "test-key".to_string(),
            "claude-3-sonnet-20240229".to_string(),
        );
        assert_eq!(llm.get_model(), "claude-3-sonnet-20240229");
    }

    #[test]
    fn test_claude_set_model() {
        let mut llm = ClaudeLLM::new("test-key".to_string());
        llm.set_model("claude-3-haiku-20240307".to_string());
        assert_eq!(llm.get_model(), "claude-3-haiku-20240307");
    }

    #[test]
    fn test_available_models() {
        let models = ClaudeLLM::available_models();
        assert!(models.len() > 0);
        assert!(models.contains(&"claude-3-opus-20240229"));
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };

        let json = serde_json::to_string(&msg);
        assert!(json.is_ok());
    }

    #[test]
    fn test_message_request_serialization() {
        let request = MessageRequest {
            model: "claude-3-opus-20240229".to_string(),
            max_tokens: 1024,
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
            temperature: Some(0.7),
            top_p: None,
            system: Some("You are helpful".to_string()),
        };

        let json = serde_json::to_string(&request);
        assert!(json.is_ok());
    }
}
