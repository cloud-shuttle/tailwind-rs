//! Validation types for enhanced validation utilities
//!
//! This module contains all the enum types and their implementations for
//! validation rules, severity levels, scopes, modes, and results.

use serde::{Deserialize, Serialize};
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
