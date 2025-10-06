//! Caching System for PostCSS Plugin
//!
//! Provides build caching and optimization to speed up repeated builds.

use crate::Result;
use std::hash::Hasher;

/// Simple content hasher for cache keys
pub struct ContentHasher {
    hasher: std::collections::hash_map::DefaultHasher,
}

impl ContentHasher {
    pub fn new() -> Self {
        Self {
            hasher: std::collections::hash_map::DefaultHasher::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        use std::hash::Hash;
        data.hash(&mut self.hasher);
    }

    pub fn finish(self) -> String {
        format!("{:x}", self.hasher.finish())
    }
}

/// Generate hash for content
pub fn hash_content(content: &[String]) -> String {
    let mut hasher = ContentHasher::new();

    for item in content {
        hasher.update(item.as_bytes());
    }

    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_hashing() {
        let content = vec!["test content".to_string(), "more content".to_string()];
        let hash1 = hash_content(&content);
        let hash2 = hash_content(&content);

        // Same content should produce same hash
        assert_eq!(hash1, hash2);

        // Different content should produce different hash
        let different_content = vec!["different content".to_string()];
        let hash3 = hash_content(&different_content);
        assert_ne!(hash1, hash3);
    }
}
