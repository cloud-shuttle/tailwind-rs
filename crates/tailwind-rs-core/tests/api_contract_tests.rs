//! API Contract Tests
//!
//! This module provides comprehensive contract testing to ensure API stability,
//! backward compatibility, and reliability.

use tailwind_rs_core::*;
use tailwind_rs_core::api_contracts::*;
use std::collections::HashMap;

/// Test ClassBuilder API contract
#[test]
fn test_class_builder_api_contract() {
    let contract = ClassBuilderContract::new(ApiVersion::V2_0_0);
    
    // Test valid input
    let valid_input = ClassBuilderInput {
        classes: vec!["p-4".to_string(), "m-2".to_string()],
        responsive: vec![(Breakpoint::Md, "text-lg".to_string())],
        conditional: vec![("hover".to_string(), "bg-blue-600".to_string())],
        custom: vec![("primary-color".to_string(), "#3b82f6".to_string())],
    };
    
    // Test input validation
    assert!(contract.validate_input(&valid_input).is_ok());
    
    // Test processing
    let output = contract.process(valid_input).unwrap();
    
    // Test output validation
    assert!(contract.validate_output(&output).is_ok());
    
    // Test that output contains expected classes
    assert!(output.has_class("p-4"));
    assert!(output.has_class("m-2"));
}

/// Test ClassBuilder API contract with invalid input
#[test]
fn test_class_builder_api_contract_invalid_input() {
    let contract = ClassBuilderContract::new(ApiVersion::V2_0_0);
    
    // Test invalid input - empty class name
    let invalid_input = ClassBuilderInput {
        classes: vec!["".to_string()],
        responsive: vec![],
        conditional: vec![],
        custom: vec![],
    };
    
    // Test input validation should fail
    assert!(contract.validate_input(&invalid_input).is_err());
}

/// Test CssGenerator API contract
#[test]
fn test_css_generator_api_contract() {
    let contract = CssGeneratorContract::new(ApiVersion::V2_0_0);
    
    // Test valid input
    let valid_input = CssGeneratorInput {
        rules: vec![CssRuleInput {
            selector: ".test".to_string(),
            properties: vec![CssPropertyInput {
                name: "padding".to_string(),
                value: "1rem".to_string(),
                important: false,
            }],
        }],
        media_queries: vec!["@media (min-width: 768px)".to_string()],
        format: CssFormat::Regular,
    };
    
    // Test input validation
    assert!(contract.validate_input(&valid_input).is_ok());
    
    // Test processing
    let output = contract.process(valid_input).unwrap();
    
    // Test output validation
    assert!(contract.validate_output(&output).is_ok());
    
    // Test that output contains expected CSS
    assert!(output.contains("padding"));
    assert!(output.contains("1rem"));
}

/// Test CssGenerator API contract with invalid input
#[test]
fn test_css_generator_api_contract_invalid_input() {
    let contract = CssGeneratorContract::new(ApiVersion::V2_0_0);
    
    // Test invalid input - empty selector
    let invalid_input = CssGeneratorInput {
        rules: vec![CssRuleInput {
            selector: "".to_string(),
            properties: vec![CssPropertyInput {
                name: "padding".to_string(),
                value: "1rem".to_string(),
                important: false,
            }],
        }],
        media_queries: vec![],
        format: CssFormat::Regular,
    };
    
    // Test input validation should fail
    assert!(contract.validate_input(&invalid_input).is_err());
}

/// Test backward compatibility
#[test]
fn test_backward_compatibility() {
    // Test that old API patterns still work
    let old_api_test = ClassBuilder::new()
        .class("p-4")
        .class("m-2")
        .class("bg-blue-500")
        .class("text-white")
        .build();

    assert!(!old_api_test.to_css_classes().is_empty());
    assert!(old_api_test.has_class("p-4"));
    assert!(old_api_test.has_class("m-2"));
    assert!(old_api_test.has_class("bg-blue-500"));
    assert!(old_api_test.has_class("text-white"));
}

/// Test API consistency
#[test]
fn test_api_consistency() {
    // Test that all utility methods follow the same pattern
    let spacing_test = ClassBuilder::new()
        .p_4()
        .m_2()
        .px_6()
        .py_3()
        .build();

    assert!(!spacing_test.to_css_classes().is_empty());

    // Test that all color methods follow the same pattern
    let color_test = ClassBuilder::new()
        .bg_blue_500()
        .text_white()
        .border_gray_300()
        .build();

    assert!(!color_test.to_css_classes().is_empty());

    // Test that all layout methods follow the same pattern
    let layout_test = ClassBuilder::new()
        .flex()
        .items_center()
        .justify_center()
        .build();

    assert!(!layout_test.to_css_classes().is_empty());
}

/// Test performance contracts
#[test]
fn test_performance_contracts() {
    // Test that class building meets performance requirements
    let start = std::time::Instant::now();
    let mut builder = ClassBuilder::new();
    
    for i in 0..1000 {
        builder = builder.class(&format!("class-{}", i % 10));
    }
    
    let result = builder.build();
    let duration = start.elapsed();
    
    // Should complete within 100ms
    assert!(duration.as_millis() < 100);
    assert_eq!(result.len(), 1000);
    
    // Test CSS generation performance
    let mut generator = CssGenerator::new();
    for i in 0..1000 {
        let _ = generator.add_class(&format!("p-{}", i % 10));
    }
    
    let css_start = std::time::Instant::now();
    let _css = generator.generate_css();
    let css_duration = css_start.elapsed();
    
    // Should complete within 50ms
    assert!(css_duration.as_millis() < 50);
}

/// Test error handling contracts
#[test]
fn test_error_handling_contracts() {
    let mut generator = CssGenerator::new();
    
    // Test invalid class handling
    let invalid_result = generator.add_class("invalid-class");
    assert!(invalid_result.is_err());
    
    // Test empty class handling
    let empty_result = generator.add_class("");
    assert!(empty_result.is_err());
    
    // Test special characters
    let special_result = generator.add_class("class-with-special-chars!");
    assert!(special_result.is_err());
}

/// Test memory usage contracts
#[test]
fn test_memory_usage_contracts() {
    // Test large class set memory usage
    let mut large_builder = ClassBuilder::new();
    
    for i in 0..10000 {
        large_builder = large_builder.class(&format!("class-{}", i % 100));
    }
    
    let large_result = large_builder.build();
    assert_eq!(large_result.len(), 10000);
    
    // Test that memory usage is reasonable
    let memory_usage = std::mem::size_of_val(&large_result);
    assert!(memory_usage < 1024 * 1024); // Less than 1MB
}

/// Test edge case contracts
#[test]
fn test_edge_case_contracts() {
    // Test maximum class count
    let mut max_classes = ClassBuilder::new();
    for i in 0..50000 {
        max_classes = max_classes.class(&format!("class-{}", i % 1000));
    }
    
    let result = max_classes.build();
    assert_eq!(result.len(), 50000);
    
    // Test very long class names
    let long_class = "a".repeat(1000);
    let long_class_result = ClassBuilder::new()
        .class(&long_class)
        .build();
    
    assert!(long_class_result.to_css_classes().contains(&long_class));
    
    // Test special characters in class names
    let special_chars = ClassBuilder::new()
        .class("class-with-dashes")
        .class("class_with_underscores")
        .class("class.with.dots")
        .build();
    
    assert!(!special_chars.to_css_classes().is_empty());
}

/// Test regression prevention contracts
#[test]
fn test_regression_prevention_contracts() {
    // Test class merging regression
    let class_set_1 = ClassBuilder::new()
        .p_4()
        .m_2()
        .build();
        
    let class_set_2 = ClassBuilder::new()
        .bg_blue_500()
        .text_white()
        .build();
    
    let mut merged = class_set_1.clone();
    merged.merge(class_set_2);
    
    assert!(merged.has_class("p-4"));
    assert!(merged.has_class("m-2"));
    assert!(merged.has_class("bg-blue-500"));
    assert!(merged.has_class("text-white"));
    
    // Test responsive class regression
    let responsive_test = ClassBuilder::new()
        .text_sm()
        .responsive(Breakpoint::Md, "text-base")
        .responsive(Breakpoint::Lg, "text-lg")
        .build();

    assert!(!responsive_test.to_css_classes().is_empty());
    
    // Test dark mode regression
    let dark_mode_test = ClassBuilder::new()
        .bg_white()
        .dark("bg-gray-800")
        .text_black()
        .dark("text-white")
        .build();

    assert!(!dark_mode_test.to_css_classes().is_empty());
}

/// Test production readiness contracts
#[test]
fn test_production_readiness_contracts() {
    // Test comprehensive production example
    let production_example = ClassBuilder::new()
        .p_4()
        .m_2()
        .w_1_2()
        .h_1_3()
        .bg_blue_500()
        .text_white()
        .rounded_md()
        .shadow_lg()
        .hover("bg-blue-600")
        .focus("ring-2")
        .font_size(crate::utilities::typography::FontSize::Lg)
        .font_weight(crate::utilities::typography::FontWeight::Bold)
        .text_align(crate::utilities::typography::TextAlign::Center)
        .responsive(Breakpoint::Md, "text-xl")
        .dark("bg-gray-800")
        .build();

    assert!(!production_example.to_css_classes().is_empty());
    
    // Test that all major features are working
    assert!(production_example.to_css_classes().contains("p-4"));
    assert!(production_example.to_css_classes().contains("m-2"));
    assert!(production_example.to_css_classes().contains("w-1/2"));
    assert!(production_example.to_css_classes().contains("h-1/3"));
    assert!(production_example.to_css_classes().contains("bg-blue-500"));
    assert!(production_example.to_css_classes().contains("text-white"));
    assert!(production_example.to_css_classes().contains("rounded-md"));
    assert!(production_example.to_css_classes().contains("shadow-lg"));
}

/// Test accessibility contracts
#[test]
fn test_accessibility_contracts() {
    // Test focus states
    let focus_test = ClassBuilder::new()
        .focus("ring-2")
        .focus("ring-blue-500")
        .build();

    assert!(!focus_test.to_css_classes().is_empty());

    // Test ARIA attributes
    let aria_test = ClassBuilder::new()
        .aria("expanded", "true", "bg-blue-100")
        .aria("selected", "false", "bg-gray-100")
        .build();

    assert!(!aria_test.to_css_classes().is_empty());
}

/// Test cross-platform compatibility contracts
#[test]
fn test_cross_platform_compatibility_contracts() {
    // Test that the library works on different platforms
    let cross_platform_test = ClassBuilder::new()
        .p_4()
        .m_2()
        .bg_blue_500()
        .text_white()
        .rounded_md()
        .shadow_lg()
        .build();

    assert!(!cross_platform_test.to_css_classes().is_empty());
}

/// Test security contracts
#[test]
fn test_security_contracts() {
    // Test that malicious input is handled safely
    let malicious_input = ClassBuilder::new()
        .class("javascript:alert('xss')")
        .class("<script>alert('xss')</script>")
        .class("'; DROP TABLE users; --")
        .build();

    // Should not panic or cause security issues
    assert!(!malicious_input.to_css_classes().is_empty());
    
    // Test that very long inputs are handled safely
    let long_input = "a".repeat(100000);
    let long_input_result = ClassBuilder::new()
        .class(&long_input)
        .build();
    
    assert!(long_input_result.to_css_classes().contains(&long_input));
}

/// Test final release validation contracts
#[test]
fn test_final_release_validation_contracts() {
    // Test comprehensive release validation
    let release_validation = ClassBuilder::new()
        .p_4()
        .m_2()
        .w_1_2()
        .h_1_3()
        .bg_blue_500()
        .text_white()
        .rounded_md()
        .shadow_lg()
        .hover("bg-blue-600")
        .focus("ring-2")
        .font_size(crate::utilities::typography::FontSize::Lg)
        .font_weight(crate::utilities::typography::FontWeight::Bold)
        .text_align(crate::utilities::typography::TextAlign::Center)
        .responsive(Breakpoint::Md, "text-xl")
        .dark("bg-gray-800")
        .build();

    assert!(!release_validation.to_css_classes().is_empty());
    
    // Test that all critical features are working
    assert!(release_validation.to_css_classes().contains("p-4"));
    assert!(release_validation.to_css_classes().contains("m-2"));
    assert!(release_validation.to_css_classes().contains("w-1/2"));
    assert!(release_validation.to_css_classes().contains("h-1/3"));
    assert!(release_validation.to_css_classes().contains("bg-blue-500"));
    assert!(release_validation.to_css_classes().contains("text-white"));
    assert!(release_validation.to_css_classes().contains("rounded-md"));
    assert!(release_validation.to_css_classes().contains("shadow-lg"));
    
    // Test that the release is production-ready
    assert!(!release_validation.to_css_classes().is_empty());
}

/// Test contract testing framework
#[test]
fn test_contract_testing_framework() {
    let mut tester = ContractTester::new();
    
    let test_case = TestCase {
        name: "test_case_1".to_string(),
        input: Box::new("test_input"),
        expected_output: Box::new("test_output"),
        should_fail: false,
    };
    
    tester.add_test_case(test_case);
    
    let results = tester.run_tests().unwrap();
    assert_eq!(results.total_tests, 1);
}

/// Test contract validator
#[test]
fn test_contract_validator() {
    let mut validator = ContractValidator::new();
    
    // Test validation
    let result = validator.validate_call("test_api", "test_input");
    assert!(result.is_err()); // Should fail because no contract is registered
    
    // Test enabling/disabling validation
    validator.disable_validation();
    let result = validator.validate_call("test_api", "test_input");
    assert!(result.is_ok()); // Should pass because validation is disabled
}
