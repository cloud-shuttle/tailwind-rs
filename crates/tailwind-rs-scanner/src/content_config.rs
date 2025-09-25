//! Content configuration for scanning
//!
//! This module provides configuration structures for content scanning
//! and file pattern matching.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Main scan configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    /// Content configuration
    pub content_config: ContentConfig,
    /// Enable parallel processing
    pub parallel_processing: bool,
    /// Maximum number of parallel workers
    pub max_workers: Option<usize>,
    /// Enable caching
    pub enable_cache: bool,
    /// Cache TTL in seconds
    pub cache_ttl: u64,
    /// Enable file watching
    pub enable_watching: bool,
    /// Watch debounce time in milliseconds
    pub watch_debounce: u64,
}

/// Content configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentConfig {
    /// File patterns to include
    pub patterns: Vec<FilePattern>,
    /// File patterns to exclude
    pub exclude_patterns: Vec<FilePattern>,
    /// Maximum file size to scan (in bytes)
    pub max_file_size: Option<u64>,
    /// File extensions to scan
    pub extensions: Vec<String>,
    /// Directories to ignore
    pub ignore_dirs: Vec<String>,
    /// Enable tree-sitter parsing
    pub enable_tree_sitter: bool,
    /// Custom class extraction rules
    pub custom_rules: Vec<ExtractionRule>,
}

/// File pattern for matching files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePattern {
    /// Pattern string (glob pattern)
    pub pattern: String,
    /// Pattern type
    pub pattern_type: PatternType,
    /// Whether pattern is case sensitive
    pub case_sensitive: bool,
}

/// Pattern matching type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// Glob pattern
    Glob,
    /// Regex pattern
    Regex,
    /// Simple string match
    String,
}

/// Custom extraction rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionRule {
    /// Rule name
    pub name: String,
    /// File pattern this rule applies to
    pub file_pattern: String,
    /// Regex pattern for class extraction
    pub class_pattern: String,
    /// Context extraction pattern
    pub context_pattern: Option<String>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            content_config: ContentConfig::default(),
            parallel_processing: true,
            max_workers: None,
            enable_cache: true,
            cache_ttl: 3600, // 1 hour
            enable_watching: false,
            watch_debounce: 100, // 100ms
        }
    }
}

impl Default for ContentConfig {
    fn default() -> Self {
        Self {
            patterns: vec![
                FilePattern::new("**/*.rs", PatternType::Glob),
                FilePattern::new("**/*.js", PatternType::Glob),
                FilePattern::new("**/*.ts", PatternType::Glob),
                FilePattern::new("**/*.jsx", PatternType::Glob),
                FilePattern::new("**/*.tsx", PatternType::Glob),
                FilePattern::new("**/*.html", PatternType::Glob),
                FilePattern::new("**/*.vue", PatternType::Glob),
                FilePattern::new("**/*.svelte", PatternType::Glob),
            ],
            exclude_patterns: vec![
                FilePattern::new("**/node_modules/**", PatternType::Glob),
                FilePattern::new("**/target/**", PatternType::Glob),
                FilePattern::new("**/.git/**", PatternType::Glob),
                FilePattern::new("**/dist/**", PatternType::Glob),
                FilePattern::new("**/build/**", PatternType::Glob),
            ],
            max_file_size: Some(10 * 1024 * 1024), // 10MB
            extensions: vec![
                "rs".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "jsx".to_string(),
                "tsx".to_string(),
                "html".to_string(),
                "vue".to_string(),
                "svelte".to_string(),
                "css".to_string(),
                "scss".to_string(),
                "less".to_string(),
            ],
            ignore_dirs: vec![
                "node_modules".to_string(),
                "target".to_string(),
                ".git".to_string(),
                "dist".to_string(),
                "build".to_string(),
                ".next".to_string(),
                ".nuxt".to_string(),
            ],
            enable_tree_sitter: true,
            custom_rules: Vec::new(),
        }
    }
}

impl FilePattern {
    /// Create a new file pattern
    pub fn new(pattern: &str, pattern_type: PatternType) -> Self {
        Self {
            pattern: pattern.to_string(),
            pattern_type,
            case_sensitive: false,
        }
    }

    /// Create a case-sensitive file pattern
    pub fn new_case_sensitive(pattern: &str, pattern_type: PatternType) -> Self {
        Self {
            pattern: pattern.to_string(),
            pattern_type,
            case_sensitive: true,
        }
    }

    /// Check if pattern matches a file path
    pub fn matches_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy();
        let target = if self.case_sensitive {
            path_str.to_string()
        } else {
            path_str.to_lowercase()
        };

        let pattern = if self.case_sensitive {
            self.pattern.clone()
        } else {
            self.pattern.to_lowercase()
        };

        match self.pattern_type {
            PatternType::Glob => {
                // Simple glob matching (in a real implementation, use a proper glob library)
                self.matches_glob(&target, &pattern)
            }
            PatternType::Regex => {
                // Regex matching
                if let Ok(regex) = regex::Regex::new(&pattern) {
                    regex.is_match(&target)
                } else {
                    false
                }
            }
            PatternType::String => {
                target.contains(&pattern)
            }
        }
    }

    /// Simple glob matching implementation
    fn matches_glob(&self, path: &str, pattern: &str) -> bool {
        // This is a simplified glob matcher
        // In a real implementation, use a proper glob library like globset
        if pattern == "**/*" {
            return true;
        }
        
        if pattern.starts_with("**/") {
            let suffix = &pattern[3..];
            if suffix.contains('*') {
                // Handle patterns like **/*.rs
                let suffix_parts: Vec<&str> = suffix.split('*').collect();
                if suffix_parts.len() == 2 {
                    let prefix = suffix_parts[0];
                    let suffix_end = suffix_parts[1];
                    return path.ends_with(suffix_end);
                }
            }
            return path.ends_with(suffix);
        }
        
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];
                return path.starts_with(prefix) && path.ends_with(suffix);
            }
        }
        
        path == pattern
    }
}

impl ExtractionRule {
    /// Create a new extraction rule
    pub fn new(name: &str, file_pattern: &str, class_pattern: &str) -> Self {
        Self {
            name: name.to_string(),
            file_pattern: file_pattern.to_string(),
            class_pattern: class_pattern.to_string(),
            context_pattern: None,
        }
    }

    /// Create an extraction rule with context
    pub fn new_with_context(
        name: &str,
        file_pattern: &str,
        class_pattern: &str,
        context_pattern: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            file_pattern: file_pattern.to_string(),
            class_pattern: class_pattern.to_string(),
            context_pattern: Some(context_pattern.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_scan_config_default() {
        let config = ScanConfig::default();
        assert!(config.parallel_processing);
        assert!(config.enable_cache);
        assert!(!config.enable_watching);
    }

    #[test]
    fn test_content_config_default() {
        let config = ContentConfig::default();
        assert!(!config.patterns.is_empty());
        assert!(!config.exclude_patterns.is_empty());
        assert!(config.enable_tree_sitter);
    }

    #[test]
    fn test_file_pattern_creation() {
        let pattern = FilePattern::new("**/*.rs", PatternType::Glob);
        assert_eq!(pattern.pattern, "**/*.rs");
        assert!(!pattern.case_sensitive);
    }

    #[test]
    fn test_file_pattern_matching() {
        let pattern = FilePattern::new("**/*.rs", PatternType::Glob);
        let rust_file = Path::new("src/main.rs");
        let js_file = Path::new("src/main.js");
        
        assert!(pattern.matches_file(rust_file));
        assert!(!pattern.matches_file(js_file));
    }

    #[test]
    fn test_extraction_rule_creation() {
        let rule = ExtractionRule::new(
            "rust_classes",
            "**/*.rs",
            r#"class\s*=\s*"([^"]+)""#,
        );
        
        assert_eq!(rule.name, "rust_classes");
        assert_eq!(rule.file_pattern, "**/*.rs");
        assert_eq!(rule.class_pattern, r#"class\s*=\s*"([^"]+)""#);
        assert!(rule.context_pattern.is_none());
    }

    #[test]
    fn test_extraction_rule_with_context() {
        let rule = ExtractionRule::new_with_context(
            "html_classes",
            "**/*.html",
            r#"class\s*=\s*"([^"]+)""#,
            r#"<(\w+)\s+[^>]*class\s*=\s*"[^"]*""#,
        );
        
        assert_eq!(rule.name, "html_classes");
        assert!(rule.context_pattern.is_some());
    }
}
