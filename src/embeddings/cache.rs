//! Embedding cache implementation

use std::collections::HashMap;
use sha2::{Sha256, Digest};

/// LRU cache for embeddings
pub struct EmbeddingCache {
    /// Cache storage
    cache: HashMap<String, Vec<f32>>,
    /// Access order for LRU
    access_order: Vec<String>,
    /// Maximum cache size
    max_size: usize,
}

impl EmbeddingCache {
    /// Create a new embedding cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            access_order: Vec::new(),
            max_size,
        }
    }

    /// Compute hash of text
    fn compute_hash(text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Get embedding from cache
    pub fn get(&mut self, text: &str) -> Option<Vec<f32>> {
        let hash = Self::compute_hash(text);

        if let Some(embedding) = self.cache.get(&hash) {
            // Update access order
            self.access_order.retain(|h| h != &hash);
            self.access_order.push(hash.clone());

            return Some(embedding.clone());
        }

        None
    }

    /// Put embedding in cache
    pub fn put(&mut self, text: &str, embedding: Vec<f32>) {
        let hash = Self::compute_hash(text);

        // If cache is full, remove least recently used
        if self.cache.len() >= self.max_size && !self.cache.contains_key(&hash) {
            if let Some(lru_hash) = self.access_order.first() {
                let lru_hash = lru_hash.clone();
                self.cache.remove(&lru_hash);
                self.access_order.remove(0);
            }
        }

        // Update access order
        self.access_order.retain(|h| h != &hash);
        self.access_order.push(hash.clone());

        self.cache.insert(hash, embedding);
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }

    /// Get cache hit rate
    pub fn hit_rate(&self) -> f32 {
        if self.access_order.is_empty() {
            return 0.0;
        }
        self.cache.len() as f32 / self.max_size as f32
    }

    /// Check if text is in cache
    pub fn contains(&self, text: &str) -> bool {
        let hash = Self::compute_hash(text);
        self.cache.contains_key(&hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_put_get() {
        let mut cache = EmbeddingCache::new(10);
        let embedding = vec![0.1, 0.2, 0.3];

        cache.put("test", embedding.clone());
        assert_eq!(cache.get("test"), Some(embedding));
    }

    #[test]
    fn test_cache_lru_eviction() {
        let mut cache = EmbeddingCache::new(2);

        cache.put("text1", vec![0.1]);
        cache.put("text2", vec![0.2]);
        cache.put("text3", vec![0.3]);

        // text1 should be evicted
        assert!(cache.get("text1").is_none());
        assert!(cache.get("text2").is_some());
        assert!(cache.get("text3").is_some());
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = EmbeddingCache::new(10);
        cache.put("text1", vec![0.1]);
        cache.put("text2", vec![0.2]);

        assert_eq!(cache.size(), 2);

        cache.clear();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_cache_contains() {
        let mut cache = EmbeddingCache::new(10);
        cache.put("text1", vec![0.1]);

        assert!(cache.contains("text1"));
        assert!(!cache.contains("text2"));
    }

    #[test]
    fn test_cache_hit_rate() {
        let mut cache = EmbeddingCache::new(10);
        cache.put("text1", vec![0.1]);

        let hit_rate = cache.hit_rate();
        assert!(hit_rate > 0.0 && hit_rate <= 1.0);
    }
}
