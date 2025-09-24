use tailwind_rs_core::utilities::enhanced_validation::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::collections::HashMap;

#[cfg(test)]
mod enhanced_validation_unit_tests {
    use super::*;

    #[test]
    fn test_validation_rule_required() {
        let rule = ValidationRule::Required;
        assert_eq!(rule.to_string(), "required");
        assert_eq!(rule.to_class_name(), "validation-required");
        assert_eq!(rule.to_css_value(), "required");
    }

    #[test]
    fn test_validation_rule_pattern() {
        let rule = ValidationRule::Pattern("test".to_string());
        assert_eq!(rule.to_string(), "pattern:test");
        assert_eq!(rule.to_class_name(), "validation-pattern-test");
        assert_eq!(rule.to_css_value(), "pattern:test");
    }

    #[test]
    fn test_validation_rule_length() {
        let rule = ValidationRule::Length(1, 10);
        assert_eq!(rule.to_string(), "length:1-10");
        assert_eq!(rule.to_class_name(), "validation-length-1-10");
        assert_eq!(rule.to_css_value(), "length:1-10");
    }

    #[test]
    fn test_validation_rule_range() {
        let rule = ValidationRule::Range(0.0, 100.0);
        assert_eq!(rule.to_string(), "range:0-100");
        assert_eq!(rule.to_class_name(), "validation-range-0-100");
        assert_eq!(rule.to_css_value(), "range:0-100");
    }

    #[test]
    fn test_validation_rule_custom() {
        let rule = ValidationRule::Custom("custom".to_string());
        assert_eq!(rule.to_string(), "custom");
        assert_eq!(rule.to_class_name(), "validation-custom");
        assert_eq!(rule.to_css_value(), "custom");
    }

    #[test]
    fn test_validation_severity_error() {
        let severity = ValidationSeverity::Error;
        assert_eq!(severity.to_string(), "error");
        assert_eq!(severity.to_class_name(), "validation-error");
        assert_eq!(severity.to_css_value(), "error");
    }

    #[test]
    fn test_validation_severity_warning() {
        let severity = ValidationSeverity::Warning;
        assert_eq!(severity.to_string(), "warning");
        assert_eq!(severity.to_class_name(), "validation-warning");
        assert_eq!(severity.to_css_value(), "warning");
    }

    #[test]
    fn test_validation_severity_info() {
        let severity = ValidationSeverity::Info;
        assert_eq!(severity.to_string(), "info");
        assert_eq!(severity.to_class_name(), "validation-info");
        assert_eq!(severity.to_css_value(), "info");
    }

    #[test]
    fn test_validation_severity_success() {
        let severity = ValidationSeverity::Success;
        assert_eq!(severity.to_string(), "success");
        assert_eq!(severity.to_class_name(), "validation-success");
        assert_eq!(severity.to_css_value(), "success");
    }

    #[test]
    fn test_validation_severity_custom() {
        let severity = ValidationSeverity::Custom("custom".to_string());
        assert_eq!(severity.to_string(), "custom");
        assert_eq!(severity.to_class_name(), "validation-custom");
        assert_eq!(severity.to_css_value(), "custom");
    }

    #[test]
    fn test_validation_scope_global() {
        let scope = ValidationScope::Global;
        assert_eq!(scope.to_string(), "global");
        assert_eq!(scope.to_class_name(), "validation-global");
        assert_eq!(scope.to_css_value(), "global");
    }

    #[test]
    fn test_validation_scope_local() {
        let scope = ValidationScope::Local;
        assert_eq!(scope.to_string(), "local");
        assert_eq!(scope.to_class_name(), "validation-local");
        assert_eq!(scope.to_css_value(), "local");
    }

    #[test]
    fn test_validation_scope_component() {
        let scope = ValidationScope::Component;
        assert_eq!(scope.to_string(), "component");
        assert_eq!(scope.to_class_name(), "validation-component");
        assert_eq!(scope.to_css_value(), "component");
    }

    #[test]
    fn test_validation_scope_page() {
        let scope = ValidationScope::Page;
        assert_eq!(scope.to_string(), "page");
        assert_eq!(scope.to_class_name(), "validation-page");
        assert_eq!(scope.to_css_value(), "page");
    }

    #[test]
    fn test_validation_scope_custom() {
        let scope = ValidationScope::Custom("custom".to_string());
        assert_eq!(scope.to_string(), "custom");
        assert_eq!(scope.to_class_name(), "validation-custom");
        assert_eq!(scope.to_css_value(), "custom");
    }

    #[test]
    fn test_validation_mode_strict() {
        let mode = ValidationMode::Strict;
        assert_eq!(mode.to_string(), "strict");
        assert_eq!(mode.to_class_name(), "validation-strict");
        assert_eq!(mode.to_css_value(), "strict");
    }

    #[test]
    fn test_validation_mode_loose() {
        let mode = ValidationMode::Loose;
        assert_eq!(mode.to_string(), "loose");
        assert_eq!(mode.to_class_name(), "validation-loose");
        assert_eq!(mode.to_css_value(), "loose");
    }

    #[test]
    fn test_validation_mode_custom() {
        let mode = ValidationMode::Custom;
        assert_eq!(mode.to_string(), "custom");
        assert_eq!(mode.to_class_name(), "validation-custom");
        assert_eq!(mode.to_css_value(), "custom");
    }

    #[test]
    fn test_validation_mode_disabled() {
        let mode = ValidationMode::Disabled;
        assert_eq!(mode.to_string(), "disabled");
        assert_eq!(mode.to_class_name(), "validation-disabled");
        assert_eq!(mode.to_css_value(), "disabled");
    }

    #[test]
    fn test_validation_mode_custom_mode() {
        let mode = ValidationMode::CustomMode("custom".to_string());
        assert_eq!(mode.to_string(), "custom");
        assert_eq!(mode.to_class_name(), "validation-custom");
        assert_eq!(mode.to_css_value(), "custom");
    }

    #[test]
    fn test_validation_result_valid() {
        let result = ValidationResult::Valid;
        assert_eq!(result.to_string(), "valid");
        assert_eq!(result.to_class_name(), "validation-valid");
        assert_eq!(result.to_css_value(), "valid");
    }

    #[test]
    fn test_validation_result_invalid() {
        let result = ValidationResult::Invalid("message".to_string());
        assert_eq!(result.to_string(), "invalid:message");
        assert_eq!(result.to_class_name(), "validation-invalid");
        assert_eq!(result.to_css_value(), "invalid:message");
    }

    #[test]
    fn test_validation_result_warning() {
        let result = ValidationResult::Warning("message".to_string());
        assert_eq!(result.to_string(), "warning:message");
        assert_eq!(result.to_class_name(), "validation-warning");
        assert_eq!(result.to_css_value(), "warning:message");
    }

    #[test]
    fn test_validation_result_info() {
        let result = ValidationResult::Info("message".to_string());
        assert_eq!(result.to_string(), "info:message");
        assert_eq!(result.to_class_name(), "validation-info");
        assert_eq!(result.to_css_value(), "info:message");
    }

    #[test]
    fn test_validation_result_custom() {
        let result = ValidationResult::Custom("custom".to_string());
        assert_eq!(result.to_string(), "custom");
        assert_eq!(result.to_class_name(), "validation-custom");
        assert_eq!(result.to_css_value(), "custom");
    }

    #[test]
    fn test_enhanced_validation_serialization() {
        let rule = ValidationRule::Required;
        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: ValidationRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rule, deserialized);
    }

    #[test]
    fn test_enhanced_validation_clone() {
        let rule = ValidationRule::Required;
        let cloned = rule.clone();
        assert_eq!(rule, cloned);
    }

    #[test]
    fn test_enhanced_validation_partial_eq() {
        let rule1 = ValidationRule::Required;
        let rule2 = ValidationRule::Required;
        let rule3 = ValidationRule::Pattern("test".to_string());
        
        assert_eq!(rule1, rule2);
        assert_ne!(rule1, rule3);
    }

    #[test]
    fn test_enhanced_validation_debug() {
        let rule = ValidationRule::Required;
        let debug = format!("{:?}", rule);
        assert!(debug.contains("Required"));
    }

    #[test]
    fn test_enhanced_validation_all_variants() {
        let rules = vec![
            ValidationRule::Required,
            ValidationRule::Pattern("test".to_string()),
            ValidationRule::Length(1, 10),
            ValidationRule::Range(0.0, 100.0),
            ValidationRule::Custom("custom".to_string()),
        ];
        
        let class_names: Vec<String> = rules.iter().map(|r| r.to_class_name()).collect();
        assert!(class_names.contains(&"validation-required".to_string()));
        assert!(class_names.contains(&"validation-pattern-test".to_string()));
        assert!(class_names.contains(&"validation-length-1-10".to_string()));
        assert!(class_names.contains(&"validation-range-0-100".to_string()));
        assert!(class_names.contains(&"validation-custom".to_string()));
    }
}

#[cfg(test)]
mod enhanced_validation_integration_tests {
    use super::*;

    #[test]
    fn test_validation_rule_with_class_builder() {
        let builder = ClassBuilder::new()
            .validation_rule(ValidationRule::Required);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-required"));
    }

    #[test]
    fn test_validation_severity_with_class_builder() {
        let builder = ClassBuilder::new()
            .validation_severity(ValidationSeverity::Error);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-error"));
    }

    #[test]
    fn test_validation_scope_with_class_builder() {
        let builder = ClassBuilder::new()
            .validation_scope(ValidationScope::Global);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-global"));
    }

    #[test]
    fn test_validation_mode_with_class_builder() {
        let builder = ClassBuilder::new()
            .validation_mode(ValidationMode::Strict);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-strict"));
    }

    #[test]
    fn test_validation_result_with_class_builder() {
        let builder = ClassBuilder::new()
            .validation_result(ValidationResult::Valid);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-valid"));
    }

    #[test]
    fn test_validation_custom_with_class_builder() {
        let mut options = HashMap::new();
        options.insert("key1".to_string(), "value1".to_string());
        let builder = ClassBuilder::new()
            .validation_custom("custom", options);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-custom"));
    }

    #[test]
    fn test_enhanced_validation_convenience_methods() {
        let builder = ClassBuilder::new()
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
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-required"));
        assert!(class_set.classes.contains("validation-pattern-test"));
        assert!(class_set.classes.contains("validation-length-1-10"));
        assert!(class_set.classes.contains("validation-range-0-100"));
        assert!(class_set.classes.contains("validation-error"));
        assert!(class_set.classes.contains("validation-warning"));
        assert!(class_set.classes.contains("validation-info"));
        assert!(class_set.classes.contains("validation-success"));
        assert!(class_set.classes.contains("validation-global"));
        assert!(class_set.classes.contains("validation-local"));
        assert!(class_set.classes.contains("validation-component"));
        assert!(class_set.classes.contains("validation-page"));
        assert!(class_set.classes.contains("validation-strict"));
        assert!(class_set.classes.contains("validation-loose"));
        assert!(class_set.classes.contains("validation-custom"));
        assert!(class_set.classes.contains("validation-disabled"));
        assert!(class_set.classes.contains("validation-valid"));
        assert!(class_set.classes.contains("validation-invalid"));
        assert!(class_set.classes.contains("validation-warning"));
        assert!(class_set.classes.contains("validation-info"));
    }

    #[test]
    fn test_enhanced_validation_with_other_utilities() {
        let builder = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .class("text-blue-500")
            .class("font-bold");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-required"));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_enhanced_validation_responsive() {
        let builder = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .responsive(Breakpoint::Md, "validation-pattern-test");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-required"));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set.responsive.get(&Breakpoint::Md).unwrap().contains("validation-pattern-test"));
    }

    #[test]
    fn test_enhanced_validation_conditional() {
        let builder = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .conditional("hover", "validation-pattern-test");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-required"));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set.conditional.get("hover").unwrap().contains("validation-pattern-test"));
    }

    #[test]
    fn test_enhanced_validation_custom_variant() {
        let builder = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .custom_variant("dark", "validation-pattern-test");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-required"));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set.conditional.get("dark").unwrap().contains("validation-pattern-test"));
    }

    #[test]
    fn test_enhanced_validation_multiple_validation() {
        let builder = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .validation_severity(ValidationSeverity::Error)
            .validation_scope(ValidationScope::Global)
            .validation_mode(ValidationMode::Strict)
            .validation_result(ValidationResult::Valid);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("validation-required"));
        assert!(class_set.classes.contains("validation-error"));
        assert!(class_set.classes.contains("validation-global"));
        assert!(class_set.classes.contains("validation-strict"));
        assert!(class_set.classes.contains("validation-valid"));
    }

    #[test]
    fn test_enhanced_validation_build_string() {
        let classes = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .class("text-blue-500")
            .build_string();
        
        assert!(classes.contains("validation-required"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_enhanced_validation_css_classes() {
        let class_set = ClassBuilder::new()
            .validation_rule(ValidationRule::Required)
            .class("font-bold")
            .build();
        
        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("validation-required"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_enhanced_validation_comprehensive_usage() {
        let class_set = ClassBuilder::new()
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
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "validation-pattern-test")
            .conditional("hover", "validation-pattern-test")
            .build();
        
        let css_classes = class_set.to_css_classes();
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
        assert!(css_classes.contains("text-blue-500"));
        assert!(css_classes.contains("font-bold"));
        assert!(css_classes.contains("md:validation-pattern-test"));
        assert!(css_classes.contains("hover:validation-pattern-test"));
    }

    #[test]
    fn test_enhanced_validation_all_variants() {
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
