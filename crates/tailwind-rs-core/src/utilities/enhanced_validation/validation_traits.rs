//! Validation traits for enhanced validation utilities
//!
//! This module contains the trait definitions and implementations for
//! enhanced validation functionality.

use crate::classes::ClassBuilder;
use std::collections::HashMap;

use super::validation_types::*;

/// Trait for adding enhanced validation to ClassBuilder
pub trait EnhancedValidationUtilities {
    /// Set validation rule
    fn validation_rule(self, rule: ValidationRule) -> Self;
    /// Set validation severity
    fn validation_severity(self, severity: ValidationSeverity) -> Self;
    /// Set validation scope
    fn validation_scope(self, scope: ValidationScope) -> Self;
    /// Set validation mode
    fn validation_mode(self, mode: ValidationMode) -> Self;
    /// Set validation result
    fn validation_result(self, result: ValidationResult) -> Self;
    /// Set validation with custom options
    fn validation_custom(self, name: &str, options: HashMap<String, String>) -> Self;
}

impl EnhancedValidationUtilities for ClassBuilder {
    fn validation_rule(self, rule: ValidationRule) -> Self {
        self.class(rule.to_class_name())
    }

    fn validation_severity(self, severity: ValidationSeverity) -> Self {
        self.class(severity.to_class_name())
    }

    fn validation_scope(self, scope: ValidationScope) -> Self {
        self.class(scope.to_class_name())
    }

    fn validation_mode(self, mode: ValidationMode) -> Self {
        self.class(mode.to_class_name())
    }

    fn validation_result(self, result: ValidationResult) -> Self {
        self.class(result.to_class_name())
    }

    fn validation_custom(self, name: &str, _options: HashMap<String, String>) -> Self {
        let validation_class = format!("validation-{}", name);
        self.class(&validation_class)
    }
}

/// Convenience methods for common validation patterns
pub trait EnhancedValidationConvenience {
    /// Set required validation
    fn validation_required(self) -> Self;
    /// Set pattern validation
    fn validation_pattern(self, pattern: &str) -> Self;
    /// Set length validation
    fn validation_length(self, min: usize, max: usize) -> Self;
    /// Set range validation
    fn validation_range(self, min: f64, max: f64) -> Self;
    /// Set error severity
    fn validation_error(self) -> Self;
    /// Set warning severity
    fn validation_warning(self) -> Self;
    /// Set info severity
    fn validation_info(self) -> Self;
    /// Set success severity
    fn validation_success(self) -> Self;
    /// Set global scope
    fn validation_global(self) -> Self;
    /// Set local scope
    fn validation_local(self) -> Self;
    /// Set component scope
    fn validation_component(self) -> Self;
    /// Set page scope
    fn validation_page(self) -> Self;
    /// Set strict mode
    fn validation_strict(self) -> Self;
    /// Set loose mode
    fn validation_loose(self) -> Self;
    /// Set custom mode
    fn validation_custom_mode(self) -> Self;
    /// Set disabled mode
    fn validation_disabled(self) -> Self;
    /// Set valid result
    fn validation_valid(self) -> Self;
    /// Set invalid result
    fn validation_invalid(self, message: &str) -> Self;
    /// Set warning result
    fn validation_warning_result(self, message: &str) -> Self;
    /// Set info result
    fn validation_info_result(self, message: &str) -> Self;
}

impl EnhancedValidationConvenience for ClassBuilder {
    fn validation_required(self) -> Self {
        self.validation_rule(ValidationRule::Required)
    }

    fn validation_pattern(self, pattern: &str) -> Self {
        self.validation_rule(ValidationRule::Pattern(pattern.to_string()))
    }

    fn validation_length(self, min: usize, max: usize) -> Self {
        self.validation_rule(ValidationRule::Length(min, max))
    }

    fn validation_range(self, min: f64, max: f64) -> Self {
        self.validation_rule(ValidationRule::Range(min, max))
    }

    fn validation_error(self) -> Self {
        self.validation_severity(ValidationSeverity::Error)
    }

    fn validation_warning(self) -> Self {
        self.validation_severity(ValidationSeverity::Warning)
    }

    fn validation_info(self) -> Self {
        self.validation_severity(ValidationSeverity::Info)
    }

    fn validation_success(self) -> Self {
        self.validation_severity(ValidationSeverity::Success)
    }

    fn validation_global(self) -> Self {
        self.validation_scope(ValidationScope::Global)
    }

    fn validation_local(self) -> Self {
        self.validation_scope(ValidationScope::Local)
    }

    fn validation_component(self) -> Self {
        self.validation_scope(ValidationScope::Component)
    }

    fn validation_page(self) -> Self {
        self.validation_scope(ValidationScope::Page)
    }

    fn validation_strict(self) -> Self {
        self.validation_mode(ValidationMode::Strict)
    }

    fn validation_loose(self) -> Self {
        self.validation_mode(ValidationMode::Loose)
    }

    fn validation_custom_mode(self) -> Self {
        self.validation_mode(ValidationMode::Custom)
    }

    fn validation_disabled(self) -> Self {
        self.validation_mode(ValidationMode::Disabled)
    }

    fn validation_valid(self) -> Self {
        self.validation_result(ValidationResult::Valid)
    }

    fn validation_invalid(self, message: &str) -> Self {
        self.validation_result(ValidationResult::Invalid(message.to_string()))
    }

    fn validation_warning_result(self, message: &str) -> Self {
        self.validation_result(ValidationResult::Warning(message.to_string()))
    }

    fn validation_info_result(self, message: &str) -> Self {
        self.validation_result(ValidationResult::Info(message.to_string()))
    }
}

// Re-export types for public access
pub use validation_types::*;
