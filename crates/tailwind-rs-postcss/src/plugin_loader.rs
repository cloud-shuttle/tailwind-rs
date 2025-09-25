//! Plugin loader for PostCSS integration
//!
//! This module provides plugin loading and management capabilities.

use crate::ast::CSSNode;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub name: String,
    pub version: Option<String>,
    pub options: serde_json::Value,
}

impl PluginConfig {
    /// Check if plugin requires JavaScript execution
    pub fn requires_js(&self) -> bool {
        // Check if plugin is a JavaScript plugin
        match self.name.as_str() {
            "autoprefixer" | "cssnano" | "postcss-preset-env" | "postcss-import" => true,
            _ => {
                // Check if it's an NPM package
                self.name.contains('/') || self.name.starts_with('@')
            }
        }
    }
}

/// Plugin loader for managing plugins
#[derive(Debug)]
pub struct PluginLoader;

/// Plugin execution result
#[derive(Debug)]
pub enum PluginResult {
    Native(NativePlugin),
    JavaScript(JSPlugin),
}

/// Native Rust plugin
#[derive(Debug)]
pub struct NativePlugin;

impl NativePlugin {
    pub fn transform(&self, ast: CSSNode) -> Result<CSSNode> {
        Ok(ast)
    }
}

/// JavaScript plugin
#[derive(Debug)]
pub struct JSPlugin {
    pub name: String,
}

impl PluginLoader {
    /// Create a new plugin loader
    pub fn new() -> Self {
        Self
    }
    
    /// Load a plugin
    pub async fn load_plugin(&self, config: &PluginConfig) -> Result<PluginResult> {
        if config.requires_js() {
            // Load JavaScript plugin
            Ok(PluginResult::JavaScript(JSPlugin {
                name: config.name.clone(),
            }))
        } else {
            // Load native Rust plugin
            Ok(PluginResult::Native(NativePlugin))
        }
    }
    
    /// Load multiple plugins
    pub async fn load_plugins(&self, configs: &[PluginConfig]) -> Result<Vec<PluginResult>> {
        let mut results = Vec::new();
        
        for config in configs {
            let result = self.load_plugin(config).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Validate plugin configuration
    pub fn validate_config(&self, config: &PluginConfig) -> Result<()> {
        if config.name.is_empty() {
            return Err(crate::error::PostCSSError::config("Plugin name cannot be empty"));
        }
        
        // Check for valid plugin names
        if !self.is_valid_plugin_name(&config.name) {
            return Err(crate::error::PostCSSError::config(
                &format!("Invalid plugin name: {}", config.name)
            ));
        }
        
        Ok(())
    }
    
    /// Check if plugin name is valid
    fn is_valid_plugin_name(&self, name: &str) -> bool {
        // Basic validation for plugin names
        !name.is_empty() && 
        name.len() <= 100 && 
        name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '/' || c == '@')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_loader_creation() {
        let _loader = PluginLoader::new();
        assert!(true); // Basic test
    }
}
