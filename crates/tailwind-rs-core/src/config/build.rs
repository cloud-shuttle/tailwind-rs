//! Build configuration for tailwind-rs

use serde::{Deserialize, Serialize};

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

    /// Set output path
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

    /// Add additional CSS file
    pub fn add_css(&mut self, path: impl Into<String>) {
        self.additional_css.push(path.into());
    }

    /// Add PostCSS plugin
    pub fn add_postcss_plugin(&mut self, plugin: impl Into<String>) {
        self.postcss_plugins.push(plugin.into());
    }

    /// Validate build configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.output.is_empty() {
            return Err("Output path cannot be empty".to_string());
        }

        if self.input.is_empty() {
            return Err("Input paths cannot be empty".to_string());
        }

        Ok(())
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl From<super::toml_config::BuildConfigToml> for BuildConfig {
    fn from(toml_config: super::toml_config::BuildConfigToml) -> Self {
        Self {
            input: toml_config
                .input
                .unwrap_or_else(|| vec!["src/**/*.rs".to_string()]),
            output: toml_config
                .output
                .unwrap_or_else(|| "dist/styles.css".to_string()),
            watch: toml_config.watch.unwrap_or(false),
            minify: toml_config.minify.unwrap_or(false),
            source_maps: toml_config.source_maps.unwrap_or(false),
            purge: toml_config.purge.unwrap_or(true),
            additional_css: toml_config.additional_css.unwrap_or_default(),
            postcss_plugins: toml_config.postcss_plugins.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_config_creation() {
        let config = BuildConfig::new();
        assert!(!config.input.is_empty());
        assert!(!config.output.is_empty());
        assert!(!config.watch);
        assert!(!config.minify);
        assert!(config.purge);
    }

    #[test]
    fn test_build_config_validation() {
        let mut config = BuildConfig::new();
        assert!(config.validate().is_ok());

        config.output = "".to_string();
        assert!(config.validate().is_err());

        config.output = "dist/styles.css".to_string();
        config.input.clear();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_build_config_methods() {
        let mut config = BuildConfig::new();

        config.add_input("src/**/*.rsx");
        assert!(config.input.contains(&"src/**/*.rsx".to_string()));

        config.set_output("build/styles.css");
        assert_eq!(config.output, "build/styles.css");

        config.enable_watch();
        assert!(config.watch);

        config.enable_minify();
        assert!(config.minify);

        config.add_css("src/custom.css");
        assert!(config
            .additional_css
            .contains(&"src/custom.css".to_string()));
    }
}
