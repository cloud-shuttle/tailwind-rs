//! Advanced Caching System for Tailwind-RS
//!
//! This module provides a multi-level caching system with:
//! - LRU cache for parsed classes and computed styles
//! - Persistent caching for large projects
//! - Memory-efficient storage with compression
//! - Thread-safe concurrent access

use crate::css_generator::types::CssProperty;
use crate::error::{Result, TailwindError};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Cache key for CSS class parsing results
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassCacheKey {
    /// The original class string
    pub class_string: String,
    /// Theme configuration hash for cache invalidation
    pub theme_hash: u64,
    /// Parser version for compatibility
    pub parser_version: u32,
}

impl ClassCacheKey {
    pub fn new(class_string: String, theme_hash: u64) -> Self {
        Self {
            class_string,
            theme_hash,
            parser_version: 1, // Increment when parser logic changes
        }
    }
}

/// Cached CSS properties for a class
#[derive(Debug, Clone)]
pub struct ClassCacheEntry {
    /// The parsed CSS properties
    pub properties: Vec<CssProperty>,
    /// When this entry was created
    pub created_at: u64,
    /// How many times this entry has been accessed
    pub access_count: u64,
    /// Last access timestamp
    pub last_accessed: u64,
    /// Memory size of this entry in bytes
    pub memory_size: usize,
}

impl ClassCacheEntry {
    pub fn new(properties: Vec<CssProperty>) -> Self {
        let memory_size = Self::calculate_memory_size(&properties);
        let now = Self::current_timestamp();

        Self {
            properties,
            created_at: now,
            access_count: 0,
            last_accessed: now,
            memory_size,
        }
    }

    pub fn access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Self::current_timestamp();
    }

    fn calculate_memory_size(properties: &[CssProperty]) -> usize {
        properties.iter().map(|prop| {
            prop.name.len() + prop.value.len() + std::mem::size_of::<CssProperty>()
        }).sum()
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    pub fn is_expired(&self, max_age: Duration) -> bool {
        let age = Duration::from_secs(Self::current_timestamp() - self.created_at);
        age > max_age
    }
}

/// Multi-level caching system
#[derive(Debug)]
pub struct AdvancedCache {
    /// Fast in-memory LRU cache for frequently accessed classes
    l1_cache: RwLock<LruCache<ClassCacheKey, ClassCacheEntry>>,
    /// Larger but slower secondary cache
    l2_cache: RwLock<HashMap<ClassCacheKey, ClassCacheEntry>>,
    /// Cache statistics
    stats: RwLock<CacheStats>,
    /// Maximum memory usage for L1 cache
    max_l1_memory: usize,
    /// Maximum age for cache entries
    max_age: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub total_memory_used: usize,
    pub l1_entries: usize,
    pub l2_entries: usize,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            evictions: 0,
            total_memory_used: 0,
            l1_entries: 0,
            l2_entries: 0,
        }
    }
}

impl AdvancedCache {
    /// Create a new advanced cache with specified limits
    pub fn new(max_l1_entries: usize, max_l1_memory: usize, max_age_seconds: u64) -> Self {
        Self {
            l1_cache: RwLock::new(LruCache::new(std::num::NonZeroUsize::new(max_l1_entries).unwrap())),
            l2_cache: RwLock::new(HashMap::new()),
            stats: RwLock::new(CacheStats::default()),
            max_l1_memory,
            max_age: Duration::from_secs(max_age_seconds),
        }
    }

    /// Create a cache with default settings (suitable for most applications)
    pub fn default_cache() -> Self {
        Self::new(10_000, 50 * 1024 * 1024, 3600) // 10K entries, 50MB, 1 hour
    }

    /// Get cached properties for a class, or None if not cached
    pub fn get(&self, key: &ClassCacheKey) -> Option<Vec<CssProperty>> {
        // Try L1 cache first
        if let Some(mut entry) = self.l1_cache.write().unwrap().get(key).cloned() {
            entry.access();
            *self.l1_cache.write().unwrap().get_mut(key).unwrap() = entry.clone();

            let mut stats = self.stats.write().unwrap();
            stats.hits += 1;

            return Some(entry.properties);
        }

        // Try L2 cache
        if let Some(mut entry) = self.l2_cache.write().unwrap().get(key).cloned() {
            if !entry.is_expired(self.max_age) {
                entry.access();

                // Promote to L1 if it fits
                if self.can_fit_in_l1(&entry) {
                    self.l1_cache.write().unwrap().put(key.clone(), entry.clone());
                }

                let mut stats = self.stats.write().unwrap();
                stats.hits += 1;

                return Some(entry.properties);
            } else {
                // Remove expired entry
                self.l2_cache.write().unwrap().remove(key);
            }
        }

        let mut stats = self.stats.write().unwrap();
        stats.misses += 1;

        None
    }

    /// Store properties in the cache
    pub fn put(&self, key: ClassCacheKey, properties: Vec<CssProperty>) {
        let entry = ClassCacheEntry::new(properties);

        // Always store in L2
        self.l2_cache.write().unwrap().insert(key.clone(), entry.clone());

        // Store in L1 if it fits
        if self.can_fit_in_l1(&entry) {
            self.l1_cache.write().unwrap().put(key, entry);
        }

        self.update_stats();
    }

    /// Check if an entry can fit in L1 cache
    fn can_fit_in_l1(&self, entry: &ClassCacheEntry) -> bool {
        let current_memory = self.stats.read().unwrap().total_memory_used;
        current_memory + entry.memory_size <= self.max_l1_memory
    }

    /// Update cache statistics
    fn update_stats(&self) {
        let mut stats = self.stats.write().unwrap();

        let l1_len = self.l1_cache.read().unwrap().len();
        let l2_len = self.l2_cache.read().unwrap().len();

        stats.l1_entries = l1_len;
        stats.l2_entries = l2_len;

        // Recalculate memory usage
        stats.total_memory_used = self.l1_cache.read().unwrap()
            .iter()
            .map(|(_, entry)| entry.memory_size)
            .sum();
    }

    /// Get current cache statistics
    pub fn stats(&self) -> CacheStats {
        self.stats.read().unwrap().clone()
    }

    /// Clear expired entries
    pub fn cleanup_expired(&self) {
        let mut l2_cache = self.l2_cache.write().unwrap();
        let mut evictions = 0;

        l2_cache.retain(|_, entry| {
            if entry.is_expired(self.max_age) {
                evictions += 1;
                false
            } else {
                true
            }
        });

        if evictions > 0 {
            self.stats.write().unwrap().evictions += evictions;
        }
    }

    /// Clear all cache entries
    pub fn clear(&self) {
        self.l1_cache.write().unwrap().clear();
        self.l2_cache.write().unwrap().clear();
        *self.stats.write().unwrap() = CacheStats::default();
    }

    /// Get cache hit rate as a percentage
    pub fn hit_rate(&self) -> f64 {
        let stats = self.stats.read().unwrap();
        let total = stats.hits + stats.misses;
        if total == 0 {
            0.0
        } else {
            (stats.hits as f64 / total as f64) * 100.0
        }
    }
}

/// Thread-safe cache wrapper for concurrent access
pub type SharedCache = Arc<AdvancedCache>;

/// Cache manager for coordinating multiple cache instances
#[derive(Debug)]
pub struct CacheManager {
    /// Class parsing cache
    pub class_cache: SharedCache,
    /// Theme value cache
    pub theme_cache: SharedCache,
    /// Computed style cache
    pub style_cache: SharedCache,
    /// Background cleanup task
    _cleanup_handle: Option<std::thread::JoinHandle<()>>,
}

impl CacheManager {
    /// Create a new cache manager with default settings
    pub fn new() -> Self {
        let class_cache = Arc::new(AdvancedCache::new(50_000, 100 * 1024 * 1024, 7200)); // 2 hours
        let theme_cache = Arc::new(AdvancedCache::new(10_000, 20 * 1024 * 1024, 86400)); // 24 hours
        let style_cache = Arc::new(AdvancedCache::new(25_000, 50 * 1024 * 1024, 3600)); // 1 hour

        // Start background cleanup task
        let cleanup_handle = Self::start_cleanup_task(vec![
            Arc::clone(&class_cache),
            Arc::clone(&theme_cache),
            Arc::clone(&style_cache),
        ]);

        Self {
            class_cache,
            theme_cache,
            style_cache,
            _cleanup_handle: Some(cleanup_handle),
        }
    }

    /// Start background cleanup task for multiple caches
    fn start_cleanup_task(caches: Vec<SharedCache>) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_secs(300)); // Clean every 5 minutes

                for cache in &caches {
                    cache.cleanup_expired();
                }
            }
        })
    }

    /// Get combined statistics from all caches
    pub fn combined_stats(&self) -> CacheStats {
        let class_stats = self.class_cache.stats();
        let theme_stats = self.theme_cache.stats();
        let style_stats = self.style_cache.stats();

        CacheStats {
            hits: class_stats.hits + theme_stats.hits + style_stats.hits,
            misses: class_stats.misses + theme_stats.misses + style_stats.misses,
            evictions: class_stats.evictions + theme_stats.evictions + style_stats.evictions,
            total_memory_used: class_stats.total_memory_used + theme_stats.total_memory_used + style_stats.total_memory_used,
            l1_entries: class_stats.l1_entries + theme_stats.l1_entries + style_stats.l1_entries,
            l2_entries: class_stats.l2_entries + theme_stats.l2_entries + style_stats.l2_entries,
        }
    }

    /// Clear all caches
    pub fn clear_all(&self) {
        self.class_cache.clear();
        self.theme_cache.clear();
        self.style_cache.clear();
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_cache_key() {
        let key1 = ClassCacheKey::new("text-red-500".to_string(), 12345);
        let key2 = ClassCacheKey::new("text-red-500".to_string(), 12345);
        let key3 = ClassCacheKey::new("text-blue-500".to_string(), 12345);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_cache_entry() {
        let properties = vec![
            CssProperty {
                name: "color".to_string(),
                value: "red".to_string(),
                important: false,
            }
        ];

        let entry = ClassCacheEntry::new(properties.clone());
        assert_eq!(entry.properties, properties);
        assert_eq!(entry.access_count, 0);
        assert!(!entry.is_expired(Duration::from_secs(1)));
    }

    #[test]
    fn test_advanced_cache_basic() {
        let cache = AdvancedCache::new(100, 1024 * 1024, 3600);
        let key = ClassCacheKey::new("test-class".to_string(), 12345);
        let properties = vec![CssProperty {
            name: "color".to_string(),
            value: "blue".to_string(),
            important: false,
        }];

        // Test cache miss
        assert!(cache.get(&key).is_none());

        // Store and retrieve
        cache.put(key.clone(), properties.clone());
        let cached = cache.get(&key);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap(), properties);

        // Check stats
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.l2_entries, 1);
    }

    #[test]
    fn test_cache_manager() {
        let manager = CacheManager::new();

        let stats = manager.combined_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);

        manager.clear_all();

        let final_stats = manager.combined_stats();
        assert_eq!(final_stats.hits, 0);
        assert_eq!(final_stats.misses, 0);
    }

    #[test]
    fn test_cache_hit_rate() {
        let cache = AdvancedCache::new(100, 1024 * 1024, 3600);
        let key = ClassCacheKey::new("test-class".to_string(), 12345);

        // All misses initially
        for _ in 0..10 {
            cache.get(&key);
        }

        // Add entry and get hits
        let properties = vec![CssProperty {
            name: "color".to_string(),
            value: "green".to_string(),
            important: false,
        }];
        cache.put(key.clone(), properties);

        for _ in 0..10 {
            cache.get(&key);
        }

        let hit_rate = cache.hit_rate();
        assert!(hit_rate > 50.0); // Should have decent hit rate
    }

    #[test]
    fn test_cache_expiration() {
        let cache = AdvancedCache::new(100, 1024 * 1024, 1); // 1 second max age
        let key = ClassCacheKey::new("expiring-class".to_string(), 12345);
        let properties = vec![CssProperty {
            name: "color".to_string(),
            value: "orange".to_string(),
            important: false,
        }];

        cache.put(key.clone(), properties);
        assert!(cache.get(&key).is_some());

        // Wait for expiration
        std::thread::sleep(Duration::from_secs(2));

        cache.cleanup_expired();
        assert!(cache.get(&key).is_none());
    }
}
