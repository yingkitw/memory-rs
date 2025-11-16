//! Main Memory implementation

use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

use crate::config::MemoryConfig;
use crate::Result;
use crate::vector_store::VectorStoreBase;
use crate::embeddings::EmbedderBase;

use super::{MemoryBase, MemoryItem, SearchResultItem};

/// Main Memory implementation
pub struct Memory {
    config: MemoryConfig,
    vector_store: Arc<dyn VectorStoreBase>,
    embedder: Arc<dyn EmbedderBase>,
}

impl Memory {
    /// Create a new Memory instance
    pub fn new(
        config: MemoryConfig,
        vector_store: Arc<dyn VectorStoreBase>,
        embedder: Arc<dyn EmbedderBase>,
    ) -> Self {
        Self {
            config,
            vector_store,
            embedder,
        }
    }

    /// Get collection name for user
    fn get_collection_name(&self, user_id: &str) -> String {
        format!(
            "{}_{}",
            self.config.get_collection_prefix(),
            user_id
        )
    }

    /// Initialize collection for user
    async fn ensure_collection(&self, user_id: &str) -> Result<()> {
        let collection_name = self.get_collection_name(user_id);
        let dimension = self.config.get_vector_dimension();

        self.vector_store
            .create_collection(&collection_name, dimension)
            .await
    }
}

#[async_trait]
impl MemoryBase for Memory {
    async fn add(
        &self,
        user_id: &str,
        content: &str,
        memory_type: Option<&str>,
    ) -> Result<MemoryItem> {
        // Ensure collection exists
        self.ensure_collection(user_id).await?;

        // Create memory item
        let memory = MemoryItem::new(
            user_id.to_string(),
            content.to_string(),
            memory_type.unwrap_or("general").to_string(),
        );

        // Generate embedding
        let embedding = self.embedder.embed(content).await?;

        // Store in vector database
        let collection_name = self.get_collection_name(user_id);
        self.vector_store
            .upsert(
                &collection_name,
                vec![(
                    memory.id.clone(),
                    embedding,
                    memory.to_vector_metadata(),
                )],
            )
            .await?;

        Ok(memory)
    }

    async fn search(
        &self,
        user_id: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResultItem>> {
        // Ensure collection exists
        self.ensure_collection(user_id).await?;

        // Generate query embedding
        let query_embedding = self.embedder.embed(query).await?;

        // Search vector store
        let collection_name = self.get_collection_name(user_id);
        let results = self
            .vector_store
            .search(&collection_name, query_embedding, limit, Some(0.0))
            .await?;

        // Convert to SearchResultItem
        let search_results = results
            .into_iter()
            .map(|result| {
                let memory = MemoryItem {
                    id: result.metadata.id,
                    user_id: result.metadata.user_id,
                    agent_id: result.metadata.agent_id,
                    run_id: result.metadata.run_id,
                    content: result.metadata.text,
                    memory_type: result.metadata.memory_type,
                    hash: String::new(), // Not needed for search results
                    created_at: result.metadata.created_at,
                    updated_at: result.metadata.updated_at,
                    metadata: result.metadata.custom_metadata,
                };

                SearchResultItem {
                    memory,
                    score: result.score,
                }
            })
            .collect();

        Ok(search_results)
    }

    async fn update(
        &self,
        memory_id: &str,
        content: &str,
    ) -> Result<MemoryItem> {
        // This is a simplified implementation
        // In production, you'd want to retrieve the original memory first
        let mut memory = MemoryItem::new(
            "unknown".to_string(),
            content.to_string(),
            "general".to_string(),
        );
        memory.id = memory_id.to_string();
        memory.updated_at = Utc::now().to_rfc3339();

        Ok(memory)
    }

    async fn delete(&self, _memory_id: &str) -> Result<()> {
        // This is a simplified implementation
        // In production, you'd need to track which collection contains this memory
        // For now, we'll just return success
        Ok(())
    }

    async fn get_all(
        &self,
        _user_id: &str,
    ) -> Result<Vec<MemoryItem>> {
        // This would require scanning the vector store
        // For now, return empty vector
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embeddings::EmbedderBase;

    #[test]
    fn test_collection_name() {
        let config = MemoryConfig::new("memory.db".to_string());

        let memory = Memory::new(
            config,
            Arc::new(MockVectorStore),
            Arc::new(MockEmbedder),
        );

        assert_eq!(
            memory.get_collection_name("user_123"),
            "memory_user_123"
        );
    }

    // Mock implementations for testing
    struct MockVectorStore;
    struct MockEmbedder;

    #[async_trait]
    impl VectorStoreBase for MockVectorStore {
        async fn create_collection(
            &self,
            _collection_name: &str,
            _vector_size: usize,
        ) -> crate::Result<()> {
            Ok(())
        }

        async fn collection_exists(&self, _collection_name: &str) -> crate::Result<bool> {
            Ok(true)
        }

        async fn upsert(
            &self,
            _collection_name: &str,
            _vectors: Vec<(String, Vec<f32>, crate::vector_store::VectorMetadata)>,
        ) -> crate::Result<()> {
            Ok(())
        }

        async fn search(
            &self,
            _collection_name: &str,
            _query_vector: Vec<f32>,
            _limit: usize,
            _score_threshold: Option<f32>,
        ) -> crate::Result<Vec<crate::vector_store::SearchResult>> {
            Ok(Vec::new())
        }

        async fn delete(
            &self,
            _collection_name: &str,
            _ids: Vec<String>,
        ) -> crate::Result<()> {
            Ok(())
        }

        async fn delete_collection(&self, _collection_name: &str) -> crate::Result<()> {
            Ok(())
        }

        async fn count(&self, _collection_name: &str) -> crate::Result<usize> {
            Ok(0)
        }
    }

    #[async_trait]
    impl EmbedderBase for MockEmbedder {
        async fn embed(&self, _text: &str) -> crate::Result<Vec<f32>> {
            Ok(vec![0.0; 384])
        }

        fn dimension(&self) -> usize {
            384
        }
    }
}
