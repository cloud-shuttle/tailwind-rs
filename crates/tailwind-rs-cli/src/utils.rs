//! # Utility Functions
//!
//! This module provides utility functions for the CLI tool.

use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

/// Utility functions for file operations
pub struct FileUtils;

impl FileUtils {
    /// Find all Rust files in a directory
    pub fn find_rust_files<P: AsRef<Path>>(dir: P) -> Result<Vec<std::path::PathBuf>> {
        let mut rust_files = Vec::new();

        for entry in WalkDir::new(dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                rust_files.push(path.to_path_buf());
            }
        }

        Ok(rust_files)
    }

    /// Check if a file exists
    pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }

    /// Create directory if it doesn't exist
    pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<()> {
        if !path.as_ref().exists() {
            std::fs::create_dir_all(path)?;
        }
        Ok(())
    }

    /// Read file content
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }

    /// Write file content
    pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Utility functions for path operations
pub struct PathUtils;

impl PathUtils {
    /// Get the relative path from a base directory
    pub fn relative_path<P: AsRef<Path>, Q: AsRef<Path>>(
        base: P,
        path: Q,
    ) -> Result<std::path::PathBuf> {
        let base = base.as_ref().canonicalize()?;
        let path = path.as_ref().canonicalize()?;

        Ok(path.strip_prefix(base)?.to_path_buf())
    }

    /// Check if a path is within a directory
    pub fn is_within<P: AsRef<Path>, Q: AsRef<Path>>(dir: P, path: Q) -> bool {
        let dir = dir.as_ref();
        let path = path.as_ref();

        path.starts_with(dir)
    }

    /// Get the file extension
    pub fn extension<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }

    /// Get the file stem (name without extension)
    pub fn stem<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|s| s.to_string())
    }
}

/// Utility functions for string operations
pub struct StringUtils;

impl StringUtils {
    /// Check if a string contains only valid characters for CSS classes
    pub fn is_valid_css_class(s: &str) -> bool {
        s.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }

    /// Sanitize a string for use as a CSS class
    pub fn sanitize_css_class(s: &str) -> String {
        s.chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect()
    }

    /// Join strings with a separator
    pub fn join_strings(strings: &[String], separator: &str) -> String {
        strings.join(separator)
    }

    /// Split a string by whitespace
    pub fn split_whitespace(s: &str) -> Vec<String> {
        s.split_whitespace().map(|s| s.to_string()).collect()
    }
}

/// Utility functions for logging
pub struct LogUtils;

impl LogUtils {
    /// Log an info message
    pub fn info(message: &str) {
        println!("ℹ️  {}", message);
    }

    /// Log a success message
    pub fn success(message: &str) {
        println!("✅ {}", message);
    }

    /// Log a warning message
    pub fn warning(message: &str) {
        println!("⚠️  {}", message);
    }

    /// Log an error message
    pub fn error(message: &str) {
        eprintln!("❌ {}", message);
    }

    /// Log a debug message
    pub fn debug(message: &str) {
        if std::env::var("RUST_LOG")
            .unwrap_or_default()
            .contains("debug")
        {
            println!("🐛 {}", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_rust_files() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();

        // Create some test files
        fs::write(src_dir.join("main.rs"), "fn main() {}").unwrap();
        fs::write(src_dir.join("lib.rs"), "pub fn lib() {}").unwrap();
        fs::write(src_dir.join("README.md"), "# Test").unwrap();

        let rust_files = FileUtils::find_rust_files(&src_dir).unwrap();

        assert_eq!(rust_files.len(), 2);
        assert!(rust_files.iter().any(|f| f.ends_with("main.rs")));
        assert!(rust_files.iter().any(|f| f.ends_with("lib.rs")));
    }

    #[test]
    fn test_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        assert!(!FileUtils::file_exists(&test_file));

        fs::write(&test_file, "test").unwrap();

        assert!(FileUtils::file_exists(&test_file));
    }

    #[test]
    fn test_ensure_dir() {
        let temp_dir = TempDir::new().unwrap();
        let new_dir = temp_dir.path().join("new_dir");

        assert!(!new_dir.exists());

        FileUtils::ensure_dir(&new_dir).unwrap();

        assert!(new_dir.exists());
        assert!(new_dir.is_dir());
    }

    #[test]
    fn test_read_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        let content = "Hello, World!";

        FileUtils::write_file(&test_file, content).unwrap();

        let read_content = FileUtils::read_file(&test_file).unwrap();

        assert_eq!(read_content, content);
    }

    #[test]
    fn test_is_valid_css_class() {
        assert!(StringUtils::is_valid_css_class("px-4"));
        assert!(StringUtils::is_valid_css_class("bg-blue-600"));
        assert!(StringUtils::is_valid_css_class("text_white"));
        assert!(!StringUtils::is_valid_css_class("px 4"));
        assert!(!StringUtils::is_valid_css_class("px.4"));
    }

    #[test]
    fn test_sanitize_css_class() {
        assert_eq!(StringUtils::sanitize_css_class("px-4"), "px-4");
        assert_eq!(StringUtils::sanitize_css_class("px 4"), "px4");
        assert_eq!(StringUtils::sanitize_css_class("px.4"), "px4");
        assert_eq!(StringUtils::sanitize_css_class("px_4"), "px_4");
    }

    #[test]
    fn test_join_strings() {
        let strings = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(StringUtils::join_strings(&strings, " "), "a b c");
        assert_eq!(StringUtils::join_strings(&strings, "-"), "a-b-c");
    }

    #[test]
    fn test_split_whitespace() {
        let result = StringUtils::split_whitespace("a b c");
        assert_eq!(result, vec!["a", "b", "c"]);

        let result = StringUtils::split_whitespace("  a   b  c  ");
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_extension() {
        assert_eq!(PathUtils::extension("file.rs"), Some("rs".to_string()));
        assert_eq!(PathUtils::extension("file.txt"), Some("txt".to_string()));
        assert_eq!(PathUtils::extension("file"), None);
    }

    #[test]
    fn test_stem() {
        assert_eq!(PathUtils::stem("file.rs"), Some("file".to_string()));
        assert_eq!(PathUtils::stem("file.txt"), Some("file".to_string()));
        assert_eq!(PathUtils::stem("file"), Some("file".to_string()));
    }
}
