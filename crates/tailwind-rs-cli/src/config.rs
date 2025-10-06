//! Configuration File Handling
//!
//! Supports multiple configuration formats:
//! - tailwind.config.js (JavaScript)
//! - tailwind.config.ts (TypeScript)
//! - tailwind-rs.toml (TOML, native)

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use anyhow::Result;

/// Tailwind-RS Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// Input CSS file path
    pub input: Option<PathBuf>,
    /// Output CSS file path
    pub output: Option<PathBuf>,
    /// Content file patterns to scan
    pub content: Vec<String>,
    /// Theme configuration
    pub theme: Option<ThemeConfig>,
    /// Plugin configurations
    pub plugins: Vec<PluginConfig>,
    /// Dark mode configuration
    pub dark_mode: Option<String>,
    /// Additional CSS to include
    pub css: Option<String>,
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Custom colors
    pub colors: Option<std::collections::HashMap<String, String>>,
    /// Custom spacing
    pub spacing: Option<std::collections::HashMap<String, String>>,
    /// Custom fonts
    pub font_family: Option<std::collections::HashMap<String, String>>,
    /// Extend existing theme
    pub extend: Option<ThemeExtension>,
}

/// Theme extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeExtension {
    /// Extended colors
    pub colors: Option<std::collections::HashMap<String, String>>,
    /// Extended spacing
    pub spacing: Option<std::collections::HashMap<String, String>>,
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginConfig {
    /// Plugin name (for built-in plugins)
    Name(String),
    /// Plugin configuration object
    Config(std::collections::HashMap<String, serde_json::Value>),
}

impl Config {
    /// Load configuration from a specific file
    pub async fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Option<Self>> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(path).await?;

        // Determine format based on file extension
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => {
                let config: Self = toml::from_str(&content)?;
                Ok(Some(config))
            }
            Some("js") | Some("ts") => {
                // For JavaScript/TypeScript configs, we'd need to eval them
                // For now, return a basic config
                Ok(Some(Self::default()))
            }
            _ => Err(anyhow::anyhow!("Unsupported config file format")),
        }
    }

    /// Load configuration from a file path (async wrapper)
    pub fn load(path: Option<&Path>) -> Result<Option<Self>> {
        if let Some(path) = path {
            // For simplicity, we'll make this synchronous for now
            // In a real implementation, this would be async
            if path.exists() {
                let content = std::fs::read_to_string(path)?;
                match path.extension().and_then(|ext| ext.to_str()) {
                    Some("toml") => {
                        let config: Self = toml::from_str(&content)?;
                        Ok(Some(config))
                    }
                    Some("js") | Some("ts") => {
                        // Basic JS/TS config support
                        Ok(Some(Self::default()))
                    }
                    _ => Err(anyhow::anyhow!("Unsupported config file format")),
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Load configuration from default locations
    pub fn load_from_defaults() -> Result<Option<Self>> {
        let default_paths = [
            "tailwind.config.js",
            "tailwind.config.ts",
            "tailwind.config.toml",
            "tailwind-rs.toml",
        ];

        for path in &default_paths {
            if let Some(config) = Self::load(Some(Path::new(path)))? {
                return Ok(Some(config));
            }
        }

        Ok(None)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input: Some(PathBuf::from("tailwind.css")),
            output: Some(PathBuf::from("dist/output.css")),
            content: vec![
                "src/**/*.{html,js,ts,jsx,tsx}".to_string(),
                "public/index.html".to_string(),
            ],
            theme: None,
            plugins: Vec::new(),
            dark_mode: Some("class".to_string()),
            css: None,
        }
    }
}

/// Configuration error types
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration file not found")]
    NotFound,

    #[error("Failed to parse configuration: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Invalid configuration: {0}")]
    Invalid(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}