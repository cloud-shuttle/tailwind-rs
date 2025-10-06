//! Configuration Handling for PostCSS Plugin
//!
//! Manages configuration loading, merging, and validation for the PostCSS plugin.

use crate::{PostCssError, PostCssOptions, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// PostCSS-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCssConfig {
    /// Content file patterns to scan
    pub content: Vec<String>,
    /// Safelist patterns (classes that should never be purged)
    pub safelist: Vec<String>,
    /// Blocklist patterns (classes that should always be purged)
    pub blocklist: Vec<String>,
    /// Theme configuration
    pub theme: Option<ThemeConfig>,
    /// Plugin configurations
    pub plugins: Vec<PluginConfig>,
    /// Dark mode configuration
    pub dark_mode: Option<String>,
    /// Enable debug mode
    pub debug: bool,
    /// Enable minification
    pub minify: bool,
    /// Configuration source
    pub source: ConfigSource,
}

/// Configuration source type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigSource {
    /// Configuration from PostCSS options
    Options,
    /// Configuration from JavaScript config file
    JavaScript(PathBuf),
    /// Configuration from TypeScript config file
    TypeScript(PathBuf),
    /// Configuration from TOML file
    Toml(PathBuf),
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Custom colors
    pub colors: Option<HashMap<String, String>>,
    /// Custom spacing
    pub spacing: Option<HashMap<String, String>>,
    /// Custom fonts
    pub font_family: Option<HashMap<String, String>>,
    /// Screen breakpoints
    pub screens: Option<HashMap<String, String>>,
    /// Extend existing theme
    pub extend: Option<ThemeExtension>,
}

/// Theme extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeExtension {
    /// Extended colors
    pub colors: Option<HashMap<String, String>>,
    /// Extended spacing
    pub spacing: Option<HashMap<String, String>>,
    /// Extended fonts
    pub font_family: Option<HashMap<String, String>>,
    /// Extended screens
    pub screens: Option<HashMap<String, String>>,
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginConfig {
    /// Plugin name (for built-in plugins)
    Name(String),
    /// Plugin configuration object
    Config(HashMap<String, serde_json::Value>),
}

impl PostCssConfig {
    /// Create configuration from PostCSS options
    pub fn from_options(options: PostCssOptions) -> Result<Self> {
        let content = options.content.unwrap_or_else(|| vec![
            "./src/**/*.{html,js,ts,jsx,tsx}".to_string(),
            "./public/index.html".to_string(),
        ]);

        let safelist = options.safelist.unwrap_or_default();
        let blocklist = options.blocklist.unwrap_or_default();

        // Load additional config if specified
        let mut config = if let Some(config_path) = &options.config {
            Self::load_from_file(config_path)?
        } else {
            Self::load_from_defaults()?
        };

        // Override with options
        config.content = content;
        config.safelist = safelist;
        config.blocklist = blocklist;
        config.debug = options.debug;
        config.minify = options.minify;

        // Merge theme if provided
        if let Some(theme) = options.theme {
            // In a real implementation, this would merge with existing theme
            println!("Theme configuration provided but not yet implemented");
        }

        // Set source
        config.source = ConfigSource::Options;

        Ok(config)
    }

    /// Load configuration from a file
    pub fn load_from_file(path: &str) -> Result<Self> {
        let path_buf = PathBuf::from(path);

        if !path_buf.exists() {
            return Err(PostCssError::Config(format!("Configuration file not found: {}", path)));
        }

        // For now, return default config
        // In a real implementation, this would parse JS/TS/TOML files
        let mut config = Self::default();
        config.source = match path_buf.extension().and_then(|ext| ext.to_str()) {
            Some("js") => ConfigSource::JavaScript(path_buf),
            Some("ts") => ConfigSource::TypeScript(path_buf),
            Some("toml") => ConfigSource::Toml(path_buf),
            _ => ConfigSource::JavaScript(path_buf), // Default to JS
        };

        Ok(config)
    }

    /// Load default configuration
    pub fn load_from_defaults() -> Result<Self> {
        let default_paths = [
            "tailwind.config.js",
            "tailwind.config.ts",
            "tailwind.config.toml",
            "tailwind-rs.toml",
        ];

        for path in &default_paths {
            if std::path::Path::new(path).exists() {
                return Self::load_from_file(path);
            }
        }

        // No config file found, use defaults
        Ok(Self::default())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Basic validation
        if self.content.is_empty() {
            return Err(PostCssError::Config("No content patterns specified".to_string()));
        }

        // Validate content patterns
        for pattern in &self.content {
            if !pattern.contains('.') && !pattern.contains('*') {
                return Err(PostCssError::Config(format!("Invalid content pattern: {}", pattern)));
            }
        }

        Ok(())
    }

    /// Get merged theme configuration
    pub fn get_theme(&self) -> ThemeConfig {
        self.theme.clone().unwrap_or_default()
    }
}

impl Default for PostCssConfig {
    fn default() -> Self {
        Self {
            content: vec![
                "./src/**/*.{html,js,ts,jsx,tsx}".to_string(),
                "./public/index.html".to_string(),
            ],
            safelist: Vec::new(),
            blocklist: Vec::new(),
            theme: None,
            plugins: Vec::new(),
            dark_mode: Some("class".to_string()),
            debug: false,
            minify: false,
            source: ConfigSource::Options,
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            colors: None,
            spacing: None,
            font_family: None,
            screens: Some([
                ("sm".to_string(), "640px".to_string()),
                ("md".to_string(), "768px".to_string()),
                ("lg".to_string(), "1024px".to_string()),
                ("xl".to_string(), "1280px".to_string()),
                ("2xl".to_string(), "1536px".to_string()),
            ].iter().cloned().collect()),
            extend: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PostCssConfig::default();
        assert!(!config.content.is_empty());
        assert!(!config.debug);
        assert!(!config.minify);
    }

    #[test]
    fn test_config_validation() {
        let config = PostCssConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config_validation() {
        let mut config = PostCssConfig::default();
        config.content = vec!["invalid-pattern-without-extension".to_string()];

        // This should fail validation
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_from_options() {
        let options = PostCssOptions {
            content: Some(vec!["./custom/**/*.{js,ts}".to_string()]),
            debug: true,
            minify: true,
            ..Default::default()
        };

        let config = PostCssConfig::from_options(options).unwrap();
        assert_eq!(config.content, vec!["./custom/**/*.{js,ts}".to_string()]);
        assert!(config.debug);
        assert!(config.minify);
    }
}
