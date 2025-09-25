//! Types and data structures for CSS purging

use std::collections::HashSet;
use thiserror::Error;

/// Configuration for CSS purging
#[derive(Debug, Clone)]
pub struct PurgeConfig {
    pub content_paths: Vec<String>,
    pub safelist: Vec<String>,
    pub blocklist: Vec<String>,
    pub preserve_comments: bool,
    pub preserve_keyframes: bool,
    pub preserve_media_queries: bool,
}

impl Default for PurgeConfig {
    fn default() -> Self {
        Self {
            content_paths: Vec::new(),
            safelist: Vec::new(),
            blocklist: Vec::new(),
            preserve_comments: true,
            preserve_keyframes: true,
            preserve_media_queries: true,
        }
    }
}

/// Advanced purge options
#[derive(Debug, Clone)]
pub struct PurgeOptions {
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub custom_extractors: Vec<String>,
    pub preserve_whitespace: bool,
    pub minify_output: bool,
}

impl Default for PurgeOptions {
    fn default() -> Self {
        Self {
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            custom_extractors: Vec::new(),
            preserve_whitespace: false,
            minify_output: true,
        }
    }
}

/// Result of CSS purging
#[derive(Debug, Clone)]
pub struct PurgeResult {
    pub purged_css: String,
    pub used_classes: usize,
    pub removed_classes: usize,
    pub original_size: usize,
    pub purged_size: usize,
    pub size_reduction_percentage: f64,
    pub processing_time: std::time::Duration,
}

/// Purge statistics
#[derive(Debug, Clone)]
pub struct PurgeStatistics {
    pub total_purges: usize,
    pub average_size_reduction: f64,
    pub processing_time: std::time::Duration,
}

/// Content file information
#[derive(Debug, Clone)]
pub struct ContentFile {
    pub path: String,
    pub content: String,
    pub file_type: FileType,
    pub classes: HashSet<String>,
}

/// File type for content scanning
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Html,
    JavaScript,
    TypeScript,
    Rust,
    Vue,
    Svelte,
    Other(String),
}

/// Class extraction result
#[derive(Debug, Clone)]
pub struct ClassExtractionResult {
    pub classes: HashSet<String>,
    pub extraction_time: std::time::Duration,
}

/// Rule filtering result
#[derive(Debug, Clone)]
pub struct RuleFilterResult {
    pub filtered_css: String,
    pub rules_removed: usize,
    pub rules_kept: usize,
    pub filtering_time: std::time::Duration,
}

/// Error types for CSS purging
#[derive(Debug, Error)]
pub enum PurgeError {
    #[error("Content scanning failed: {error}")]
    ContentScanningFailed { error: String },
    
    #[error("Class extraction failed: {error}")]
    ClassExtractionFailed { error: String },
    
    #[error("Rule filtering failed: {error}")]
    RuleFilteringFailed { error: String },
    
    #[error("File reading failed: {path} - {error}")]
    FileReadingFailed { path: String, error: String },
    
    #[error("Invalid configuration: {error}")]
    InvalidConfiguration { error: String },
    
    #[error("Memory limit exceeded")]
    MemoryLimitExceeded,
    
    #[error("Processing timeout")]
    ProcessingTimeout,
}
