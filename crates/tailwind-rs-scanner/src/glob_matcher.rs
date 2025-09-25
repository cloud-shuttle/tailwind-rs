//! Glob pattern matching implementation
//!
//! This module provides glob pattern matching capabilities for
//! file discovery and filtering.

use crate::error::{ScannerError, Result};

/// Glob pattern matcher
#[derive(Debug)]
pub struct GlobMatcher {
    /// Compiled patterns
    patterns: Vec<GlobPattern>,
}

/// Glob pattern
#[derive(Debug, Clone)]
pub struct GlobPattern {
    /// Pattern string
    pub pattern: String,
    /// Whether pattern is case sensitive
    pub case_sensitive: bool,
}

impl GlobMatcher {
    /// Create a new glob matcher
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Add a pattern
    pub fn add_pattern(&mut self, pattern: &str) -> Result<()> {
        self.patterns.push(GlobPattern {
            pattern: pattern.to_string(),
            case_sensitive: false,
        });
        Ok(())
    }

    /// Check if path matches any pattern
    pub fn matches(&self, path: &str) -> bool {
        for pattern in &self.patterns {
            if pattern.matches(path) {
                return true;
            }
        }
        false
    }
}

impl GlobPattern {
    /// Check if pattern matches path
    pub fn matches(&self, path: &str) -> bool {
        // Simple glob matching implementation
        // In a real implementation, use a proper glob library
        if self.pattern == "**/*" {
            return true;
        }
        
        if self.pattern.starts_with("**/") {
            let suffix = &self.pattern[3..];
            if suffix.contains('*') {
                // Handle patterns like **/*.rs
                let suffix_parts: Vec<&str> = suffix.split('*').collect();
                if suffix_parts.len() == 2 {
                    let _prefix = suffix_parts[0];
                    let suffix_end = suffix_parts[1];
                    return path.ends_with(suffix_end);
                }
            }
            return path.ends_with(suffix);
        }
        
        path == self.pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_matcher_creation() {
        let matcher = GlobMatcher::new();
        assert!(matcher.patterns.is_empty());
    }

    #[test]
    fn test_glob_pattern_matching() {
        let pattern = GlobPattern {
            pattern: "**/*.rs".to_string(),
            case_sensitive: false,
        };
        
        assert!(pattern.matches("src/main.rs"));
        assert!(!pattern.matches("src/main.js"));
    }
}
