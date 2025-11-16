//! In-memory vector store implementation

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::{Result, Error};
use super::{VectorStoreBase, VectorMetadata, SearchResult};

/// Vector entry stored in memory
#[derive(Clone)]
struct VectorEntry {
    vector: Vec<f32>,
    metadata: VectorMetadata,
}

/// In-memory vector store implementation
pub struct InMemoryStore {
    collections: Arc<RwLock<HashMap<String, HashMap<String, VectorEntry>>>>,
}

impl InMemoryStore {
    /// Create a new in-memory store
    pub fn new() -> Self {
        Self {
            collections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl VectorStoreBase for InMemoryStore {
    async fn create_collection(
        &self,
        collection_name: &str,
        _vector_size: usize,
    ) -> Result<()> {
        let mut collections = self.collections.write().await;
        collections.entry(collection_name.to_string())
            .or_insert_with(HashMap::new);
        Ok(())
    }

    async fn collection_exists(&self, collection_name: &str) -> Result<bool> {
        let collections = self.collections.read().await;
        Ok(collections.contains_key(collection_name))
    }

    async fn upsert(
        &self,
        collection_name: &str,
        vectors: Vec<(String, Vec<f32>, VectorMetadata)>,
    ) -> Result<()> {
        let mut collections = self.collections.write().await;
        let collection = collections
            .entry(collection_name.to_string())
            .or_insert_with(HashMap::new);

        for (id, vector, metadata) in vectors {
            collection.insert(id, VectorEntry { vector, metadata });
        }
        Ok(())
    }

    async fn search(
        &self,
        collection_name: &str,
        query_vector: Vec<f32>,
        limit: usize,
        score_threshold: Option<f32>,
    ) -> Result<Vec<SearchResult>> {
        let collections = self.collections.read().await;
        let collection = collections
            .get(collection_name)
            .ok_or_else(|| Error::vector_store(format!("Collection not found: {}", collection_name)))?;

        // Compute cosine similarity for all vectors
        let mut results: Vec<_> = collection
            .iter()
            .filter_map(|(id, entry)| {
                let score = cosine_similarity(&query_vector, &entry.vector);
                if let Some(threshold) = score_threshold {
                    if score < threshold {
                        return None;
                    }
                }
                Some((id.clone(), score, entry.metadata.clone()))
            })
            .collect();

        // Sort by score descending
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top limit results
        let search_results = results
            .into_iter()
            .take(limit)
            .map(|(id, score, metadata)| SearchResult {
                id,
                score,
                metadata,
            })
            .collect();

        Ok(search_results)
    }

    async fn delete(
        &self,
        collection_name: &str,
        ids: Vec<String>,
    ) -> Result<()> {
        let mut collections = self.collections.write().await;
        if let Some(collection) = collections.get_mut(collection_name) {
            for id in ids {
                collection.remove(&id);
            }
        }
        Ok(())
    }

    async fn delete_collection(&self, collection_name: &str) -> Result<()> {
        let mut collections = self.collections.write().await;
        collections.remove(collection_name);
        Ok(())
    }

    async fn count(&self, collection_name: &str) -> Result<usize> {
        let collections = self.collections.read().await;
        Ok(collections
            .get(collection_name)
            .map(|c| c.len())
            .unwrap_or(0))
    }
}

/// Compute cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_collection() {
        let store = InMemoryStore::new();
        let result = store.create_collection("test_collection", 384).await;
        assert!(result.is_ok());
        assert!(store.collection_exists("test_collection").await.unwrap());
    }

    #[tokio::test]
    async fn test_upsert_and_search() {
        let store = InMemoryStore::new();
        store.create_collection("test", 3).await.unwrap();

        let metadata = VectorMetadata {
            id: "1".to_string(),
            user_id: "user1".to_string(),
            agent_id: None,
            run_id: None,
            text: "hello world".to_string(),
            memory_type: "fact".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
            custom_metadata: Default::default(),
        };

        store
            .upsert(
                "test",
                vec![(
                    "1".to_string(),
                    vec![1.0, 0.0, 0.0],
                    metadata,
                )],
            )
            .await
            .unwrap();

        let results = store
            .search("test", vec![1.0, 0.0, 0.0], 10, None)
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].score > 0.99);
    }

    #[tokio::test]
    async fn test_cosine_similarity() {
        assert!((cosine_similarity(&[1.0, 0.0], &[1.0, 0.0]) - 1.0).abs() < 0.001);
        assert!((cosine_similarity(&[1.0, 0.0], &[0.0, 1.0]) - 0.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_delete() {
        let store = InMemoryStore::new();
        store.create_collection("test", 3).await.unwrap();

        let metadata = VectorMetadata {
            id: "1".to_string(),
            user_id: "user1".to_string(),
            agent_id: None,
            run_id: None,
            text: "test".to_string(),
            memory_type: "fact".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
            custom_metadata: Default::default(),
        };

        store
            .upsert("test", vec![("1".to_string(), vec![1.0, 0.0, 0.0], metadata)])
            .await
            .unwrap();

        assert_eq!(store.count("test").await.unwrap(), 1);
        store.delete("test", vec!["1".to_string()]).await.unwrap();
        assert_eq!(store.count("test").await.unwrap(), 0);
    }
}
