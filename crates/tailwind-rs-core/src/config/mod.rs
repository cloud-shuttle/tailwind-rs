//! Configuration system for tailwind-rs
//!
//! This module provides configuration management for the tailwind-rs system,
//! including build settings, theme configuration, and responsive breakpoints.

pub mod build;
pub mod parser;
pub mod toml_config;

// Re-export main types
pub use build::BuildConfig;
pub use toml_config::TailwindConfigToml;

use crate::error::{Result, TailwindError};
use crate::responsive::ResponsiveConfig;
use crate::theme::Theme;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main configuration for tailwind-rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TailwindConfig {
    /// Build configuration
    pub build: BuildConfig,
    /// Theme configuration
    pub theme: Theme,
    /// Responsive configuration
    pub responsive: ResponsiveConfig,
    /// Plugin configuration
    pub plugins: Vec<String>,
    /// Custom configuration
    pub custom: HashMap<String, serde_json::Value>,
}

impl TailwindConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self {
            build: BuildConfig::new(),
            theme: crate::theme::create_default_theme(),
            responsive: ResponsiveConfig::new(),
            plugins: Vec::new(),
            custom: HashMap::new(),
        }
    }

    /// Load configuration from a file
    pub fn from_file(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let content = std::fs::read_to_string(&path).map_err(|e| {
            TailwindError::config(format!("Failed to read config file {:?}: {}", path, e))
        })?;

        Self::from_str(&content)
    }

    /// Load configuration from a string
    pub fn from_str(content: &str) -> Result<Self> {
        // Try TOML first, then JSON
        if content.trim().starts_with('[') || content.trim().starts_with('#') {
            let toml_config: TailwindConfigToml = toml::from_str(content)
                .map_err(|e| TailwindError::config(format!("TOML parsing error: {}", e)))?;
            Ok(toml_config.into())
        } else {
            serde_json::from_str(content)
                .map_err(|e| TailwindError::config(format!("JSON parsing error: {}", e)))
        }
    }

    /// Save configuration to a file
    pub fn save_to_file(&self, path: impl Into<PathBuf>) -> Result<()> {
        let path = path.into();
        let content = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let toml_config: TailwindConfigToml = self.clone().into();
            toml::to_string_pretty(&toml_config)
                .map_err(|e| TailwindError::config(format!("TOML serialization error: {}", e)))?
        } else {
            serde_json::to_string_pretty(self)
                .map_err(|e| TailwindError::config(format!("JSON serialization error: {}", e)))?
        };

        std::fs::write(&path, content).map_err(|e| {
            TailwindError::config(format!("Failed to write config file {:?}: {}", path, e))
        })?;

        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Basic validation
        if self.build.output.is_empty() {
            return Err(TailwindError::config(
                "Build output path cannot be empty".to_string(),
            ));
        }

        if self.build.input.is_empty() {
            return Err(TailwindError::config(
                "Build input paths cannot be empty".to_string(),
            ));
        }

        // Validate theme
        self.theme.validate()?;

        // Validate responsive config
        self.responsive.validate()?;

        Ok(())
    }

    /// Convert TOML values to JSON values
    fn convert_toml_to_json_values(
        toml_values: HashMap<String, toml::Value>,
    ) -> HashMap<String, serde_json::Value> {
        let mut json_values = HashMap::new();
        for (key, value) in toml_values {
            match value {
                toml::Value::String(s) => {
                    json_values.insert(key.clone(), serde_json::Value::String(s));
                }
                toml::Value::Integer(i) => {
                    json_values.insert(key.clone(), serde_json::Value::Number(i.into()));
                }
                toml::Value::Float(f) => {
                    json_values.insert(
                        key,
                        serde_json::Value::Number(
                            serde_json::Number::from_f64(f).unwrap_or(serde_json::Number::from(0)),
                        ),
                    );
                }
                toml::Value::Boolean(b) => {
                    json_values.insert(key, serde_json::Value::Bool(b));
                }
                _ => {} // Skip complex types for now
            }
        }
        json_values
    }

    /// Convert JSON values to TOML values
    fn convert_json_to_toml_values(
        json_values: &HashMap<String, serde_json::Value>,
    ) -> HashMap<String, toml::Value> {
        let mut toml_values = HashMap::new();
        for (key, value) in json_values {
            match value {
                serde_json::Value::String(s) => {
                    toml_values.insert(key.clone(), toml::Value::String(s.clone()));
                }
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        toml_values.insert(key.clone(), toml::Value::Integer(i));
                    } else if let Some(f) = n.as_f64() {
                        toml_values.insert(key.clone(), toml::Value::Float(f));
                    }
                }
                serde_json::Value::Bool(b) => {
                    toml_values.insert(key.clone(), toml::Value::Boolean(*b));
                }
                _ => {} // Skip complex types for now
            }
        }
        toml_values
    }

    /// Convert breakpoints to TOML format
    fn convert_breakpoints_to_toml(
        breakpoints: &HashMap<
            crate::responsive::Breakpoint,
            crate::responsive::responsive_config::BreakpointConfig,
        >,
    ) -> HashMap<String, u32> {
        let mut toml_breakpoints = HashMap::new();
        for (breakpoint, config) in breakpoints {
            toml_breakpoints.insert(breakpoint.to_string().to_lowercase(), config.min_width);
        }
        toml_breakpoints
    }
}

impl Default for TailwindConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TailwindConfigToml> for TailwindConfig {
    fn from(toml_config: TailwindConfigToml) -> Self {
        Self {
            build: toml_config.build.into(),
            theme: toml_config.theme.into(),
            responsive: toml_config.responsive.into(),
            plugins: toml_config.plugins.unwrap_or_default(),
            custom: Self::convert_toml_to_json_values(toml_config.custom.unwrap_or_default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = TailwindConfig::new();
        assert!(!config.build.input.is_empty());
        assert!(!config.build.output.is_empty());
    }

    #[test]
    fn test_config_validation() {
        let mut config = TailwindConfig::new();
        assert!(config.validate().is_ok());

        config.build.output = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_toml_parsing() {
        let toml_content = r#"
[build]
input = ["src/**/*.rs"]
output = "dist/styles.css"
minify = true

[theme]
name = "default"

[responsive]
breakpoints = { sm = 640, md = 768 }
container_centering = true
container_padding = 16
"#;

        let config = TailwindConfig::from_str(toml_content).unwrap();
        assert_eq!(config.build.output, "dist/styles.css");
        assert!(config.build.minify);
    }
}
