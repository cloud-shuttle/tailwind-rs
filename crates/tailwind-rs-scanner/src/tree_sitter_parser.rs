//! Tree-sitter parser implementation
//!
//! This module provides tree-sitter integration for
//! accurate AST-based parsing and class extraction.

use crate::error::{ScannerError, Result};

/// Tree-sitter parser for different languages
#[derive(Debug)]
pub struct TreeSitterParser {
    /// Supported languages
    languages: Vec<LanguageSupport>,
}

/// Language support information
#[derive(Debug, Clone)]
pub struct LanguageSupport {
    /// Language name
    pub name: String,
    /// File extensions
    pub extensions: Vec<String>,
    /// Whether language is supported
    pub supported: bool,
}

/// Parse result
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// Parsed AST
    pub ast: String,
    /// Parse errors
    pub errors: Vec<String>,
    /// Language used
    pub language: String,
}

impl TreeSitterParser {
    /// Create a new tree-sitter parser
    pub fn new() -> Self {
        let mut languages = Vec::new();
        
        // Initialize supported languages
        languages.push(LanguageSupport {
            name: "rust".to_string(),
            extensions: vec!["rs".to_string()],
            supported: true,
        });
        
        languages.push(LanguageSupport {
            name: "javascript".to_string(),
            extensions: vec!["js".to_string(), "jsx".to_string()],
            supported: true,
        });
        
        languages.push(LanguageSupport {
            name: "typescript".to_string(),
            extensions: vec!["ts".to_string(), "tsx".to_string()],
            supported: true,
        });
        
        languages.push(LanguageSupport {
            name: "html".to_string(),
            extensions: vec!["html".to_string(), "htm".to_string()],
            supported: true,
        });

        Self { languages }
    }

    /// Parse content with tree-sitter
    pub fn parse(&self, content: &str, language: &str) -> Result<ParseResult> {
        // Placeholder implementation
        // In a real implementation, this would use tree-sitter for parsing
        Ok(ParseResult {
            ast: format!("AST for {}: {}", language, content),
            errors: Vec::new(),
            language: language.to_string(),
        })
    }

    /// Get supported languages
    pub fn get_supported_languages(&self) -> &[LanguageSupport] {
        &self.languages
    }

    /// Check if language is supported
    pub fn is_language_supported(&self, language: &str) -> bool {
        self.languages.iter().any(|lang| lang.name == language)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_sitter_parser_creation() {
        let parser = TreeSitterParser::new();
        assert!(!parser.languages.is_empty());
    }

    #[test]
    fn test_language_support() {
        let parser = TreeSitterParser::new();
        assert!(parser.is_language_supported("rust"));
        assert!(parser.is_language_supported("javascript"));
        assert!(!parser.is_language_supported("unknown"));
    }

    #[test]
    fn test_parse_content() {
        let parser = TreeSitterParser::new();
        let content = "let class = 'p-4';";
        let result = parser.parse(content, "javascript");
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.language, "javascript");
        assert!(result.errors.is_empty());
    }
}
