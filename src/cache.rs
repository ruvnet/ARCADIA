//! High-performance caching module for ARCADIA
//!
//! Provides efficient caching strategies for vector embeddings, AI decisions,
//! and frequently accessed game data to minimize API calls and computation.

use moka::future::Cache;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;

/// Cache configuration with performance tuning parameters
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries in the cache
    pub max_capacity: u64,
    /// Time to live for cache entries
    pub ttl: Duration,
    /// Time to idle before eviction
    pub tti: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: 10_000,
            ttl: Duration::from_secs(3600), // 1 hour
            tti: Duration::from_secs(300),  // 5 minutes
        }
    }
}

/// High-performance cache manager with LRU eviction and TTL support
pub struct CacheManager<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    cache: Cache<K, V>,
    config: CacheConfig,
}

impl<K, V> CacheManager<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Create a new cache manager with the given configuration
    pub fn new(config: CacheConfig) -> Self {
        let cache = Cache::builder()
            .max_capacity(config.max_capacity)
            .time_to_live(config.ttl)
            .time_to_idle(config.tti)
            .build();

        Self { cache, config }
    }

    /// Get a value from the cache
    pub async fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key).await
    }

    /// Insert a value into the cache
    pub async fn insert(&self, key: K, value: V) {
        self.cache.insert(key, value).await;
    }

    /// Get or insert a value using the provided async function
    pub async fn get_or_insert_with<F, Fut>(&self, key: K, f: F) -> V
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = V>,
    {
        self.cache
            .get_or_insert_with(key, async move { f().await })
            .await
    }

    /// Invalidate a cache entry
    pub async fn invalidate(&self, key: &K) {
        self.cache.invalidate(key).await;
    }

    /// Clear all cache entries
    pub async fn clear(&self) {
        self.cache.invalidate_all();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let entry_count = self.cache.entry_count();
        let weighted_size = self.cache.weighted_size();

        CacheStats {
            entry_count,
            weighted_size,
            max_capacity: self.config.max_capacity,
        }
    }
}

/// Cache statistics for monitoring
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of entries in the cache
    pub entry_count: u64,
    /// Total weighted size of cache
    pub weighted_size: u64,
    /// Maximum capacity
    pub max_capacity: u64,
}

impl CacheStats {
    /// Calculate cache utilization percentage
    pub fn utilization(&self) -> f64 {
        (self.entry_count as f64 / self.max_capacity as f64) * 100.0
    }

    /// Check if cache is near capacity
    pub fn is_near_capacity(&self, threshold: f64) -> bool {
        self.utilization() >= threshold
    }
}

/// Specialized cache for vector embeddings
pub type EmbeddingCache = CacheManager<String, Vec<f32>>;

/// Specialized cache for AI decision results
pub type AIDecisionCache = CacheManager<String, serde_json::Value>;

/// Create a default embedding cache
pub fn create_embedding_cache() -> EmbeddingCache {
    CacheManager::new(CacheConfig {
        max_capacity: 5_000,
        ttl: Duration::from_secs(7200), // 2 hours for embeddings
        tti: Duration::from_secs(1800), // 30 minutes idle
    })
}

/// Create a default AI decision cache
pub fn create_ai_decision_cache() -> AIDecisionCache {
    CacheManager::new(CacheConfig {
        max_capacity: 10_000,
        ttl: Duration::from_secs(300),  // 5 minutes for AI decisions
        tti: Duration::from_secs(60),   // 1 minute idle
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache = CacheManager::new(CacheConfig::default());

        cache.insert("key1".to_string(), "value1".to_string()).await;
        let value = cache.get(&"key1".to_string()).await;

        assert_eq!(value, Some("value1".to_string()));
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        let cache = CacheManager::new(CacheConfig::default());

        cache.insert("key1".to_string(), "value1".to_string()).await;
        cache.invalidate(&"key1".to_string()).await;
        let value = cache.get(&"key1".to_string()).await;

        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_get_or_insert_with() {
        let cache = CacheManager::new(CacheConfig::default());

        let value = cache
            .get_or_insert_with("key1".to_string(), || async { "computed_value".to_string() })
            .await;

        assert_eq!(value, "computed_value");

        // Should return cached value without recomputing
        let cached_value = cache.get(&"key1".to_string()).await;
        assert_eq!(cached_value, Some("computed_value".to_string()));
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = CacheManager::new(CacheConfig {
            max_capacity: 100,
            ..Default::default()
        });

        cache.insert("key1".to_string(), "value1".to_string()).await;
        cache.insert("key2".to_string(), "value2".to_string()).await;

        let stats = cache.stats();
        assert_eq!(stats.entry_count, 2);
        assert_eq!(stats.max_capacity, 100);
        assert_eq!(stats.utilization(), 2.0);
    }

    #[tokio::test]
    async fn test_embedding_cache() {
        let cache = create_embedding_cache();

        let embedding = vec![0.1, 0.2, 0.3];
        cache.insert("test_text".to_string(), embedding.clone()).await;

        let cached = cache.get(&"test_text".to_string()).await;
        assert_eq!(cached, Some(embedding));
    }
}
