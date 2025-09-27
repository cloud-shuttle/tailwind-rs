//! Errors for Autoprefixer
//!
//! This module provides error types for autoprefixer functionality.

use thiserror::Error;

/// Errors that can occur during autoprefixer operations
#[derive(Error, Debug)]
pub enum AutoprefixerError {
    #[error("Invalid CSS: {0}")]
    InvalidCss(String),
    
    #[error("Unsupported browser: {0}")]
    UnsupportedBrowser(String),
    
    #[error("Invalid property: {0}")]
    InvalidProperty(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Prefix generation error: {0}")]
    PrefixGenerationError(String),
    
    #[error("Browser data error: {0}")]
    BrowserDataError(String),
    
    #[error("CanIUse data error: {0}")]
    CanIUseDataError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}
