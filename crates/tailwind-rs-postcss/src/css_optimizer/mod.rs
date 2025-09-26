//! CSS Optimization System
//!
//! This module provides comprehensive CSS optimization functionality including
//! rule merging, property optimization, selector optimization, and CSS minification.

pub mod analyzer;
pub mod minifier;
pub mod property_optimizer;
pub mod rule_merger;
pub mod selector_optimizer;
pub mod types;

// Re-export types for convenience

/// Main CSS optimizer for comprehensive CSS optimization
pub struct CSSOptimizer {
    analyzer: analyzer::CSSAnalyzer,
    rule_merger: rule_merger::RuleMerger,
    property_optimizer: property_optimizer::PropertyOptimizer,
    selector_optimizer: selector_optimizer::SelectorOptimizer,
    minifier: minifier::CSSMinifier,
    config: types::OptimizationConfig,
}

impl CSSOptimizer {
    /// Create a new CSS optimizer with default configuration
    pub fn new() -> Self {
        Self::with_config(types::OptimizationConfig::default())
    }

    /// Create a new CSS optimizer with custom configuration
    pub fn with_config(config: types::OptimizationConfig) -> Self {
        Self {
            analyzer: analyzer::CSSAnalyzer::new(),
            rule_merger: rule_merger::RuleMerger::new(),
            property_optimizer: property_optimizer::PropertyOptimizer::new(),
            selector_optimizer: selector_optimizer::SelectorOptimizer::new(),
            minifier: minifier::CSSMinifier::new(),
            config,
        }
    }

    /// Optimize CSS with comprehensive analysis and optimization
    pub fn optimize(
        &mut self,
        css: &str,
    ) -> Result<types::OptimizationResult, types::OptimizationError> {
        let start_time = std::time::Instant::now();
        let original_size = css.len();

        // Step 1: Analyze CSS for optimization opportunities
        let analysis = self.analyzer.analyze(css)?;

        // Step 2: Apply optimizations based on configuration
        let mut optimized_css = css.to_string();
        let mut metrics = types::OptimizationMetrics::new();

        if self.config.merge_rules {
            let merge_result = self.rule_merger.merge_rules(&optimized_css)?;
            optimized_css = merge_result.optimized_css;
            metrics.rules_merged = merge_result.rules_merged;
        }

        if self.config.optimize_properties {
            let property_result = self
                .property_optimizer
                .optimize_properties(&optimized_css)?;
            optimized_css = property_result.optimized_css;
            metrics.properties_optimized = property_result.properties_optimized;
            metrics.duplicates_removed = property_result.duplicates_removed;
        }

        if self.config.optimize_selectors {
            let selector_result = self.selector_optimizer.optimize_selectors(&optimized_css)?;
            optimized_css = selector_result.optimized_css;
            metrics.selectors_optimized = selector_result.selectors_optimized;
        }

        if self.config.minify {
            let minified = self.minifier.minify(&optimized_css)?;
            optimized_css = minified;
        }

        let processing_time = start_time.elapsed();
        let final_size = optimized_css.len();
        let size_reduction = if original_size > 0 {
            ((original_size - final_size) as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        metrics.original_size = original_size;
        metrics.optimized_size = final_size;
        metrics.size_reduction_percentage = size_reduction;
        metrics.processing_time = processing_time;

        Ok(types::OptimizationResult {
            optimized_css,
            metrics,
            analysis,
        })
    }

    /// Get optimization statistics
    pub fn get_statistics(&self) -> types::OptimizationStatistics {
        types::OptimizationStatistics {
            total_optimizations: 0, // Will be updated during optimization
            average_size_reduction: 0.0,
            processing_time: std::time::Duration::from_secs(0),
        }
    }
}

// Re-export types for convenience
pub use types::*;
