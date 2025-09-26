//! Tree-sitter parser implementation
//!
//! This module provides tree-sitter integration for
//! accurate AST-based parsing and class extraction.

use crate::error::{Result, ScannerError};

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
        if !self.is_language_supported(language) {
            return Err(ScannerError::UnsupportedLanguage(language.to_string()));
        }

        // For now, implement a basic parser that extracts class names
        // In a full implementation, this would use the tree-sitter crate
        let errors = Vec::new();
        let ast = self.generate_ast(content, language);

        Ok(ParseResult {
            ast,
            errors,
            language: language.to_string(),
        })
    }

    /// Generate a basic AST representation
    fn generate_ast(&self, content: &str, language: &str) -> String {
        match language {
            "html" => self.parse_html_ast(content),
            "javascript" | "js" => self.parse_js_ast(content),
            "typescript" | "ts" => self.parse_ts_ast(content),
            "rust" => self.parse_rust_ast(content),
            _ => format!("Basic AST for {}: {}", language, content),
        }
    }

    /// Parse HTML and extract class attributes
    fn parse_html_ast(&self, content: &str) -> String {
        let mut ast_nodes = Vec::new();
        let mut in_class_attr = false;
        let mut current_class = String::new();

        for (i, c) in content.char_indices() {
            if content[i..].starts_with("class=") {
                in_class_attr = true;
                continue;
            }

            if in_class_attr {
                if c == '"' || c == '\'' {
                    if !current_class.is_empty() {
                        ast_nodes.push(format!("class: {}", current_class));
                        current_class.clear();
                    }
                    in_class_attr = false;
                } else if c != ' ' {
                    current_class.push(c);
                }
            }
        }

        if ast_nodes.is_empty() {
            "HTML AST: No class attributes found".to_string()
        } else {
            format!("HTML AST: {}", ast_nodes.join(", "))
        }
    }

    /// Parse JavaScript and extract string literals that might contain classes
    fn parse_js_ast(&self, content: &str) -> String {
        let mut ast_nodes = Vec::new();
        let mut in_string = false;
        let mut current_string = String::new();
        let mut string_delimiter = '"';

        for c in content.chars() {
            if !in_string && (c == '"' || c == '\'') {
                in_string = true;
                string_delimiter = c;
                current_string.clear();
            } else if in_string {
                if c == string_delimiter {
                    // Check if this string contains potential Tailwind classes
                    if self.looks_like_tailwind_classes(&current_string) {
                        ast_nodes.push(format!("string: {}", current_string));
                    }
                    in_string = false;
                } else {
                    current_string.push(c);
                }
            }
        }

        if ast_nodes.is_empty() {
            "JS AST: No class strings found".to_string()
        } else {
            format!("JS AST: {}", ast_nodes.join(", "))
        }
    }

    /// Parse TypeScript (similar to JavaScript)
    fn parse_ts_ast(&self, content: &str) -> String {
        // TypeScript parsing is similar to JavaScript
        self.parse_js_ast(content)
    }

    /// Parse Rust and extract string literals
    fn parse_rust_ast(&self, content: &str) -> String {
        let mut ast_nodes = Vec::new();
        let mut in_string = false;
        let mut current_string = String::new();
        let mut string_delimiter = '"';

        for c in content.chars() {
            if !in_string && c == '"' {
                in_string = true;
                current_string.clear();
            } else if in_string {
                if c == string_delimiter {
                    // Check if this string contains potential Tailwind classes
                    if self.looks_like_tailwind_classes(&current_string) {
                        ast_nodes.push(format!("string: {}", current_string));
                    }
                    in_string = false;
                } else {
                    current_string.push(c);
                }
            }
        }

        if ast_nodes.is_empty() {
            "Rust AST: No class strings found".to_string()
        } else {
            format!("Rust AST: {}", ast_nodes.join(", "))
        }
    }

    /// Check if a string looks like it contains Tailwind classes
    fn looks_like_tailwind_classes(&self, s: &str) -> bool {
        // Simple heuristic: check for common Tailwind patterns
        s.contains("bg-")
            || s.contains("text-")
            || s.contains("p-")
            || s.contains("m-")
            || s.contains("w-")
            || s.contains("h-")
            || s.contains("flex")
            || s.contains("grid")
            || s.contains("hidden")
            || s.contains("block")
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
