//! Caching implementation for content scanning
//!
//! This module provides caching capabilities for efficient
//! content scanning and file watching.

use crate::class_extractor::ExtractedClass;
use crate::error::{Result, ScannerError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

/// Scan cache for storing processed results
#[derive(Debug)]
pub struct ScanCache {
    /// Cache entries
    entries: HashMap<PathBuf, CacheEntry>,
    /// Cache statistics
    stats: CacheStats,
}

/// Cache entry for a file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheEntry {
    /// File path
    pub path: PathBuf,
    /// Extracted classes
    pub classes: Vec<ExtractedClass>,
    /// File modification time
    pub modified: SystemTime,
    /// Cache creation time
    pub created: SystemTime,
    /// Cache TTL
    pub ttl: Duration,
}

/// Cache statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheStats {
    /// Total cache entries
    pub total_entries: usize,
    /// Cache hits
    pub hits: usize,
    /// Cache misses
    pub misses: usize,
    /// Cache size in bytes
    pub size_bytes: usize,
    /// Average entry size
    pub average_entry_size: usize,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            total_entries: 0,
            hits: 0,
            misses: 0,
            size_bytes: 0,
            average_entry_size: 0,
        }
    }
}

impl ScanCache {
    /// Create a new scan cache
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            stats: CacheStats::default(),
        }
    }

    /// Get cached classes for a file
    pub fn get_file_classes(&mut self, path: &PathBuf) -> Option<&Vec<ExtractedClass>> {
        if let Some(entry) = self.entries.get(path) {
            if entry.is_valid() {
                self.stats.hits += 1;
                Some(&entry.classes)
            } else {
                self.stats.misses += 1;
                None
            }
        } else {
            self.stats.misses += 1;
            None
        }
    }

    /// Update file classes in cache
    pub fn update_file(&mut self, path: PathBuf, classes: Vec<ExtractedClass>) {
        let entry = CacheEntry {
            path: path.clone(),
            classes,
            modified: SystemTime::now(),
            created: SystemTime::now(),
            ttl: Duration::from_secs(3600), // 1 hour default TTL
        };

        self.entries.insert(path, entry);
        self.stats.total_entries = self.entries.len();
    }

    /// Check if file is cached and valid
    pub fn is_cached(&self, path: &PathBuf) -> bool {
        if let Some(entry) = self.entries.get(path) {
            entry.is_valid()
        } else {
            false
        }
    }

    /// Remove file from cache
    pub fn remove_file(&mut self, path: &PathBuf) {
        self.entries.remove(path);
        self.stats.total_entries = self.entries.len();
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.stats = CacheStats::default();
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        self.stats.clone()
    }

    /// Clean expired entries
    pub fn clean_expired(&mut self) {
        let now = SystemTime::now();
        self.entries.retain(|_, entry| entry.is_valid_at(now));
        self.stats.total_entries = self.entries.len();
    }
}

impl CacheEntry {
    /// Check if cache entry is valid
    pub fn is_valid(&self) -> bool {
        self.is_valid_at(SystemTime::now())
    }

    /// Check if cache entry is valid at a specific time
    pub fn is_valid_at(&self, now: SystemTime) -> bool {
        if let Ok(elapsed) = now.duration_since(self.created) {
            elapsed < self.ttl
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class_extractor::ClassContext;

    #[test]
    fn test_scan_cache_creation() {
        let cache = ScanCache::new();
        assert_eq!(cache.entries.len(), 0);
    }

    #[test]
    fn test_cache_entry_creation() {
        let path = PathBuf::from("test.rs");
        let classes = vec![ExtractedClass {
            class_name: "p-4".to_string(),
            context: ClassContext::new(),
            line: 1,
            column: 1,
        }];

        let entry = CacheEntry {
            path: path.clone(),
            classes,
            modified: SystemTime::now(),
            created: SystemTime::now(),
            ttl: Duration::from_secs(3600),
        };

        assert_eq!(entry.path, path);
        assert!(entry.is_valid());
    }

    #[test]
    fn test_cache_operations() {
        let mut cache = ScanCache::new();
        let path = PathBuf::from("test.rs");
        let classes = vec![ExtractedClass {
            class_name: "p-4".to_string(),
            context: ClassContext::new(),
            line: 1,
            column: 1,
        }];

        // Add to cache
        cache.update_file(path.clone(), classes);
        assert!(cache.is_cached(&path));

        // Get from cache
        let cached_classes = cache.get_file_classes(&path);
        assert!(cached_classes.is_some());

        // Remove from cache
        cache.remove_file(&path);
        assert!(!cache.is_cached(&path));
    }

    #[test]
    fn test_cache_stats() {
        let cache = ScanCache::new();
        let stats = cache.get_stats();
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }
}
