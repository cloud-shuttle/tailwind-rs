//! Advanced CSS optimization system
//!
//! This module provides comprehensive CSS optimization features including minification,
//! compression, rule merging, and performance optimizations.

use crate::css_generator::{CssGenerator, CssRule, CssProperty};
use crate::error::Result;
use std::collections::HashMap;

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

/// Results of CSS optimization
#[derive(Debug, Clone)]
pub struct OptimizationResults {
    /// Original CSS size
    pub original_size: usize,
    /// Optimized CSS size
    pub optimized_size: usize,
    /// Size reduction in bytes
    pub size_reduction: usize,
    /// Size reduction percentage
    pub reduction_percentage: f64,
    /// Number of rules before optimization
    pub original_rules: usize,
    /// Number of rules after optimization
    pub optimized_rules: usize,
    /// Number of properties before optimization
    pub original_properties: usize,
    /// Number of properties after optimization
    pub optimized_properties: usize,
    /// Statistics
    pub stats: OptimizationStats,
}

/// Statistics for optimization operation
#[derive(Debug, Clone)]
pub struct OptimizationStats {
    /// Number of rules merged
    pub rules_merged: usize,
    /// Number of properties optimized
    pub properties_optimized: usize,
    /// Number of selectors optimized
    pub selectors_optimized: usize,
    /// Number of empty rules removed
    pub empty_rules_removed: usize,
    /// Number of duplicate properties removed
    pub duplicate_properties_removed: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// Internal statistics tracking for optimization operations
#[derive(Debug, Clone, Default)]
struct OptimizationTracker {
    empty_rules_removed: usize,
    duplicate_properties_removed: usize,
    selectors_optimized: usize,
}

/// Advanced CSS optimizer
#[derive(Debug, Clone)]
pub struct CssOptimizer {
    config: OptimizationConfig,
}

impl CssOptimizer {
    /// Create a new CSS optimizer with default configuration
    pub fn new() -> Self {
        Self {
            config: OptimizationConfig::default(),
        }
    }

    /// Create a new CSS optimizer with custom configuration
    pub fn with_config(config: OptimizationConfig) -> Self {
        Self { config }
    }

    /// Optimize CSS from a generator
    pub fn optimize(&self, generator: &mut CssGenerator) -> Result<OptimizationResults> {
        let start_time = std::time::Instant::now();
        
        // Get original metrics
        let original_css = generator.generate_css();
        let original_size = original_css.len();
        let original_rules = generator.rule_count();
        let original_properties = self.count_properties(generator);

        // Initialize optimization tracker
        let mut tracker = OptimizationTracker::default();

        // Apply optimizations
        if self.config.remove_empty_rules {
            tracker.empty_rules_removed = self.remove_empty_rules(generator);
        }

        if self.config.remove_duplicates {
            tracker.duplicate_properties_removed = self.remove_duplicate_properties(generator);
        }

        if self.config.optimize_properties {
            self.optimize_properties(generator);
        }

        if self.config.merge_rules {
            self.merge_compatible_rules(generator);
        }

        if self.config.sort_properties {
            self.sort_properties(generator);
        }

        // Get optimized metrics
        let optimized_css = if self.config.minify {
            generator.generate_minified_css()
        } else {
            generator.generate_css()
        };
        
        let optimized_size = optimized_css.len();
        let optimized_rules = generator.rule_count();
        let optimized_properties = self.count_properties(generator);

        // Calculate results
        let size_reduction = original_size.saturating_sub(optimized_size);
        let reduction_percentage = if original_size > 0 {
            (size_reduction as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        let stats = OptimizationStats {
            rules_merged: original_rules.saturating_sub(optimized_rules),
            properties_optimized: original_properties.saturating_sub(optimized_properties),
            selectors_optimized: tracker.selectors_optimized,
            empty_rules_removed: tracker.empty_rules_removed,
            duplicate_properties_removed: tracker.duplicate_properties_removed,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
        };

        Ok(OptimizationResults {
            original_size,
            optimized_size,
            size_reduction,
            reduction_percentage,
            original_rules,
            optimized_rules,
            original_properties,
            optimized_properties,
            stats,
        })
    }

    /// Optimize CSS from a string
    pub fn optimize_css(&self, css: &str) -> Result<String> {
        let mut generator = CssGenerator::new();
        
        // Parse CSS into generator (simplified implementation)
        self.parse_css_into_generator(css, &mut generator)?;
        
        // Optimize
        self.optimize(&mut generator)?;
        
        // Return optimized CSS
        if self.config.minify {
            Ok(generator.generate_minified_css())
        } else {
            Ok(generator.generate_css())
        }
    }

    /// Get the current configuration
    pub fn get_config(&self) -> &OptimizationConfig {
        &self.config
    }

    /// Update the configuration
    pub fn set_config(&mut self, config: OptimizationConfig) {
        self.config = config;
    }

    /// Count total properties in generator
    fn count_properties(&self, generator: &CssGenerator) -> usize {
        generator.get_rules().values()
            .map(|rule| rule.properties.len())
            .sum()
    }

    /// Remove empty CSS rules
    fn remove_empty_rules(&self, generator: &mut CssGenerator) -> usize {
        let rules = generator.get_rules().clone();
        let mut removed_count = 0;
        for (selector, rule) in rules {
            if rule.properties.is_empty() {
                generator.remove_rule(&selector);
                removed_count += 1;
            }
        }
        removed_count
    }

    /// Remove duplicate properties within rules
    fn remove_duplicate_properties(&self, generator: &mut CssGenerator) -> usize {
        let rules = generator.get_rules().clone();
        let mut total_removed = 0;
        for (selector, rule) in rules {
            let mut seen_properties = std::collections::HashSet::new();
            let mut unique_properties = Vec::new();
            
            for property in &rule.properties {
                if seen_properties.insert(&property.name) {
                    unique_properties.push(property.clone());
                }
            }
            
            let removed_count = rule.properties.len() - unique_properties.len();
            if removed_count > 0 {
                total_removed += removed_count;
                let updated_rule = CssRule {
                    selector: rule.selector.clone(),
                    properties: unique_properties,
                    media_query: rule.media_query.clone(),
                    specificity: rule.specificity,
                };
                generator.update_rule(&selector, updated_rule);
            }
        }
        total_removed
    }

    /// Optimize CSS properties
    fn optimize_properties(&self, generator: &mut CssGenerator) {
        let rules = generator.get_rules().clone();
        for (selector, rule) in rules {
            let mut optimized_properties = Vec::new();
            
            for property in &rule.properties {
                let optimized_property = CssProperty {
                    name: property.name.clone(),
                    value: self.optimize_property_value(&property.value),
                    important: property.important,
                };
                optimized_properties.push(optimized_property);
            }
            
            // Update the rule with optimized properties
            let updated_rule = CssRule {
                selector: rule.selector.clone(),
                properties: optimized_properties,
                media_query: rule.media_query.clone(),
                specificity: rule.specificity,
            };
            generator.update_rule(&selector, updated_rule);
        }
    }

    /// Optimize a single property value
    fn optimize_property_value(&self, value: &str) -> String {
        let mut optimized = value.to_string();
        
        // Convert 0px to 0
        optimized = optimized.replace("0px", "0");
        optimized = optimized.replace("0em", "0");
        optimized = optimized.replace("0rem", "0");
        
        // Convert redundant units
        optimized = optimized.replace("0.0", "0");
        optimized = optimized.replace("1.0", "1");
        
        optimized
    }

    /// Merge compatible CSS rules
    fn merge_compatible_rules(&self, generator: &mut CssGenerator) {
        let rules = generator.get_rules().clone();
        let mut merged_rules: HashMap<String, CssRule> = HashMap::new();
        
        for (selector, rule) in rules {
            // Simple merging: combine rules with same properties
            if let Some(existing_rule) = merged_rules.get_mut(&selector) {
                // Merge properties
                for property in &rule.properties {
                    if !existing_rule.properties.iter().any(|p| p.name == property.name) {
                        existing_rule.properties.push(property.clone());
                    }
                }
            } else {
                merged_rules.insert(selector, rule);
            }
        }
        
        // Update generator with merged rules
        for (selector, rule) in merged_rules {
            generator.update_rule(&selector, rule);
        }
    }

    /// Sort CSS properties for better compression
    fn sort_properties(&self, generator: &mut CssGenerator) {
        let rules = generator.get_rules().clone();
        
        for (selector, rule) in rules {
            let mut sorted_properties = rule.properties.clone();
            sorted_properties.sort_by(|a, b| a.name.cmp(&b.name));
            
            let sorted_rule = CssRule {
                selector: rule.selector.clone(),
                properties: sorted_properties,
                media_query: rule.media_query.clone(),
                specificity: rule.specificity,
            };
            generator.update_rule(&selector, sorted_rule);
        }
    }

    /// Parse CSS string into generator (simplified implementation)
    fn parse_css_into_generator(&self, css: &str, generator: &mut CssGenerator) -> Result<()> {
        // Simple CSS parsing - extract basic rules
        let lines: Vec<&str> = css.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Look for CSS rules (simplified pattern matching)
            if line.ends_with('{') && line.contains('.') {
                let selector = line.replace('{', "").trim().to_string();
                
                // Collect properties until we find the closing brace
                let mut properties = Vec::new();
                i += 1;
                
                while i < lines.len() && !lines[i].trim().starts_with('}') {
                    let prop_line = lines[i].trim();
                    if prop_line.contains(':') && prop_line.ends_with(';') {
                        let parts: Vec<&str> = prop_line.split(':').collect();
                        if parts.len() == 2 {
                            let name = parts[0].trim().to_string();
                            let value = parts[1].trim().replace(';', "").to_string();
                            properties.push(CssProperty {
                                name,
                                value,
                                important: false,
                            });
                        }
                    }
                    i += 1;
                }
                
                // Add the rule to the generator
                let rule = CssRule {
                    selector,
                    properties,
                    media_query: None,
                    specificity: 1,
                };
                let selector = rule.selector.clone();
                generator.update_rule(&selector, rule);
            }
            i += 1;
        }
        
        Ok(())
    }

    /// Advanced CSS compression
    pub fn compress_css(&self, css: &str) -> Result<String> {
        let mut compressed = css.to_string();

        // Always remove comments and optimize, regardless of advanced_compression setting
        // Remove comments
        compressed = self.remove_comments(&compressed);
        
        // Remove unnecessary whitespace
        compressed = self.remove_unnecessary_whitespace(&compressed);
        
        // Optimize colors
        compressed = self.optimize_colors(&compressed);
        
        // Optimize units
        compressed = self.optimize_units(&compressed);

        Ok(compressed)
    }


    /// Remove CSS comments
    fn remove_comments(&self, css: &str) -> String {
        let mut result = String::new();
        let mut chars = css.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '/' && chars.peek() == Some(&'*') {
                // Skip comment
                chars.next(); // consume *
                while let Some(c) = chars.next() {
                    if c == '*' && chars.peek() == Some(&'/') {
                        chars.next(); // consume /
                        break;
                    }
                }
            } else {
                result.push(c);
            }
        }
        
        result
    }

    /// Remove unnecessary whitespace
    fn remove_unnecessary_whitespace(&self, css: &str) -> String {
        css.chars()
            .filter(|c| !c.is_whitespace() || *c == ' ')
            .collect::<String>()
            .replace(" {", "{")
            .replace("{ ", "{")
            .replace("} ", "}")
            .replace("; ", ";")
            .replace(": ", ":")
            .replace(", ", ",")
    }

    /// Optimize color values
    fn optimize_colors(&self, css: &str) -> String {
        let mut optimized = css.to_string();
        
        // Convert #ffffff to #fff (simplified without backreferences)
        optimized = regex::Regex::new(r"#([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])")
            .unwrap()
            .replace_all(&optimized, |caps: &regex::Captures| {
                let r1 = &caps[1];
                let g1 = &caps[2];
                let b1 = &caps[3];
                let r2 = &caps[4];
                let g2 = &caps[5];
                let b2 = &caps[6];
                
                // Only compress if all pairs are the same
                if r1 == r2 && g1 == g2 && b1 == b2 {
                    format!("#{}{}{}", r1, g1, b1)
                } else {
                    caps[0].to_string()
                }
            })
            .to_string();
        
        // Convert rgb(255, 255, 255) to #ffffff (simplified without backreferences)
        optimized = regex::Regex::new(r"rgb\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)")
            .unwrap()
            .replace_all(&optimized, |caps: &regex::Captures| {
                let r = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
                let g = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
                let b = caps.get(3).unwrap().as_str().parse::<u8>().unwrap();
                format!("#{:02x}{:02x}{:02x}", r, g, b)
            })
            .to_string();
        
        optimized
    }

    /// Optimize CSS units
    fn optimize_units(&self, css: &str) -> String {
        let mut optimized = css.to_string();
        
        // Convert 0px to 0
        optimized = regex::Regex::new(r"(\d+)px")
            .unwrap()
            .replace_all(&optimized, "$1")
            .to_string();
        
        // Convert 0em to 0
        optimized = regex::Regex::new(r"(\d+)em")
            .unwrap()
            .replace_all(&optimized, "$1")
            .to_string();
        
        optimized
    }
}

impl Default for CssOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = CssOptimizer::new();
        assert!(optimizer.get_config().minify);
        assert!(optimizer.get_config().merge_rules);
    }

    #[test]
    fn test_custom_config() {
        let config = OptimizationConfig {
            minify: false,
            merge_rules: false,
            optimize_properties: false,
            optimize_selectors: false,
            remove_empty_rules: false,
            remove_duplicates: false,
            sort_properties: false,
            advanced_compression: true,
            compression_level: 9,
        };
        
        let optimizer = CssOptimizer::with_config(config);
        assert!(!optimizer.get_config().minify);
        assert!(optimizer.get_config().advanced_compression);
        assert_eq!(optimizer.get_config().compression_level, 9);
    }

    #[test]
    fn test_optimize_css() {
        let optimizer = CssOptimizer::new();
        let css = r#"
            .test {
                padding: 1rem;
                margin: 0px;
                color: #ffffff;
            }
        "#;
        
        let result = optimizer.optimize_css(css).unwrap();
        assert!(result.len() <= css.len());
    }

    #[test]
    fn test_compress_css() {
        let optimizer = CssOptimizer::new();
        let css = r#"
            /* This is a comment */
            .test {
                padding: 1rem;
                margin: 0px;
                color: #ffffff;
            }
        "#;
        
        let compressed = optimizer.compress_css(css).unwrap();
        assert!(!compressed.contains("/*"));
        assert!(!compressed.contains("*/"));
        assert!(compressed.len() < css.len());
    }

    #[test]
    fn test_remove_comments() {
        let optimizer = CssOptimizer::new();
        let css = "/* comment */ .test { color: red; }";
        let result = optimizer.remove_comments(css);
        assert!(!result.contains("/*"));
        assert!(!result.contains("*/"));
    }

    #[test]
    fn test_remove_unnecessary_whitespace() {
        let optimizer = CssOptimizer::new();
        let css = ".test {\n  color: red;\n  margin: 0px;\n}";
        let result = optimizer.remove_unnecessary_whitespace(css);
        assert!(!result.contains('\n'));
        assert!(!result.contains("  "));
    }

    #[test]
    fn test_optimize_colors() {
        let optimizer = CssOptimizer::new();
        let css = "color: #ffffff; background: #000000;";
        let result = optimizer.optimize_colors(css);
        assert!(result.contains("#fff"));
        assert!(result.contains("#000"));
    }

    #[test]
    fn test_optimize_units() {
        let optimizer = CssOptimizer::new();
        let css = "margin: 0px; padding: 1rem;";
        let result = optimizer.optimize_units(css);
        assert!(result.contains("margin: 0"));
        assert!(result.contains("padding: 1rem"));
    }

    #[test]
    fn test_optimize_generator() {
        let mut generator = CssGenerator::new();
        generator.add_class("p-4").unwrap();
        generator.add_class("bg-blue-500").unwrap();
        
        let optimizer = CssOptimizer::new();
        let results = optimizer.optimize(&mut generator).unwrap();
        
        assert!(results.original_size > 0);
        assert!(results.optimized_size > 0);
        assert!(results.reduction_percentage >= 0.0);
    }
}
