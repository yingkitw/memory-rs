//! Memory management core module

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::Result;
use crate::vector_store::VectorMetadata;

pub mod main;
pub mod dedup;
pub mod batch;

pub use main::Memory;

/// Memory item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    /// Unique identifier
    pub id: String,

    /// User ID
    pub user_id: String,

    /// Agent ID (optional)
    pub agent_id: Option<String>,

    /// Run ID (optional)
    pub run_id: Option<String>,

    /// Memory content
    pub content: String,

    /// Memory type (fact, insight, preference, etc.)
    pub memory_type: String,

    /// Hash of content for deduplication
    pub hash: String,

    /// Creation timestamp
    pub created_at: String,

    /// Last updated timestamp
    pub updated_at: String,

    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl MemoryItem {
    /// Create a new memory item
    pub fn new(
        user_id: String,
        content: String,
        memory_type: String,
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let hash = Self::compute_hash(&content);
        let now = Utc::now().to_rfc3339();

        Self {
            id,
            user_id,
            agent_id: None,
            run_id: None,
            content,
            memory_type,
            hash,
            created_at: now.clone(),
            updated_at: now,
            metadata: HashMap::new(),
        }
    }

    /// Compute hash of content
    fn compute_hash(content: &str) -> String {
        crate::utils::compute_hash(content)
    }

    /// Set agent ID
    pub fn with_agent_id(mut self, agent_id: String) -> Self {
        self.agent_id = Some(agent_id);
        self
    }

    /// Set run ID
    pub fn with_run_id(mut self, run_id: String) -> Self {
        self.run_id = Some(run_id);
        self
    }

    /// Add custom metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Convert to vector metadata
    pub fn to_vector_metadata(&self) -> VectorMetadata {
        VectorMetadata {
            id: self.id.clone(),
            user_id: self.user_id.clone(),
            agent_id: self.agent_id.clone(),
            run_id: self.run_id.clone(),
            text: self.content.clone(),
            memory_type: self.memory_type.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            custom_metadata: self.metadata.clone(),
        }
    }
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItem {
    /// Memory item
    pub memory: MemoryItem,

    /// Relevance score (0-1)
    pub score: f32,
}

/// Base trait for memory implementations
#[async_trait]
pub trait MemoryBase: Send + Sync {
    /// Add a new memory
    async fn add(
        &self,
        user_id: &str,
        content: &str,
        memory_type: Option<&str>,
    ) -> Result<MemoryItem>;

    /// Search memories
    async fn search(
        &self,
        user_id: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResultItem>>;

    /// Update a memory
    async fn update(
        &self,
        memory_id: &str,
        content: &str,
    ) -> Result<MemoryItem>;

    /// Delete a memory
    async fn delete(&self, memory_id: &str) -> Result<()>;

    /// Get all memories for a user
    async fn get_all(
        &self,
        user_id: &str,
    ) -> Result<Vec<MemoryItem>>;
}
