//! Types and data structures for CSS optimization

use thiserror::Error;

/// Configuration for CSS optimization
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub merge_rules: bool,
    pub optimize_properties: bool,
    pub optimize_selectors: bool,
    pub minify: bool,
    pub level: OptimizationLevel,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            merge_rules: true,
            optimize_properties: true,
            optimize_selectors: true,
            minify: true,
            level: OptimizationLevel::Standard,
        }
    }
}

/// Optimization level
#[derive(Debug, Clone)]
pub enum OptimizationLevel {
    Minimal,
    Standard,
    Aggressive,
}

/// Result of CSS optimization
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub optimized_css: String,
    pub metrics: OptimizationMetrics,
    pub analysis: AnalysisReport,
}

/// Optimization metrics
#[derive(Debug, Clone)]
pub struct OptimizationMetrics {
    pub original_size: usize,
    pub optimized_size: usize,
    pub size_reduction_percentage: f64,
    pub rules_merged: usize,
    pub properties_optimized: usize,
    pub duplicates_removed: usize,
    pub selectors_optimized: usize,
    pub processing_time: std::time::Duration,
}

impl OptimizationMetrics {
    pub fn new() -> Self {
        Self {
            original_size: 0,
            optimized_size: 0,
            size_reduction_percentage: 0.0,
            rules_merged: 0,
            properties_optimized: 0,
            duplicates_removed: 0,
            selectors_optimized: 0,
            processing_time: std::time::Duration::from_secs(0),
        }
    }
}

/// Analysis report
#[derive(Debug, Clone)]
pub struct AnalysisReport {
    pub rules: RuleAnalysis,
    pub properties: PropertyAnalysis,
    pub selectors: SelectorAnalysis,
    pub analysis_time: std::time::Duration,
}

/// Rule analysis results
#[derive(Debug, Clone)]
pub struct RuleAnalysis {
    pub total_rules: usize,
    pub empty_rules: Vec<String>,
    pub duplicate_rules: Vec<DuplicateRule>,
    pub mergeable_rules: Vec<MergeableRule>,
}

/// Property analysis results
#[derive(Debug, Clone)]
pub struct PropertyAnalysis {
    pub total_properties: usize,
    pub duplicate_properties: Vec<DuplicateProperty>,
    pub redundant_properties: Vec<RedundantProperty>,
    pub optimizable_properties: Vec<OptimizableProperty>,
}

/// Selector analysis results
#[derive(Debug, Clone)]
pub struct SelectorAnalysis {
    pub total_selectors: usize,
    pub redundant_selectors: Vec<String>,
    pub optimizable_selectors: Vec<OptimizableSelector>,
    pub complex_selectors: Vec<ComplexSelector>,
}

/// CSS rule representation
#[derive(Debug, Clone)]
pub struct CSSRule {
    pub selector: String,
    pub properties: Vec<CSSProperty>,
}

/// CSS property representation
#[derive(Debug, Clone)]
pub struct CSSProperty {
    pub name: String,
    pub value: String,
}

/// Duplicate rule information
#[derive(Debug, Clone)]
pub struct DuplicateRule {
    pub selector: String,
    pub count: usize,
}

/// Mergeable rule information
#[derive(Debug, Clone)]
pub struct MergeableRule {
    pub selectors: Vec<String>,
    pub properties: Vec<CSSProperty>,
}

/// Duplicate property information
#[derive(Debug, Clone)]
pub struct DuplicateProperty {
    pub name: String,
    pub values: Vec<String>,
}

/// Redundant property information
#[derive(Debug, Clone)]
pub struct RedundantProperty {
    pub name: String,
    pub reason: String,
}

/// Optimizable property information
#[derive(Debug, Clone)]
pub struct OptimizableProperty {
    pub name: String,
    pub current_value: String,
    pub optimized_value: String,
}

/// Optimizable selector information
#[derive(Debug, Clone)]
pub struct OptimizableSelector {
    pub current: String,
    pub optimized: String,
}

/// Complex selector information
#[derive(Debug, Clone)]
pub struct ComplexSelector {
    pub selector: String,
    pub complexity_score: usize,
}

/// Property rule for optimization
#[derive(Debug, Clone)]
pub struct PropertyRule {
    pub name: String,
    pub shorthand: bool,
    pub longhand: Vec<String>,
}

/// Selector rule for optimization
#[derive(Debug, Clone)]
pub struct SelectorRule {
    pub name: String,
    pub pattern: String,
    pub optimization: SelectorOptimization,
}

/// Selector optimization type
#[derive(Debug, Clone)]
pub enum SelectorOptimization {
    Remove,
    Simplify,
    Merge,
}

/// Rule merge result
#[derive(Debug, Clone)]
pub struct RuleMergeResult {
    pub optimized_css: String,
    pub rules_merged: usize,
    pub merge_time: std::time::Duration,
}

/// Property optimization result
#[derive(Debug, Clone)]
pub struct PropertyOptimizationResult {
    pub optimized_css: String,
    pub properties_optimized: usize,
    pub duplicates_removed: usize,
    pub optimization_time: std::time::Duration,
}

/// Selector optimization result
#[derive(Debug, Clone)]
pub struct SelectorOptimizationResult {
    pub optimized_css: String,
    pub selectors_optimized: usize,
    pub optimization_time: std::time::Duration,
}

/// Optimization statistics
#[derive(Debug, Clone)]
pub struct OptimizationStatistics {
    pub total_optimizations: usize,
    pub average_size_reduction: f64,
    pub processing_time: std::time::Duration,
}

/// Error types for CSS optimization
#[derive(Debug, Error)]
pub enum OptimizationError {
    #[error("CSS parsing failed: {error}")]
    ParsingFailed { error: String },

    #[error("Optimization failed: {error}")]
    OptimizationFailed { error: String },

    #[error("Invalid CSS syntax: {error}")]
    InvalidCSS { error: String },

    #[error("Memory limit exceeded")]
    MemoryLimitExceeded,

    #[error("Processing timeout")]
    ProcessingTimeout,
}
