//! Class extraction implementation
//!
//! This module provides class extraction capabilities for different
//! file types and languages.

use crate::file_scanner::{FileInfo, FileType};
use crate::content_config::ExtractionRule;
use crate::error::{ScannerError, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Class extractor for different file types
#[derive(Debug)]
pub struct ClassExtractor {
    /// Compiled regex patterns for different file types
    patterns: HashMap<FileType, Vec<Regex>>,
    /// Custom extraction rules
    custom_rules: Vec<ExtractionRule>,
}

/// Extracted class with context
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtractedClass {
    /// Class name
    pub class_name: String,
    /// Context information
    pub context: ClassContext,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
}

/// Class context information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassContext {
    /// Surrounding code context
    pub code_context: String,
    /// File type
    pub file_type: String,
    /// Whether class is in a string literal
    pub in_string: bool,
    /// Whether class is in a comment
    pub in_comment: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ClassContext {
    /// Create a new class context
    pub fn new() -> Self {
        Self {
            code_context: String::new(),
            file_type: String::new(),
            in_string: false,
            in_comment: false,
            metadata: HashMap::new(),
        }
    }

    /// Create with file type
    pub fn with_file_type(file_type: &str) -> Self {
        Self {
            code_context: String::new(),
            file_type: file_type.to_string(),
            in_string: false,
            in_comment: false,
            metadata: HashMap::new(),
        }
    }
}

impl Default for ClassContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassExtractor {
    /// Create a new class extractor
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // Initialize patterns for different file types
        Self::initialize_patterns(&mut patterns);
        
        Self {
            patterns,
            custom_rules: Vec::new(),
        }
    }

    /// Initialize regex patterns for different file types
    fn initialize_patterns(patterns: &mut HashMap<FileType, Vec<Regex>>) {
        // Rust patterns
        let rust_patterns = vec![
            Regex::new(r#"class\s*=\s*"([^"]+)""#).unwrap(),
            Regex::new(r#"class\s*=\s*'([^']+)'"#).unwrap(),
            Regex::new(r#"class\s*=\s*\{([^}]+)\}"#).unwrap(),
        ];
        patterns.insert(FileType::Rust, rust_patterns);

        // JavaScript/TypeScript patterns
        let js_patterns = vec![
            Regex::new(r#"className\s*=\s*"([^"]+)""#).unwrap(),
            Regex::new(r#"className\s*=\s*'([^']+)'"#).unwrap(),
            Regex::new(r#"className\s*=\s*\{([^}]+)\}"#).unwrap(),
            Regex::new(r#"class\s*=\s*"([^"]+)""#).unwrap(),
            Regex::new(r#"class\s*=\s*'([^']+)'"#).unwrap(),
        ];
        patterns.insert(FileType::JavaScript, js_patterns.clone());
        patterns.insert(FileType::TypeScript, js_patterns);

        // HTML patterns
        let html_patterns = vec![
            Regex::new(r#"class\s*=\s*"([^"]+)""#).unwrap(),
            Regex::new(r#"class\s*=\s*'([^']+)'"#).unwrap(),
        ];
        patterns.insert(FileType::Html, html_patterns);

        // Vue patterns
        let vue_patterns = vec![
            Regex::new(r#"class\s*=\s*"([^"]+)""#).unwrap(),
            Regex::new(r#":class\s*=\s*"([^"]+)""#).unwrap(),
            Regex::new(r#"v-bind:class\s*=\s*"([^"]+)""#).unwrap(),
        ];
        patterns.insert(FileType::Vue, vue_patterns);

        // Svelte patterns
        let svelte_patterns = vec![
            Regex::new(r#"class\s*=\s*"([^"]+)""#).unwrap(),
            Regex::new(r#":class\s*=\s*"([^"]+)""#).unwrap(),
        ];
        patterns.insert(FileType::Svelte, svelte_patterns);

        // CSS patterns
        let css_patterns = vec![
            Regex::new(r#"@apply\s+([^;]+);"#).unwrap(),
        ];
        patterns.insert(FileType::Css, css_patterns.clone());
        patterns.insert(FileType::Scss, css_patterns.clone());
        patterns.insert(FileType::Less, css_patterns);
    }

    /// Extract classes from file information
    pub async fn extract_classes(&self, file_info: &FileInfo) -> Result<Vec<ExtractedClass>> {
        let content = file_info.content.as_ref()
            .ok_or_else(|| ScannerError::FileContentNotAvailable("File content not available".to_string()))?;
        
        let mut classes = Vec::new();
        
        // Extract using file type patterns
        if let Some(patterns) = self.patterns.get(&file_info.file_type) {
            for pattern in patterns {
                let matches = self.extract_with_pattern(content, pattern, &file_info.file_type);
                classes.extend(matches);
            }
        }
        
        // Extract using custom rules
        for rule in &self.custom_rules {
            if self.rule_applies_to_file(rule, &file_info.path) {
                if let Ok(pattern) = Regex::new(&rule.class_pattern) {
                    let matches = self.extract_with_pattern(content, &pattern, &file_info.file_type);
                    classes.extend(matches);
                }
            }
        }
        
        Ok(classes)
    }

    /// Extract classes using a specific pattern
    fn extract_with_pattern(
        &self,
        content: &str,
        pattern: &Regex,
        file_type: &FileType,
    ) -> Vec<ExtractedClass> {
        let mut classes = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            for mat in pattern.find_iter(line) {
                let class_name = mat.as_str().to_string();
                let context = ClassContext {
                    code_context: line.to_string(),
                    file_type: format!("{:?}", file_type),
                    in_string: self.is_in_string(line, mat.start()),
                    in_comment: self.is_in_comment(line, mat.start()),
                    metadata: HashMap::new(),
                };
                
                classes.push(ExtractedClass {
                    class_name,
                    context,
                    line: line_num + 1,
                    column: mat.start() + 1,
                });
            }
        }
        
        classes
    }

    /// Check if position is inside a string literal
    fn is_in_string(&self, line: &str, position: usize) -> bool {
        let before = &line[..position];
        let single_quotes = before.matches('\'').count();
        let double_quotes = before.matches('"').count();
        let backticks = before.matches('`').count();
        
        (single_quotes % 2 == 1) || (double_quotes % 2 == 1) || (backticks % 2 == 1)
    }

    /// Check if position is inside a comment
    fn is_in_comment(&self, line: &str, position: usize) -> bool {
        let before = &line[..position];
        before.contains("//") || before.contains("/*")
    }

    /// Check if a rule applies to a file
    fn rule_applies_to_file(&self, rule: &ExtractionRule, file_path: &std::path::Path) -> bool {
        // Simple pattern matching - in a real implementation, use proper glob matching
        let path_str = file_path.to_string_lossy();
        path_str.contains(&rule.file_pattern)
    }

    /// Add a custom extraction rule
    pub fn add_custom_rule(&mut self, rule: ExtractionRule) -> Result<()> {
        // Validate the regex pattern
        Regex::new(&rule.class_pattern)?;
        self.custom_rules.push(rule);
        Ok(())
    }

    /// Get all custom rules
    pub fn get_custom_rules(&self) -> &[ExtractionRule] {
        &self.custom_rules
    }

    /// Clear all custom rules
    pub fn clear_custom_rules(&mut self) {
        self.custom_rules.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_class_extractor_creation() {
        let extractor = ClassExtractor::new();
        assert!(!extractor.patterns.is_empty());
    }

    #[test]
    fn test_class_context_creation() {
        let context = ClassContext::new();
        assert!(context.code_context.is_empty());
        assert!(!context.in_string);
        assert!(!context.in_comment);
    }

    #[test]
    fn test_class_context_with_file_type() {
        let context = ClassContext::with_file_type("rust");
        assert_eq!(context.file_type, "rust");
    }

    #[test]
    fn test_extracted_class_creation() {
        let class = ExtractedClass {
            class_name: "p-4".to_string(),
            context: ClassContext::new(),
            line: 1,
            column: 1,
        };
        
        assert_eq!(class.class_name, "p-4");
        assert_eq!(class.line, 1);
        assert_eq!(class.column, 1);
    }

    #[test]
    fn test_string_detection() {
        let extractor = ClassExtractor::new();
        let line = r#"let class = "p-4 bg-blue-500";"#;
        
        assert!(extractor.is_in_string(line, 20));
        assert!(!extractor.is_in_string(line, 5));
    }

    #[test]
    fn test_comment_detection() {
        let extractor = ClassExtractor::new();
        let line = r#"// This is a comment"#;
        
        assert!(extractor.is_in_comment(line, 10));
    }

    #[test]
    fn test_custom_rule_addition() {
        let mut extractor = ClassExtractor::new();
        let rule = ExtractionRule::new(
            "test_rule",
            "**/*.test",
            r#"test\s*=\s*"([^"]+)""#,
        );
        
        let result = extractor.add_custom_rule(rule);
        assert!(result.is_ok());
        assert_eq!(extractor.custom_rules.len(), 1);
    }
}
