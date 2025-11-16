//! Watsonx LLM implementation

use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use crate::error::{Error, Result};
use super::{LlmBase, GenerationParams};

/// Watsonx LLM implementation
pub struct WatsonxLLM {
    api_key: String,
    project_id: String,
    model: String,
    client: Client,
}

impl WatsonxLLM {
    /// Create a new Watsonx LLM instance
    pub fn new(api_key: String, project_id: String, model: String) -> Self {
        Self {
            api_key,
            project_id,
            model,
            client: Client::new(),
        }
    }

    /// Create with default model (ibm/granite-4-h-small)
    pub fn with_defaults(api_key: String, project_id: String) -> Self {
        Self::new(
            api_key,
            project_id,
            "ibm/granite-4-h-small".to_string(),
        )
    }
}

#[async_trait]
impl LlmBase for WatsonxLLM {
    async fn generate(
        &self,
        prompt: &str,
        params: Option<GenerationParams>,
    ) -> Result<String> {
        let params = params.unwrap_or_default();

        let body = json!({
            "model_id": self.model,
            "input": prompt,
            "parameters": {
                "max_tokens": params.max_tokens.unwrap_or(1024),
                "temperature": params.temperature.unwrap_or(0.7),
                "top_p": params.top_p.unwrap_or(0.9),
            },
            "project_id": self.project_id,
        });

        let response = self
            .client
            .post("https://api.watsonx.ai/v1/text/generation")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::llm(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::llm(format!(
                "API error: {}",
                response.status()
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::llm(format!("Failed to parse response: {}", e)))?;

        let text = result
            .get("results")
            .and_then(|r| r.get(0))
            .and_then(|r| r.get("generated_text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| Error::llm("Invalid response format"))?
            .to_string();

        Ok(text)
    }

    async fn generate_stream(
        &self,
        prompt: &str,
        params: Option<GenerationParams>,
    ) -> Result<String> {
        // For now, use regular generation
        // TODO: Implement actual streaming with SSE
        self.generate(prompt, params).await
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watsonx_creation() {
        let llm = WatsonxLLM::with_defaults(
            "test-key".to_string(),
            "test-project".to_string(),
        );

        assert_eq!(llm.model_name(), "ibm/granite-4-h-small");
    }

    #[tokio::test]
    #[ignore] // Requires valid API credentials
    async fn test_generate() {
        let llm = WatsonxLLM::with_defaults(
            std::env::var("WATSONX_API_KEY").unwrap_or_default(),
            std::env::var("WATSONX_PROJECT_ID").unwrap_or_default(),
        );

        let result = llm.generate("Hello, world!", None).await;
        assert!(result.is_ok());
    }
}
