use std::collections::HashMap;
use tailwind_rs_core::utilities::enhanced_validation::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod enhanced_validation_baseline_tests {
    use super::*;

    #[test]
    fn test_validation_rule_required_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-required");
    }

    #[test]
    fn test_validation_rule_pattern_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Pattern("test".to_string()))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-pattern-test");
    }

    #[test]
    fn test_validation_rule_length_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Length(1, 10))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-length-1-10");
    }

    #[test]
    fn test_validation_rule_range_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Range(0.0, 100.0))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-range-0-100");
    }

    #[test]
    fn test_validation_rule_custom_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Custom("custom".to_string()))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-custom");
    }

    #[test]
    fn test_validation_severity_error_baseline() {
        let classes = ClassBuilder::new()
            .validation_severity(ValidationSeverity::Error)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-error");
    }

    #[test]
    fn test_validation_severity_warning_baseline() {
        let classes = ClassBuilder::new()
            .validation_severity(ValidationSeverity::Warning)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-warning");
    }

    #[test]
    fn test_validation_severity_info_baseline() {
        let classes = ClassBuilder::new()
            .validation_severity(ValidationSeverity::Info)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-info");
    }

    #[test]
    fn test_validation_severity_success_baseline() {
        let classes = ClassBuilder::new()
            .validation_severity(ValidationSeverity::Success)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-success");
    }

    #[test]
    fn test_validation_severity_custom_baseline() {
        let classes = ClassBuilder::new()
            .validation_severity(ValidationSeverity::Custom("custom".to_string()))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-custom");
    }

    #[test]
    fn test_validation_scope_global_baseline() {
        let classes = ClassBuilder::new()
            .validation_scope(ValidationScope::Global)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-global");
    }

    #[test]
    fn test_validation_scope_local_baseline() {
        let classes = ClassBuilder::new()
            .validation_scope(ValidationScope::Local)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-local");
    }

    #[test]
    fn test_validation_scope_component_baseline() {
        let classes = ClassBuilder::new()
            .validation_scope(ValidationScope::Component)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-component");
    }

    #[test]
    fn test_validation_scope_page_baseline() {
        let classes = ClassBuilder::new()
            .validation_scope(ValidationScope::Page)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-page");
    }

    #[test]
    fn test_validation_scope_custom_baseline() {
        let classes = ClassBuilder::new()
            .validation_scope(ValidationScope::Custom("custom".to_string()))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-custom");
    }

    #[test]
    fn test_validation_mode_strict_baseline() {
        let classes = ClassBuilder::new()
            .validation_mode(ValidationMode::Strict)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-strict");
    }

    #[test]
    fn test_validation_mode_loose_baseline() {
        let classes = ClassBuilder::new()
            .validation_mode(ValidationMode::Loose)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-loose");
    }

    #[test]
    fn test_validation_mode_custom_baseline() {
        let classes = ClassBuilder::new()
            .validation_mode(ValidationMode::Custom)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-custom");
    }

    #[test]
    fn test_validation_mode_disabled_baseline() {
        let classes = ClassBuilder::new()
            .validation_mode(ValidationMode::Disabled)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-disabled");
    }

    #[test]
    fn test_validation_mode_custom_mode_baseline() {
        let classes = ClassBuilder::new()
            .validation_mode(ValidationMode::CustomMode("custom".to_string()))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-custom");
    }

    #[test]
    fn test_validation_result_valid_baseline() {
        let classes = ClassBuilder::new()
            .validation_result(ValidationResult::Valid)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-valid");
    }

    #[test]
    fn test_validation_result_invalid_baseline() {
        let classes = ClassBuilder::new()
            .validation_result(ValidationResult::Invalid("message".to_string()))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-invalid");
    }

    #[test]
    fn test_validation_result_warning_baseline() {
        let classes = ClassBuilder::new()
            .validation_result(ValidationResult::Warning("message".to_string()))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-warning");
    }

    #[test]
    fn test_validation_result_info_baseline() {
        let classes = ClassBuilder::new()
            .validation_result(ValidationResult::Info("message".to_string()))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-info");
    }

    #[test]
    fn test_validation_result_custom_baseline() {
        let classes = ClassBuilder::new()
            .validation_result(ValidationResult::Custom("custom".to_string()))
            .build();
        assert_eq!(classes.to_css_classes(), "validation-custom");
    }

    #[test]
    fn test_validation_required_baseline() {
        let classes = ClassBuilder::new().validation_required().build();
        assert_eq!(classes.to_css_classes(), "validation-required");
    }

    #[test]
    fn test_validation_pattern_baseline() {
        let classes = ClassBuilder::new().validation_pattern("test").build();
        assert_eq!(classes.to_css_classes(), "validation-pattern-test");
    }

    #[test]
    fn test_validation_length_baseline() {
        let classes = ClassBuilder::new().validation_length(1, 10).build();
        assert_eq!(classes.to_css_classes(), "validation-length-1-10");
    }

    #[test]
    fn test_validation_range_baseline() {
        let classes = ClassBuilder::new().validation_range(0.0, 100.0).build();
        assert_eq!(classes.to_css_classes(), "validation-range-0-100");
    }

    #[test]
    fn test_validation_error_baseline() {
        let classes = ClassBuilder::new().validation_error().build();
        assert_eq!(classes.to_css_classes(), "validation-error");
    }

    #[test]
    fn test_validation_warning_baseline() {
        let classes = ClassBuilder::new().validation_warning().build();
        assert_eq!(classes.to_css_classes(), "validation-warning");
    }

    #[test]
    fn test_validation_info_baseline() {
        let classes = ClassBuilder::new().validation_info().build();
        assert_eq!(classes.to_css_classes(), "validation-info");
    }

    #[test]
    fn test_validation_success_baseline() {
        let classes = ClassBuilder::new().validation_success().build();
        assert_eq!(classes.to_css_classes(), "validation-success");
    }

    #[test]
    fn test_validation_global_baseline() {
        let classes = ClassBuilder::new().validation_global().build();
        assert_eq!(classes.to_css_classes(), "validation-global");
    }

    #[test]
    fn test_validation_local_baseline() {
        let classes = ClassBuilder::new().validation_local().build();
        assert_eq!(classes.to_css_classes(), "validation-local");
    }

    #[test]
    fn test_validation_component_baseline() {
        let classes = ClassBuilder::new().validation_component().build();
        assert_eq!(classes.to_css_classes(), "validation-component");
    }

    #[test]
    fn test_validation_page_baseline() {
        let classes = ClassBuilder::new().validation_page().build();
        assert_eq!(classes.to_css_classes(), "validation-page");
    }

    #[test]
    fn test_validation_strict_baseline() {
        let classes = ClassBuilder::new().validation_strict().build();
        assert_eq!(classes.to_css_classes(), "validation-strict");
    }

    #[test]
    fn test_validation_loose_baseline() {
        let classes = ClassBuilder::new().validation_loose().build();
        assert_eq!(classes.to_css_classes(), "validation-loose");
    }

    #[test]
    fn test_validation_custom_mode_baseline() {
        let classes = ClassBuilder::new().validation_custom_mode().build();
        assert_eq!(classes.to_css_classes(), "validation-custom");
    }

    #[test]
    fn test_validation_disabled_baseline() {
        let classes = ClassBuilder::new().validation_disabled().build();
        assert_eq!(classes.to_css_classes(), "validation-disabled");
    }

    #[test]
    fn test_validation_valid_baseline() {
        let classes = ClassBuilder::new().validation_valid().build();
        assert_eq!(classes.to_css_classes(), "validation-valid");
    }

    #[test]
    fn test_validation_invalid_baseline() {
        let classes = ClassBuilder::new().validation_invalid("message").build();
        assert_eq!(classes.to_css_classes(), "validation-invalid");
    }

    #[test]
    fn test_validation_warning_result_baseline() {
        let classes = ClassBuilder::new()
            .validation_warning_result("message")
            .build();
        assert_eq!(classes.to_css_classes(), "validation-warning");
    }

    #[test]
    fn test_validation_info_result_baseline() {
        let classes = ClassBuilder::new()
            .validation_info_result("message")
            .build();
        assert_eq!(classes.to_css_classes(), "validation-info");
    }

    #[test]
    fn test_validation_custom_baseline() {
        let mut options = HashMap::new();
        options.insert("key1".to_string(), "value1".to_string());
        let classes = ClassBuilder::new()
            .validation_custom("custom", options)
            .build();
        assert_eq!(classes.to_css_classes(), "validation-custom");
    }

    #[test]
    fn test_enhanced_validation_comprehensive_baseline() {
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
            .validation_info_result("message")
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("validation-required"));
        assert!(css_classes.contains("validation-error"));
        assert!(css_classes.contains("validation-global"));
        assert!(css_classes.contains("validation-strict"));
        assert!(css_classes.contains("validation-valid"));
        assert!(css_classes.contains("validation-pattern-test"));
        assert!(css_classes.contains("validation-length-1-10"));
        assert!(css_classes.contains("validation-range-0-100"));
        assert!(css_classes.contains("validation-warning"));
        assert!(css_classes.contains("validation-info"));
        assert!(css_classes.contains("validation-success"));
        assert!(css_classes.contains("validation-local"));
        assert!(css_classes.contains("validation-component"));
        assert!(css_classes.contains("validation-page"));
        assert!(css_classes.contains("validation-loose"));
        assert!(css_classes.contains("validation-custom"));
        assert!(css_classes.contains("validation-disabled"));
        assert!(css_classes.contains("validation-invalid"));
    }

    #[test]
    fn test_enhanced_validation_with_other_utilities_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .class("text-blue-500")
            .class("font-bold")
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("validation-required"));
        assert!(css_classes.contains("text-blue-500"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_enhanced_validation_responsive_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .responsive(Breakpoint::Md, "validation-pattern-test")
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("validation-required"));
        assert!(css_classes.contains("md:validation-pattern-test"));
    }

    #[test]
    fn test_enhanced_validation_conditional_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .conditional("hover", "validation-pattern-test")
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("validation-required"));
        assert!(css_classes.contains("hover:validation-pattern-test"));
    }

    #[test]
    fn test_enhanced_validation_custom_variant_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .custom_variant("dark", "validation-pattern-test")
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("validation-required"));
        assert!(css_classes.contains("dark:validation-pattern-test"));
    }

    #[test]
    fn test_enhanced_validation_multiple_validation_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .validation_severity(ValidationSeverity::Error)
            .validation_scope(ValidationScope::Global)
            .validation_mode(ValidationMode::Strict)
            .validation_result(ValidationResult::Valid)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("validation-required"));
        assert!(css_classes.contains("validation-error"));
        assert!(css_classes.contains("validation-global"));
        assert!(css_classes.contains("validation-strict"));
        assert!(css_classes.contains("validation-valid"));
    }

    #[test]
    fn test_enhanced_validation_build_string_baseline() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .class("text-blue-500")
            .build_string();

        assert!(classes.contains("validation-required"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_enhanced_validation_css_classes_baseline() {
        let class_set = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .class("font-bold")
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("validation-required"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_enhanced_validation_all_variants_baseline() {
        let class_set = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .validation_rule(ValidationRule::Pattern("test".to_string()))
            .validation_rule(ValidationRule::Length(1, 10))
            .validation_rule(ValidationRule::Range(0.0, 100.0))
            .validation_rule(ValidationRule::Custom("custom".to_string()))
            .validation_severity(ValidationSeverity::Error)
            .validation_severity(ValidationSeverity::Warning)
            .validation_severity(ValidationSeverity::Info)
            .validation_severity(ValidationSeverity::Success)
            .validation_severity(ValidationSeverity::Custom("custom".to_string()))
            .validation_scope(ValidationScope::Global)
            .validation_scope(ValidationScope::Local)
            .validation_scope(ValidationScope::Component)
            .validation_scope(ValidationScope::Page)
            .validation_scope(ValidationScope::Custom("custom".to_string()))
            .validation_mode(ValidationMode::Strict)
            .validation_mode(ValidationMode::Loose)
            .validation_mode(ValidationMode::Custom)
            .validation_mode(ValidationMode::Disabled)
            .validation_mode(ValidationMode::CustomMode("custom".to_string()))
            .validation_result(ValidationResult::Valid)
            .validation_result(ValidationResult::Invalid("message".to_string()))
            .validation_result(ValidationResult::Warning("message".to_string()))
            .validation_result(ValidationResult::Info("message".to_string()))
            .validation_result(ValidationResult::Custom("custom".to_string()))
            .build();

        let css_classes = class_set.to_css_classes();

        // Test that all enhanced validation utilities are present
        assert!(css_classes.contains("validation-required"));
        assert!(css_classes.contains("validation-pattern-test"));
        assert!(css_classes.contains("validation-length-1-10"));
        assert!(css_classes.contains("validation-range-0-100"));
        assert!(css_classes.contains("validation-custom"));
        assert!(css_classes.contains("validation-error"));
        assert!(css_classes.contains("validation-warning"));
        assert!(css_classes.contains("validation-info"));
        assert!(css_classes.contains("validation-success"));
        assert!(css_classes.contains("validation-global"));
        assert!(css_classes.contains("validation-local"));
        assert!(css_classes.contains("validation-component"));
        assert!(css_classes.contains("validation-page"));
        assert!(css_classes.contains("validation-strict"));
        assert!(css_classes.contains("validation-loose"));
        assert!(css_classes.contains("validation-custom"));
        assert!(css_classes.contains("validation-disabled"));
        assert!(css_classes.contains("validation-valid"));
        assert!(css_classes.contains("validation-invalid"));
        assert!(css_classes.contains("validation-warning"));
        assert!(css_classes.contains("validation-info"));
    }
}
