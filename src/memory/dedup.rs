//! Memory deduplication module

use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// Deduplication strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeduplicationStrategy {
    /// Exact hash matching
    Exact,
    /// Similarity-based (requires embeddings)
    Similarity,
    /// No deduplication
    None,
}

/// Deduplication engine
pub struct Deduplicator {
    strategy: DeduplicationStrategy,
    cache: HashMap<String, String>, // hash -> id mapping
    similarity_threshold: f32,
}

impl Deduplicator {
    /// Create a new deduplicator
    pub fn new(strategy: DeduplicationStrategy) -> Self {
        Self {
            strategy,
            cache: HashMap::new(),
            similarity_threshold: 0.95,
        }
    }

    /// Create with similarity threshold
    pub fn with_threshold(strategy: DeduplicationStrategy, threshold: f32) -> Self {
        Self {
            strategy,
            cache: HashMap::new(),
            similarity_threshold: threshold,
        }
    }

    /// Compute hash of content
    pub fn compute_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Check if content is duplicate
    pub fn is_duplicate(&self, content: &str) -> bool {
        if self.strategy == DeduplicationStrategy::None {
            return false;
        }

        let hash = Self::compute_hash(content);
        self.cache.contains_key(&hash)
    }

    /// Register content
    pub fn register(&mut self, content: &str, id: String) {
        if self.strategy == DeduplicationStrategy::None {
            return;
        }

        let hash = Self::compute_hash(content);
        self.cache.insert(hash, id);
    }

    /// Get duplicate ID if exists
    pub fn get_duplicate(&self, content: &str) -> Option<String> {
        if self.strategy == DeduplicationStrategy::None {
            return None;
        }

        let hash = Self::compute_hash(content);
        self.cache.get(&hash).cloned()
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Compute similarity between two vectors (cosine similarity)
    pub fn compute_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
        if vec1.len() != vec2.len() || vec1.is_empty() {
            return 0.0;
        }

        let dot_product: f32 = vec1.iter().zip(vec2).map(|(a, b)| a * b).sum();
        let norm1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm1 == 0.0 || norm2 == 0.0 {
            return 0.0;
        }

        dot_product / (norm1 * norm2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_deduplication() {
        let mut dedup = Deduplicator::new(DeduplicationStrategy::Exact);

        let content = "I like coffee";
        dedup.register(content, "id_1".to_string());

        assert!(dedup.is_duplicate(content));
        assert_eq!(dedup.get_duplicate(content), Some("id_1".to_string()));
    }

    #[test]
    fn test_no_deduplication() {
        let dedup = Deduplicator::new(DeduplicationStrategy::None);

        let content = "I like coffee";
        assert!(!dedup.is_duplicate(content));
    }

    #[test]
    fn test_hash_computation() {
        let content = "I like coffee";
        let hash1 = Deduplicator::compute_hash(content);
        let hash2 = Deduplicator::compute_hash(content);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_similarity() {
        let vec1 = vec![1.0, 0.0, 0.0];
        let vec2 = vec![1.0, 0.0, 0.0];
        let vec3 = vec![0.0, 1.0, 0.0];

        assert_eq!(Deduplicator::compute_similarity(&vec1, &vec2), 1.0);
        assert_eq!(Deduplicator::compute_similarity(&vec1, &vec3), 0.0);
    }

    #[test]
    fn test_cache_size() {
        let mut dedup = Deduplicator::new(DeduplicationStrategy::Exact);

        dedup.register("content1", "id_1".to_string());
        dedup.register("content2", "id_2".to_string());

        assert_eq!(dedup.cache_size(), 2);

        dedup.clear();
        assert_eq!(dedup.cache_size(), 0);
    }
}
