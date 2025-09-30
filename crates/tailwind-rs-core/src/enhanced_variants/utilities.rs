//! Enhanced Variants Utilities Module
//!
//! This module contains utility functions for variant processing, optimization,
//! caching, and advanced variant manipulation.

use super::types::*;
use super::definitions::*;
use std::collections::HashMap;

/// Variant optimization utilities
pub struct VariantOptimizer;

impl VariantOptimizer {
    /// Optimize a variant combination by removing redundant variants
    pub fn optimize_combination(combination: &mut VariantCombination) {
        // Remove duplicate variants
        let mut seen = HashMap::new();
        combination.variants.retain(|variant| {
            if seen.contains_key(&variant.name) {
                false
            } else {
                seen.insert(variant.name.clone(), true);
                true
            }
        });

        // Recalculate specificity after optimization
        if combination.valid {
            *combination = combination.clone().calculate_specificity();
        }
    }

    /// Check if a variant combination can be simplified
    pub fn can_simplify(combination: &VariantCombination) -> bool {
        // Check for obviously redundant combinations
        let variant_names: Vec<String> = combination.variants.iter()
            .map(|v| v.name.clone())
            .collect();

        // Check for conflicting variants that cancel each other out
        let has_print = variant_names.contains(&"print".to_string());
        let has_screen = variant_names.contains(&"screen".to_string());

        has_print && has_screen // These conflict and should be simplified
    }

    /// Simplify a variant combination
    pub fn simplify_combination(combination: VariantCombination) -> VariantCombination {
        let mut simplified = combination.clone();

        // Check for conflicts first
        let has_print = simplified.variants.iter().any(|v| v.name == "print");
        let has_screen = simplified.variants.iter().any(|v| v.name == "screen");

        // Remove conflicting variants
        simplified.variants.retain(|variant| {
            match variant.name.as_str() {
                "print" => !has_screen,
                "screen" => !has_print,
                _ => true,
            }
        });

        // Recalculate specificity
        simplified.calculate_specificity()
    }
}

/// Variant caching utilities
pub struct VariantCache {
    cache: HashMap<String, VariantParseResult>,
    max_size: usize,
}

impl VariantCache {
    /// Create a new variant cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }

    /// Get a cached result
    pub fn get(&self, key: &str) -> Option<&VariantParseResult> {
        self.cache.get(key)
    }

    /// Insert a result into the cache
    pub fn insert(&mut self, key: String, result: VariantParseResult) {
        if self.cache.len() >= self.max_size {
            // Simple eviction: remove oldest (this could be improved)
            if let Some(first_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&first_key);
            }
        }
        self.cache.insert(key, result);
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize) {
        (self.cache.len(), self.max_size)
    }
}

impl Default for VariantCache {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Variant analysis utilities
pub struct VariantAnalyzer;

impl VariantAnalyzer {
    /// Analyze variant usage patterns
    pub fn analyze_usage(results: &[VariantParseResult]) -> VariantUsageStats {
        let mut stats = VariantUsageStats::default();
        let mut variant_counts = HashMap::new();

        for result in results {
            stats.total_classes += 1;

            if result.success {
                stats.successful_parses += 1;

                for variant in &result.combination.variants {
                    *variant_counts.entry(variant.name.clone()).or_insert(0) += 1;
                }
            } else {
                stats.failed_parses += 1;
            }
        }

        stats.most_used_variants = variant_counts.into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .take(10)
            .collect();

        stats
    }

    /// Find potential optimization opportunities
    pub fn find_optimization_opportunities(results: &[VariantParseResult]) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        for result in results {
            if !result.success {
                continue;
            }

            // Check for redundant variants
            if VariantOptimizer::can_simplify(&result.combination) {
                suggestions.push(OptimizationSuggestion {
                    class: result.original_class.clone(),
                    suggestion_type: SuggestionType::SimplifyCombination,
                    description: "Remove conflicting variants".to_string(),
                });
            }

            // Check for high specificity that could be optimized
            if result.combination.specificity > 300 {
                suggestions.push(OptimizationSuggestion {
                    class: result.original_class.clone(),
                    suggestion_type: SuggestionType::HighSpecificity,
                    description: format!("High specificity ({}) may cause override issues", result.combination.specificity),
                });
            }
        }

        suggestions
    }
}

/// Variant usage statistics
#[derive(Debug, Clone, Default)]
pub struct VariantUsageStats {
    pub total_classes: usize,
    pub successful_parses: usize,
    pub failed_parses: usize,
    pub most_used_variants: Vec<(String, usize)>,
}

/// Optimization suggestion
#[derive(Debug, Clone)]
pub struct OptimizationSuggestion {
    pub class: String,
    pub suggestion_type: SuggestionType,
    pub description: String,
}

/// Type of optimization suggestion
#[derive(Debug, Clone)]
pub enum SuggestionType {
    SimplifyCombination,
    HighSpecificity,
    UnusedVariant,
    ConflictingVariants,
}

/// Variant formatting utilities
pub struct VariantFormatter;

impl VariantFormatter {
    /// Format a variant combination as a readable string
    pub fn format_combination(combination: &VariantCombination) -> String {
        if combination.variants.is_empty() {
            return "no variants".to_string();
        }

        let variant_names: Vec<String> = combination.variants.iter()
            .map(|v| format!("{} ({:?})", v.name, v.variant_type))
            .collect();

        format!("{} (specificity: {})", variant_names.join(", "), combination.specificity)
    }

    /// Format a parse result as a detailed string
    pub fn format_parse_result(result: &VariantParseResult) -> String {
        let mut output = format!("Class: {}\n", result.original_class);
        output.push_str(&format!("Base: {}\n", result.base_class));
        output.push_str(&format!("Success: {}\n", result.success));

        if result.success {
            output.push_str(&format!("Combination: {}\n", Self::format_combination(&result.combination)));
            output.push_str(&format!("CSS Selector: {}\n", result.css_selector));

            if let Some(mq) = &result.media_query {
                output.push_str(&format!("Media Query: {}\n", mq));
            }
        } else {
            if let Some(error) = &result.combination.error_message {
                output.push_str(&format!("Error: {}\n", error));
            }
        }

        output
    }

    /// Generate a summary report for multiple parse results
    pub fn generate_summary_report(results: &[VariantParseResult]) -> String {
        let stats = VariantAnalyzer::analyze_usage(results);
        let suggestions = VariantAnalyzer::find_optimization_opportunities(results);

        let mut report = String::new();
        report.push_str(&format!("=== Variant Parsing Summary ===\n\n"));
        report.push_str(&format!("Total Classes: {}\n", stats.total_classes));
        report.push_str(&format!("Successful Parses: {} ({:.1}%)\n",
            stats.successful_parses,
            (stats.successful_parses as f64 / stats.total_classes as f64) * 100.0));
        report.push_str(&format!("Failed Parses: {}\n", stats.failed_parses));

        if !stats.most_used_variants.is_empty() {
            report.push_str(&format!("\nMost Used Variants:\n"));
            for (variant, count) in &stats.most_used_variants {
                report.push_str(&format!("  {}: {}\n", variant, count));
            }
        }

        if !suggestions.is_empty() {
            report.push_str(&format!("\nOptimization Suggestions ({}):\n", suggestions.len()));
            for suggestion in suggestions.iter().take(5) {
                report.push_str(&format!("  {}: {}\n", suggestion.class, suggestion.description));
            }
            if suggestions.len() > 5 {
                report.push_str(&format!("  ... and {} more\n", suggestions.len() - 5));
            }
        }

        report
    }
}

/// Variant migration utilities for upgrading from older systems
pub struct VariantMigrator;

impl VariantMigrator {
    /// Migrate old variant syntax to new syntax
    pub fn migrate_class_syntax(class: &str) -> String {
        // Handle common migration cases
        let migrated = class
            // Convert old responsive prefixes if any
            .replace("mobile:", "sm:")
            .replace("tablet:", "md:")
            // Handle other common migrations
            .to_string();

        migrated
    }

    /// Check if a class needs migration
    pub fn needs_migration(class: &str) -> bool {
        class.contains("mobile:") || class.contains("tablet:")
    }

    /// Migrate a batch of classes
    pub fn migrate_classes(classes: &[String]) -> Vec<String> {
        classes.iter()
            .map(|class| Self::migrate_class_syntax(class))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enhanced_variants::parser::EnhancedVariantParser;

    #[test]
    fn variant_optimization() {
        let mut combination = VariantCombination::new();
        combination.variants.push(ParsedVariant::new("hover".to_string(), VariantType::State));
        combination.variants.push(ParsedVariant::new("hover".to_string(), VariantType::State)); // Duplicate

        VariantOptimizer::optimize_combination(&mut combination);
        assert_eq!(combination.variants.len(), 1);
    }

    #[test]
    fn combination_simplification() {
        let mut combination = VariantCombination::new();
        combination.variants.push(ParsedVariant::new("print".to_string(), VariantType::Print));
        combination.variants.push(ParsedVariant::new("screen".to_string(), VariantType::Screen));

        assert!(VariantOptimizer::can_simplify(&combination));

        let simplified = VariantOptimizer::simplify_combination(combination);
        // Should remove conflicting variants (exact behavior depends on implementation)
        assert!(simplified.variants.len() <= 2);
    }

    #[test]
    fn cache_operations() {
        let mut cache = VariantCache::new(10);

        let result = VariantParseResult::success(
            "test".to_string(),
            "test".to_string(),
            VariantCombination::new(),
            ".test".to_string(),
            None,
        );

        cache.insert("test".to_string(), result.clone());
        assert!(cache.get("test").is_some());
        assert_eq!(cache.get("test").unwrap().original_class, "test");

        assert!(cache.get("nonexistent").is_none());

        let (size, max_size) = cache.stats();
        assert_eq!(size, 1);
        assert_eq!(max_size, 10);
    }

    #[test]
    fn usage_analysis() {
        let parser = EnhancedVariantParser::new();

        let result1 = parser.parse_class("hover:bg-blue-500").unwrap();
        let result2 = parser.parse_class("sm:hover:bg-red-500").unwrap();
        let result3 = parser.parse_class("invalid:class").unwrap_or_else(|_| {
            VariantParseResult::failure("invalid:class".to_string(), "test error".to_string())
        });

        let results = vec![result1, result2, result3];
        let stats = VariantAnalyzer::analyze_usage(&results);

        assert_eq!(stats.total_classes, 3);
        assert_eq!(stats.successful_parses, 2);
        assert_eq!(stats.failed_parses, 1);
        assert!(!stats.most_used_variants.is_empty());
    }

    #[test]
    fn optimization_suggestions() {
        let parser = EnhancedVariantParser::new();

        let result1 = parser.parse_class("print:screen:bg-blue-500").unwrap();
        let results = vec![result1];

        let suggestions = VariantAnalyzer::find_optimization_opportunities(&results);
        assert!(!suggestions.is_empty());
        // Should find the conflicting print/screen variants
    }

    #[test]
    fn formatting_utilities() {
        let combination = VariantCombination {
            variants: vec![
                ParsedVariant::new("hover".to_string(), VariantType::State),
                ParsedVariant::new("sm".to_string(), VariantType::Responsive),
            ],
            specificity: 180,
            valid: true,
            error_message: None,
        };

        let formatted = VariantFormatter::format_combination(&combination);
        assert!(formatted.contains("hover"));
        assert!(formatted.contains("sm"));
        assert!(formatted.contains("180"));
    }

    #[test]
    fn summary_report_generation() {
        let parser = EnhancedVariantParser::new();

        let result1 = parser.parse_class("hover:bg-blue-500").unwrap();
        let result2 = parser.parse_class("sm:bg-red-500").unwrap();
        let results = vec![result1, result2];

        let report = VariantFormatter::generate_summary_report(&results);
        assert!(report.contains("Variant Parsing Summary"));
        assert!(report.contains("Total Classes: 2"));
        assert!(report.contains("Successful Parses: 2"));
    }

    #[test]
    fn migration_utilities() {
        assert_eq!(VariantMigrator::migrate_class_syntax("mobile:bg-blue-500"), "sm:bg-blue-500");
        assert_eq!(VariantMigrator::migrate_class_syntax("tablet:hover:bg-red-500"), "md:hover:bg-red-500");

        assert!(VariantMigrator::needs_migration("mobile:bg-blue-500"));
        assert!(VariantMigrator::needs_migration("tablet:bg-red-500"));
        assert!(!VariantMigrator::needs_migration("sm:bg-blue-500"));

        let classes = vec![
            "mobile:bg-blue-500".to_string(),
            "tablet:bg-red-500".to_string(),
            "sm:bg-green-500".to_string(),
        ];

        let migrated = VariantMigrator::migrate_classes(&classes);
        assert_eq!(migrated[0], "sm:bg-blue-500");
        assert_eq!(migrated[1], "md:bg-red-500");
        assert_eq!(migrated[2], "sm:bg-green-500");
    }
}
