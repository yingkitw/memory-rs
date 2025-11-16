//! Pinecone vector store implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Result;
use super::{VectorStoreBase, VectorMetadata, SearchResult};

/// Pinecone vector store
pub struct PineconeStore {
    /// API key
    api_key: String,
    /// Index name
    index_name: String,
    /// API endpoint
    endpoint: String,
    /// HTTP client
    client: reqwest::Client,
}

/// Pinecone vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PineconeVector {
    /// Vector ID
    pub id: String,
    /// Vector values
    pub values: Vec<f32>,
    /// Metadata
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Pinecone upsert request
#[derive(Debug, Serialize)]
pub struct UpsertRequest {
    pub vectors: Vec<PineconeVector>,
}

/// Pinecone query request
#[derive(Debug, Serialize)]
pub struct QueryRequest {
    pub vector: Vec<f32>,
    pub top_k: usize,
    pub include_metadata: bool,
    pub filter: Option<HashMap<String, serde_json::Value>>,
}

/// Pinecone query response
#[derive(Debug, Deserialize)]
pub struct QueryResponse {
    pub matches: Vec<QueryMatch>,
}

/// Query match result
#[derive(Debug, Deserialize)]
pub struct QueryMatch {
    pub id: String,
    pub score: f32,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl PineconeStore {
    /// Create a new Pinecone store
    pub async fn new(api_key: String, index_name: String, endpoint: String) -> Result<Self> {
        Ok(Self {
            api_key,
            index_name,
            endpoint,
            client: reqwest::Client::new(),
        })
    }

    /// Get index stats
    pub async fn get_index_stats(&self) -> Result<String> {
        let url = format!("{}/describe_index_stats", self.endpoint);
        
        let response = self.client
            .post(&url)
            .header("Api-Key", &self.api_key)
            .send()
            .await?;

        Ok(response.text().await?)
    }

    /// Delete index
    pub async fn delete_index(&self) -> Result<()> {
        let url = format!("{}/delete_index", self.endpoint);
        
        self.client
            .delete(&url)
            .header("Api-Key", &self.api_key)
            .send()
            .await?;

        Ok(())
    }
}

#[async_trait]
impl VectorStoreBase for PineconeStore {
    /// Create collection
    async fn create_collection(&self, _name: &str, _dimension: usize) -> Result<()> {
        // Pinecone collections are created via API/console
        // This is a placeholder
        Ok(())
    }

    /// Check if collection exists
    async fn collection_exists(&self, _name: &str) -> Result<bool> {
        // Placeholder implementation
        Ok(true)
    }

    /// Upsert vectors
    async fn upsert(&self, _collection: &str, vectors: Vec<(String, Vec<f32>, VectorMetadata)>) -> Result<()> {
        let pinecone_vectors: Vec<PineconeVector> = vectors
            .into_iter()
            .map(|(id, values, metadata)| {
                let mut meta = HashMap::new();
                meta.insert("id".to_string(), serde_json::Value::String(metadata.id));
                meta.insert("user_id".to_string(), serde_json::Value::String(metadata.user_id));
                meta.insert("text".to_string(), serde_json::Value::String(metadata.text));
                meta.insert("memory_type".to_string(), serde_json::Value::String(metadata.memory_type));
                meta.insert("created_at".to_string(), serde_json::Value::String(metadata.created_at));
                meta.insert("updated_at".to_string(), serde_json::Value::String(metadata.updated_at));

                PineconeVector {
                    id,
                    values,
                    metadata: Some(meta),
                }
            })
            .collect();

        let request = UpsertRequest {
            vectors: pinecone_vectors,
        };

        let url = format!("{}/vectors/upsert", self.endpoint);
        
        self.client
            .post(&url)
            .header("Api-Key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        Ok(())
    }

    /// Search vectors
    async fn search(&self, _collection: &str, query_vector: Vec<f32>, limit: usize, _score_threshold: Option<f32>) -> Result<Vec<SearchResult>> {
        let request = QueryRequest {
            vector: query_vector,
            top_k: limit,
            include_metadata: true,
            filter: None,
        };

        let url = format!("{}/query", self.endpoint);
        
        let response = self.client
            .post(&url)
            .header("Api-Key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        let query_response: QueryResponse = response.json().await?;

        let results = query_response
            .matches
            .into_iter()
            .map(|m| {
                let metadata = m.metadata.as_ref().map(|meta| {
                    VectorMetadata {
                        id: meta.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        user_id: meta.get("user_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        agent_id: meta.get("agent_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        run_id: meta.get("run_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        text: meta.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        memory_type: meta.get("memory_type").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        created_at: meta.get("created_at").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        updated_at: meta.get("updated_at").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        custom_metadata: std::collections::HashMap::new(),
                    }
                }).unwrap_or_else(|| VectorMetadata {
                    id: m.id.clone(),
                    user_id: String::new(),
                    agent_id: None,
                    run_id: None,
                    text: String::new(),
                    memory_type: String::new(),
                    created_at: String::new(),
                    updated_at: String::new(),
                    custom_metadata: std::collections::HashMap::new(),
                });

                SearchResult {
                    id: m.id,
                    score: m.score,
                    metadata,
                }
            })
            .collect();

        Ok(results)
    }

    /// Delete vectors
    async fn delete(&self, _collection: &str, ids: Vec<String>) -> Result<()> {
        let url = format!("{}/vectors/delete", self.endpoint);
        
        let request = serde_json::json!({
            "ids": ids
        });

        self.client
            .post(&url)
            .header("Api-Key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        Ok(())
    }

    /// Delete collection
    async fn delete_collection(&self, _name: &str) -> Result<()> {
        self.delete_index().await
    }

    /// Count vectors
    async fn count(&self, _collection: &str) -> Result<usize> {
        let stats = self.get_index_stats().await?;
        // Parse stats to get count (placeholder)
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinecone_vector_creation() {
        let vector = PineconeVector {
            id: "test_id".to_string(),
            values: vec![0.1, 0.2, 0.3],
            metadata: None,
        };

        assert_eq!(vector.id, "test_id");
        assert_eq!(vector.values.len(), 3);
    }

    #[test]
    fn test_query_request_creation() {
        let request = QueryRequest {
            vector: vec![0.1, 0.2, 0.3],
            top_k: 5,
            include_metadata: true,
            filter: None,
        };

        assert_eq!(request.top_k, 5);
        assert!(request.include_metadata);
    }

    #[tokio::test]
    async fn test_pinecone_store_creation() {
        let store = PineconeStore::new(
            "test-key".to_string(),
            "test-index".to_string(),
            "https://api.pinecone.io".to_string(),
        )
        .await;

        assert!(store.is_ok());
    }

    #[test]
    fn test_upsert_request_serialization() {
        let mut metadata = HashMap::new();
        metadata.insert("content".to_string(), serde_json::Value::String("test".to_string()));

        let vector = PineconeVector {
            id: "id1".to_string(),
            values: vec![0.1, 0.2],
            metadata: Some(metadata),
        };

        let request = UpsertRequest {
            vectors: vec![vector],
        };

        let json = serde_json::to_string(&request);
        assert!(json.is_ok());
    }
}
