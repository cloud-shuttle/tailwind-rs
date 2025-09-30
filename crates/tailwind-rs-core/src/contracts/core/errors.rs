//! Contract Errors
//!
//! Comprehensive error types for contract violations and API stability issues.

use std::collections::HashMap;

/// Primary contract error types
#[derive(Debug, Clone, PartialEq)]
pub enum ContractError {
    /// Input data violates contract requirements
    InvalidInput(String),

    /// Input data is structurally invalid
    MalformedInput { field: String, reason: String },

    /// Input data violates business rules
    BusinessRuleViolation { rule: String, details: String },

    /// Internal processing failure
    ProcessingError(String),

    /// External dependency failure
    DependencyError { dependency: String, error: String },

    /// Resource exhaustion
    ResourceExhausted { resource: String, limit: u64, used: u64 },

    /// Output data violates contract guarantees
    InvalidOutput(String),

    /// Output incomplete or missing required fields
    IncompleteOutput { missing_fields: Vec<String> },

    /// Output inconsistent with input promises
    InconsistentOutput { expected: String, actual: String },

    /// Backward compatibility violation
    BackwardCompatibilityViolation(String),

    /// Version incompatibility
    VersionMismatch {
        expected: super::traits::ApiVersion,
        actual: super::traits::ApiVersion
    },

    /// Breaking API change detected
    BreakingChange {
        component: String,
        change_type: BreakingChangeType
    },
}

/// Types of breaking changes
#[derive(Debug, Clone, PartialEq)]
pub enum BreakingChangeType {
    RemovedMethod,
    ChangedSignature,
    ChangedBehavior,
    RemovedField,
    ChangedFieldType,
    AddedRequiredField,
}

/// Display implementation for contract errors
impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractError::InvalidInput(msg) =>
                write!(f, "Invalid input: {}", msg),
            ContractError::MalformedInput { field, reason } =>
                write!(f, "Malformed input in field '{}': {}", field, reason),
            ContractError::BusinessRuleViolation { rule, details } =>
                write!(f, "Business rule violation '{}': {}", rule, details),
            ContractError::ProcessingError(msg) =>
                write!(f, "Processing error: {}", msg),
            ContractError::DependencyError { dependency, error } =>
                write!(f, "Dependency '{}' error: {}", dependency, error),
            ContractError::ResourceExhausted { resource, limit, used } =>
                write!(f, "Resource '{}' exhausted: {}/{} used", resource, used, limit),
            ContractError::InvalidOutput(msg) =>
                write!(f, "Invalid output: {}", msg),
            ContractError::IncompleteOutput { missing_fields } =>
                write!(f, "Incomplete output, missing fields: {:?}", missing_fields),
            ContractError::InconsistentOutput { expected, actual } =>
                write!(f, "Inconsistent output - expected: {}, actual: {}", expected, actual),
            ContractError::BackwardCompatibilityViolation(msg) =>
                write!(f, "Backward compatibility violation: {}", msg),
            ContractError::VersionMismatch { expected, actual } =>
                write!(f, "Version mismatch - expected: {}, actual: {}", expected, actual),
            ContractError::BreakingChange { component, change_type } =>
                write!(f, "Breaking change in '{}' of type: {:?}", component, change_type),
        }
    }
}

impl std::error::Error for ContractError {}

/// Convert standard library errors to contract errors
impl From<std::io::Error> for ContractError {
    fn from(err: std::io::Error) -> Self {
        ContractError::ProcessingError(format!("IO error: {}", err))
    }
}

impl From<serde_json::Error> for ContractError {
    fn from(err: serde_json::Error) -> Self {
        ContractError::MalformedInput {
            field: "json".to_string(),
            reason: err.to_string(),
        }
    }
}

// Note: serde_yaml support can be added when the dependency is available
// impl From<serde_yaml::Error> for ContractError {
//     fn from(err: serde_yaml::Error) -> Self {
//         ContractError::MalformedInput {
//             field: "yaml".to_string(),
//             reason: err.to_string(),
//         }
//     }
// }

/// Context-aware error with additional information
#[derive(Debug, Clone)]
pub struct ContextualError {
    pub error: ContractError,
    pub context: ErrorContext,
}

/// Error context for debugging and monitoring
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub input_size: Option<usize>,
    pub processing_time: Option<std::time::Duration>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
}

impl ErrorContext {
    pub fn new(operation: &str) -> Self {
        Self {
            operation: operation.to_string(),
            timestamp: chrono::Utc::now(),
            input_size: None,
            processing_time: None,
            user_id: None,
            session_id: None,
        }
    }

    pub fn with_input_size(mut self, size: usize) -> Self {
        self.input_size = Some(size);
        self
    }

    pub fn with_processing_time(mut self, time: std::time::Duration) -> Self {
        self.processing_time = Some(time);
        self
    }

    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }
}

impl ContractError {
    /// Create a contextual error
    pub fn with_context(self, context: ErrorContext) -> ContextualError {
        ContextualError { error: self, context }
    }

    /// Get error code for external systems
    pub fn error_code(&self) -> u32 {
        match self {
            ContractError::InvalidInput(_) => 1001,
            ContractError::MalformedInput { .. } => 1002,
            ContractError::BusinessRuleViolation { .. } => 1003,
            ContractError::ProcessingError(_) => 2001,
            ContractError::DependencyError { .. } => 2002,
            ContractError::ResourceExhausted { .. } => 2003,
            ContractError::InvalidOutput(_) => 3001,
            ContractError::IncompleteOutput { .. } => 3002,
            ContractError::InconsistentOutput { .. } => 3003,
            ContractError::BackwardCompatibilityViolation(_) => 4001,
            ContractError::VersionMismatch { .. } => 4002,
            ContractError::BreakingChange { .. } => 4003,
        }
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(self,
            ContractError::ProcessingError(_) |
            ContractError::DependencyError { .. } |
            ContractError::ResourceExhausted { .. }
        )
    }

    /// Get recovery strategy for the error
    pub fn recovery_strategy(&self) -> RecoveryStrategy {
        match self {
            ContractError::ProcessingError(_) =>
                RecoveryStrategy::Retry { max_attempts: 3, backoff: std::time::Duration::from_millis(100) },
            ContractError::DependencyError { .. } =>
                RecoveryStrategy::Retry { max_attempts: 2, backoff: std::time::Duration::from_millis(500) },
            ContractError::ResourceExhausted { .. } =>
                RecoveryStrategy::Fail { critical: false },
            ContractError::BreakingChange { .. } =>
                RecoveryStrategy::Fail { critical: true },
            _ => RecoveryStrategy::Fail { critical: false },
        }
    }
}

/// Recovery strategies for errors
#[derive(Clone)]
pub enum RecoveryStrategy {
    Retry { max_attempts: u32, backoff: std::time::Duration },
    Fallback { alternative: String }, // Simplified - would need FnOnce in real implementation
    Skip { reason: String },
    Fail { critical: bool },
}

impl std::fmt::Debug for RecoveryStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecoveryStrategy::Retry { max_attempts, backoff } =>
                write!(f, "Retry {{ max_attempts: {}, backoff: {:?} }}", max_attempts, backoff),
            RecoveryStrategy::Fallback { alternative } =>
                write!(f, "Fallback {{ alternative: {} }}", alternative),
            RecoveryStrategy::Skip { reason } =>
                write!(f, "Skip {{ reason: {} }}", reason),
            RecoveryStrategy::Fail { critical } =>
                write!(f, "Fail {{ critical: {} }}", critical),
        }
    }
}

/// Error metrics for monitoring and alerting
#[derive(Debug, Clone, Default)]
pub struct ErrorMetrics {
    pub error_counts: HashMap<String, u64>,
    pub error_rates: HashMap<String, f64>,
    pub recent_errors: std::collections::VecDeque<ContextualError>,
    pub time_window: std::time::Duration,
}

impl ErrorMetrics {
    pub fn new(time_window: std::time::Duration) -> Self {
        Self {
            error_counts: HashMap::new(),
            error_rates: HashMap::new(),
            recent_errors: std::collections::VecDeque::new(),
            time_window,
        }
    }

    pub fn record_error(&mut self, error: ContextualError) {
        let error_key = format!("{:?}", error.error);

        // Update counts
        *self.error_counts.entry(error_key.clone()).or_insert(0) += 1;

        // Add to recent errors, maintaining time window
        self.recent_errors.push_back(error);
        self.cleanup_old_errors();

        // Update rates (simplified calculation)
        let total_errors: u64 = self.error_counts.values().sum();
        let window_seconds = self.time_window.as_secs_f64();
        self.error_rates.insert(error_key, total_errors as f64 / window_seconds);
    }

    pub fn get_error_rate(&self, error_type: &str) -> f64 {
        self.error_rates.get(error_type).copied().unwrap_or(0.0)
    }

    pub fn get_total_errors(&self) -> u64 {
        self.error_counts.values().sum()
    }

    fn cleanup_old_errors(&mut self) {
        let cutoff = chrono::Utc::now() - self.time_window;
        while let Some(error) = self.recent_errors.front() {
            if error.context.timestamp < cutoff {
                self.recent_errors.pop_front();
            } else {
                break;
            }
        }
    }
}

/// Error threshold configuration
#[derive(Debug, Clone)]
pub struct ErrorThresholds {
    pub max_errors_per_minute: u32,
    pub max_error_rate: f64,
    pub alert_on_breaking_changes: bool,
    pub alert_on_version_mismatches: bool,
}

impl Default for ErrorThresholds {
    fn default() -> Self {
        Self {
            max_errors_per_minute: 10,
            max_error_rate: 0.1,
            alert_on_breaking_changes: true,
            alert_on_version_mismatches: true,
        }
    }
}

impl ErrorThresholds {
    pub fn check_thresholds(&self, metrics: &ErrorMetrics) -> Vec<ThresholdViolation> {
        let mut violations = Vec::new();

        // Check error rate
        let total_rate: f64 = metrics.error_rates.values().sum();
        if total_rate > self.max_error_rate {
            violations.push(ThresholdViolation::ErrorRateExceeded {
                current: total_rate,
                threshold: self.max_error_rate,
            });
        }

        // Check specific error types
        for (error_type, count) in &metrics.error_counts {
            if error_type.contains("BreakingChange") && self.alert_on_breaking_changes {
                violations.push(ThresholdViolation::BreakingChangeDetected {
                    error_type: error_type.clone(),
                    count: *count,
                });
            }

            if error_type.contains("VersionMismatch") && self.alert_on_version_mismatches {
                violations.push(ThresholdViolation::VersionMismatchDetected {
                    error_type: error_type.clone(),
                    count: *count,
                });
            }
        }

        violations
    }
}

/// Threshold violation types
#[derive(Debug, Clone)]
pub enum ThresholdViolation {
    ErrorRateExceeded { current: f64, threshold: f64 },
    BreakingChangeDetected { error_type: String, count: u64 },
    VersionMismatchDetected { error_type: String, count: u64 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_formatting() {
        let err = ContractError::InvalidInput("test message".to_string());
        assert_eq!(err.to_string(), "Invalid input: test message");
    }

    #[test]
    fn error_code_assignment() {
        assert_eq!(ContractError::InvalidInput("test".to_string()).error_code(), 1001);
        assert_eq!(ContractError::ProcessingError("test".to_string()).error_code(), 2001);
        assert_eq!(ContractError::BreakingChange {
            component: "test".to_string(),
            change_type: BreakingChangeType::RemovedMethod
        }.error_code(), 4003);
    }

    #[test]
    fn error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let contract_err: ContractError = io_err.into();
        assert!(matches!(contract_err, ContractError::ProcessingError(_)));
    }

    #[test]
    fn recovery_strategy() {
        let processing_err = ContractError::ProcessingError("test".to_string());
        let strategy = processing_err.recovery_strategy();
        assert!(matches!(strategy, RecoveryStrategy::Retry { .. }));

        let breaking_change = ContractError::BreakingChange {
            component: "test".to_string(),
            change_type: BreakingChangeType::RemovedMethod,
        };
        let strategy = breaking_change.recovery_strategy();
        assert!(matches!(strategy, RecoveryStrategy::Fail { critical: true }));
    }

    #[test]
    fn error_metrics_tracking() {
        let mut metrics = ErrorMetrics::new(std::time::Duration::from_secs(60));

        let context = ErrorContext::new("test_operation");
        let error = ContractError::InvalidInput("test".to_string()).with_context(context);

        metrics.record_error(error);
        assert_eq!(metrics.get_total_errors(), 1);
        assert!(metrics.get_error_rate("InvalidInput(\"test\")") > 0.0);
    }

    #[test]
    fn threshold_checking() {
        let thresholds = ErrorThresholds {
            max_error_rate: 0.5,
            ..Default::default()
        };

        let mut metrics = ErrorMetrics::new(std::time::Duration::from_secs(60));
        // Add multiple errors to trigger threshold
        for i in 0..10 {
            let context = ErrorContext::new(&format!("test_operation_{}", i));
            let error = ContractError::InvalidInput(format!("test {}", i)).with_context(context);
            metrics.record_error(error);
        }

        let violations = thresholds.check_thresholds(&metrics);
        assert!(!violations.is_empty());
    }
}
