//! Vector store abstraction and implementations

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::Result;

pub mod qdrant;
pub mod backends;

pub use qdrant::InMemoryStore;
pub use backends::{BackendType, BackendConfig};

/// Metadata associated with a vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorMetadata {
    /// Unique identifier for the vector
    pub id: String,

    /// User ID associated with this memory
    pub user_id: String,

    /// Agent ID (optional)
    pub agent_id: Option<String>,

    /// Run ID (optional)
    pub run_id: Option<String>,

    /// Original text content
    pub text: String,

    /// Memory type (fact, insight, etc.)
    pub memory_type: String,

    /// Creation timestamp
    pub created_at: String,

    /// Last updated timestamp
    pub updated_at: String,

    /// Additional custom metadata
    pub custom_metadata: std::collections::HashMap<String, String>,
}

/// Vector search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Vector ID
    pub id: String,

    /// Similarity score (0-1)
    pub score: f32,

    /// Associated metadata
    pub metadata: VectorMetadata,
}

/// Base trait for vector store implementations
#[async_trait]
pub trait VectorStoreBase: Send + Sync {
    /// Create or get a collection
    async fn create_collection(
        &self,
        collection_name: &str,
        vector_size: usize,
    ) -> Result<()>;

    /// Check if collection exists
    async fn collection_exists(&self, collection_name: &str) -> Result<bool>;

    /// Upsert vectors (insert or update)
    async fn upsert(
        &self,
        collection_name: &str,
        vectors: Vec<(String, Vec<f32>, VectorMetadata)>,
    ) -> Result<()>;

    /// Search for similar vectors
    async fn search(
        &self,
        collection_name: &str,
        query_vector: Vec<f32>,
        limit: usize,
        score_threshold: Option<f32>,
    ) -> Result<Vec<SearchResult>>;

    /// Delete vectors by IDs
    async fn delete(
        &self,
        collection_name: &str,
        ids: Vec<String>,
    ) -> Result<()>;

    /// Delete collection
    async fn delete_collection(&self, collection_name: &str) -> Result<()>;

    /// Get vector count in collection
    async fn count(&self, collection_name: &str) -> Result<usize>;
}
