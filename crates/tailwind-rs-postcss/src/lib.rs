//! PostCSS Plugin for Tailwind-RS
//!
//! This crate provides a PostCSS plugin interface for Tailwind-RS,
//! enabling seamless integration with existing build toolchains like
//! Webpack, Vite, Rollup, and other PostCSS-compatible tools.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod plugin;
pub mod cache;
pub mod config;

// Re-export main types for easier access
pub use plugin::TailwindRsPlugin;
pub use config::{PostCssConfig, ConfigSource};

/// Error types for PostCSS operations
#[derive(Debug, thiserror::Error)]
pub enum PostCssError {
    #[error("CSS processing error: {0}")]
    Processing(String),

    #[error("Content extraction error: {0}")]
    Extraction(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Glob pattern error: {0}")]
    Glob(#[from] glob::PatternError),

    #[error("Plugin error: {0}")]
    Plugin(String),
}

/// Result type alias for PostCSS operations
pub type Result<T> = std::result::Result<T, PostCssError>;

/// CSS AST representation (simplified for PostCSS integration)
#[derive(Debug, Clone)]
pub struct CssAst {
    pub rules: Vec<CssRule>,
    pub at_rules: Vec<AtRule>,
}

#[derive(Debug, Clone)]
pub struct CssRule {
    pub selectors: Vec<String>,
    pub declarations: Vec<CssDeclaration>,
}

#[derive(Debug, Clone)]
pub struct CssDeclaration {
    pub property: String,
    pub value: String,
    pub important: bool,
}

#[derive(Debug, Clone)]
pub struct AtRule {
    pub name: String,
    pub params: String,
    pub rules: Vec<CssRule>,
}

/// CSS generation callback type
pub type CssGeneratorFn = Box<dyn Fn(&str) -> Result<String> + Send + Sync>;

/// Content extraction callback type
pub type ContentExtractorFn = Box<dyn Fn(&[String]) -> Result<std::collections::HashSet<String>> + Send + Sync>;

/// PostCSS plugin options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCssOptions {
    /// Content file patterns to scan
    pub content: Option<Vec<String>>,

    /// Configuration file path
    pub config: Option<String>,

    /// Theme configuration
    pub theme: Option<serde_json::Value>,

    /// Plugin configurations
    pub plugins: Option<Vec<serde_json::Value>>,

    /// Safelist patterns
    pub safelist: Option<Vec<String>>,

    /// Blocklist patterns
    pub blocklist: Option<Vec<String>>,

    /// Enable debug mode
    #[serde(default)]
    pub debug: bool,

    /// Enable minification
    #[serde(default)]
    pub minify: bool,
}

impl Default for PostCssOptions {
    fn default() -> Self {
        Self {
            content: Some(vec![
                "./src/**/*.{html,js,ts,jsx,tsx}".to_string(),
                "./public/index.html".to_string(),
            ]),
            config: None,
            theme: None,
            plugins: None,
            safelist: None,
            blocklist: None,
            debug: false,
            minify: false,
        }
    }
}

/// Plugin trait for extending PostCSS functionality
pub trait PostCssPlugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;

    /// Plugin version
    fn version(&self) -> &str;

    /// Process CSS AST
    fn process_css(&self, ast: &mut CssAst, config: &PostCssConfig) -> Result<()> {
        Ok(())
    }

    /// Add custom utilities
    fn add_utilities(&self, utilities: &mut HashMap<String, Vec<CssDeclaration>>) -> Result<()> {
        Ok(())
    }

    /// Extract additional content
    fn extract_content(&self, content: &mut Vec<String>) -> Result<()> {
        Ok(())
    }
}

/// Utility functions for PostCSS integration
pub mod utils {
    use super::*;

    /// Convert CSS AST to string
    pub fn css_ast_to_string(ast: &CssAst) -> String {
        let mut output = String::new();

        // Process at-rules first (like @tailwind directives)
        for at_rule in &ast.at_rules {
            output.push_str(&format!("@{} {} {{\n", at_rule.name, at_rule.params));

            for rule in &at_rule.rules {
                output.push_str(&css_rule_to_string(rule));
            }

            output.push_str("}\n\n");
        }

        // Process regular rules
        for rule in &ast.rules {
            output.push_str(&css_rule_to_string(rule));
        }

        output
    }

    /// Convert CSS rule to string
    pub fn css_rule_to_string(rule: &CssRule) -> String {
        if rule.selectors.is_empty() {
            return String::new();
        }

        let selectors = rule.selectors.join(", ");
        let mut output = format!("{} {{\n", selectors);

        for decl in &rule.declarations {
            let important = if decl.important { " !important" } else { "" };
            output.push_str(&format!("  {}: {}{};\n", decl.property, decl.value, important));
        }

        output.push_str("}\n\n");
        output
    }

    /// Parse CSS string into AST (basic implementation)
    pub fn parse_css_to_ast(css: &str) -> Result<CssAst> {
        let mut ast = CssAst {
            rules: Vec::new(),
            at_rules: Vec::new(),
        };

        // Basic CSS parsing (simplified)
        // In a real implementation, this would use a proper CSS parser

        for line in css.lines() {
            let line = line.trim();

            if line.starts_with("@tailwind") {
                // Handle @tailwind directives
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    // Strip trailing semicolon from parameter
                    let param = parts[1].trim_end_matches(';');
                    let at_rule = AtRule {
                        name: "tailwind".to_string(),
                        params: param.to_string(),
                        rules: Vec::new(), // Will be filled by processor
                    };
                    ast.at_rules.push(at_rule);
                }
            }
        }

        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_ast_to_string() {
        let mut ast = CssAst {
            rules: vec![
                CssRule {
                    selectors: vec![".bg-blue-500".to_string()],
                    declarations: vec![
                        CssDeclaration {
                            property: "background-color".to_string(),
                            value: "rgb(59, 130, 246)".to_string(),
                            important: false,
                        }
                    ],
                }
            ],
            at_rules: Vec::new(),
        };

        let css = utils::css_ast_to_string(&ast);
        assert!(css.contains(".bg-blue-500"));
        assert!(css.contains("background-color: rgb(59, 130, 246);"));
    }
}