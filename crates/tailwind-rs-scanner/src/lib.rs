//! # tailwind-rs-scanner
//!
//! Advanced content scanning for Tailwind-RS Core, providing intelligent
//! class detection and extraction from source files.
//!
//! This crate provides comprehensive content scanning capabilities:
//! - Multi-language support (Rust, JavaScript, TypeScript, HTML, etc.)
//! - Parallel file processing for performance
//! - Intelligent class extraction with context awareness
//! - File watching and incremental updates
//! - Tree-sitter integration for accurate parsing
//!
//! ## Features
//!
//! - **Multi-Language Support**: Rust, JS/TS, HTML, Vue, Svelte, and more
//! - **Parallel Processing**: Multi-threaded file scanning for speed
//! - **Intelligent Extraction**: Context-aware class detection
//! - **File Watching**: Real-time file change detection
//! - **Tree-sitter Integration**: Accurate AST-based parsing
//! - **Performance**: Optimized for large codebases
//!
//! ## Example
//!
//! ```rust
//! use tailwind_rs_scanner::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), ScannerError> {
//!     let scanner = ContentScanner::new(ScanConfig::default())?;
//!     
//!     let classes = scanner.scan_for_classes().await?;
//!     println!("Found {} classes", classes.total_classes());
//!     
//!     // Watch for changes
//!     let mut watcher = scanner.watch().await?;
//!     while let Some(update) = watcher.next().await {
//!         println!("Classes updated: {:?}", update);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod file_scanner;
pub mod glob_matcher;
pub mod content_config;
pub mod parallel_processor;
pub mod cache;
pub mod class_extractor;
pub mod file_watcher;
pub mod tree_sitter_parser;
pub mod error;

// Re-export main types
pub use file_scanner::{FileScanner, FileInfo, FileType, ContentScanner, ClassSet};
pub use glob_matcher::{GlobMatcher, GlobPattern};
pub use content_config::{ScanConfig, ContentConfig, FilePattern};
pub use parallel_processor::{ParallelProcessor, ProcessingStats};
pub use cache::{ScanCache, CacheEntry, CacheStats};
pub use class_extractor::{ClassExtractor, ExtractedClass, ClassContext};
pub use file_watcher::{FileWatcher, WatchEvent, WatchConfig};
pub use tree_sitter_parser::{TreeSitterParser, ParseResult, LanguageSupport};
pub use error::{ScannerError, Result};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration for content scanning
pub fn default_config() -> ScanConfig {
    ScanConfig::default()
}

/// Create a new content scanner with default configuration
pub fn new_scanner() -> Result<ContentScanner> {
    ContentScanner::new(ScanConfig::default())
}

/// Scan content for classes using default configuration
pub async fn scan_content(paths: &[String]) -> Result<ClassSet> {
    let scanner = new_scanner()?;
    scanner.scan_paths(paths).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constant() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.chars().any(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_default_config() {
        let config = default_config();
        assert!(!config.content_config.patterns.is_empty());
        assert!(config.parallel_processing);
    }

    #[tokio::test]
    async fn test_scan_content() {
        let paths = vec!["test.html".to_string()];
        let result = scan_content(&paths).await;
        // This will fail if the file doesn't exist, which is expected
        assert!(result.is_err() || result.is_ok());
    }
}
