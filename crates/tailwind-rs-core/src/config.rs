//! Configuration system for tailwind-rs

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
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(content: &str) -> Result<Self> {
        // Try TOML first, then JSON
        let trimmed = content.trim();
        if trimmed.starts_with('[')
            || trimmed.starts_with('#')
            || trimmed.starts_with("plugins")
            || trimmed.starts_with("custom")
        {
            // TOML format
            let config: TailwindConfigToml = toml::from_str(content).map_err(|e| {
                TailwindError::config(format!("Failed to parse TOML config: {}", e))
            })?;
            Ok(config.into())
        } else {
            // JSON format
            serde_json::from_str(content)
                .map_err(|e| TailwindError::config(format!("Failed to parse JSON config: {}", e)))
        }
    }

    /// Save configuration to a file
    pub fn save_to_file(&self, path: impl Into<PathBuf>) -> Result<()> {
        let path = path.into();
        let content = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let toml_config: TailwindConfigToml = self.clone().into();
            toml::to_string_pretty(&toml_config).map_err(|e| {
                TailwindError::config(format!("Failed to serialize TOML config: {}", e))
            })?
        } else {
            serde_json::to_string_pretty(self).map_err(|e| {
                TailwindError::config(format!("Failed to serialize JSON config: {}", e))
            })?
        };

        std::fs::write(&path, content).map_err(|e| {
            TailwindError::config(format!("Failed to write config file {:?}: {}", path, e))
        })?;

        Ok(())
    }

    /// Add a plugin
    pub fn add_plugin(&mut self, plugin: impl Into<String>) {
        self.plugins.push(plugin.into());
    }

    /// Remove a plugin
    pub fn remove_plugin(&mut self, plugin: &str) {
        self.plugins.retain(|p| p != plugin);
    }

    /// Set a custom configuration value
    pub fn set_custom(&mut self, key: impl Into<String>, value: serde_json::Value) {
        self.custom.insert(key.into(), value);
    }

    /// Get a custom configuration value
    pub fn get_custom(&self, key: &str) -> Option<&serde_json::Value> {
        self.custom.get(key)
    }
}

impl Default for TailwindConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Build configuration for tailwind-rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Input paths for source files
    pub input: Vec<String>,
    /// Output path for CSS file
    pub output: String,
    /// Watch mode for development
    pub watch: bool,
    /// Minify output CSS
    pub minify: bool,
    /// Source maps generation
    pub source_maps: bool,
    /// Purge unused CSS
    pub purge: bool,
    /// Additional CSS to include
    pub additional_css: Vec<String>,
    /// PostCSS plugins
    pub postcss_plugins: Vec<String>,
}

impl BuildConfig {
    /// Create a new build configuration
    pub fn new() -> Self {
        Self {
            input: vec!["src/**/*.rs".to_string()],
            output: "dist/styles.css".to_string(),
            watch: false,
            minify: false,
            source_maps: false,
            purge: true,
            additional_css: Vec::new(),
            postcss_plugins: Vec::new(),
        }
    }

    /// Add an input path
    pub fn add_input(&mut self, path: impl Into<String>) {
        self.input.push(path.into());
    }

    /// Set the output path
    pub fn set_output(&mut self, path: impl Into<String>) {
        self.output = path.into();
    }

    /// Enable watch mode
    pub fn enable_watch(&mut self) {
        self.watch = true;
    }

    /// Disable watch mode
    pub fn disable_watch(&mut self) {
        self.watch = false;
    }

    /// Enable minification
    pub fn enable_minify(&mut self) {
        self.minify = true;
    }

    /// Disable minification
    pub fn disable_minify(&mut self) {
        self.minify = false;
    }

    /// Enable source maps
    pub fn enable_source_maps(&mut self) {
        self.source_maps = true;
    }

    /// Disable source maps
    pub fn disable_source_maps(&mut self) {
        self.source_maps = false;
    }

    /// Enable CSS purging
    pub fn enable_purge(&mut self) {
        self.purge = true;
    }

    /// Disable CSS purging
    pub fn disable_purge(&mut self) {
        self.purge = false;
    }

    /// Add additional CSS
    pub fn add_additional_css(&mut self, css: impl Into<String>) {
        self.additional_css.push(css.into());
    }

    /// Add a PostCSS plugin
    pub fn add_postcss_plugin(&mut self, plugin: impl Into<String>) {
        self.postcss_plugins.push(plugin.into());
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// TOML-specific configuration structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TailwindConfigToml {
    #[serde(rename = "build")]
    pub build: BuildConfigToml,
    #[serde(rename = "theme")]
    pub theme: ThemeToml,
    #[serde(rename = "responsive")]
    pub responsive: ResponsiveConfigToml,
    #[serde(rename = "plugins")]
    pub plugins: Vec<String>,
    #[serde(rename = "custom")]
    pub custom: HashMap<String, toml::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BuildConfigToml {
    pub input: Vec<String>,
    pub output: String,
    pub watch: bool,
    pub minify: bool,
    pub source_maps: bool,
    pub purge: bool,
    pub additional_css: Vec<String>,
    pub postcss_plugins: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResponsiveConfigToml {
    pub breakpoints: HashMap<String, u32>,
    pub container_centering: bool,
    pub container_padding: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ThemeToml {
    pub name: String,
    pub colors: HashMap<String, String>,
    pub spacing: HashMap<String, String>,
    pub border_radius: HashMap<String, String>,
    pub box_shadows: HashMap<String, String>,
    pub custom: HashMap<String, toml::Value>,
}

impl From<TailwindConfigToml> for TailwindConfig {
    fn from(toml_config: TailwindConfigToml) -> Self {
        let mut theme = Theme::new(toml_config.theme.name);

        // Convert colors
        for (name, value) in toml_config.theme.colors {
            theme.add_color(name, crate::theme::Color::hex(value));
        }

        // Convert spacing
        for (name, value) in toml_config.theme.spacing {
            theme.add_spacing(
                name,
                crate::theme::Spacing::rem(value.parse().unwrap_or(1.0)),
            );
        }

        // Convert border radius
        for (name, value) in toml_config.theme.border_radius {
            theme.add_border_radius(
                name,
                crate::theme::BorderRadius::rem(value.parse().unwrap_or(0.0)),
            );
        }

        // Convert box shadows
        for (name, _value) in toml_config.theme.box_shadows {
            theme.add_box_shadow(
                name,
                crate::theme::BoxShadow::new(
                    0.0,
                    1.0,
                    2.0,
                    0.0,
                    crate::theme::Color::hex("#000000"),
                    false,
                ),
            );
        }

        let responsive = ResponsiveConfig::new();
        // TODO: Fix responsive config mapping after refactoring
        // responsive.breakpoints = toml_config.responsive.breakpoints;
        // responsive.container_centering = toml_config.responsive.container_centering;
        // responsive.container_padding =
        //     crate::responsive::ResponsiveValue::new(toml_config.responsive.container_padding);

        Self {
            build: BuildConfig {
                input: toml_config.build.input,
                output: toml_config.build.output,
                watch: toml_config.build.watch,
                minify: toml_config.build.minify,
                source_maps: toml_config.build.source_maps,
                purge: toml_config.build.purge,
                additional_css: toml_config.build.additional_css,
                postcss_plugins: toml_config.build.postcss_plugins,
            },
            theme,
            responsive,
            plugins: toml_config.plugins,
            custom: HashMap::new(), // TODO: Convert TOML values to JSON values
        }
    }
}

impl From<TailwindConfig> for TailwindConfigToml {
    fn from(config: TailwindConfig) -> Self {
        let mut theme_colors = HashMap::new();
        for (name, color) in config.theme.colors {
            theme_colors.insert(name, color.to_css());
        }

        let mut theme_spacing = HashMap::new();
        for (name, spacing) in config.theme.spacing {
            theme_spacing.insert(name, spacing.to_css());
        }

        let mut theme_border_radius = HashMap::new();
        for (name, radius) in config.theme.border_radius {
            theme_border_radius.insert(name, radius.to_css());
        }

        let mut theme_box_shadows = HashMap::new();
        for (name, shadow) in config.theme.box_shadows {
            theme_box_shadows.insert(name, shadow.to_css());
        }

        Self {
            build: BuildConfigToml {
                input: config.build.input,
                output: config.build.output,
                watch: config.build.watch,
                minify: config.build.minify,
                source_maps: config.build.source_maps,
                purge: config.build.purge,
                additional_css: config.build.additional_css,
                postcss_plugins: config.build.postcss_plugins,
            },
            theme: ThemeToml {
                name: config.theme.name,
                colors: theme_colors,
                spacing: theme_spacing,
                border_radius: theme_border_radius,
                box_shadows: theme_box_shadows,
                custom: HashMap::new(), // TODO: Convert JSON values to TOML values
            },
            responsive: ResponsiveConfigToml {
                // TODO: Fix responsive config mapping after refactoring
                breakpoints: HashMap::new(),
                container_centering: false,
                container_padding: 0,
            },
            plugins: config.plugins,
            custom: HashMap::new(), // TODO: Convert JSON values to TOML values
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tailwind_config_creation() {
        let config = TailwindConfig::new();
        assert_eq!(config.build.input, vec!["src/**/*.rs"]);
        assert_eq!(config.build.output, "dist/styles.css");
        assert!(!config.build.watch);
        assert!(!config.build.minify);
        assert!(!config.build.source_maps);
        assert!(config.build.purge);
    }

    #[test]
    fn test_build_config_methods() {
        let mut config = BuildConfig::new();

        config.add_input("examples/**/*.rs");
        assert!(config.input.contains(&"examples/**/*.rs".to_string()));

        config.set_output("public/css/styles.css");
        assert_eq!(config.output, "public/css/styles.css");

        config.enable_watch();
        assert!(config.watch);

        config.enable_minify();
        assert!(config.minify);

        config.enable_source_maps();
        assert!(config.source_maps);

        config.disable_purge();
        assert!(!config.purge);
    }

    #[test]
    fn test_tailwind_config_plugins() {
        let mut config = TailwindConfig::new();

        config.add_plugin("tailwindcss-forms");
        config.add_plugin("tailwindcss-typography");

        assert_eq!(config.plugins.len(), 2);
        assert!(config.plugins.contains(&"tailwindcss-forms".to_string()));
        assert!(
            config
                .plugins
                .contains(&"tailwindcss-typography".to_string())
        );

        config.remove_plugin("tailwindcss-forms");
        assert_eq!(config.plugins.len(), 1);
        assert!(!config.plugins.contains(&"tailwindcss-forms".to_string()));
        assert!(
            config
                .plugins
                .contains(&"tailwindcss-typography".to_string())
        );
    }

    #[test]
    fn test_tailwind_config_custom() {
        let mut config = TailwindConfig::new();

        config.set_custom("custom_key", serde_json::json!("custom_value"));
        assert_eq!(
            config.get_custom("custom_key"),
            Some(&serde_json::json!("custom_value"))
        );
        assert_eq!(config.get_custom("nonexistent"), None);
    }

    #[test]
    fn test_config_from_str_json() {
        let json_config = r#"{
            "build": {
                "input": ["src/**/*.rs"],
                "output": "dist/styles.css",
                "watch": false,
                "minify": false,
                "source_maps": false,
                "purge": true,
                "additional_css": [],
                "postcss_plugins": []
            },
            "theme": {
                "name": "default",
                "colors": {},
                "spacing": {},
                "border_radius": {},
                "box_shadows": {},
                "custom": {}
            },
            "responsive": {
                "breakpoints": {
                    "Sm": {
                        "min_width": 640,
                        "max_width": null,
                        "enabled": true,
                        "media_query": null
                    },
                    "Md": {
                        "min_width": 768,
                        "max_width": null,
                        "enabled": true,
                        "media_query": null
                    },
                    "Lg": {
                        "min_width": 1024,
                        "max_width": null,
                        "enabled": true,
                        "media_query": null
                    },
                    "Xl": {
                        "min_width": 1280,
                        "max_width": null,
                        "enabled": true,
                        "media_query": null
                    },
                    "Xl2": {
                        "min_width": 1536,
                        "max_width": null,
                        "enabled": true,
                        "media_query": null
                    }
                },
                "container_centering": true,
                "container_padding": {
                    "Base": 16
                },
                "defaults": {
                    "default_breakpoint": "Base",
                    "include_base": true,
                    "mobile_first": true
                }
            },
            "plugins": [],
            "custom": {}
        }"#;

        let config = TailwindConfig::from_str(json_config).unwrap();
        assert_eq!(config.build.output, "dist/styles.css");
        assert_eq!(config.theme.name, "default");
    }

    #[test]
    fn test_config_from_str_toml() {
        let toml_config = r#"plugins = []
custom = {}

[build]
input = ["src/**/*.rs"]
output = "dist/styles.css"
watch = false
minify = false
source_maps = false
purge = true
additional_css = []
postcss_plugins = []

[theme]
name = "default"
colors = {}
spacing = {}
border_radius = {}
box_shadows = {}
custom = {}

[responsive]
breakpoints = { sm = 640, md = 768, lg = 1024, xl = 1280, "2xl" = 1536 }
container_centering = true
container_padding = 16
"#;

        let config = TailwindConfig::from_str(toml_config).unwrap();
        assert_eq!(config.build.output, "dist/styles.css");
        assert_eq!(config.theme.name, "default");
    }
}
