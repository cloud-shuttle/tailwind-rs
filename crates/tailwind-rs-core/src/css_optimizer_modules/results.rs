//! CSS Optimization Results Module
//!
//! Handles the results and statistics from CSS optimization operations:
//! - OptimizationResults struct for comprehensive optimization outcomes
//! - OptimizationStats struct for detailed statistics
//! - OptimizationTracker for internal tracking

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

impl OptimizationResults {
    /// Create new optimization results
    pub fn new(
        original_size: usize,
        optimized_size: usize,
        original_rules: usize,
        optimized_rules: usize,
        original_properties: usize,
        optimized_properties: usize,
        stats: OptimizationStats,
    ) -> Self {
        let size_reduction = original_size.saturating_sub(optimized_size);
        let reduction_percentage = if original_size > 0 {
            (size_reduction as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        Self {
            original_size,
            optimized_size,
            size_reduction,
            reduction_percentage,
            original_rules,
            optimized_rules,
            original_properties,
            optimized_properties,
            stats,
        }
    }

    /// Check if optimization was successful (reduced size)
    pub fn is_successful(&self) -> bool {
        self.optimized_size < self.original_size
    }

    /// Get the size reduction as a formatted string
    pub fn size_reduction_string(&self) -> String {
        format!("{:.2}% ({:.2}KB saved)",
                self.reduction_percentage,
                self.size_reduction as f64 / 1024.0)
    }

    /// Get a summary of the optimization results
    pub fn summary(&self) -> String {
        format!(
            "CSS Optimization Results:\n\
             Original: {} bytes ({} rules, {} properties)\n\
             Optimized: {} bytes ({} rules, {} properties)\n\
             Reduction: {}\n\
             Rules merged: {}\n\
             Properties optimized: {}\n\
             Empty rules removed: {}\n\
             Processing time: {}ms",
            self.original_size,
            self.original_rules,
            self.original_properties,
            self.optimized_size,
            self.optimized_rules,
            self.optimized_properties,
            self.size_reduction_string(),
            self.stats.rules_merged,
            self.stats.properties_optimized,
            self.stats.empty_rules_removed,
            self.stats.processing_time_ms
        )
    }
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

impl Default for OptimizationStats {
    fn default() -> Self {
        Self {
            rules_merged: 0,
            properties_optimized: 0,
            selectors_optimized: 0,
            empty_rules_removed: 0,
            duplicate_properties_removed: 0,
            processing_time_ms: 0,
        }
    }
}

impl OptimizationStats {
    /// Create new optimization stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Increment rules merged counter
    pub fn increment_rules_merged(&mut self) {
        self.rules_merged += 1;
    }

    /// Increment properties optimized counter
    pub fn increment_properties_optimized(&mut self) {
        self.properties_optimized += 1;
    }

    /// Increment selectors optimized counter
    pub fn increment_selectors_optimized(&mut self) {
        self.selectors_optimized += 1;
    }

    /// Increment empty rules removed counter
    pub fn increment_empty_rules_removed(&mut self) {
        self.empty_rules_removed += 1;
    }

    /// Increment duplicate properties removed counter
    pub fn increment_duplicate_properties_removed(&mut self) {
        self.duplicate_properties_removed += 1;
    }

    /// Set processing time
    pub fn set_processing_time(&mut self, time_ms: u64) {
        self.processing_time_ms = time_ms;
    }

    /// Get total optimizations performed
    pub fn total_optimizations(&self) -> usize {
        self.rules_merged +
        self.properties_optimized +
        self.selectors_optimized +
        self.empty_rules_removed +
        self.duplicate_properties_removed
    }

    /// Check if any optimizations were performed
    pub fn has_optimizations(&self) -> bool {
        self.total_optimizations() > 0
    }
}

/// Internal statistics tracking for optimization operations
#[derive(Debug, Clone, Default)]
pub struct OptimizationTracker {
    pub(crate) empty_rules_removed: usize,
    pub(crate) duplicate_properties_removed: usize,
    pub(crate) selectors_optimized: usize,
}

impl OptimizationTracker {
    /// Create new optimization tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Increment empty rules removed
    pub fn increment_empty_rules_removed(&mut self) {
        self.empty_rules_removed += 1;
    }

    /// Increment duplicate properties removed
    pub fn increment_duplicate_properties_removed(&mut self) {
        self.duplicate_properties_removed += 1;
    }

    /// Increment selectors optimized
    pub fn increment_selectors_optimized(&mut self) {
        self.selectors_optimized += 1;
    }

    /// Convert to OptimizationStats
    pub fn into_stats(self, processing_time_ms: u64) -> OptimizationStats {
        OptimizationStats {
            rules_merged: 0, // Not tracked in basic tracker
            properties_optimized: 0, // Not tracked in basic tracker
            selectors_optimized: self.selectors_optimized,
            empty_rules_removed: self.empty_rules_removed,
            duplicate_properties_removed: self.duplicate_properties_removed,
            processing_time_ms,
        }
    }

    /// Reset all counters
    pub fn reset(&mut self) {
        self.empty_rules_removed = 0;
        self.duplicate_properties_removed = 0;
        self.selectors_optimized = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimization_results_creation() {
        let stats = OptimizationStats::default();
        let results = OptimizationResults::new(1000, 800, 50, 45, 200, 180, stats);

        assert_eq!(results.original_size, 1000);
        assert_eq!(results.optimized_size, 800);
        assert_eq!(results.size_reduction, 200);
        assert_eq!(results.reduction_percentage, 20.0);
        assert_eq!(results.original_rules, 50);
        assert_eq!(results.optimized_rules, 45);
        assert_eq!(results.original_properties, 200);
        assert_eq!(results.optimized_properties, 180);
    }

    #[test]
    fn optimization_results_success_check() {
        let stats = OptimizationStats::default();
        let successful = OptimizationResults::new(1000, 800, 50, 45, 200, 180, stats);
        let unsuccessful = OptimizationResults::new(1000, 1000, 50, 50, 200, 200, stats);

        assert!(successful.is_successful());
        assert!(!unsuccessful.is_successful());
    }

    #[test]
    fn optimization_results_size_reduction_string() {
        let stats = OptimizationStats::default();
        let results = OptimizationResults::new(2048, 1024, 50, 45, 200, 180, stats);

        let reduction_str = results.size_reduction_string();
        assert!(reduction_str.contains("50.00%"));
        assert!(reduction_str.contains("1.00KB"));
    }

    #[test]
    fn optimization_stats_default() {
        let stats = OptimizationStats::default();

        assert_eq!(stats.rules_merged, 0);
        assert_eq!(stats.properties_optimized, 0);
        assert_eq!(stats.selectors_optimized, 0);
        assert_eq!(stats.empty_rules_removed, 0);
        assert_eq!(stats.duplicate_properties_removed, 0);
        assert_eq!(stats.processing_time_ms, 0);
    }

    #[test]
    fn optimization_stats_methods() {
        let mut stats = OptimizationStats::new();

        stats.increment_rules_merged();
        stats.increment_properties_optimized();
        stats.increment_selectors_optimized();
        stats.increment_empty_rules_removed();
        stats.increment_duplicate_properties_removed();
        stats.set_processing_time(150);

        assert_eq!(stats.rules_merged, 1);
        assert_eq!(stats.properties_optimized, 1);
        assert_eq!(stats.selectors_optimized, 1);
        assert_eq!(stats.empty_rules_removed, 1);
        assert_eq!(stats.duplicate_properties_removed, 1);
        assert_eq!(stats.processing_time_ms, 150);
        assert_eq!(stats.total_optimizations(), 5);
        assert!(stats.has_optimizations());
    }

    #[test]
    fn optimization_tracker() {
        let mut tracker = OptimizationTracker::new();

        tracker.increment_empty_rules_removed();
        tracker.increment_duplicate_properties_removed();
        tracker.increment_selectors_optimized();

        assert_eq!(tracker.empty_rules_removed, 1);
        assert_eq!(tracker.duplicate_properties_removed, 1);
        assert_eq!(tracker.selectors_optimized, 1);

        let stats = tracker.into_stats(100);
        assert_eq!(stats.empty_rules_removed, 1);
        assert_eq!(stats.duplicate_properties_removed, 1);
        assert_eq!(stats.selectors_optimized, 1);
        assert_eq!(stats.processing_time_ms, 100);
    }

    #[test]
    fn zero_size_handling() {
        let stats = OptimizationStats::default();
        let results = OptimizationResults::new(0, 0, 0, 0, 0, 0, stats);

        assert_eq!(results.reduction_percentage, 0.0);
        assert!(!results.is_successful());
    }
}
