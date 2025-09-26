//! Selector optimizer for simplification and optimization

use super::types::*;
use regex::Regex;
use std::collections::HashMap;

/// Selector optimizer for simplification and optimization
pub struct SelectorOptimizer {
    selector_rules: HashMap<String, SelectorRule>,
}

impl SelectorOptimizer {
    /// Create new selector optimizer
    pub fn new() -> Self {
        Self {
            selector_rules: Self::build_selector_rules(),
        }
    }

    /// Optimize CSS selectors
    pub fn optimize_selectors(
        &self,
        css: &str,
    ) -> Result<SelectorOptimizationResult, OptimizationError> {
        let start_time = std::time::Instant::now();
        let mut selectors_optimized = 0;
        let mut optimized_css = css.to_string();

        // Find and optimize redundant selectors
        let redundant_selectors = self.find_redundant_selectors(css)?;

        for selector in redundant_selectors {
            optimized_css = self.remove_redundant_selector(&optimized_css, &selector)?;
            selectors_optimized += 1;
        }

        Ok(SelectorOptimizationResult {
            optimized_css,
            selectors_optimized,
            optimization_time: start_time.elapsed(),
        })
    }

    /// Find redundant selectors
    fn find_redundant_selectors(&self, css: &str) -> Result<Vec<String>, OptimizationError> {
        let mut redundant = Vec::new();
        let rule_pattern = Regex::new(r"([^{]+)\s*\{([^}]+)\}").unwrap();
        let mut selector_map: HashMap<String, Vec<String>> = HashMap::new();

        for cap in rule_pattern.captures_iter(css) {
            let selector = cap[1].trim().to_string();
            let properties = cap[2].trim().to_string();

            selector_map
                .entry(properties)
                .or_insert_with(Vec::new)
                .push(selector);
        }

        // Find selectors with identical properties
        for (_, selectors) in selector_map {
            if selectors.len() > 1 {
                // Keep the most specific selector, mark others as redundant
                let mut sorted_selectors = selectors;
                sorted_selectors.sort_by(|a, b| self.compare_specificity(a, b));

                for selector in sorted_selectors.iter().skip(1) {
                    redundant.push(selector.clone());
                }
            }
        }

        Ok(redundant)
    }

    /// Remove redundant selector from CSS
    fn remove_redundant_selector(
        &self,
        css: &str,
        selector: &str,
    ) -> Result<String, OptimizationError> {
        let rule_pattern =
            Regex::new(&format!(r"{}[^{{]*\{{[^}}]*\}}", regex::escape(selector))).unwrap();
        Ok(rule_pattern.replace_all(css, "").to_string())
    }

    /// Compare selector specificity
    fn compare_specificity(&self, a: &str, b: &str) -> std::cmp::Ordering {
        let specificity_a = self.calculate_specificity(a);
        let specificity_b = self.calculate_specificity(b);
        specificity_b.cmp(&specificity_a) // Higher specificity first
    }

    /// Calculate selector specificity
    fn calculate_specificity(&self, selector: &str) -> usize {
        let mut specificity = 0;

        // Count IDs
        specificity += selector.matches('#').count() * 100;

        // Count classes and attributes
        specificity += selector.matches('.').count() * 10;
        specificity += selector.matches('[').count() * 10;

        // Count elements
        specificity += selector.split_whitespace().count();

        specificity
    }

    /// Build selector rules for optimization
    fn build_selector_rules() -> HashMap<String, SelectorRule> {
        let mut rules = HashMap::new();

        rules.insert(
            "universal".to_string(),
            SelectorRule {
                name: "universal".to_string(),
                pattern: "*".to_string(),
                optimization: SelectorOptimization::Remove,
            },
        );

        rules
    }
}
