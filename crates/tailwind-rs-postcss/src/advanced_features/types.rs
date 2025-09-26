//! Types and data structures for advanced PostCSS features

use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use thiserror::Error;

/// CSS linting configuration
#[derive(Debug, Clone)]
pub struct LinterConfig {
    pub rules: HashMap<String, RuleConfig>,
    pub severity: SeverityLevel,
    pub auto_fix: bool,
    pub ignore_patterns: Vec<String>,
    pub custom_rules: Vec<CustomRule>,
}

impl Default for LinterConfig {
    fn default() -> Self {
        Self {
            rules: get_default_rules(),
            severity: SeverityLevel::Warning,
            auto_fix: false,
            ignore_patterns: Vec::new(),
            custom_rules: Vec::new(),
        }
    }
}

fn get_default_rules() -> HashMap<String, RuleConfig> {
    let mut rules = HashMap::new();

    rules.insert(
        "no-duplicate-selectors".to_string(),
        RuleConfig {
            enabled: true,
            severity: SeverityLevel::Warning,
            options: HashMap::new(),
        },
    );

    rules.insert(
        "no-empty-rules".to_string(),
        RuleConfig {
            enabled: true,
            severity: SeverityLevel::Warning,
            options: HashMap::new(),
        },
    );

    rules.insert(
        "no-important".to_string(),
        RuleConfig {
            enabled: true,
            severity: SeverityLevel::Warning,
            options: HashMap::new(),
        },
    );

    rules.insert(
        "selector-max-specificity".to_string(),
        RuleConfig {
            enabled: true,
            severity: SeverityLevel::Warning,
            options: {
                let mut opts = HashMap::new();
                opts.insert(
                    "max".to_string(),
                    Value::Number(serde_json::Number::from(3)),
                );
                opts
            },
        },
    );

    rules
}

/// Rule configuration
#[derive(Debug, Clone)]
pub struct RuleConfig {
    pub enabled: bool,
    pub severity: SeverityLevel,
    pub options: HashMap<String, Value>,
}

/// Severity level for linting
#[derive(Debug, Clone, PartialEq)]
pub enum SeverityLevel {
    Error,
    Warning,
    Info,
    Off,
}

/// Linting options
#[derive(Debug, Clone)]
pub struct LintOptions {
    pub auto_fix: bool,
    pub ignore_warnings: bool,
    pub max_issues: Option<usize>,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

impl Default for LintOptions {
    fn default() -> Self {
        Self {
            auto_fix: false,
            ignore_warnings: false,
            max_issues: None,
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
        }
    }
}

/// Lint result
#[derive(Debug, Clone)]
pub struct LintResult {
    pub issues: Vec<LintIssue>,
    pub fixes: Vec<LintFix>,
    pub statistics: LintStatistics,
    pub suggestions: Vec<LintSuggestion>,
}

/// Lint issue
#[derive(Debug, Clone)]
pub struct LintIssue {
    pub rule: String,
    pub severity: SeverityLevel,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub end_line: Option<usize>,
    pub end_column: Option<usize>,
    pub fix: Option<LintFix>,
}

/// Lint fix
#[derive(Debug, Clone)]
pub struct LintFix {
    pub rule: String,
    pub message: String,
    pub fix_type: FixType,
    pub replacement: String,
    pub range: TextRange,
}

/// Fix type
#[derive(Debug, Clone)]
pub enum FixType {
    Replace,
    Insert,
    Delete,
    Reorder,
}

/// Text range
#[derive(Debug, Clone)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

impl TextRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// Lint statistics
#[derive(Debug, Clone)]
pub struct LintStatistics {
    pub total_issues: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub fixable_count: usize,
}

/// Lint suggestion
#[derive(Debug, Clone)]
pub struct LintSuggestion {
    pub rule: String,
    pub message: String,
    pub severity: SeverityLevel,
    pub line: usize,
    pub column: usize,
    pub fix: Option<LintFix>,
}

/// Custom rule
#[derive(Debug, Clone)]
pub struct CustomRule {
    pub name: String,
    pub description: String,
    pub severity: SeverityLevel,
    pub enabled: bool,
    pub options: HashMap<String, Value>,
}

/// Source file information
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: String,
    pub content: String,
    pub mtime: SystemTime,
    pub size: usize,
}

/// Source mapping
#[derive(Debug, Clone)]
pub struct SourceMapping {
    pub generated_line: usize,
    pub generated_column: usize,
    pub source_line: usize,
    pub source_column: usize,
    pub source_file: Option<usize>,
    pub name: Option<usize>,
}

/// Transformation information
#[derive(Debug, Clone)]
pub struct Transformation {
    pub name: String,
    pub input_range: TextRange,
    pub output_range: TextRange,
    pub source_file: String,
}

/// Source location
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub content: String,
}

/// Performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub total_time: Duration,
    pub parsing_time: Duration,
    pub transformation_time: Duration,
    pub generation_time: Duration,
    pub memory_usage: usize,
    pub cpu_usage: f64,
}

/// Operation metrics
#[derive(Debug, Clone)]
pub struct OperationMetrics {
    pub operation: String,
    pub duration: Duration,
    pub memory_delta: i64,
    pub cpu_usage: f64,
    pub input_size: usize,
    pub output_size: usize,
}

/// Performance alert
#[derive(Debug, Clone)]
pub struct PerformanceAlert {
    pub operation: String,
    pub issue: String,
    pub severity: AlertSeverity,
    pub timestamp: SystemTime,
    pub metrics: OperationMetrics,
}

/// Alert severity
#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub total_time: Duration,
    pub operations: HashMap<String, OperationMetrics>,
    pub memory_usage: usize,
    pub cpu_usage: f64,
    pub alerts: Vec<PerformanceAlert>,
    pub recommendations: Vec<String>,
}

/// Debug information
#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub steps: Vec<StepDebugInfo>,
    pub total_time: Duration,
    pub memory_usage: usize,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            total_time: Duration::from_secs(0),
            memory_usage: 0,
        }
    }
}

/// Step debug information
#[derive(Debug, Clone)]
pub struct StepDebugInfo {
    pub step_name: String,
    pub step_index: usize,
    pub input_css: String,
    pub input_size: usize,
    pub output_css: String,
    pub output_size: usize,
    pub transformations: Vec<Transformation>,
    pub performance: OperationMetrics,
}

/// CSS analysis
#[derive(Debug, Clone)]
pub struct CSSAnalysis {
    pub selectors: SelectorAnalysis,
    pub properties: PropertyAnalysis,
    pub specificity: SpecificityAnalysis,
    pub performance: PerformanceAnalysis,
    pub maintainability: MaintainabilityAnalysis,
}

/// Selector analysis
#[derive(Debug, Clone)]
pub struct SelectorAnalysis {
    pub total_selectors: usize,
    pub complex_selectors: usize,
    pub duplicate_selectors: usize,
    pub specificity_scores: Vec<usize>,
}

/// Property analysis
#[derive(Debug, Clone)]
pub struct PropertyAnalysis {
    pub total_properties: usize,
    pub duplicate_properties: usize,
    pub unused_properties: usize,
    pub property_usage: HashMap<String, usize>,
}

/// Specificity analysis
#[derive(Debug, Clone)]
pub struct SpecificityAnalysis {
    pub max_specificity: usize,
    pub avg_specificity: f64,
    pub high_specificity_selectors: Vec<String>,
}

/// Performance analysis
#[derive(Debug, Clone)]
pub struct PerformanceAnalysis {
    pub estimated_size: usize,
    pub complexity_score: f64,
    pub optimization_opportunities: Vec<String>,
}

/// Maintainability analysis
#[derive(Debug, Clone)]
pub struct MaintainabilityAnalysis {
    pub maintainability_score: f64,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Format options
#[derive(Debug, Clone)]
pub struct FormatOptions {
    pub rules: Vec<FormatRule>,
    pub indent_size: usize,
    pub use_tabs: bool,
    pub line_ending: LineEnding,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            rules: vec![FormatRule::Indentation(2), FormatRule::LineBreaks(true)],
            indent_size: 2,
            use_tabs: false,
            line_ending: LineEnding::LF,
        }
    }
}

/// Format rule
#[derive(Debug, Clone)]
pub enum FormatRule {
    Indentation(usize),
    LineBreaks(bool),
    Spacing(SpacingRule),
    Sorting(SortOptions),
}

/// Spacing rule
#[derive(Debug, Clone)]
pub struct SpacingRule {
    pub before_selector: bool,
    pub after_selector: bool,
    pub before_property: bool,
    pub after_property: bool,
}

/// Sort options
#[derive(Debug, Clone)]
pub struct SortOptions {
    pub sort_properties: bool,
    pub sort_selectors: bool,
    pub custom_order: Vec<String>,
}

/// Line ending
#[derive(Debug, Clone)]
pub enum LineEnding {
    LF,
    CRLF,
    CR,
}

/// Error types for advanced features
#[derive(Debug, Error)]
pub enum AdvancedFeatureError {
    #[error("Linting failed: {error}")]
    LintingFailed { error: String },

    #[error("Source map generation failed: {error}")]
    SourceMapFailed { error: String },

    #[error("Performance monitoring failed: {error}")]
    PerformanceMonitoringFailed { error: String },

    #[error("Debugging failed: {error}")]
    DebuggingFailed { error: String },

    #[error("Analysis failed: {error}")]
    AnalysisFailed { error: String },

    #[error("Formatting failed: {error}")]
    FormattingFailed { error: String },
}
