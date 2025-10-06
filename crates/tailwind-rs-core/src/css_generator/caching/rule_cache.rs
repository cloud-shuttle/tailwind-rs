//! Rule caching for generated CSS rules
//! Provides performance optimization for repeated rule generation

use crate::css_generator::types::CssRule;
use std::collections::HashMap;

/// Cache for generated CSS rules
#[derive(Debug)]
pub struct RuleCache {
    cache: HashMap<String, CssRule>,
    max_size: usize,
}

impl RuleCache {
    /// Create a new rule cache
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            max_size: 1000,
        }
    }

    /// Get a cached rule
    pub fn get(&self, key: &str) -> Option<&CssRule> {
        self.cache.get(key)
    }

    /// Insert a rule into the cache
    pub fn insert(&mut self, key: String, rule: CssRule) {
        if self.cache.len() >= self.max_size {
            // Simple eviction: remove oldest entry
            if let Some(first_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&first_key);
            }
        }
        self.cache.insert(key, rule);
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize) {
        (self.cache.len(), self.max_size)
    }
}
