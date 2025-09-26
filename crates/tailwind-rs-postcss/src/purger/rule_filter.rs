//! Rule filter for CSS purging

use super::types::*;
use regex::Regex;
use std::collections::HashSet;

/// Rule filter for CSS purging
pub struct RuleFilter {
    css_rule_pattern: Regex,
    comment_pattern: Regex,
    keyframe_pattern: Regex,
    media_query_pattern: Regex,
}

impl RuleFilter {
    /// Create new rule filter
    pub fn new() -> Self {
        Self {
            css_rule_pattern: Regex::new(r"([^{]+)\s*\{([^}]+)\}").unwrap(),
            comment_pattern: Regex::new(r"/\*.*?\*/").unwrap(),
            keyframe_pattern: Regex::new(r"@keyframes\s+([^{]+)\s*\{([^}]+)\}").unwrap(),
            media_query_pattern: Regex::new(r"@media\s+([^{]+)\s*\{([^}]+)\}").unwrap(),
        }
    }

    /// Filter CSS rules based on used classes
    pub fn filter_rules(
        &self,
        css: &str,
        used_classes: &HashSet<String>,
        config: &PurgeConfig,
    ) -> Result<String, PurgeError> {
        let mut filtered_css = String::new();
        let mut _rules_removed = 0;
        let mut _rules_kept = 0;

        // Process CSS rules
        for cap in self.css_rule_pattern.captures_iter(css) {
            let selector = cap[1].trim();
            let properties = cap[2].trim();

            if self.should_keep_rule(selector, used_classes, config) {
                let rule = format!("{} {{\n{}\n}}", selector, properties);
                filtered_css.push_str(&rule);
                _rules_kept += 1;
            } else {
                _rules_removed += 1;
            }
        }

        // Preserve comments if configured
        if config.preserve_comments {
            filtered_css = self.preserve_comments(&filtered_css);
        }

        // Preserve keyframes if configured
        if config.preserve_keyframes {
            filtered_css = self.preserve_keyframes(&filtered_css);
        }

        // Preserve media queries if configured
        if config.preserve_media_queries {
            filtered_css = self.preserve_media_queries(&filtered_css);
        }

        Ok(filtered_css)
    }

    /// Filter CSS rules with advanced options
    pub fn filter_rules_advanced(
        &self,
        css: &str,
        used_classes: &HashSet<String>,
        config: &PurgeConfig,
        options: &PurgeOptions,
    ) -> Result<String, PurgeError> {
        let mut filtered_css = self.filter_rules(css, used_classes, config)?;

        // Apply advanced options
        if options.minify_output {
            filtered_css = self.minify_css(&filtered_css);
        }

        if !options.preserve_whitespace {
            filtered_css = self.remove_whitespace(&filtered_css);
        }

        Ok(filtered_css)
    }

    /// Check if a rule should be kept
    fn should_keep_rule(
        &self,
        selector: &str,
        used_classes: &HashSet<String>,
        config: &PurgeConfig,
    ) -> bool {
        // Check safelist
        if config
            .safelist
            .iter()
            .any(|pattern| selector.contains(pattern))
        {
            return true;
        }

        // Check blocklist
        if config
            .blocklist
            .iter()
            .any(|pattern| selector.contains(pattern))
        {
            return false;
        }

        // Extract classes from selector
        let selector_classes = self.extract_classes_from_selector(selector);

        // Check if any class is used
        selector_classes
            .iter()
            .any(|class| used_classes.contains(class))
    }

    /// Extract classes from a CSS selector
    fn extract_classes_from_selector(&self, selector: &str) -> HashSet<String> {
        let mut classes = HashSet::new();

        // Split selector by combinators
        let parts: Vec<&str> = selector.split(&[' ', '>', '+', '~', ','][..]).collect();

        for part in parts {
            let part = part.trim();
            if part.starts_with('.') {
                // Extract class name
                let class_name = part.strip_prefix('.').unwrap_or(part);
                classes.insert(class_name.to_string());
            }
        }

        classes
    }

    /// Preserve comments in CSS
    fn preserve_comments(&self, css: &str) -> String {
        // This is a simplified implementation
        // In a real implementation, you'd need to parse and preserve comments properly
        css.to_string()
    }

    /// Preserve keyframes in CSS
    fn preserve_keyframes(&self, css: &str) -> String {
        // This is a simplified implementation
        // In a real implementation, you'd need to parse and preserve keyframes properly
        css.to_string()
    }

    /// Preserve media queries in CSS
    fn preserve_media_queries(&self, css: &str) -> String {
        // This is a simplified implementation
        // In a real implementation, you'd need to parse and preserve media queries properly
        css.to_string()
    }

    /// Minify CSS
    fn minify_css(&self, css: &str) -> String {
        // This is a simplified implementation
        // In a real implementation, you'd need proper CSS minification
        css.replace('\n', "").replace("  ", " ").trim().to_string()
    }

    /// Remove whitespace from CSS
    fn remove_whitespace(&self, css: &str) -> String {
        css.replace('\n', "")
            .replace('\t', "")
            .replace("  ", " ")
            .trim()
            .to_string()
    }
}
