//! Core Autoprefixer functionality
//!
//! This module contains the main Autoprefixer struct and its core methods.

use regex::Regex;
use std::collections::HashMap;
use crate::autoprefixer::autoprefixer_module::{
    BrowserData, CanIUseData, AutoprefixerConfig, PrefixCache, AutoprefixerError
};

/// Main autoprefixer for adding vendor prefixes to CSS properties
pub struct Autoprefixer {
    browser_data: BrowserData,
    caniuse_data: CanIUseData,
    config: AutoprefixerConfig,
    cache: PrefixCache,
}

impl Autoprefixer {
    /// Create a new autoprefixer with default configuration
    pub fn new() -> Self {
        Self::with_config(AutoprefixerConfig::default())
    }

    /// Create a new autoprefixer with custom configuration
    pub fn with_config(config: AutoprefixerConfig) -> Self {
        Self {
            browser_data: BrowserData::new(),
            caniuse_data: CanIUseData::new(),
            config,
            cache: PrefixCache::new(),
        }
    }

    /// Add vendor prefixes based on browser support
    pub fn add_prefixes(
        &self,
        css: &str,
        browsers: &[String],
    ) -> Result<String, AutoprefixerError> {
        let mut result = String::new();
        let mut in_rule = false;
        let mut current_rule = String::new();

        for line in css.lines() {
            if line.trim().ends_with('{') {
                in_rule = true;
                current_rule = line.to_string();
            } else if in_rule && line.trim() == "}" {
                current_rule.push_str(line);

                let prefixed_rule = self.prefix_rule(&current_rule, browsers)?;
                result.push_str(&prefixed_rule);
                result.push('\n');

                in_rule = false;
                current_rule.clear();
            } else if in_rule {
                current_rule.push_str(line);
                current_rule.push('\n');
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        Ok(result)
    }

    /// Add prefixes with advanced options
    pub fn add_prefixes_advanced(
        &self,
        css: &str,
        options: &PrefixOptions,
    ) -> Result<PrefixResult, AutoprefixerError> {
        let start_time = std::time::Instant::now();
        let original_size = css.len();

        let prefixed_css = self.add_prefixes(css, &options.browsers)?;
        let prefixed_size = prefixed_css.len();

        let prefixes_added = self.count_prefixes_added(&css, &prefixed_css);
        let prefixes_removed = self.count_prefixes_removed(&css, &prefixed_css);
        let properties_processed = self.count_properties_processed(&css);

        let processing_time = start_time.elapsed().as_millis() as usize;

        Ok(PrefixResult {
            prefixed_css,
            prefixes_added: HashMap::new(),
            prefixes_removed: HashMap::new(),
            statistics: PrefixStatistics {
                original_size,
                prefixed_size,
                prefixes_added,
                prefixes_removed,
                properties_processed,
                processing_time_ms: processing_time,
            },
        })
    }

    /// Check if a property needs prefixes for the given browsers
    pub fn needs_prefix(&self, property: &str, browsers: &[String]) -> bool {
        self.check_browser_compatibility(property, browsers)
    }

    /// Prefix a CSS rule
    fn prefix_rule(&self, rule: &str, browsers: &[String]) -> Result<String, AutoprefixerError> {
        let mut result = String::new();
        let mut in_declaration = false;
        let mut current_declaration = String::new();

        for line in rule.lines() {
            if line.trim().ends_with('{') {
                result.push_str(line);
                result.push('\n');
            } else if line.trim() == "}" {
                if !current_declaration.is_empty() {
                    let prefixed_declaration = self.prefix_declaration(&current_declaration, browsers)?;
                    result.push_str(&prefixed_declaration);
                }
                result.push_str(line);
                result.push('\n');
            } else if line.trim().ends_with(';') {
                current_declaration.push_str(line);
                let prefixed_declaration = self.prefix_declaration(&current_declaration, browsers)?;
                result.push_str(&prefixed_declaration);
                current_declaration.clear();
            } else {
                current_declaration.push_str(line);
                current_declaration.push('\n');
            }
        }

        Ok(result)
    }

    /// Prefix a CSS declaration
    fn prefix_declaration(
        &self,
        declaration: &str,
        browsers: &[String],
    ) -> Result<String, AutoprefixerError> {
        let mut result = String::new();
        let mut lines = declaration.lines();

        if let Some(line) = lines.next() {
            let trimmed = line.trim();
            if let Some(colon_pos) = trimmed.find(':') {
                let property = &trimmed[..colon_pos].trim();
                let value = &trimmed[colon_pos + 1..].trim();

                if self.needs_prefix(property, browsers) {
                    let prefixes = self.get_prefixes_for_property(property);
                    for prefix in prefixes {
                        result.push_str(&format!("{}: {};\n", prefix, value));
                    }
                }
                result.push_str(&format!("{}: {};\n", property, value));
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        for line in lines {
            result.push_str(line);
            result.push('\n');
        }

        Ok(result)
    }

    /// Get prefixes for a specific property
    fn get_prefixes_for_property(&self, property: &str) -> Vec<String> {
        let mut prefixes = Vec::new();

        if self.needs_webkit_prefix(property) {
            prefixes.push(format!("-webkit-{}", property));
        }
        if self.needs_moz_prefix(property) {
            prefixes.push(format!("-moz-{}", property));
        }
        if self.needs_ms_prefix(property) {
            prefixes.push(format!("-ms-{}", property));
        }
        if self.needs_o_prefix(property) {
            prefixes.push(format!("-o-{}", property));
        }

        prefixes
    }

    /// Check if a property needs webkit prefix
    fn needs_webkit_prefix(&self, property: &str) -> bool {
        match property {
            "transform" | "transform-origin" | "transform-style" | "backface-visibility" => true,
            "perspective" | "perspective-origin" | "animation" | "animation-name" => true,
            "animation-duration" | "animation-timing-function" | "animation-delay" => true,
            "animation-iteration-count" | "animation-direction" | "animation-fill-mode" => true,
            "animation-play-state" | "transition" | "transition-property" => true,
            "transition-duration" | "transition-timing-function" | "transition-delay" => true,
            "flex" | "flex-direction" | "flex-wrap" | "flex-flow" | "flex-grow" => true,
            "flex-shrink" | "flex-basis" | "justify-content" | "align-items" => true,
            "align-self" | "align-content" | "order" | "filter" | "backdrop-filter" => true,
            "mask" | "mask-image" | "mask-mode" | "mask-position" | "mask-size" => true,
            "mask-repeat" | "mask-origin" | "mask-clip" | "mask-composite" => true,
            "clip-path" | "shape-outside" | "shape-margin" | "shape-image-threshold" => true,
            "text-decoration" | "text-decoration-line" | "text-decoration-style" => true,
            "text-decoration-color" | "text-decoration-thickness" => true,
            "text-underline-offset" | "text-decoration-skip" | "text-decoration-skip-ink" => true,
            "text-emphasis" | "text-emphasis-style" | "text-emphasis-color" => true,
            "text-emphasis-position" | "text-underline-position" => true,
            "text-underline-offset" | "text-decoration-skip" | "text-decoration-skip-ink" => true,
            "text-emphasis" | "text-emphasis-style" | "text-emphasis-color" => true,
            "text-emphasis-position" | "text-underline-position" => true,
            _ => false,
        }
    }

    /// Check if a property needs moz prefix
    fn needs_moz_prefix(&self, property: &str) -> bool {
        match property {
            "transform" | "transform-origin" | "transform-style" | "backface-visibility" => true,
            "perspective" | "perspective-origin" | "animation" | "animation-name" => true,
            "animation-duration" | "animation-timing-function" | "animation-delay" => true,
            "animation-iteration-count" | "animation-direction" | "animation-fill-mode" => true,
            "animation-play-state" | "transition" | "transition-property" => true,
            "transition-duration" | "transition-timing-function" | "transition-delay" => true,
            "flex" | "flex-direction" | "flex-wrap" | "flex-flow" | "flex-grow" => true,
            "flex-shrink" | "flex-basis" | "justify-content" | "align-items" => true,
            "align-self" | "align-content" | "order" | "filter" | "backdrop-filter" => true,
            "mask" | "mask-image" | "mask-mode" | "mask-position" | "mask-size" => true,
            "mask-repeat" | "mask-origin" | "mask-clip" | "mask-composite" => true,
            "clip-path" | "shape-outside" | "shape-margin" | "shape-image-threshold" => true,
            "text-decoration" | "text-decoration-line" | "text-decoration-style" => true,
            "text-decoration-color" | "text-decoration-thickness" => true,
            "text-underline-offset" | "text-decoration-skip" | "text-decoration-skip-ink" => true,
            "text-emphasis" | "text-emphasis-style" | "text-emphasis-color" => true,
            "text-emphasis-position" | "text-underline-position" => true,
            _ => false,
        }
    }

    /// Check if a property needs ms prefix
    fn needs_ms_prefix(&self, property: &str) -> bool {
        match property {
            "transform" | "transform-origin" | "transform-style" | "backface-visibility" => true,
            "perspective" | "perspective-origin" | "animation" | "animation-name" => true,
            "animation-duration" | "animation-timing-function" | "animation-delay" => true,
            "animation-iteration-count" | "animation-direction" | "animation-fill-mode" => true,
            "animation-play-state" | "transition" | "transition-property" => true,
            "transition-duration" | "transition-timing-function" | "transition-delay" => true,
            "flex" | "flex-direction" | "flex-wrap" | "flex-flow" | "flex-grow" => true,
            "flex-shrink" | "flex-basis" | "justify-content" | "align-items" => true,
            "align-self" | "align-content" | "order" | "filter" | "backdrop-filter" => true,
            "mask" | "mask-image" | "mask-mode" | "mask-position" | "mask-size" => true,
            "mask-repeat" | "mask-origin" | "mask-clip" | "mask-composite" => true,
            "clip-path" | "shape-outside" | "shape-margin" | "shape-image-threshold" => true,
            "text-decoration" | "text-decoration-line" | "text-decoration-style" => true,
            "text-decoration-color" | "text-decoration-thickness" => true,
            "text-underline-offset" | "text-decoration-skip" | "text-decoration-skip-ink" => true,
            "text-emphasis" | "text-emphasis-style" | "text-emphasis-color" => true,
            "text-emphasis-position" | "text-underline-position" => true,
            _ => false,
        }
    }

    /// Check if a property needs o prefix
    fn needs_o_prefix(&self, property: &str) -> bool {
        match property {
            "transform" | "transform-origin" | "transform-style" | "backface-visibility" => true,
            "perspective" | "perspective-origin" | "animation" | "animation-name" => true,
            "animation-duration" | "animation-timing-function" | "animation-delay" => true,
            "animation-iteration-count" | "animation-direction" | "animation-fill-mode" => true,
            "animation-play-state" | "transition" | "transition-property" => true,
            "transition-duration" | "transition-timing-function" | "transition-delay" => true,
            "flex" | "flex-direction" | "flex-wrap" | "flex-flow" | "flex-grow" => true,
            "flex-shrink" | "flex-basis" | "justify-content" | "align-items" => true,
            "align-self" | "align-content" | "order" | "filter" | "backdrop-filter" => true,
            "mask" | "mask-image" | "mask-mode" | "mask-position" | "mask-size" => true,
            "mask-repeat" | "mask-origin" | "mask-clip" | "mask-composite" => true,
            "clip-path" | "shape-outside" | "shape-margin" | "shape-image-threshold" => true,
            "text-decoration" | "text-decoration-line" | "text-decoration-style" => true,
            "text-decoration-color" | "text-decoration-thickness" => true,
            "text-underline-offset" | "text-decoration-skip" | "text-decoration-skip-ink" => true,
            "text-emphasis" | "text-emphasis-style" | "text-emphasis-color" => true,
            "text-emphasis-position" | "text-underline-position" => true,
            _ => false,
        }
    }

    /// Check browser compatibility for a property
    fn check_browser_compatibility(&self, property: &str, browsers: &[String]) -> bool {
        // Simplified compatibility check
        // In a real implementation, this would check against actual browser data
        match property {
            "transform" | "flex" | "grid" | "filter" | "backdrop-filter" => true,
            "mask" | "clip-path" | "shape-outside" => true,
            "text-decoration" | "text-emphasis" => true,
            _ => false,
        }
    }

    /// Count prefixes added
    fn count_prefixes_added(&self, original: &str, prefixed: &str) -> usize {
        let original_lines = original.lines().count();
        let prefixed_lines = prefixed.lines().count();
        if prefixed_lines > original_lines {
            prefixed_lines - original_lines
        } else {
            0
        }
    }

    /// Count prefixes removed
    fn count_prefixes_removed(&self, original: &str, prefixed: &str) -> usize {
        let original_lines = original.lines().count();
        let prefixed_lines = prefixed.lines().count();
        if original_lines > prefixed_lines {
            original_lines - prefixed_lines
        } else {
            0
        }
    }

    /// Count prefixes in CSS
    fn count_prefixes_in_css(&self, css: &str) -> usize {
        css.lines()
            .filter(|line| {
                line.contains("-webkit-") || line.contains("-moz-") || 
                line.contains("-ms-") || line.contains("-o-")
            })
            .count()
    }

    /// Count properties processed
    fn count_properties_processed(&self, css: &str) -> usize {
        css.lines()
            .filter(|line| line.contains(':') && line.contains(';'))
            .count()
    }
}

/// Prefix options for advanced autoprefixing
pub struct PrefixOptions {
    pub browsers: Vec<String>,
    pub cascade: bool,
    pub remove: bool,
    pub add: bool,
}

/// Result of prefixing operation
pub struct PrefixResult {
    pub prefixed_css: String,
    pub prefixes_added: HashMap<String, usize>,
    pub prefixes_removed: HashMap<String, usize>,
    pub statistics: PrefixStatistics,
}

/// Statistics about prefixing operation
pub struct PrefixStatistics {
    pub original_size: usize,
    pub prefixed_size: usize,
    pub prefixes_added: usize,
    pub prefixes_removed: usize,
    pub properties_processed: usize,
    pub processing_time_ms: usize,
}
