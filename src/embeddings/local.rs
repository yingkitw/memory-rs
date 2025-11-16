//! Local embedder implementation using simple hashing

use async_trait::async_trait;
use sha2::{Sha256, Digest};
use crate::error::Result;
use super::EmbedderBase;

/// Local embedder using SHA256-based hashing
pub struct LocalEmbedder {
    dimension: usize,
}

impl LocalEmbedder {
    /// Create a new local embedder
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    /// Create with default dimension (384)
    pub fn with_defaults() -> Self {
        Self::new(384)
    }

    /// Generate embedding from text using hash-based approach
    fn text_to_embedding(text: &str, dimension: usize) -> Vec<f32> {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        let hash = hasher.finalize();

        // Convert hash bytes to floats in range [0, 1]
        let mut embedding = Vec::with_capacity(dimension);
        for i in 0..dimension {
            let byte_idx = (i * hash.len()) / dimension;
            let byte_val = hash[byte_idx] as f32 / 255.0;
            // Normalize to [-1, 1] range for better similarity computation
            embedding.push((byte_val - 0.5) * 2.0);
        }
        embedding
    }
}

#[async_trait]
impl EmbedderBase for LocalEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        Ok(Self::text_to_embedding(text, self.dimension))
    }

    async fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        Ok(texts
            .into_iter()
            .map(|text| Self::text_to_embedding(text, self.dimension))
            .collect())
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_embedder_creation() {
        let embedder = LocalEmbedder::with_defaults();
        assert_eq!(embedder.dimension(), 384);
    }

    #[tokio::test]
    async fn test_embed() {
        let embedder = LocalEmbedder::new(128);
        let embedding = embedder.embed("hello world").await.unwrap();
        assert_eq!(embedding.len(), 128);
        assert!(embedding.iter().all(|v| *v >= -1.0 && *v <= 1.0));
    }

    #[tokio::test]
    async fn test_embed_batch() {
        let embedder = LocalEmbedder::new(64);
        let embeddings = embedder
            .embed_batch(vec!["hello", "world", "test"])
            .await
            .unwrap();
        assert_eq!(embeddings.len(), 3);
        assert!(embeddings.iter().all(|e| e.len() == 64));
    }

    #[tokio::test]
    async fn test_deterministic() {
        let embedder = LocalEmbedder::new(256);
        let emb1 = embedder.embed("same text").await.unwrap();
        let emb2 = embedder.embed("same text").await.unwrap();
        assert_eq!(emb1, emb2);
    }

    #[tokio::test]
    async fn test_different_text() {
        let embedder = LocalEmbedder::new(256);
        let emb1 = embedder.embed("text one").await.unwrap();
        let emb2 = embedder.embed("text two").await.unwrap();
        assert_ne!(emb1, emb2);
    }
}
