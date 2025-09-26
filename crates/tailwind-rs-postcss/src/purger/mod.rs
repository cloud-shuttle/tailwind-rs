//! CSS Purging System
//!
//! This module provides comprehensive CSS purging functionality for removing
//! unused CSS classes, essential for Tailwind CSS production builds.

pub mod class_extractor;
pub mod content_scanner;
pub mod rule_filter;
pub mod types;

use types::*;

/// Main CSS purger for removing unused CSS classes
pub struct CSSPurger {
    content_scanner: content_scanner::ContentScanner,
    class_extractor: class_extractor::ClassExtractor,
    rule_filter: rule_filter::RuleFilter,
    config: PurgeConfig,
}

impl CSSPurger {
    /// Create a new CSS purger with default configuration
    pub fn new() -> Self {
        Self::with_config(PurgeConfig::default())
    }

    /// Create a new CSS purger with custom configuration
    pub fn with_config(config: PurgeConfig) -> Self {
        Self {
            content_scanner: content_scanner::ContentScanner::new(),
            class_extractor: class_extractor::ClassExtractor::new(),
            rule_filter: rule_filter::RuleFilter::new(),
            config,
        }
    }

    /// Purge unused CSS from content
    pub fn purge(
        &mut self,
        css: &str,
        content_paths: &[String],
    ) -> Result<PurgeResult, PurgeError> {
        let start_time = std::time::Instant::now();

        // Step 1: Scan content for used classes
        let used_classes = self.content_scanner.scan_content(content_paths)?;

        // Step 2: Extract CSS classes from CSS
        let css_classes = self.class_extractor.extract_classes(css)?;

        // Step 3: Filter CSS rules based on used classes
        let filtered_css = self
            .rule_filter
            .filter_rules(css, &used_classes, &self.config)?;

        let processing_time = start_time.elapsed();
        let original_size = css.len();
        let purged_size = filtered_css.len();
        let size_reduction = if original_size > 0 {
            ((original_size - purged_size) as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        Ok(PurgeResult {
            purged_css: filtered_css,
            used_classes: used_classes.len(),
            removed_classes: css_classes.len() - used_classes.len(),
            original_size,
            purged_size,
            size_reduction_percentage: size_reduction,
            processing_time,
        })
    }

    /// Purge with advanced options
    pub fn purge_advanced(
        &mut self,
        css: &str,
        content_paths: &[String],
        options: &PurgeOptions,
    ) -> Result<PurgeResult, PurgeError> {
        let start_time = std::time::Instant::now();

        // Step 1: Scan content for used classes with advanced options
        let used_classes = self
            .content_scanner
            .scan_content_advanced(content_paths, options)?;

        // Step 2: Extract CSS classes from CSS
        let css_classes = self.class_extractor.extract_classes(css)?;

        // Step 3: Filter CSS rules based on used classes with advanced options
        let filtered_css =
            self.rule_filter
                .filter_rules_advanced(css, &used_classes, &self.config, options)?;

        let processing_time = start_time.elapsed();
        let original_size = css.len();
        let purged_size = filtered_css.len();
        let size_reduction = if original_size > 0 {
            ((original_size - purged_size) as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        Ok(PurgeResult {
            purged_css: filtered_css,
            used_classes: used_classes.len(),
            removed_classes: css_classes.len() - used_classes.len(),
            original_size,
            purged_size,
            size_reduction_percentage: size_reduction,
            processing_time,
        })
    }

    /// Get purge statistics
    pub fn get_statistics(&self) -> PurgeStatistics {
        PurgeStatistics {
            total_purges: 0, // Will be updated during purging
            average_size_reduction: 0.0,
            processing_time: std::time::Duration::from_secs(0),
        }
    }
}

// Re-export types for convenience
pub use types::*;
