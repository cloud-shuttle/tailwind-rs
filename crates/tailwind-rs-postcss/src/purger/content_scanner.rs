//! Content scanner for finding used CSS classes

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use super::types::*;

/// Content scanner for finding used CSS classes
pub struct ContentScanner {
    file_extensions: Vec<String>,
    class_patterns: Vec<String>,
}

impl ContentScanner {
    /// Create new content scanner
    pub fn new() -> Self {
        Self {
            file_extensions: Self::get_default_extensions(),
            class_patterns: Self::get_default_patterns(),
        }
    }
    
    /// Scan content for used classes
    pub fn scan_content(&self, content_paths: &[String]) -> Result<HashSet<String>, PurgeError> {
        let mut used_classes = HashSet::new();
        
        for path in content_paths {
            let classes = self.scan_file(path)?;
            used_classes.extend(classes);
        }
        
        Ok(used_classes)
    }
    
    /// Scan content with advanced options
    pub fn scan_content_advanced(&self, content_paths: &[String], options: &PurgeOptions) -> Result<HashSet<String>, PurgeError> {
        let mut used_classes = HashSet::new();
        
        for path in content_paths {
            if self.should_scan_file(path, options) {
                let classes = self.scan_file(path)?;
                used_classes.extend(classes);
            }
        }
        
        Ok(used_classes)
    }
    
    /// Scan a single file for classes
    fn scan_file(&self, path: &str) -> Result<HashSet<String>, PurgeError> {
        let content = fs::read_to_string(path)
            .map_err(|e| PurgeError::FileReadingFailed { 
                path: path.to_string(), 
                error: e.to_string() 
            })?;
        
        let file_type = self.detect_file_type(path);
        self.extract_classes_from_content(&content, &file_type)
    }
    
    /// Extract classes from content based on file type
    fn extract_classes_from_content(&self, content: &str, file_type: &FileType) -> Result<HashSet<String>, PurgeError> {
        let mut classes = HashSet::new();
        
        match file_type {
            FileType::Html => {
                classes.extend(self.extract_from_html(content));
            }
            FileType::JavaScript | FileType::TypeScript => {
                classes.extend(self.extract_from_js(content));
            }
            FileType::Rust => {
                classes.extend(self.extract_from_rust(content));
            }
            FileType::Vue => {
                classes.extend(self.extract_from_vue(content));
            }
            FileType::Svelte => {
                classes.extend(self.extract_from_svelte(content));
            }
            FileType::Other(_) => {
                classes.extend(self.extract_generic(content));
            }
        }
        
        Ok(classes)
    }
    
    /// Extract classes from HTML content
    fn extract_from_html(&self, content: &str) -> HashSet<String> {
        let mut classes = HashSet::new();
        let class_pattern = regex::Regex::new(r#"class\s*=\s*["']([^"']+)["']"#).unwrap();
        
        for cap in class_pattern.captures_iter(content) {
            let class_attr = &cap[1];
            for class_name in class_attr.split_whitespace() {
                classes.insert(class_name.to_string());
            }
        }
        
        classes
    }
    
    /// Extract classes from JavaScript/TypeScript content
    fn extract_from_js(&self, content: &str) -> HashSet<String> {
        let mut classes = HashSet::new();
        
        // Look for className patterns
        let class_patterns = vec![
            r#"className\s*=\s*["']([^"']+)["']"#,
            r#"class\s*=\s*["']([^"']+)["']"#,
            r#"class:\s*["']([^"']+)["']"#,
        ];
        
        for pattern in class_patterns {
            let regex = regex::Regex::new(pattern).unwrap();
            for cap in regex.captures_iter(content) {
                let class_attr = &cap[1];
                for class_name in class_attr.split_whitespace() {
                    classes.insert(class_name.to_string());
                }
            }
        }
        
        classes
    }
    
    /// Extract classes from Rust content
    fn extract_from_rust(&self, content: &str) -> HashSet<String> {
        let mut classes = HashSet::new();
        
        // Look for class! macro patterns
        let class_patterns = vec![
            r#"class!\s*\(\s*["']([^"']+)["']"#,
            r#"class\s*=\s*["']([^"']+)["']"#,
        ];
        
        for pattern in class_patterns {
            let regex = regex::Regex::new(pattern).unwrap();
            for cap in regex.captures_iter(content) {
                let class_attr = &cap[1];
                for class_name in class_attr.split_whitespace() {
                    classes.insert(class_name.to_string());
                }
            }
        }
        
        classes
    }
    
    /// Extract classes from Vue content
    fn extract_from_vue(&self, content: &str) -> HashSet<String> {
        let mut classes = HashSet::new();
        
        // Look for class patterns in Vue templates
        let class_patterns = vec![
            r#"class\s*=\s*["']([^"']+)["']"#,
            r#":class\s*=\s*["']([^"']+)["']"#,
        ];
        
        for pattern in class_patterns {
            let regex = regex::Regex::new(pattern).unwrap();
            for cap in regex.captures_iter(content) {
                let class_attr = &cap[1];
                for class_name in class_attr.split_whitespace() {
                    classes.insert(class_name.to_string());
                }
            }
        }
        
        classes
    }
    
    /// Extract classes from Svelte content
    fn extract_from_svelte(&self, content: &str) -> HashSet<String> {
        let mut classes = HashSet::new();
        
        // Look for class patterns in Svelte templates
        let class_patterns = vec![
            r#"class\s*=\s*["']([^"']+)["']"#,
            r#"class:\s*["']([^"']+)["']"#,
        ];
        
        for pattern in class_patterns {
            let regex = regex::Regex::new(pattern).unwrap();
            for cap in regex.captures_iter(content) {
                let class_attr = &cap[1];
                for class_name in class_attr.split_whitespace() {
                    classes.insert(class_name.to_string());
                }
            }
        }
        
        classes
    }
    
    /// Extract classes using generic patterns
    fn extract_generic(&self, content: &str) -> HashSet<String> {
        let mut classes = HashSet::new();
        
        // Use generic class patterns
        for pattern in &self.class_patterns {
            let regex = regex::Regex::new(pattern).unwrap();
            for cap in regex.captures_iter(content) {
                let class_attr = &cap[1];
                for class_name in class_attr.split_whitespace() {
                    classes.insert(class_name.to_string());
                }
            }
        }
        
        classes
    }
    
    /// Detect file type from path
    fn detect_file_type(&self, path: &str) -> FileType {
        let path = Path::new(path);
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        match extension {
            "html" | "htm" => FileType::Html,
            "js" | "jsx" => FileType::JavaScript,
            "ts" | "tsx" => FileType::TypeScript,
            "rs" => FileType::Rust,
            "vue" => FileType::Vue,
            "svelte" => FileType::Svelte,
            _ => FileType::Other(extension.to_string()),
        }
    }
    
    /// Check if file should be scanned based on options
    fn should_scan_file(&self, path: &str, options: &PurgeOptions) -> bool {
        // Check include patterns
        if !options.include_patterns.is_empty() {
            let should_include = options.include_patterns.iter()
                .any(|pattern| path.contains(pattern));
            if !should_include {
                return false;
            }
        }
        
        // Check exclude patterns
        if options.exclude_patterns.iter().any(|pattern| path.contains(pattern)) {
            return false;
        }
        
        true
    }
    
    /// Get default file extensions
    fn get_default_extensions() -> Vec<String> {
        vec![
            "html".to_string(),
            "htm".to_string(),
            "js".to_string(),
            "jsx".to_string(),
            "ts".to_string(),
            "tsx".to_string(),
            "rs".to_string(),
            "vue".to_string(),
            "svelte".to_string(),
        ]
    }
    
    /// Get default class patterns
    fn get_default_patterns() -> Vec<String> {
        vec![
            r#"class\s*=\s*["']([^"']+)["']"#.to_string(),
            r#"className\s*=\s*["']([^"']+)["']"#.to_string(),
            r#"class!\s*\(\s*["']([^"']+)["']"#.to_string(),
        ]
    }
}
