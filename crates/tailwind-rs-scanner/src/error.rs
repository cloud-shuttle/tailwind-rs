//! Error handling for content scanning
//!
//! This module provides error types and handling for all
//! content scanning operations.

use thiserror::Error;

/// Main error type for content scanning operations
#[derive(Error, Debug, Clone)]
pub enum ScannerError {
    #[error("File I/O error: {0}")]
    IoError(String),
    
    #[error("File content not available: {0}")]
    FileContentNotAvailable(String),
    
    #[error("Pattern matching error: {0}")]
    PatternError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Watch error: {0}")]
    WatchError(String),
    
    #[error("Tree-sitter error: {0}")]
    TreeSitterError(String),
    
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),
    
    #[error("Generic error: {0}")]
    Generic(String),
}

/// Result type alias for content scanning operations
pub type Result<T> = std::result::Result<T, ScannerError>;

impl From<std::io::Error> for ScannerError {
    fn from(err: std::io::Error) -> Self {
        ScannerError::IoError(err.to_string())
    }
}

impl From<regex::Error> for ScannerError {
    fn from(err: regex::Error) -> Self {
        ScannerError::PatternError(err.to_string())
    }
}

impl From<anyhow::Error> for ScannerError {
    fn from(err: anyhow::Error) -> Self {
        ScannerError::Generic(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_error_creation() {
        let error = ScannerError::IoError("File not found".to_string());
        assert!(error.to_string().contains("File I/O error"));
    }

    #[test]
    fn test_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let scanner_error: ScannerError = io_error.into();
        assert!(matches!(scanner_error, ScannerError::IoError(_)));
    }
}
