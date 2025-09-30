//! CSS Optimization Configuration Module
//!
//! Handles configuration settings for CSS optimization including:
//! - OptimizationConfig struct for various optimization options
//! - Default configurations and builder patterns

/// Configuration for CSS optimization
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Whether to enable minification
    pub minify: bool,
    /// Whether to enable rule merging
    pub merge_rules: bool,
    /// Whether to enable property optimization
    pub optimize_properties: bool,
    /// Whether to enable selector optimization
    pub optimize_selectors: bool,
    /// Whether to remove empty rules
    pub remove_empty_rules: bool,
    /// Whether to remove duplicate properties
    pub remove_duplicates: bool,
    /// Whether to sort properties
    pub sort_properties: bool,
    /// Whether to enable advanced compression
    pub advanced_compression: bool,
    /// Maximum compression level (0-9)
    pub compression_level: u8,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            minify: true,
            merge_rules: true,
            optimize_properties: true,
            optimize_selectors: true,
            remove_empty_rules: true,
            remove_duplicates: true,
            sort_properties: true,
            advanced_compression: false,
            compression_level: 6,
        }
    }
}

impl OptimizationConfig {
    /// Create a new optimization configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a minimal optimization configuration (only basic minification)
    pub fn minimal() -> Self {
        Self {
            minify: true,
            merge_rules: false,
            optimize_properties: false,
            optimize_selectors: false,
            remove_empty_rules: true,
            remove_duplicates: true,
            sort_properties: false,
            advanced_compression: false,
            compression_level: 0,
        }
    }

    /// Create a maximum optimization configuration (all features enabled)
    pub fn maximum() -> Self {
        Self {
            minify: true,
            merge_rules: true,
            optimize_properties: true,
            optimize_selectors: true,
            remove_empty_rules: true,
            remove_duplicates: true,
            sort_properties: true,
            advanced_compression: true,
            compression_level: 9,
        }
    }

    /// Set minification enabled/disabled
    pub fn minify(mut self, enabled: bool) -> Self {
        self.minify = enabled;
        self
    }

    /// Set rule merging enabled/disabled
    pub fn merge_rules(mut self, enabled: bool) -> Self {
        self.merge_rules = enabled;
        self
    }

    /// Set property optimization enabled/disabled
    pub fn optimize_properties(mut self, enabled: bool) -> Self {
        self.optimize_properties = enabled;
        self
    }

    /// Set selector optimization enabled/disabled
    pub fn optimize_selectors(mut self, enabled: bool) -> Self {
        self.optimize_selectors = enabled;
        self
    }

    /// Set empty rule removal enabled/disabled
    pub fn remove_empty_rules(mut self, enabled: bool) -> Self {
        self.remove_empty_rules = enabled;
        self
    }

    /// Set duplicate removal enabled/disabled
    pub fn remove_duplicates(mut self, enabled: bool) -> Self {
        self.remove_duplicates = enabled;
        self
    }

    /// Set property sorting enabled/disabled
    pub fn sort_properties(mut self, enabled: bool) -> Self {
        self.sort_properties = enabled;
        self
    }

    /// Set advanced compression enabled/disabled
    pub fn advanced_compression(mut self, enabled: bool) -> Self {
        self.advanced_compression = enabled;
        self
    }

    /// Set compression level (0-9)
    pub fn compression_level(mut self, level: u8) -> Self {
        self.compression_level = level.min(9);
        self
    }

    /// Check if any optimization features are enabled
    pub fn has_optimizations(&self) -> bool {
        self.minify ||
        self.merge_rules ||
        self.optimize_properties ||
        self.optimize_selectors ||
        self.remove_empty_rules ||
        self.remove_duplicates ||
        self.sort_properties ||
        self.advanced_compression
    }

    /// Get a human-readable description of the current configuration
    pub fn description(&self) -> String {
        let mut features = Vec::new();

        if self.minify { features.push("minification"); }
        if self.merge_rules { features.push("rule merging"); }
        if self.optimize_properties { features.push("property optimization"); }
        if self.optimize_selectors { features.push("selector optimization"); }
        if self.remove_empty_rules { features.push("empty rule removal"); }
        if self.remove_duplicates { features.push("duplicate removal"); }
        if self.sort_properties { features.push("property sorting"); }
        if self.advanced_compression { features.push("advanced compression"); }

        if features.is_empty() {
            "No optimizations enabled".to_string()
        } else {
            format!("Enabled optimizations: {}", features.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = OptimizationConfig::default();

        assert!(config.minify);
        assert!(config.merge_rules);
        assert!(config.optimize_properties);
        assert!(config.optimize_selectors);
        assert!(config.remove_empty_rules);
        assert!(config.remove_duplicates);
        assert!(config.sort_properties);
        assert!(!config.advanced_compression);
        assert_eq!(config.compression_level, 6);
    }

    #[test]
    fn minimal_config() {
        let config = OptimizationConfig::minimal();

        assert!(config.minify);
        assert!(config.remove_empty_rules);
        assert!(config.remove_duplicates);
        assert!(!config.merge_rules);
        assert!(!config.optimize_properties);
        assert!(!config.optimize_selectors);
        assert!(!config.sort_properties);
        assert!(!config.advanced_compression);
        assert_eq!(config.compression_level, 0);
    }

    #[test]
    fn maximum_config() {
        let config = OptimizationConfig::maximum();

        assert!(config.minify);
        assert!(config.merge_rules);
        assert!(config.optimize_properties);
        assert!(config.optimize_selectors);
        assert!(config.remove_empty_rules);
        assert!(config.remove_duplicates);
        assert!(config.sort_properties);
        assert!(config.advanced_compression);
        assert_eq!(config.compression_level, 9);
    }

    #[test]
    fn builder_pattern() {
        let config = OptimizationConfig::new()
            .minify(true)
            .merge_rules(false)
            .compression_level(5);

        assert!(config.minify);
        assert!(!config.merge_rules);
        assert_eq!(config.compression_level, 5);
    }

    #[test]
    fn compression_level_clamping() {
        let config = OptimizationConfig::new().compression_level(15);
        assert_eq!(config.compression_level, 9); // Should be clamped to 9
    }

    #[test]
    fn has_optimizations_check() {
        let minimal = OptimizationConfig::minimal();
        assert!(minimal.has_optimizations());

        let disabled = OptimizationConfig {
            minify: false,
            merge_rules: false,
            optimize_properties: false,
            optimize_selectors: false,
            remove_empty_rules: false,
            remove_duplicates: false,
            sort_properties: false,
            advanced_compression: false,
            compression_level: 0,
        };
        assert!(!disabled.has_optimizations());
    }

    #[test]
    fn config_description() {
        let default_config = OptimizationConfig::default();
        let desc = default_config.description();
        assert!(desc.contains("minification"));
        assert!(desc.contains("rule merging"));
        assert!(desc.contains("property optimization"));

        let minimal_config = OptimizationConfig::minimal();
        let desc = minimal_config.description();
        assert!(desc.contains("minification"));
        assert!(desc.contains("empty rule removal"));
        assert!(!desc.contains("rule merging"));
    }
}
