//! Default embedder implementation using Watsonx

use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use crate::error::{Error, Result};
use super::EmbedderBase;

/// Default embedder using Watsonx
pub struct DefaultEmbedder {
    api_key: String,
    project_id: String,
    model: String,
    dimension: usize,
    client: Client,
}

impl DefaultEmbedder {
    /// Create a new default embedder
    pub fn new(
        api_key: String,
        project_id: String,
        model: String,
        dimension: usize,
    ) -> Self {
        Self {
            api_key,
            project_id,
            model,
            dimension,
            client: Client::new(),
        }
    }

    /// Create with default settings
    pub fn with_defaults(api_key: String, project_id: String) -> Self {
        Self::new(
            api_key,
            project_id,
            "sentence-transformers/all-minilm-l6-v2".to_string(),
            384,
        )
    }
}

#[async_trait]
impl EmbedderBase for DefaultEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let body = json!({
            "model_id": self.model,
            "input": [text],
            "project_id": self.project_id,
        });

        let response = self
            .client
            .post("https://api.watsonx.ai/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::embedding(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::embedding(format!(
                "API error: {}",
                response.status()
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::embedding(format!("Failed to parse response: {}", e)))?;

        let embedding = result
            .get("results")
            .and_then(|r| r.get(0))
            .and_then(|r| r.get("embedding"))
            .and_then(|e| e.as_array())
            .ok_or_else(|| Error::embedding("Invalid response format"))?
            .iter()
            .filter_map(|v| v.as_f64())
            .map(|v| v as f32)
            .collect();

        Ok(embedding)
    }

    async fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        let body = json!({
            "model_id": self.model,
            "input": texts,
            "project_id": self.project_id,
        });

        let response = self
            .client
            .post("https://api.watsonx.ai/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::embedding(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::embedding(format!(
                "API error: {}",
                response.status()
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::embedding(format!("Failed to parse response: {}", e)))?;

        let embeddings = result
            .get("results")
            .and_then(|r| r.as_array())
            .ok_or_else(|| Error::embedding("Invalid response format"))?
            .iter()
            .filter_map(|item| {
                item.get("embedding")
                    .and_then(|e| e.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_f64())
                            .map(|v| v as f32)
                            .collect::<Vec<_>>()
                    })
            })
            .collect();

        Ok(embeddings)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedder_creation() {
        let embedder = DefaultEmbedder::with_defaults(
            "test-key".to_string(),
            "test-project".to_string(),
        );

        assert_eq!(embedder.dimension(), 384);
    }
}
