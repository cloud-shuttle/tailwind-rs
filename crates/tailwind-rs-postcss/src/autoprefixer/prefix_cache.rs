//! Prefix Cache for Autoprefixer
//!
//! This module provides caching functionality for autoprefixer.

use std::collections::HashMap;

/// Cache for prefix operations
pub struct PrefixCache {
    property_cache: HashMap<String, Vec<String>>,
    browser_cache: HashMap<String, SupportLevel>,
}

impl PrefixCache {
    pub fn new() -> Self {
        Self {
            property_cache: HashMap::new(),
            browser_cache: HashMap::new(),
        }
    }

    pub fn get_cached_prefixes(&self, property: &str) -> Option<&Vec<String>> {
        self.property_cache.get(property)
    }

    pub fn cache_prefixes(&mut self, property: String, prefixes: Vec<String>) {
        self.property_cache.insert(property, prefixes);
    }

    pub fn get_cached_css(&self, css: &str) -> Option<&Vec<String>> {
        self.property_cache.get(css)
    }

    pub fn cache_css(&mut self, css: String, prefixed: String) {
        self.property_cache.insert(css, vec![prefixed]);
    }
}

/// Support level for a feature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportLevel {
    None,
    Partial,
    Full,
}
