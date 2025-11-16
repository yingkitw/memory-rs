//! Weaviate vector store implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Result;
use super::{VectorStoreBase, VectorMetadata, SearchResult};

/// Weaviate vector store
pub struct WeaviateStore {
    /// API endpoint
    endpoint: String,
    /// HTTP client
    client: reqwest::Client,
    /// API key (optional)
    api_key: Option<String>,
}

/// Weaviate object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaviateObject {
    /// Class name
    pub class: String,
    /// Vector
    pub vector: Vec<f32>,
    /// Properties
    pub properties: HashMap<String, serde_json::Value>,
}

/// Weaviate batch request
#[derive(Debug, Serialize)]
pub struct BatchRequest {
    pub objects: Vec<WeaviateObject>,
}

/// Weaviate search result
#[derive(Debug, Deserialize)]
pub struct WeaviateSearchResult {
    pub data: SearchData,
}

/// Search data
#[derive(Debug, Deserialize)]
pub struct SearchData {
    pub get: Option<Vec<WeaviateResultObject>>,
}

/// Result object
#[derive(Debug, Deserialize)]
pub struct WeaviateResultObject {
    #[serde(rename = "_additional")]
    pub additional: Option<AdditionalData>,
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Additional metadata
#[derive(Debug, Deserialize)]
pub struct AdditionalData {
    pub distance: Option<f32>,
    pub certainty: Option<f32>,
}

impl WeaviateStore {
    /// Create a new Weaviate store
    pub async fn new(endpoint: String) -> Result<Self> {
        Ok(Self {
            endpoint,
            client: reqwest::Client::new(),
            api_key: None,
        })
    }

    /// Create with API key
    pub async fn with_api_key(endpoint: String, api_key: String) -> Result<Self> {
        Ok(Self {
            endpoint,
            client: reqwest::Client::new(),
            api_key: Some(api_key),
        })
    }

    /// Check if schema exists
    pub async fn schema_exists(&self, class_name: &str) -> Result<bool> {
        let url = format!("{}/v1/schema/{}", self.endpoint, class_name);
        
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }

    /// Create schema
    pub async fn create_schema(&self, class_name: &str) -> Result<()> {
        let url = format!("{}/v1/schema", self.endpoint);
        
        let schema = serde_json::json!({
            "class": class_name,
            "vectorizer": "none",
            "properties": [
                {
                    "name": "content",
                    "dataType": ["text"]
                },
                {
                    "name": "user_id",
                    "dataType": ["text"]
                },
                {
                    "name": "memory_type",
                    "dataType": ["text"]
                }
            ]
        });

        self.client
            .post(&url)
            .json(&schema)
            .send()
            .await?;

        Ok(())
    }

    /// Delete schema
    pub async fn delete_schema(&self, class_name: &str) -> Result<()> {
        let url = format!("{}/v1/schema/{}", self.endpoint, class_name);
        
        self.client.delete(&url).send().await?;
        Ok(())
    }
}

#[async_trait]
impl VectorStoreBase for WeaviateStore {
    /// Create collection
    async fn create_collection(&self, name: &str, _dimension: usize) -> Result<()> {
        self.create_schema(name).await
    }

    /// Check if collection exists
    async fn collection_exists(&self, name: &str) -> Result<bool> {
        self.schema_exists(name).await
    }

    /// Upsert vectors
    async fn upsert(&self, collection: &str, vectors: Vec<(String, Vec<f32>, VectorMetadata)>) -> Result<()> {
        let objects: Vec<WeaviateObject> = vectors
            .into_iter()
            .map(|(id, vector, metadata)| {
                let mut properties = HashMap::new();
                properties.insert("id".to_string(), serde_json::Value::String(metadata.id));
                properties.insert("user_id".to_string(), serde_json::Value::String(metadata.user_id));
                properties.insert("text".to_string(), serde_json::Value::String(metadata.text));
                properties.insert("memory_type".to_string(), serde_json::Value::String(metadata.memory_type));
                properties.insert("created_at".to_string(), serde_json::Value::String(metadata.created_at));
                properties.insert("updated_at".to_string(), serde_json::Value::String(metadata.updated_at));

                WeaviateObject {
                    class: collection.to_string(),
                    vector,
                    properties,
                }
            })
            .collect();

        let url = format!("{}/v1/batch/objects", self.endpoint);
        
        let request = BatchRequest { objects };

        self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        Ok(())
    }

    /// Search vectors
    async fn search(&self, collection: &str, query_vector: Vec<f32>, limit: usize, _score_threshold: Option<f32>) -> Result<Vec<SearchResult>> {
        let query = format!(
            r#"{{
                Get {{
                    {} (nearVector: {{vector: {:?}}}, limit: {}) {{
                        id
                        user_id
                        text
                        memory_type
                        created_at
                        updated_at
                        _additional {{
                            distance
                            certainty
                        }}
                    }}
                }}
            }}"#,
            collection, query_vector, limit
        );

        let url = format!("{}/v1/graphql", self.endpoint);
        
        let response = self.client
            .post(&url)
            .json(&serde_json::json!({"query": query}))
            .send()
            .await?;

        let search_result: WeaviateSearchResult = response.json().await?;

        let results = search_result
            .data
            .get
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(idx, obj)| {
                let metadata = VectorMetadata {
                    id: obj.properties
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    user_id: obj.properties
                        .get("user_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    agent_id: None,
                    run_id: None,
                    text: obj.properties
                        .get("text")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    memory_type: obj.properties
                        .get("memory_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    created_at: obj.properties
                        .get("created_at")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    updated_at: obj.properties
                        .get("updated_at")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    custom_metadata: std::collections::HashMap::new(),
                };

                let score = obj.additional
                    .as_ref()
                    .and_then(|a| a.certainty)
                    .unwrap_or(1.0 - (obj.additional.as_ref().and_then(|a| a.distance).unwrap_or(0.0)));

                SearchResult {
                    id: format!("{}_{}", collection, idx),
                    score,
                    metadata,
                }
            })
            .collect();

        Ok(results)
    }

    /// Delete vectors
    async fn delete(&self, collection: &str, ids: Vec<String>) -> Result<()> {
        let url = format!("{}/v1/batch/objects", self.endpoint);
        
        for id in ids {
            self.client
                .delete(&format!("{}/{}", url, id))
                .send()
                .await?;
        }

        Ok(())
    }

    /// Delete collection
    async fn delete_collection(&self, name: &str) -> Result<()> {
        self.delete_schema(name).await
    }

    /// Count vectors
    async fn count(&self, collection: &str) -> Result<usize> {
        let query = format!(
            r#"{{
                Aggregate {{
                    {} {{
                        meta {{
                            count
                        }}
                    }}
                }}
            }}"#,
            collection
        );

        let url = format!("{}/v1/graphql", self.endpoint);
        
        let response = self.client
            .post(&url)
            .json(&serde_json::json!({"query": query}))
            .send()
            .await?;

        // Parse response to extract count (placeholder)
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weaviate_object_creation() {
        let mut properties = HashMap::new();
        properties.insert("content".to_string(), serde_json::Value::String("test".to_string()));

        let obj = WeaviateObject {
            class: "Memory".to_string(),
            vector: vec![0.1, 0.2, 0.3],
            properties,
        };

        assert_eq!(obj.class, "Memory");
        assert_eq!(obj.vector.len(), 3);
    }

    #[tokio::test]
    async fn test_weaviate_store_creation() {
        let store = WeaviateStore::new("http://localhost:8080".to_string()).await;
        assert!(store.is_ok());
    }

    #[tokio::test]
    async fn test_weaviate_with_api_key() {
        let store = WeaviateStore::with_api_key(
            "http://localhost:8080".to_string(),
            "test-key".to_string(),
        )
        .await;

        assert!(store.is_ok());
        let store = store.unwrap();
        assert_eq!(store.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_batch_request_serialization() {
        let mut properties = HashMap::new();
        properties.insert("content".to_string(), serde_json::Value::String("test".to_string()));

        let obj = WeaviateObject {
            class: "Memory".to_string(),
            vector: vec![0.1, 0.2],
            properties,
        };

        let request = BatchRequest {
            objects: vec![obj],
        };

        let json = serde_json::to_string(&request);
        assert!(json.is_ok());
    }
}
