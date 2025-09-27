//! Tests for enhanced validation utilities
//!
//! This module contains comprehensive tests for all validation functionality.

use crate::classes::ClassBuilder;

use super::validation_types::*;

// Re-export for testing
pub use super::validation_traits::*;

#[cfg(test)]
mod tests {
    use super::*;

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
