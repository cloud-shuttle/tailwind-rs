//! Error handling for PostCSS integration
//!
//! This module provides comprehensive error types and handling for all
//! PostCSS-related operations.

use thiserror::Error;

/// Main error type for PostCSS operations
#[derive(Error, Debug, Clone)]
pub enum PostCSSError {
    #[error("Parse error: {message} at line {line}, column {column}")]
    ParseError {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Transform error: {message}")]
    TransformError { message: String },

    #[error("Plugin error: {message}")]
    PluginError { message: String },

    #[error("JavaScript bridge error: {message}")]
    JavaScriptBridgeError { message: String },

    #[error("Source map error: {message}")]
    SourceMapError { message: String },

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("Invalid AST: {0}")]
    InvalidAST(String),

    #[error("JavaScript bridge not available")]
    JavaScriptBridgeNotAvailable,

    #[error("Plugin not found: {name}")]
    PluginNotFound { name: String },

    #[error("Plugin execution failed: {name} - {message}")]
    PluginExecutionFailed { name: String, message: String },

    #[error("Memory allocation failed: {message}")]
    MemoryError { message: String },

    #[error("File I/O error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Generic error: {0}")]
    Generic(String),
}

/// Result type alias for PostCSS operations
pub type Result<T> = std::result::Result<T, PostCSSError>;

impl From<std::io::Error> for PostCSSError {
    fn from(err: std::io::Error) -> Self {
        PostCSSError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for PostCSSError {
    fn from(err: serde_json::Error) -> Self {
        PostCSSError::SerializationError(err.to_string())
    }
}

impl From<anyhow::Error> for PostCSSError {
    fn from(err: anyhow::Error) -> Self {
        PostCSSError::Generic(err.to_string())
    }
}

impl PostCSSError {
    /// Create a configuration error
    pub fn config(message: &str) -> Self {
        PostCSSError::ConfigError {
            message: message.to_string(),
        }
    }
}

/// Error context for better debugging
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub additional_info: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    pub fn new(operation: String) -> Self {
        Self {
            operation,
            file: None,
            line: None,
            column: None,
            additional_info: std::collections::HashMap::new(),
        }
    }

    pub fn with_file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }

    pub fn with_position(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    pub fn with_info(mut self, key: String, value: String) -> Self {
        self.additional_info.insert(key, value);
        self
    }
}

/// Enhanced error with context
#[derive(Debug, Clone)]
pub struct ContextualError {
    pub error: PostCSSError,
    pub context: ErrorContext,
}

impl ContextualError {
    pub fn new(error: PostCSSError, context: ErrorContext) -> Self {
        Self { error, context }
    }

    pub fn with_context(error: PostCSSError, operation: &str) -> Self {
        Self {
            error,
            context: ErrorContext::new(operation.to_string()),
        }
    }
}

impl std::fmt::Display for ContextualError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} in {}", self.error, self.context.operation)?;

        if let Some(file) = &self.context.file {
            write!(f, " (file: {})", file)?;
        }

        if let (Some(line), Some(column)) = (self.context.line, self.context.column) {
            write!(f, " at line {}, column {}", line, column)?;
        }

        if !self.context.additional_info.is_empty() {
            write!(f, " [Additional info: ")?;
            for (key, value) in &self.context.additional_info {
                write!(f, "{}={}, ", key, value)?;
            }
            write!(f, "]")?;
        }

        Ok(())
    }
}

impl std::error::Error for ContextualError {}

/// Error recovery strategies
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Skip the problematic operation and continue
    Skip,
    /// Use a fallback value
    Fallback(String),
    /// Retry with different parameters
    Retry,
    /// Abort the entire operation
    Abort,
}

/// Error handler trait for custom error handling
pub trait ErrorHandler {
    fn handle_parse_error(&self, error: &PostCSSError) -> RecoveryStrategy;
    fn handle_plugin_error(&self, error: &PostCSSError) -> RecoveryStrategy;
    fn handle_transform_error(&self, error: &PostCSSError) -> RecoveryStrategy;
}

/// Default error handler implementation
#[derive(Debug, Clone)]
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn handle_parse_error(&self, _error: &PostCSSError) -> RecoveryStrategy {
        RecoveryStrategy::Abort
    }

    fn handle_plugin_error(&self, _error: &PostCSSError) -> RecoveryStrategy {
        RecoveryStrategy::Skip
    }

    fn handle_transform_error(&self, _error: &PostCSSError) -> RecoveryStrategy {
        RecoveryStrategy::Abort
    }
}

/// Error reporting utilities
pub struct ErrorReporter {
    errors: Vec<ContextualError>,
    warnings: Vec<ContextualError>,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: PostCSSError, context: ErrorContext) {
        self.errors.push(ContextualError::new(error, context));
    }

    pub fn add_warning(&mut self, error: PostCSSError, context: ErrorContext) {
        self.warnings.push(ContextualError::new(error, context));
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn get_errors(&self) -> &[ContextualError] {
        &self.errors
    }

    pub fn get_warnings(&self) -> &[ContextualError] {
        &self.warnings
    }

    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
    }

    pub fn to_report(&self) -> String {
        let mut report = String::new();

        if !self.errors.is_empty() {
            report.push_str("Errors:\n");
            for (i, error) in self.errors.iter().enumerate() {
                report.push_str(&format!("  {}. {}\n", i + 1, error));
            }
        }

        if !self.warnings.is_empty() {
            report.push_str("Warnings:\n");
            for (i, warning) in self.warnings.iter().enumerate() {
                report.push_str(&format!("  {}. {}\n", i + 1, warning));
            }
        }

        report
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = PostCSSError::ParseError {
            message: "Invalid CSS syntax".to_string(),
            line: 10,
            column: 5,
        };

        assert!(error.to_string().contains("Parse error"));
        assert!(error.to_string().contains("line 10"));
        assert!(error.to_string().contains("column 5"));
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("CSS parsing".to_string())
            .with_file("styles.css".to_string())
            .with_position(10, 5)
            .with_info("selector".to_string(), ".test".to_string());

        assert_eq!(context.operation, "CSS parsing");
        assert_eq!(context.file, Some("styles.css".to_string()));
        assert_eq!(context.line, Some(10));
        assert_eq!(context.column, Some(5));
        assert_eq!(
            context.additional_info.get("selector"),
            Some(&".test".to_string())
        );
    }

    #[test]
    fn test_error_reporter() {
        let mut reporter = ErrorReporter::new();

        let error = PostCSSError::ParseError {
            message: "Invalid syntax".to_string(),
            line: 1,
            column: 1,
        };
        let context = ErrorContext::new("parsing".to_string());

        reporter.add_error(error, context);

        assert!(reporter.has_errors());
        assert!(!reporter.has_warnings());
        assert_eq!(reporter.get_errors().len(), 1);
    }

    #[test]
    fn test_recovery_strategies() {
        let handler = DefaultErrorHandler;

        let parse_error = PostCSSError::ParseError {
            message: "Invalid CSS".to_string(),
            line: 1,
            column: 1,
        };

        let strategy = handler.handle_parse_error(&parse_error);
        assert!(matches!(strategy, RecoveryStrategy::Abort));
    }
}
