//! Enhanced Validation utilities for tailwind-rs
//!
//! This module provides utilities for enhanced validation features.
//! It includes support for class validation, property validation, and validation rules.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Validation rule types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationRule {
    /// Required validation
    Required,
    /// Pattern validation
    Pattern(String),
    /// Length validation
    Length(usize, usize),
    /// Range validation
    Range(f64, f64),
    /// Custom validation
    Custom(String),
}

impl fmt::Display for ValidationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationRule::Required => write!(f, "required"),
            ValidationRule::Pattern(pattern) => write!(f, "pattern:{}", pattern),
            ValidationRule::Length(min, max) => write!(f, "length:{}-{}", min, max),
            ValidationRule::Range(min, max) => write!(f, "range:{}-{}", min, max),
            ValidationRule::Custom(rule) => write!(f, "{}", rule),
        }
    }
}

impl ValidationRule {
    /// Get the CSS class name for this validation rule
    pub fn to_class_name(&self) -> String {
        match self {
            ValidationRule::Required => "validation-required".to_string(),
            ValidationRule::Pattern(pattern) => format!(
                "validation-pattern-{}",
                pattern
                    .replace(":", "-")
                    .replace("(", "")
                    .replace(")", "")
                    .replace("*", "star")
                    .replace("+", "plus")
                    .replace("?", "question")
                    .replace("^", "caret")
                    .replace("$", "dollar")
                    .replace("|", "pipe")
                    .replace("\\", "backslash")
                    .replace("/", "slash")
                    .replace(" ", "-")
            ),
            ValidationRule::Length(min, max) => format!("validation-length-{}-{}", min, max),
            ValidationRule::Range(min, max) => format!("validation-range-{}-{}", min, max),
            ValidationRule::Custom(rule) => format!(
                "validation-{}",
                rule.replace(":", "-")
                    .replace("(", "")
                    .replace(")", "")
                    .replace("*", "star")
                    .replace("+", "plus")
                    .replace("?", "question")
                    .replace("^", "caret")
                    .replace("$", "dollar")
                    .replace("|", "pipe")
                    .replace("\\", "backslash")
                    .replace("/", "slash")
                    .replace(" ", "-")
            ),
        }
    }

    /// Get the CSS value for this validation rule
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Validation severity levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValidationSeverity {
    /// Error level
    Error,
    /// Warning level
    Warning,
    /// Info level
    Info,
    /// Success level
    Success,
    /// Custom severity
    Custom(String),
}

impl fmt::Display for ValidationSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationSeverity::Error => write!(f, "error"),
            ValidationSeverity::Warning => write!(f, "warning"),
            ValidationSeverity::Info => write!(f, "info"),
            ValidationSeverity::Success => write!(f, "success"),
            ValidationSeverity::Custom(severity) => write!(f, "{}", severity),
        }
    }
}

impl ValidationSeverity {
    /// Get the CSS class name for this validation severity
    pub fn to_class_name(&self) -> String {
        match self {
            ValidationSeverity::Error => "validation-error".to_string(),
            ValidationSeverity::Warning => "validation-warning".to_string(),
            ValidationSeverity::Info => "validation-info".to_string(),
            ValidationSeverity::Success => "validation-success".to_string(),
            ValidationSeverity::Custom(severity) => format!("validation-{}", severity),
        }
    }

    /// Get the CSS value for this validation severity
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Validation scope types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValidationScope {
    /// Global validation
    Global,
    /// Local validation
    Local,
    /// Component validation
    Component,
    /// Page validation
    Page,
    /// Custom scope
    Custom(String),
}

impl fmt::Display for ValidationScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationScope::Global => write!(f, "global"),
            ValidationScope::Local => write!(f, "local"),
            ValidationScope::Component => write!(f, "component"),
            ValidationScope::Page => write!(f, "page"),
            ValidationScope::Custom(scope) => write!(f, "{}", scope),
        }
    }
}

impl ValidationScope {
    /// Get the CSS class name for this validation scope
    pub fn to_class_name(&self) -> String {
        match self {
            ValidationScope::Global => "validation-global".to_string(),
            ValidationScope::Local => "validation-local".to_string(),
            ValidationScope::Component => "validation-component".to_string(),
            ValidationScope::Page => "validation-page".to_string(),
            ValidationScope::Custom(scope) => format!("validation-{}", scope),
        }
    }

    /// Get the CSS value for this validation scope
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Validation mode types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValidationMode {
    /// Strict validation
    Strict,
    /// Loose validation
    Loose,
    /// Custom validation
    Custom,
    /// Disabled validation
    Disabled,
    /// Custom mode
    CustomMode(String),
}

impl fmt::Display for ValidationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationMode::Strict => write!(f, "strict"),
            ValidationMode::Loose => write!(f, "loose"),
            ValidationMode::Custom => write!(f, "custom"),
            ValidationMode::Disabled => write!(f, "disabled"),
            ValidationMode::CustomMode(mode) => write!(f, "{}", mode),
        }
    }
}

impl ValidationMode {
    /// Get the CSS class name for this validation mode
    pub fn to_class_name(&self) -> String {
        match self {
            ValidationMode::Strict => "validation-strict".to_string(),
            ValidationMode::Loose => "validation-loose".to_string(),
            ValidationMode::Custom => "validation-custom".to_string(),
            ValidationMode::Disabled => "validation-disabled".to_string(),
            ValidationMode::CustomMode(mode) => format!("validation-{}", mode),
        }
    }

    /// Get the CSS value for this validation mode
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Validation result types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValidationResult {
    /// Valid result
    Valid,
    /// Invalid result
    Invalid(String),
    /// Warning result
    Warning(String),
    /// Info result
    Info(String),
    /// Custom result
    Custom(String),
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationResult::Valid => write!(f, "valid"),
            ValidationResult::Invalid(message) => write!(f, "invalid:{}", message),
            ValidationResult::Warning(message) => write!(f, "warning:{}", message),
            ValidationResult::Info(message) => write!(f, "info:{}", message),
            ValidationResult::Custom(result) => write!(f, "{}", result),
        }
    }
}

impl ValidationResult {
    /// Get the CSS class name for this validation result
    pub fn to_class_name(&self) -> String {
        match self {
            ValidationResult::Valid => "validation-valid".to_string(),
            ValidationResult::Invalid(_) => "validation-invalid".to_string(),
            ValidationResult::Warning(_) => "validation-warning".to_string(),
            ValidationResult::Info(_) => "validation-info".to_string(),
            ValidationResult::Custom(result) => format!("validation-{}", result),
        }
    }

    /// Get the CSS value for this validation result
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;

    #[test]
    fn test_validation_rule_enum_values() {
        assert_eq!(ValidationRule::Required.to_string(), "required");
        assert_eq!(
            ValidationRule::Pattern("test".to_string()).to_string(),
            "pattern:test"
        );
        assert_eq!(ValidationRule::Length(1, 10).to_string(), "length:1-10");
        assert_eq!(ValidationRule::Range(0.0, 100.0).to_string(), "range:0-100");
        assert_eq!(
            ValidationRule::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_validation_rule_class_names() {
        assert_eq!(
            ValidationRule::Required.to_class_name(),
            "validation-required"
        );
        assert_eq!(
            ValidationRule::Pattern("test".to_string()).to_class_name(),
            "validation-pattern-test"
        );
        assert_eq!(
            ValidationRule::Length(1, 10).to_class_name(),
            "validation-length-1-10"
        );
        assert_eq!(
            ValidationRule::Range(0.0, 100.0).to_class_name(),
            "validation-range-0-100"
        );
        assert_eq!(
            ValidationRule::Custom("custom".to_string()).to_class_name(),
            "validation-custom"
        );
    }

    #[test]
    fn test_validation_severity_enum_values() {
        assert_eq!(ValidationSeverity::Error.to_string(), "error");
        assert_eq!(ValidationSeverity::Warning.to_string(), "warning");
        assert_eq!(ValidationSeverity::Info.to_string(), "info");
        assert_eq!(ValidationSeverity::Success.to_string(), "success");
        assert_eq!(
            ValidationSeverity::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_validation_severity_class_names() {
        assert_eq!(
            ValidationSeverity::Error.to_class_name(),
            "validation-error"
        );
        assert_eq!(
            ValidationSeverity::Warning.to_class_name(),
            "validation-warning"
        );
        assert_eq!(ValidationSeverity::Info.to_class_name(), "validation-info");
        assert_eq!(
            ValidationSeverity::Success.to_class_name(),
            "validation-success"
        );
        assert_eq!(
            ValidationSeverity::Custom("custom".to_string()).to_class_name(),
            "validation-custom"
        );
    }

    #[test]
    fn test_validation_scope_enum_values() {
        assert_eq!(ValidationScope::Global.to_string(), "global");
        assert_eq!(ValidationScope::Local.to_string(), "local");
        assert_eq!(ValidationScope::Component.to_string(), "component");
        assert_eq!(ValidationScope::Page.to_string(), "page");
        assert_eq!(
            ValidationScope::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_validation_scope_class_names() {
        assert_eq!(ValidationScope::Global.to_class_name(), "validation-global");
        assert_eq!(ValidationScope::Local.to_class_name(), "validation-local");
        assert_eq!(
            ValidationScope::Component.to_class_name(),
            "validation-component"
        );
        assert_eq!(ValidationScope::Page.to_class_name(), "validation-page");
        assert_eq!(
            ValidationScope::Custom("custom".to_string()).to_class_name(),
            "validation-custom"
        );
    }

    #[test]
    fn test_validation_mode_enum_values() {
        assert_eq!(ValidationMode::Strict.to_string(), "strict");
        assert_eq!(ValidationMode::Loose.to_string(), "loose");
        assert_eq!(ValidationMode::Custom.to_string(), "custom");
        assert_eq!(ValidationMode::Disabled.to_string(), "disabled");
        assert_eq!(
            ValidationMode::CustomMode("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_validation_mode_class_names() {
        assert_eq!(ValidationMode::Strict.to_class_name(), "validation-strict");
        assert_eq!(ValidationMode::Loose.to_class_name(), "validation-loose");
        assert_eq!(ValidationMode::Custom.to_class_name(), "validation-custom");
        assert_eq!(
            ValidationMode::Disabled.to_class_name(),
            "validation-disabled"
        );
        assert_eq!(
            ValidationMode::CustomMode("custom".to_string()).to_class_name(),
            "validation-custom"
        );
    }

    #[test]
    fn test_validation_result_enum_values() {
        assert_eq!(ValidationResult::Valid.to_string(), "valid");
        assert_eq!(
            ValidationResult::Invalid("message".to_string()).to_string(),
            "invalid:message"
        );
        assert_eq!(
            ValidationResult::Warning("message".to_string()).to_string(),
            "warning:message"
        );
        assert_eq!(
            ValidationResult::Info("message".to_string()).to_string(),
            "info:message"
        );
        assert_eq!(
            ValidationResult::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_validation_result_class_names() {
        assert_eq!(ValidationResult::Valid.to_class_name(), "validation-valid");
        assert_eq!(
            ValidationResult::Invalid("message".to_string()).to_class_name(),
            "validation-invalid"
        );
        assert_eq!(
            ValidationResult::Warning("message".to_string()).to_class_name(),
            "validation-warning"
        );
        assert_eq!(
            ValidationResult::Info("message".to_string()).to_class_name(),
            "validation-info"
        );
        assert_eq!(
            ValidationResult::Custom("custom".to_string()).to_class_name(),
            "validation-custom"
        );
    }

    #[test]
    fn test_enhanced_validation_utilities() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .validation_severity(ValidationSeverity::Error)
            .validation_scope(ValidationScope::Global)
            .validation_mode(ValidationMode::Strict)
            .validation_result(ValidationResult::Valid);

        let result = classes.build();
        assert!(result.classes.contains("validation-required"));
        assert!(result.classes.contains("validation-error"));
        assert!(result.classes.contains("validation-global"));
        assert!(result.classes.contains("validation-strict"));
        assert!(result.classes.contains("validation-valid"));
    }

    #[test]
    fn test_enhanced_validation_convenience() {
        let classes = ClassBuilder::new()
            .validation_required()
            .validation_pattern("test")
            .validation_length(1, 10)
            .validation_range(0.0, 100.0)
            .validation_error()
            .validation_warning()
            .validation_info()
            .validation_success()
            .validation_global()
            .validation_local()
            .validation_component()
            .validation_page()
            .validation_strict()
            .validation_loose()
            .validation_custom_mode()
            .validation_disabled()
            .validation_valid()
            .validation_invalid("message")
            .validation_warning_result("message")
            .validation_info_result("message");

        let result = classes.build();
        assert!(result.classes.contains("validation-required"));
        assert!(result.classes.contains("validation-pattern-test"));
        assert!(result.classes.contains("validation-length-1-10"));
        assert!(result.classes.contains("validation-range-0-100"));
        assert!(result.classes.contains("validation-error"));
        assert!(result.classes.contains("validation-warning"));
        assert!(result.classes.contains("validation-info"));
        assert!(result.classes.contains("validation-success"));
        assert!(result.classes.contains("validation-global"));
        assert!(result.classes.contains("validation-local"));
        assert!(result.classes.contains("validation-component"));
        assert!(result.classes.contains("validation-page"));
        assert!(result.classes.contains("validation-strict"));
        assert!(result.classes.contains("validation-loose"));
        assert!(result.classes.contains("validation-custom"));
        assert!(result.classes.contains("validation-disabled"));
        assert!(result.classes.contains("validation-valid"));
        assert!(result.classes.contains("validation-invalid"));
        assert!(result.classes.contains("validation-warning"));
        assert!(result.classes.contains("validation-info"));
    }

    #[test]
    fn test_enhanced_validation_serialization() {
        let validation_rule = ValidationRule::Required;
        let serialized = serde_json::to_string(&validation_rule).unwrap();
        let deserialized: ValidationRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(validation_rule, deserialized);

        let validation_severity = ValidationSeverity::Error;
        let serialized = serde_json::to_string(&validation_severity).unwrap();
        let deserialized: ValidationSeverity = serde_json::from_str(&serialized).unwrap();
        assert_eq!(validation_severity, deserialized);

        let validation_scope = ValidationScope::Global;
        let serialized = serde_json::to_string(&validation_scope).unwrap();
        let deserialized: ValidationScope = serde_json::from_str(&serialized).unwrap();
        assert_eq!(validation_scope, deserialized);

        let validation_mode = ValidationMode::Strict;
        let serialized = serde_json::to_string(&validation_mode).unwrap();
        let deserialized: ValidationMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(validation_mode, deserialized);

        let validation_result = ValidationResult::Valid;
        let serialized = serde_json::to_string(&validation_result).unwrap();
        let deserialized: ValidationResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(validation_result, deserialized);
    }

    #[test]
    fn test_enhanced_validation_comprehensive_usage() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .validation_severity(ValidationSeverity::Error)
            .validation_scope(ValidationScope::Global)
            .validation_mode(ValidationMode::Strict)
            .validation_result(ValidationResult::Valid)
            .validation_required()
            .validation_pattern("test")
            .validation_length(1, 10)
            .validation_range(0.0, 100.0)
            .validation_error()
            .validation_warning()
            .validation_info()
            .validation_success()
            .validation_global()
            .validation_local()
            .validation_component()
            .validation_page()
            .validation_strict()
            .validation_loose()
            .validation_custom_mode()
            .validation_disabled()
            .validation_valid()
            .validation_invalid("message")
            .validation_warning_result("message")
            .validation_info_result("message");

        let result = classes.build();
        assert!(result.classes.contains("validation-required"));
        assert!(result.classes.contains("validation-error"));
        assert!(result.classes.contains("validation-global"));
        assert!(result.classes.contains("validation-strict"));
        assert!(result.classes.contains("validation-valid"));
        assert!(result.classes.contains("validation-pattern-test"));
        assert!(result.classes.contains("validation-length-1-10"));
        assert!(result.classes.contains("validation-range-0-100"));
        assert!(result.classes.contains("validation-warning"));
        assert!(result.classes.contains("validation-info"));
        assert!(result.classes.contains("validation-success"));
        assert!(result.classes.contains("validation-local"));
        assert!(result.classes.contains("validation-component"));
        assert!(result.classes.contains("validation-page"));
        assert!(result.classes.contains("validation-loose"));
        assert!(result.classes.contains("validation-custom"));
        assert!(result.classes.contains("validation-disabled"));
        assert!(result.classes.contains("validation-invalid"));
    }
}
