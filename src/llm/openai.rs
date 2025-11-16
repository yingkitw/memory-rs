//! OpenAI LLM integration

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::Result;
use super::{LlmBase, GenerationParams};

/// OpenAI LLM provider
pub struct OpenAILLM {
    /// API key
    api_key: String,
    /// Model name
    model: String,
    /// HTTP client
    client: reqwest::Client,
    /// API endpoint
    endpoint: String,
}

/// OpenAI chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message role (system, user, assistant)
    pub role: String,
    /// Message content
    pub content: String,
}

/// OpenAI chat request
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

/// OpenAI chat response
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

/// Response choice
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ChatMessage,
    pub finish_reason: String,
}

/// Token usage
#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

impl OpenAILLM {
    /// Create a new OpenAI LLM
    pub fn new(api_key: String) -> Self {
        Self::with_model(api_key, "gpt-4".to_string())
    }

    /// Create with specific model
    pub fn with_model(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
            endpoint: "https://api.openai.com/v1".to_string(),
        }
    }

    /// Create with custom endpoint
    pub fn with_endpoint(api_key: String, model: String, endpoint: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
            endpoint,
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

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/models", self.endpoint);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        let models_response: serde_json::Value = response.json().await?;
        
        let models = models_response
            .get("data")
            .and_then(|d| d.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| m.get("id").and_then(|id| id.as_str()).map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(models)
    }
}

#[async_trait]
impl LlmBase for OpenAILLM {
    /// Generate text
    async fn generate(&self, prompt: &str, params: Option<GenerationParams>) -> Result<String> {
        let params = params.unwrap_or_default();
        
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            max_tokens: params.max_tokens,
            temperature: params.temperature,
            top_p: params.top_p,
        };

        let url = format!("{}/chat/completions", self.endpoint);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;

        let text = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
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
    fn test_openai_creation() {
        let llm = OpenAILLM::new("test-key".to_string());
        assert_eq!(llm.get_model(), "gpt-4");
    }

    #[test]
    fn test_openai_with_model() {
        let llm = OpenAILLM::with_model("test-key".to_string(), "gpt-3.5-turbo".to_string());
        assert_eq!(llm.get_model(), "gpt-3.5-turbo");
    }

    #[test]
    fn test_openai_set_model() {
        let mut llm = OpenAILLM::new("test-key".to_string());
        llm.set_model("gpt-3.5-turbo".to_string());
        assert_eq!(llm.get_model(), "gpt-3.5-turbo");
    }

    #[test]
    fn test_chat_message_serialization() {
        let msg = ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };

        let json = serde_json::to_string(&msg);
        assert!(json.is_ok());
    }

    #[test]
    fn test_chat_request_serialization() {
        let request = ChatRequest {
            model: "gpt-4".to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
            max_tokens: Some(100),
            temperature: Some(0.7),
            top_p: None,
        };

        let json = serde_json::to_string(&request);
        assert!(json.is_ok());
    }
}
