//! Error types for tailwind-rs-core

use thiserror::Error;

/// Result type alias for tailwind-rs operations
pub type Result<T> = std::result::Result<T, TailwindError>;

/// Main error type for tailwind-rs operations
#[derive(Error, Debug)]
pub enum TailwindError {
    /// Configuration errors
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Theme-related errors
    #[error("Theme error: {message}")]
    Theme { message: String },

    /// Class generation errors
    #[error("Class generation error: {message}")]
    ClassGeneration { message: String },

    /// Build process errors
    #[error("Build error: {message}")]
    Build { message: String },

    /// Validation errors
    #[error("Validation error: {message}")]
    Validation { message: String },

    /// File I/O errors
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Generic error wrapper
    #[error("Generic error: {0}")]
    Generic(#[from] anyhow::Error),
}

impl TailwindError {
    /// Create a new configuration error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a new theme error
    pub fn theme(message: impl Into<String>) -> Self {
        Self::Theme {
            message: message.into(),
        }
    }

    /// Create a new class generation error
    pub fn class_generation(message: impl Into<String>) -> Self {
        Self::ClassGeneration {
            message: message.into(),
        }
    }

    /// Create a new build error
    pub fn build(message: impl Into<String>) -> Self {
        Self::Build {
            message: message.into(),
        }
    }

    /// Create a new validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error() {
        let error = TailwindError::config("Invalid configuration");
        assert!(matches!(error, TailwindError::Config { .. }));
        assert!(error.to_string().contains("Invalid configuration"));
    }

    #[test]
    fn test_theme_error() {
        let error = TailwindError::theme("Theme not found");
        assert!(matches!(error, TailwindError::Theme { .. }));
        assert!(error.to_string().contains("Theme not found"));
    }

    #[test]
    fn test_class_generation_error() {
        let error = TailwindError::class_generation("Invalid class name");
        assert!(matches!(error, TailwindError::ClassGeneration { .. }));
        assert!(error.to_string().contains("Invalid class name"));
    }

    #[test]
    fn test_build_error() {
        let error = TailwindError::build("Build failed");
        assert!(matches!(error, TailwindError::Build { .. }));
        assert!(error.to_string().contains("Build failed"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let tailwind_error: TailwindError = io_error.into();
        assert!(matches!(tailwind_error, TailwindError::Io(_)));
    }

    #[test]
    fn test_json_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let tailwind_error: TailwindError = json_error.into();
        assert!(matches!(tailwind_error, TailwindError::Json(_)));
    }

    #[test]
    fn test_generic_error_conversion() {
        let anyhow_error = anyhow::anyhow!("Generic error");
        let tailwind_error: TailwindError = anyhow_error.into();
        assert!(matches!(tailwind_error, TailwindError::Generic(_)));
    }
}
