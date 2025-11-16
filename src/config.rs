//! Configuration types for mem0-rs

use serde::{Deserialize, Serialize};

/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Qdrant vector store URL
    pub vector_store_url: String,

    /// Watsonx API key
    pub watsonx_api_key: String,

    /// Watsonx project ID
    pub watsonx_project_id: Option<String>,

    /// LLM model name (default: ibm/granite-4-h-small)
    pub llm_model: Option<String>,

    /// Embedding model name
    pub embedding_model: Option<String>,

    /// Vector dimension (default: 384)
    pub vector_dimension: Option<usize>,

    /// Collection name prefix (default: "mem0")
    pub collection_prefix: Option<String>,

    /// Enable telemetry (default: true)
    pub enable_telemetry: Option<bool>,

    /// Batch size for operations (default: 32)
    pub batch_size: Option<usize>,
}

impl MemoryConfig {
    /// Create a new memory configuration
    pub fn new(vector_store_url: String, watsonx_api_key: String) -> Self {
        Self {
            vector_store_url,
            watsonx_api_key,
            watsonx_project_id: None,
            llm_model: Some("ibm/granite-4-h-small".to_string()),
            embedding_model: None,
            vector_dimension: Some(384),
            collection_prefix: Some("mem0".to_string()),
            enable_telemetry: Some(true),
            batch_size: Some(32),
        }
    }

    /// Set Watsonx project ID
    pub fn with_project_id(mut self, project_id: String) -> Self {
        self.watsonx_project_id = Some(project_id);
        self
    }

    /// Set LLM model
    pub fn with_llm_model(mut self, model: String) -> Self {
        self.llm_model = Some(model);
        self
    }

    /// Set embedding model
    pub fn with_embedding_model(mut self, model: String) -> Self {
        self.embedding_model = Some(model);
        self
    }

    /// Set vector dimension
    pub fn with_vector_dimension(mut self, dim: usize) -> Self {
        self.vector_dimension = Some(dim);
        self
    }

    /// Set collection prefix
    pub fn with_collection_prefix(mut self, prefix: String) -> Self {
        self.collection_prefix = Some(prefix);
        self
    }

    /// Enable/disable telemetry
    pub fn with_telemetry(mut self, enabled: bool) -> Self {
        self.enable_telemetry = Some(enabled);
        self
    }

    /// Set batch size
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }

    /// Get LLM model name
    pub fn get_llm_model(&self) -> String {
        self.llm_model
            .clone()
            .unwrap_or_else(|| "ibm/granite-4-h-small".to_string())
    }

    /// Get vector dimension
    pub fn get_vector_dimension(&self) -> usize {
        self.vector_dimension.unwrap_or(384)
    }

    /// Get collection prefix
    pub fn get_collection_prefix(&self) -> String {
        self.collection_prefix
            .clone()
            .unwrap_or_else(|| "mem0".to_string())
    }

    /// Check if telemetry is enabled
    pub fn is_telemetry_enabled(&self) -> bool {
        self.enable_telemetry.unwrap_or(true)
    }

    /// Get batch size
    pub fn get_batch_size(&self) -> usize {
        self.batch_size.unwrap_or(32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = MemoryConfig::new(
            "http://localhost:6334".to_string(),
            "test-key".to_string(),
        );

        assert_eq!(config.get_llm_model(), "ibm/granite-4-h-small");
        assert_eq!(config.get_vector_dimension(), 384);
        assert_eq!(config.get_collection_prefix(), "mem0");
        assert!(config.is_telemetry_enabled());
        assert_eq!(config.get_batch_size(), 32);
    }

    #[test]
    fn test_config_builder() {
        let config = MemoryConfig::new(
            "http://localhost:6334".to_string(),
            "test-key".to_string(),
        )
        .with_vector_dimension(768)
        .with_collection_prefix("custom".to_string())
        .with_telemetry(false);

        assert_eq!(config.get_vector_dimension(), 768);
        assert_eq!(config.get_collection_prefix(), "custom");
        assert!(!config.is_telemetry_enabled());
    }
}
