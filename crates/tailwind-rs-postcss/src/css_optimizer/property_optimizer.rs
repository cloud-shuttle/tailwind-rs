//! Property optimizer for deduplication and optimization

use super::types::*;
use regex::Regex;
use std::collections::HashMap;

/// Property optimizer for deduplication and optimization
pub struct PropertyOptimizer {
    property_rules: HashMap<String, PropertyRule>,
}

impl PropertyOptimizer {
    /// Create new property optimizer
    pub fn new() -> Self {
        Self {
            property_rules: Self::build_property_rules(),
        }
    }

    /// Optimize CSS properties
    pub fn optimize_properties(
        &self,
        css: &str,
    ) -> Result<PropertyOptimizationResult, OptimizationError> {
        let start_time = std::time::Instant::now();
        let mut properties_optimized = 0;
        let mut duplicates_removed = 0;
        let mut optimized_css = css.to_string();

        // Remove duplicate properties within rules
        let rule_pattern = Regex::new(r"([^{]+)\s*\{([^}]+)\}").unwrap();

        for cap in rule_pattern.captures_iter(css) {
            let selector = cap[1].trim();
            let properties_str = cap[2].trim();

            let optimized_properties = self.optimize_rule_properties(properties_str)?;
            let original_count = self.count_properties(properties_str);
            let optimized_count = optimized_properties.len();

            if optimized_count < original_count {
                duplicates_removed += original_count - optimized_count;
                properties_optimized += optimized_count;

                // Replace the rule with optimized version
                let optimized_rule = format!(
                    "{} {{\n{}\n}}",
                    selector,
                    optimized_properties
                        .iter()
                        .map(|p| format!("  {}: {};", p.name, p.value))
                        .collect::<Vec<_>>()
                        .join("\n")
                );

                let old_rule = &cap[0];
                optimized_css = optimized_css.replace(old_rule, &optimized_rule);
            }
        }

        Ok(PropertyOptimizationResult {
            optimized_css,
            properties_optimized,
            duplicates_removed,
            optimization_time: start_time.elapsed(),
        })
    }

    /// Optimize properties within a single rule
    fn optimize_rule_properties(
        &self,
        properties_str: &str,
    ) -> Result<Vec<CSSProperty>, OptimizationError> {
        let mut properties = Vec::new();
        let mut seen_properties: HashMap<String, String> = HashMap::new();

        let property_pattern = Regex::new(r"([^:]+):\s*([^;]+);").unwrap();

        for cap in property_pattern.captures_iter(properties_str) {
            let name = cap[1].trim().to_string();
            let value = cap[2].trim().to_string();

            // Keep the last value for duplicate properties (CSS cascade rule)
            seen_properties.insert(name, value);
        }

        for (name, value) in seen_properties {
            properties.push(CSSProperty { name, value });
        }

        Ok(properties)
    }

    /// Count properties in a string
    fn count_properties(&self, properties_str: &str) -> usize {
        properties_str.matches(';').count()
    }

    /// Build property rules for optimization
    fn build_property_rules() -> HashMap<String, PropertyRule> {
        let mut rules = HashMap::new();

        // Add rules for common property optimizations
        rules.insert(
            "margin".to_string(),
            PropertyRule {
                name: "margin".to_string(),
                shorthand: true,
                longhand: vec![
                    "margin-top".to_string(),
                    "margin-right".to_string(),
                    "margin-bottom".to_string(),
                    "margin-left".to_string(),
                ],
            },
        );

        rules.insert(
            "padding".to_string(),
            PropertyRule {
                name: "padding".to_string(),
                shorthand: true,
                longhand: vec![
                    "padding-top".to_string(),
                    "padding-right".to_string(),
                    "padding-bottom".to_string(),
                    "padding-left".to_string(),
                ],
            },
        );

        rules
    }
}
