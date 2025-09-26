//! Rule merger for consolidating compatible CSS rules

use super::types::*;
use regex::Regex;
use std::collections::HashMap;

/// Rule merger for consolidating compatible CSS rules
pub struct RuleMerger {
    compatibility_matrix: HashMap<String, Vec<String>>,
}

impl RuleMerger {
    /// Create new rule merger
    pub fn new() -> Self {
        Self {
            compatibility_matrix: Self::build_compatibility_matrix(),
        }
    }

    /// Merge compatible CSS rules
    pub fn merge_rules(&self, css: &str) -> Result<RuleMergeResult, OptimizationError> {
        let start_time = std::time::Instant::now();
        let mut rules_merged = 0;
        let mut optimized_css = css.to_string();

        // Find and merge duplicate selectors
        let duplicate_selectors = self.find_duplicate_selectors(css)?;

        for (selector, rules) in duplicate_selectors {
            if rules.len() > 1 {
                let merged_rule = self.merge_rule_properties(&rules)?;
                optimized_css =
                    self.replace_duplicate_rules(&optimized_css, &selector, &merged_rule)?;
                rules_merged += 1;
            }
        }

        Ok(RuleMergeResult {
            optimized_css,
            rules_merged,
            merge_time: start_time.elapsed(),
        })
    }

    /// Find duplicate selectors in CSS
    fn find_duplicate_selectors(
        &self,
        css: &str,
    ) -> Result<HashMap<String, Vec<CSSRule>>, OptimizationError> {
        let mut selector_map: HashMap<String, Vec<CSSRule>> = HashMap::new();
        let rule_pattern = Regex::new(r"([^{]+)\s*\{([^}]+)\}").unwrap();

        for cap in rule_pattern.captures_iter(css) {
            let selector = cap[1].trim().to_string();
            let properties = cap[2].trim();

            let rule = CSSRule {
                selector: selector.clone(),
                properties: self.parse_properties(properties)?,
            };

            selector_map
                .entry(selector)
                .or_insert_with(Vec::new)
                .push(rule);
        }

        Ok(selector_map)
    }

    /// Merge properties from multiple rules
    fn merge_rule_properties(&self, rules: &[CSSRule]) -> Result<CSSRule, OptimizationError> {
        let mut merged_properties = Vec::new();
        let mut seen_properties: HashMap<String, String> = HashMap::new();

        for rule in rules {
            for property in &rule.properties {
                seen_properties.insert(property.name.clone(), property.value.clone());
            }
        }

        for (name, value) in seen_properties {
            merged_properties.push(CSSProperty { name, value });
        }

        Ok(CSSRule {
            selector: rules[0].selector.clone(),
            properties: merged_properties,
        })
    }

    /// Replace duplicate rules with merged rule
    fn replace_duplicate_rules(
        &self,
        css: &str,
        selector: &str,
        merged_rule: &CSSRule,
    ) -> Result<String, OptimizationError> {
        let mut result = css.to_string();
        let rule_pattern =
            Regex::new(&format!(r"{}[^{{]*\{{[^}}]*\}}", regex::escape(selector))).unwrap();

        // Remove all instances of the selector
        result = rule_pattern.replace_all(&result, "").to_string();

        // Add merged rule
        let merged_css = format!(
            "{} {{\n{}\n}}",
            merged_rule.selector,
            merged_rule
                .properties
                .iter()
                .map(|p| format!("  {}: {};", p.name, p.value))
                .collect::<Vec<_>>()
                .join("\n")
        );

        result.push_str(&merged_css);
        Ok(result)
    }

    /// Parse CSS properties from string
    fn parse_properties(
        &self,
        properties_str: &str,
    ) -> Result<Vec<CSSProperty>, OptimizationError> {
        let mut properties = Vec::new();
        let property_pattern = Regex::new(r"([^:]+):\s*([^;]+);").unwrap();

        for cap in property_pattern.captures_iter(properties_str) {
            properties.push(CSSProperty {
                name: cap[1].trim().to_string(),
                value: cap[2].trim().to_string(),
            });
        }

        Ok(properties)
    }

    /// Build compatibility matrix for different selector types
    fn build_compatibility_matrix() -> HashMap<String, Vec<String>> {
        let mut matrix = HashMap::new();

        // Class selectors can be merged with other classes
        matrix.insert(
            "class".to_string(),
            vec!["class".to_string(), "element".to_string()],
        );

        // Element selectors can be merged with classes
        matrix.insert(
            "element".to_string(),
            vec!["element".to_string(), "class".to_string()],
        );

        matrix
    }
}
