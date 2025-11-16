//! Embeddings abstraction and implementations

use async_trait::async_trait;
use crate::Result;

pub mod default;
pub mod cache;

pub use default::DefaultEmbedder;
pub use cache::EmbeddingCache;

/// Base trait for embedding implementations
#[async_trait]
pub trait EmbedderBase: Send + Sync {
    /// Generate embedding for text
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;

    /// Generate embeddings for multiple texts
    async fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        let mut embeddings = Vec::new();
        for text in texts {
            embeddings.push(self.embed(text).await?);
        }
        Ok(embeddings)
    }

    /// Get embedding dimension
    fn dimension(&self) -> usize;
}
