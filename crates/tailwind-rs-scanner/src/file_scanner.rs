//! File scanner implementation
//!
//! This module provides file discovery and scanning capabilities for
//! detecting Tailwind classes in source files.

use crate::cache::{CacheEntry, CacheStats, ScanCache};
use crate::class_extractor::{ClassContext, ClassExtractor, ExtractedClass};
use crate::content_config::{ContentConfig, FilePattern, ScanConfig};
use crate::error::{Result, ScannerError};
use crate::parallel_processor::{ParallelProcessor, ProcessingStats};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main content scanner
#[derive(Debug)]
pub struct ContentScanner {
    config: ScanConfig,
    file_scanner: FileScanner,
    class_extractor: ClassExtractor,
    parallel_processor: ParallelProcessor,
    cache: Arc<RwLock<ScanCache>>,
}

/// File scanner for discovering and reading files
#[derive(Debug)]
pub struct FileScanner {
    config: ContentConfig,
    file_types: HashMap<String, FileType>,
}

/// File information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub file_type: FileType,
    pub size: u64,
    pub modified: std::time::SystemTime,
    pub content: Option<String>,
}

/// Supported file types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileType {
    Rust,
    JavaScript,
    TypeScript,
    Html,
    Vue,
    Svelte,
    Css,
    Scss,
    Less,
    Json,
    Yaml,
    Toml,
    Markdown,
    Other(String),
}

/// Set of extracted classes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassSet {
    pub classes: HashSet<String>,
    pub file_classes: HashMap<PathBuf, Vec<ExtractedClass>>,
    pub total_files: usize,
    pub total_classes: usize,
    pub unique_classes: usize,
    pub processing_stats: ProcessingStats,
}

impl ClassSet {
    /// Create a new empty class set
    pub fn new() -> Self {
        Self {
            classes: HashSet::new(),
            file_classes: HashMap::new(),
            total_files: 0,
            total_classes: 0,
            unique_classes: 0,
            processing_stats: ProcessingStats::default(),
        }
    }

    /// Add classes from a file
    pub fn add_file_classes(&mut self, file_path: PathBuf, classes: Vec<ExtractedClass>) {
        let unique_classes: HashSet<String> =
            classes.iter().map(|c| c.class_name.clone()).collect();

        self.classes.extend(unique_classes);
        self.file_classes.insert(file_path, classes);
        self.total_files += 1;
        self.unique_classes = self.classes.len();
    }

    /// Get total number of classes
    pub fn total_classes(&self) -> usize {
        self.total_classes
    }

    /// Get unique number of classes
    pub fn unique_classes(&self) -> usize {
        self.unique_classes
    }

    /// Get classes for a specific file
    pub fn get_file_classes(&self, file_path: &Path) -> Option<&Vec<ExtractedClass>> {
        self.file_classes.get(file_path)
    }

    /// Check if a class exists
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.contains(class)
    }

    /// Get all classes as a sorted vector
    pub fn get_all_classes(&self) -> Vec<String> {
        let mut classes: Vec<String> = self.classes.iter().cloned().collect();
        classes.sort();
        classes
    }
}

impl Default for ClassSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentScanner {
    /// Create a new content scanner
    pub fn new(config: ScanConfig) -> Result<Self> {
        let file_scanner = FileScanner::new(config.content_config.clone())?;
        let class_extractor = ClassExtractor::new();
        let parallel_processor = ParallelProcessor::new(config.parallel_processing);
        let cache = Arc::new(RwLock::new(ScanCache::new()));

        Ok(Self {
            config,
            file_scanner,
            class_extractor,
            parallel_processor,
            cache,
        })
    }

    /// Scan for classes in specified paths
    pub async fn scan_paths(&self, paths: &[String]) -> Result<ClassSet> {
        let start_time = std::time::Instant::now();

        // Discover files
        let files = self.file_scanner.discover_files(paths).await?;

        // Process files in parallel
        let results = self
            .parallel_processor
            .process_files(&files, &self.class_extractor, &self.config)
            .await?;

        // Build class set
        let mut class_set = ClassSet::new();
        for (file_path, classes) in results {
            class_set.add_file_classes(file_path, classes);
        }

        class_set.processing_stats.total_time = start_time.elapsed();
        class_set.total_classes = class_set.classes.len();

        // Update cache
        {
            let mut cache = self.cache.write().await;
            for (file_path, classes) in &class_set.file_classes {
                cache.update_file(file_path.clone(), classes.clone());
            }
        }

        Ok(class_set)
    }

    /// Scan for classes in a single file
    pub async fn scan_file(&self, file_path: &Path) -> Result<Vec<ExtractedClass>> {
        let file_info = self.file_scanner.read_file(file_path).await?;
        self.class_extractor.extract_classes(&file_info).await
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        cache.get_stats()
    }

    /// Clear cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Get scanner configuration
    pub fn get_config(&self) -> &ScanConfig {
        &self.config
    }

    /// Update scanner configuration
    pub fn update_config(&mut self, config: ScanConfig) -> Result<()> {
        self.config = config;
        self.file_scanner = FileScanner::new(self.config.content_config.clone())?;
        self.parallel_processor = ParallelProcessor::new(self.config.parallel_processing);
        Ok(())
    }
}

impl FileScanner {
    /// Create a new file scanner
    pub fn new(config: ContentConfig) -> Result<Self> {
        let mut file_types = HashMap::new();

        // Initialize file type mappings
        file_types.insert("rs".to_string(), FileType::Rust);
        file_types.insert("js".to_string(), FileType::JavaScript);
        file_types.insert("jsx".to_string(), FileType::JavaScript);
        file_types.insert("ts".to_string(), FileType::TypeScript);
        file_types.insert("tsx".to_string(), FileType::TypeScript);
        file_types.insert("html".to_string(), FileType::Html);
        file_types.insert("htm".to_string(), FileType::Html);
        file_types.insert("vue".to_string(), FileType::Vue);
        file_types.insert("svelte".to_string(), FileType::Svelte);
        file_types.insert("css".to_string(), FileType::Css);
        file_types.insert("scss".to_string(), FileType::Scss);
        file_types.insert("sass".to_string(), FileType::Scss);
        file_types.insert("less".to_string(), FileType::Less);
        file_types.insert("json".to_string(), FileType::Json);
        file_types.insert("yaml".to_string(), FileType::Yaml);
        file_types.insert("yml".to_string(), FileType::Yaml);
        file_types.insert("toml".to_string(), FileType::Toml);
        file_types.insert("md".to_string(), FileType::Markdown);
        file_types.insert("markdown".to_string(), FileType::Markdown);

        Ok(Self { config, file_types })
    }

    /// Discover files matching the configuration patterns
    pub async fn discover_files(&self, paths: &[String]) -> Result<Vec<FileInfo>> {
        let mut files = Vec::new();

        for path in paths {
            let path = Path::new(path);
            if path.is_file() {
                // Single file
                if let Ok(file_info) = self.read_file(path).await {
                    files.push(file_info);
                }
            } else if path.is_dir() {
                // Directory - scan recursively
                let dir_files = self.scan_directory(path).await?;
                files.extend(dir_files);
            }
        }

        Ok(files)
    }

    /// Scan a directory for files
    async fn scan_directory(&self, dir_path: &Path) -> Result<Vec<FileInfo>> {
        let mut files = Vec::new();

        for entry in walkdir::WalkDir::new(dir_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Ok(file_info) = self.read_file(entry.path()).await {
                    // Check if file matches any of the patterns
                    if self.matches_patterns(&file_info) {
                        files.push(file_info);
                    }
                }
            }
        }

        Ok(files)
    }

    /// Read a file and return file information
    pub async fn read_file(&self, file_path: &Path) -> Result<FileInfo> {
        let metadata = std::fs::metadata(file_path)?;
        let file_type = self.detect_file_type(file_path);
        let content = if self.should_read_content(&file_type) {
            Some(std::fs::read_to_string(file_path)?)
        } else {
            None
        };

        Ok(FileInfo {
            path: file_path.to_path_buf(),
            file_type,
            size: metadata.len(),
            modified: metadata.modified()?,
            content,
        })
    }

    /// Detect file type from extension
    fn detect_file_type(&self, file_path: &Path) -> FileType {
        if let Some(extension) = file_path.extension() {
            if let Some(extension_str) = extension.to_str() {
                if let Some(file_type) = self.file_types.get(extension_str) {
                    return file_type.clone();
                }
            }
        }
        FileType::Other("unknown".to_string())
    }

    /// Check if file matches any of the configured patterns
    fn matches_patterns(&self, file_info: &FileInfo) -> bool {
        if self.config.patterns.is_empty() {
            return true; // No patterns means match all
        }

        for pattern in &self.config.patterns {
            if pattern.matches_file(&file_info.path) {
                return true;
            }
        }

        false
    }

    /// Check if we should read file content
    fn should_read_content(&self, file_type: &FileType) -> bool {
        match file_type {
            FileType::Rust
            | FileType::JavaScript
            | FileType::TypeScript
            | FileType::Html
            | FileType::Vue
            | FileType::Svelte
            | FileType::Css
            | FileType::Scss
            | FileType::Less
            | FileType::Markdown => true,
            _ => false,
        }
    }
}

impl Default for ProcessingStats {
    fn default() -> Self {
        Self {
            total_files: 0,
            processed_files: 0,
            total_classes: 0,
            unique_classes: 0,
            total_time: std::time::Duration::from_millis(0),
            average_file_time: std::time::Duration::from_millis(0),
            memory_usage: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_class_set_creation() {
        let class_set = ClassSet::new();
        assert_eq!(class_set.total_classes(), 0);
        assert_eq!(class_set.unique_classes(), 0);
    }

    #[test]
    fn test_class_set_operations() {
        let mut class_set = ClassSet::new();

        let classes = vec![
            ExtractedClass {
                class_name: "p-4".to_string(),
                context: ClassContext::new(),
                line: 1,
                column: 1,
            },
            ExtractedClass {
                class_name: "bg-blue-500".to_string(),
                context: ClassContext::new(),
                line: 2,
                column: 1,
            },
        ];

        class_set.add_file_classes(PathBuf::from("test.rs"), classes);

        assert_eq!(class_set.total_files, 1);
        assert_eq!(class_set.unique_classes(), 2);
        assert!(class_set.has_class("p-4"));
        assert!(class_set.has_class("bg-blue-500"));
        assert!(!class_set.has_class("text-red-500"));
    }

    #[test]
    fn test_file_type_detection() {
        let scanner = FileScanner::new(ContentConfig::default()).unwrap();

        let rust_file = Path::new("test.rs");
        let js_file = Path::new("test.js");
        let html_file = Path::new("test.html");

        assert_eq!(scanner.detect_file_type(rust_file), FileType::Rust);
        assert_eq!(scanner.detect_file_type(js_file), FileType::JavaScript);
        assert_eq!(scanner.detect_file_type(html_file), FileType::Html);
    }

    #[tokio::test]
    async fn test_content_scanner_creation() {
        let config = ScanConfig::default();
        let scanner = ContentScanner::new(config);
        assert!(scanner.is_ok());
    }
}
