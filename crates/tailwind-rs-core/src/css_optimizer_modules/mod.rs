//! CSS Optimization System Module
//!
//! Comprehensive CSS optimization system for Tailwind-RS:
//! - Configuration management for optimization settings
//! - Results tracking and statistics reporting
//! - Main CSS optimizer implementation with various optimization techniques
//! - Internal tracking for optimization operations

use crate::css_generator::{CssGenerator, CssProperty, CssRule};
use crate::error::Result;
use std::collections::HashMap;
use std::time::{Duration, Instant};

// Re-export all optimization utility types and traits
pub mod config;
pub mod results;

// Re-export all types for easy access
pub use config::OptimizationConfig;
pub use results::{OptimizationResults, OptimizationStats, OptimizationTracker};

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

    /// Optimize CSS rules
    pub fn optimize(&self, rules: Vec<CssRule>) -> Result<OptimizationResults> {
        let start_time = Instant::now();
        let mut tracker = OptimizationTracker::new();

        let original_size = self.calculate_size(&rules);
        let original_rules = rules.len();
        let original_properties = self.count_properties(&rules);

        let mut optimized_rules = rules;

        // Apply optimizations in order
        if self.config.remove_empty_rules {
            optimized_rules = self.remove_empty_rules(optimized_rules, &mut tracker);
        }

        if self.config.remove_duplicates {
            optimized_rules = self.remove_duplicate_properties(optimized_rules, &mut tracker);
        }

        if self.config.merge_rules {
            optimized_rules = self.merge_identical_rules(optimized_rules);
        }

        if self.config.optimize_properties {
            optimized_rules = self.optimize_properties(optimized_rules);
        }

        if self.config.optimize_selectors {
            optimized_rules = self.optimize_selectors(optimized_rules, &mut tracker);
        }

        if self.config.sort_properties {
            optimized_rules = self.sort_properties(optimized_rules);
        }

        let optimized_size = self.calculate_size(&optimized_rules);
        let optimized_properties = self.count_properties(&optimized_rules);
        let processing_time = start_time.elapsed().as_millis() as u64;

        let stats = tracker.into_stats(processing_time);

        Ok(OptimizationResults::new(
            original_size,
            optimized_size,
            original_rules,
            optimized_rules.len(),
            original_properties,
            optimized_properties,
            stats,
        ))
    }

    /// Optimize CSS as string
    pub fn optimize_css(&self, css: &str) -> Result<(String, OptimizationResults)> {
        // Parse CSS string into rules (simplified implementation)
        let rules = self.parse_css_to_rules(css)?;
        let results = self.optimize(rules)?;
        let optimized_css = self.generate_css_from_rules(&results)?;

        Ok((optimized_css, results))
    }

    /// Remove empty CSS rules
    fn remove_empty_rules(&self, rules: Vec<CssRule>, tracker: &mut OptimizationTracker) -> Vec<CssRule> {
        rules.into_iter().filter(|rule| {
            if rule.properties.is_empty() {
                tracker.increment_empty_rules_removed();
                false
            } else {
                true
            }
        }).collect()
    }

    /// Remove duplicate properties within rules
    fn remove_duplicate_properties(&self, rules: Vec<CssRule>, tracker: &mut OptimizationTracker) -> Vec<CssRule> {
        rules.into_iter().map(|rule| {
            let mut seen = HashMap::new();
            let mut unique_properties = Vec::new();

            for property in rule.properties {
                if !seen.contains_key(&property.name) {
                    seen.insert(property.name.clone(), true);
                    unique_properties.push(property);
                } else {
                    tracker.increment_duplicate_properties_removed();
                }
            }

            CssRule {
                selector: rule.selector,
                properties: unique_properties,
                media_query: rule.media_query,
                specificity: rule.specificity,
            }
        }).collect()
    }

    /// Merge identical rules
    fn merge_identical_rules(&self, rules: Vec<CssRule>) -> Vec<CssRule> {
        let mut rule_map: HashMap<String, CssRule> = HashMap::new();

        for rule in rules {
            if let Some(existing) = rule_map.get_mut(&rule.selector) {
                // Merge properties
                for property in rule.properties {
                    if !existing.properties.iter().any(|p| p.name == property.name) {
                        existing.properties.push(property);
                    }
                }
            } else {
                rule_map.insert(rule.selector.clone(), rule);
            }
        }

        rule_map.into_values().collect()
    }

    /// Optimize CSS properties
    fn optimize_properties(&self, rules: Vec<CssRule>) -> Vec<CssRule> {
        rules.into_iter().map(|rule| {
            let optimized_properties = rule.properties.into_iter().map(|property| {
                // Apply property-specific optimizations
                CssProperty {
                    name: self.optimize_property_name(&property.name),
                    value: self.optimize_property_value(&property.value),
                    important: property.important,
                }
            }).collect();

            CssRule {
                selector: rule.selector,
                properties: optimized_properties,
                media_query: rule.media_query,
                specificity: rule.specificity,
            }
        }).collect()
    }

    /// Optimize CSS selectors
    fn optimize_selectors(&self, rules: Vec<CssRule>, tracker: &mut OptimizationTracker) -> Vec<CssRule> {
        rules.into_iter().map(|rule| {
            // Basic selector optimization
            let optimized_selector = rule.selector.trim().to_string();

            tracker.increment_selectors_optimized();

            CssRule {
                selector: optimized_selector,
                properties: rule.properties,
                media_query: rule.media_query,
                specificity: rule.specificity,
            }
        }).collect()
    }

    /// Sort CSS properties within rules
    fn sort_properties(&self, rules: Vec<CssRule>) -> Vec<CssRule> {
        rules.into_iter().map(|rule| {
            let mut sorted_properties = rule.properties;
            sorted_properties.sort_by(|a, b| a.name.cmp(&b.name));

            CssRule {
                selector: rule.selector,
                properties: sorted_properties,
                media_query: rule.media_query,
                specificity: rule.specificity,
            }
        }).collect()
    }

    /// Optimize property name
    fn optimize_property_name(&self, name: &str) -> String {
        // Basic property name optimization
        name.trim().to_lowercase()
    }

    /// Optimize property value
    fn optimize_property_value(&self, value: &str) -> String {
        // Basic property value optimization
        value.trim().to_string()
    }

    /// Calculate total size of CSS rules
    fn calculate_size(&self, rules: &[CssRule]) -> usize {
        rules.iter().map(|rule| {
            rule.selector.len() +
            rule.properties.iter().map(|p| p.name.len() + p.value.len() + 2).sum::<usize>() +
            4 // for braces and semicolon
        }).sum()
    }

    /// Count total properties in rules
    fn count_properties(&self, rules: &[CssRule]) -> usize {
        rules.iter().map(|rule| rule.properties.len()).sum()
    }

    /// Parse CSS string to rules (simplified)
    fn parse_css_to_rules(&self, _css: &str) -> Result<Vec<CssRule>> {
        // Simplified implementation - would need full CSS parser
        Ok(Vec::new())
    }

    /// Generate CSS string from rules (simplified)
    fn generate_css_from_rules(&self, _results: &OptimizationResults) -> Result<String> {
        // Simplified implementation - would need full CSS generator
        Ok(String::new())
    }

    /// Get current configuration
    pub fn config(&self) -> &OptimizationConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: OptimizationConfig) {
        self.config = config;
    }
}

impl Default for CssOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// CSS optimization utilities trait for extending CssGenerator
pub trait CssOptimization {
    /// Optimize the generated CSS
    fn optimize_css(&self, config: OptimizationConfig) -> Result<OptimizationResults>;
}

impl CssOptimization for CssGenerator {
    fn optimize_css(&self, config: OptimizationConfig) -> Result<OptimizationResults> {
        let optimizer = CssOptimizer::with_config(config);
        // This would need access to the internal rules - simplified for now
        Ok(OptimizationResults::new(0, 0, 0, 0, 0, 0, OptimizationStats::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_optimizer_creation() {
        let optimizer = CssOptimizer::new();
        assert!(optimizer.config().minify);
    }

    #[test]
    fn css_optimizer_with_config() {
        let config = OptimizationConfig::minimal();
        let optimizer = CssOptimizer::with_config(config);
        assert!(optimizer.config().minify);
        assert!(!optimizer.config().merge_rules);
    }

    #[test]
    fn remove_empty_rules() {
        let optimizer = CssOptimizer::new();
        let mut tracker = OptimizationTracker::new();

        let rules = vec![
            CssRule {
                selector: ".test1".to_string(),
                properties: vec![CssProperty {
                    name: "color".to_string(),
                    value: "red".to_string(),
                    important: false,
                }],
                important: false,
            },
            CssRule {
                selector: ".empty".to_string(),
                properties: vec![],
                important: false,
            },
        ];

        let result = optimizer.remove_empty_rules(rules, &mut tracker);
        assert_eq!(result.len(), 1);
        assert_eq!(tracker.empty_rules_removed, 1);
    }

    #[test]
    fn remove_duplicate_properties() {
        let optimizer = CssOptimizer::new();
        let mut tracker = OptimizationTracker::new();

        let rules = vec![
            CssRule {
                selector: ".test".to_string(),
                properties: vec![
                    CssProperty {
                        name: "color".to_string(),
                        value: "red".to_string(),
                        important: false,
                    },
                    CssProperty {
                        name: "color".to_string(),
                        value: "blue".to_string(),
                        important: false,
                    },
                ],
                important: false,
            },
        ];

        let result = optimizer.remove_duplicate_properties(rules, &mut tracker);
        assert_eq!(result[0].properties.len(), 1);
        assert_eq!(tracker.duplicate_properties_removed, 1);
    }

    #[test]
    fn merge_identical_rules() {
        let optimizer = CssOptimizer::new();

        let rules = vec![
            CssRule {
                selector: ".test".to_string(),
                properties: vec![CssProperty {
                    name: "color".to_string(),
                    value: "red".to_string(),
                    important: false,
                }],
                important: false,
            },
            CssRule {
                selector: ".test".to_string(),
                properties: vec![CssProperty {
                    name: "background".to_string(),
                    value: "blue".to_string(),
                    important: false,
                }],
                important: false,
            },
        ];

        let result = optimizer.merge_identical_rules(rules);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].properties.len(), 2);
    }

    #[test]
    fn calculate_size() {
        let optimizer = CssOptimizer::new();

        let rules = vec![
            CssRule {
                selector: ".test".to_string(),
                properties: vec![CssProperty {
                    name: "color".to_string(),
                    value: "red".to_string(),
                    important: false,
                }],
                important: false,
            },
        ];

        let size = optimizer.calculate_size(&rules);
        assert!(size > 0); // Should calculate some size
    }

    #[test]
    fn count_properties() {
        let optimizer = CssOptimizer::new();

        let rules = vec![
            CssRule {
                selector: ".test".to_string(),
                properties: vec![
                    CssProperty {
                        name: "color".to_string(),
                        value: "red".to_string(),
                        important: false,
                    },
                    CssProperty {
                        name: "background".to_string(),
                        value: "blue".to_string(),
                        important: false,
                    },
                ],
                important: false,
            },
        ];

        let count = optimizer.count_properties(&rules);
        assert_eq!(count, 2);
    }
}
