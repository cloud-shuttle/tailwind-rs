//! Content Extraction Engine
//!
//! Scans source files for Tailwind CSS classes and extracts them
//! for processing by the CSS generator.

use crate::{PostCssError, Result};
use glob::Pattern;
use regex::Regex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tokio::fs;
use walkdir::WalkDir;

/// Content extractor for finding Tailwind classes in source files
pub struct ContentExtractor {
    /// Regex for extracting class attributes
    class_regex: Regex,
    /// Regex for extracting className attributes
    class_name_regex: Regex,
}

impl ContentExtractor {
    /// Create a new content extractor
    pub fn new() -> Result<Self> {
        // Regex patterns for finding classes in different contexts
        let class_regex = Regex::new(r#"(?i)class\s*=\s*["']([^"']*)["']"#)?;
        let class_name_regex = Regex::new(r#"(?i)classname\s*[:=]\s*(["'`])([^"']*)\1"#)?;

        Ok(Self {
            class_regex,
            class_name_regex,
        })
    }

    /// Extract content from file patterns
    pub async fn extract_from_patterns(&self, patterns: &[String]) -> Result<Vec<String>> {
        let mut all_content = Vec::new();

        for pattern in patterns {
            let files = self.find_files(pattern)?;
            println!("📁 Scanning {} files matching '{}'", files.len(), pattern);

            for file_path in files {
                let content = fs::read_to_string(&file_path).await
                    .map_err(|e| PostCssError::Io(e))?;

                all_content.push(content);
            }
        }

        Ok(all_content)
    }

    /// Extract Tailwind classes from content
    pub async fn extract_classes(&self, content: &[String]) -> Result<HashSet<String>> {
        let mut all_classes = HashSet::new();

        for content_item in content {
            let classes = self.extract_classes_from_content(content_item);
            all_classes.extend(classes);
        }

        Ok(all_classes)
    }

    /// Extract classes from a single content string
    fn extract_classes_from_content(&self, content: &str) -> HashSet<String> {
        let mut classes = HashSet::new();

        // Extract from class attributes (HTML)
        for cap in self.class_regex.captures_iter(content) {
            if let Some(class_attr) = cap.get(1) {
                self.parse_class_string(class_attr.as_str(), &mut classes);
            }
        }

        // Extract from className attributes (JS/TS/React)
        for cap in self.class_name_regex.captures_iter(content) {
            if let Some(class_attr) = cap.get(2) {
                self.parse_class_string(class_attr.as_str(), &mut classes);
            }
        }

        // Extract from template literals and dynamic expressions
        self.extract_dynamic_classes(content, &mut classes);

        classes
    }

    /// Parse class string and extract individual classes
    fn parse_class_string(&self, class_str: &str, classes: &mut HashSet<String>) {
        // Handle template literals and expressions
        if class_str.contains("${") || class_str.contains("`") {
            self.parse_template_literal(class_str, classes);
        } else {
            // Simple space-separated classes
            for class in class_str.split_whitespace() {
                let class = class.trim().trim_matches(|c: char| !c.is_alphanumeric() && c != '-');
                if self.is_valid_tailwind_class(class) {
                    classes.insert(class.to_string());
                }
            }
        }
    }

    /// Parse template literals with expressions
    fn parse_template_literal(&self, template: &str, classes: &mut HashSet<String>) {
        // Extract static parts from template literals
        // This is a simplified implementation - a full implementation would
        // evaluate JavaScript expressions

        let parts: Vec<&str> = template.split("${").collect();

        for part in parts {
            if let Some(end) = part.find("}") {
                let after_expr = &part[end + 1..];
                self.parse_class_string(after_expr, classes);
            } else {
                self.parse_class_string(part, classes);
            }
        }
    }

    /// Extract classes from dynamic expressions
    fn extract_dynamic_classes(&self, content: &str, classes: &mut HashSet<String>) {
        // Look for common patterns in JavaScript/TypeScript

        // Template literals with class names
        let template_regex = Regex::new(r#"`([^`]*(?:bg-|text-|flex|items-|justify-|p-|m-|w-|h-)[^`]*?)"#).unwrap();
        for cap in template_regex.captures_iter(content) {
            if let Some(template) = cap.get(1) {
                self.parse_class_string(template.as_str(), classes);
            }
        }

        // clsx/classnames patterns
        let clsx_regex = Regex::new(r#"(?:clsx|classnames|classNames)\s*\(\s*\{([^}]+)\}"#).unwrap();
        for cap in clsx_regex.captures_iter(content) {
            if let Some(obj_content) = cap.get(1) {
                // Extract class names from object properties
                for prop in obj_content.as_str().split(',') {
                    if let Some(colon_pos) = prop.find(':') {
                        let class_part = prop[colon_pos + 1..].trim();
                        if class_part.starts_with('\'') || class_part.starts_with('"') {
                            let class_name = class_part.trim_matches(|c| c == '\'' || c == '"' || c == '`');
                            if self.is_valid_tailwind_class(class_name) {
                                classes.insert(class_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    /// Check if a string looks like a valid Tailwind class
    fn is_valid_tailwind_class(&self, class: &str) -> bool {
        if class.is_empty() {
            return false;
        }

        // Basic validation - contains at least one dash and starts with letter
        class.contains('-') && class.chars().next().unwrap().is_alphabetic()
    }

    /// Find files matching a glob pattern
    fn find_files(&self, pattern: &str) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if pattern.contains('*') {
            // Glob pattern
            let glob_pattern = Pattern::new(pattern)?;

            for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    let path_str = path.to_string_lossy();
                    if glob_pattern.matches(&path_str) {
                        files.push(path.to_path_buf());
                    }
                }
            }
        } else {
            // Direct file path
            let path = Path::new(pattern);
            if path.exists() && path.is_file() {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }

    /// Get statistics about extraction
    pub fn get_stats(&self) -> ExtractionStats {
        ExtractionStats {
            files_scanned: 0, // Would track in real implementation
            classes_found: 0,
            patterns_processed: 0,
        }
    }
}

/// Extraction statistics
#[derive(Debug, Clone)]
pub struct ExtractionStats {
    pub files_scanned: usize,
    pub classes_found: usize,
    pub patterns_processed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_extraction_from_html() {
        let extractor = ContentExtractor::new().unwrap();

        let html = r#"
        <div class="bg-blue-500 hover:bg-blue-700 text-center">
            <span class="flex items-center">Hello</span>
        </div>
        "#;

        let classes = extractor.extract_classes_from_content(html);

        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("text-center"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("items-center"));
    }

    #[test]
    fn test_class_extraction_from_js() {
        let extractor = ContentExtractor::new().unwrap();

        let js = r#"
        const classes = "bg-red-500 text-white";
        const element = <div className={classes}>Hello</div>;
        const another = <span className="flex items-center">World</span>;
        "#;

        let classes = extractor.extract_classes_from_content(js);

        assert!(classes.contains("bg-red-500"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("items-center"));
    }

    #[test]
    fn test_dynamic_class_extraction() {
        let extractor = ContentExtractor::new().unwrap();

        let js = r#"
        const color = "blue";
        const element = <div className={`bg-${color}-500 text-white`}>Dynamic</div>;
        "#;

        let classes = extractor.extract_classes_from_content(js);

        // Should extract static parts that look like classes
        assert!(classes.contains("text-white"));
    }

    #[test]
    fn test_invalid_class_filtering() {
        let extractor = ContentExtractor::new().unwrap();

        let content = r#"
        <div class="bg-blue-500 not-a-tailwind-class component-class">
        "#;

        let classes = extractor.extract_classes_from_content(content);

        assert!(classes.contains("bg-blue-500"));
        assert!(!classes.contains("not-a-tailwind-class"));
        assert!(!classes.contains("component-class"));
    }
}
