//! Validation errors
//!
//! This module provides validation error types and handling.

use thiserror::Error;

/// Represents validation errors
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid class name: {0}")]
    InvalidClass(String),

    #[error("Class conflict: {0} conflicts with {1}")]
    ClassConflict(String, String),

    #[error("Deprecated class: {0}")]
    DeprecatedClass(String),

    #[error("Unsupported class: {0}")]
    UnsupportedClass(String),

    #[error("Invalid custom variant: {0}")]
    InvalidCustomVariant(String),

    #[error("Custom variant validation failed: {0}")]
    CustomVariantValidation(String),
}

impl ValidationError {
    /// Get the error message
    pub fn message(&self) -> String {
        match self {
            ValidationError::InvalidClass(class) => format!("Invalid class name: {}", class),
            ValidationError::ClassConflict(class1, class2) => {
                format!("Class conflict: {} conflicts with {}", class1, class2)
            }
            ValidationError::DeprecatedClass(class) => format!("Deprecated class: {}", class),
            ValidationError::UnsupportedClass(class) => format!("Unsupported class: {}", class),
            ValidationError::InvalidCustomVariant(variant) => {
                format!("Invalid custom variant: {}", variant)
            }
            ValidationError::CustomVariantValidation(variant) => {
                format!("Custom variant validation failed: {}", variant)
            }
        }
    }

    /// Check if this is a critical error
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            ValidationError::InvalidClass(_) | ValidationError::ClassConflict(_, _)
        )
    }
}
