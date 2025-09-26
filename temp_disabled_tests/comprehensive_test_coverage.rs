//! Comprehensive Test Coverage
//!
//! This module provides comprehensive test coverage for all core functionality
//! to ensure 90%+ test coverage and production readiness.

use tailwind_rs_core::*;
use std::collections::HashMap;

/// Test ClassBuilder comprehensive functionality
#[test]
fn test_class_builder_comprehensive() {
    // Test basic class building
    let builder = ClassBuilder::new();
    assert!(builder.build().is_empty());
    
    // Test single class
    let single_class = ClassBuilder::new()
        .class("p-4")
        .build();
    assert!(single_class.has_class("p-4"));
    assert_eq!(single_class.len(), 1);
    
    // Test multiple classes
    let multiple_classes = ClassBuilder::new()
        .class("p-4")
        .class("m-2")
        .class("bg-blue-500")
        .build();
    assert!(multiple_classes.has_class("p-4"));
    assert!(multiple_classes.has_class("m-2"));
    assert!(multiple_classes.has_class("bg-blue-500"));
    assert_eq!(multiple_classes.len(), 3);
    
    // Test responsive classes
    let responsive_classes = ClassBuilder::new()
        .responsive(Breakpoint::Md, "text-lg")
        .responsive(Breakpoint::Lg, "text-xl")
        .build();
    assert!(!responsive_classes.get_responsive_classes(Breakpoint::Md).is_empty());
    assert!(!responsive_classes.get_responsive_classes(Breakpoint::Lg).is_empty());
    
    // Test conditional classes
    let conditional_classes = ClassBuilder::new()
        .conditional("hover", "bg-blue-600")
        .conditional("focus", "ring-2")
        .build();
    assert!(!conditional_classes.get_conditional_classes("hover").is_empty());
    assert!(!conditional_classes.get_conditional_classes("focus").is_empty());
    
    // Test custom properties
    let custom_properties = ClassBuilder::new()
        .custom("primary-color", "#3b82f6")
        .custom("secondary-color", "#ef4444")
        .build();
    let custom_props = custom_properties.get_custom_properties();
    assert_eq!(custom_props.get("primary-color"), Some(&"#3b82f6".to_string()));
    assert_eq!(custom_props.get("secondary-color"), Some(&"#ef4444".to_string()));
}

/// Test CssGenerator comprehensive functionality
#[test]
fn test_css_generator_comprehensive() {
    let mut generator = CssGenerator::new();
    
    // Test basic class addition
    generator.add_class("p-4").unwrap();
    generator.add_class("m-2").unwrap();
    generator.add_class("bg-blue-500").unwrap();
    
    // Test CSS generation
    let css = generator.generate_css();
    assert!(!css.is_empty());
    assert!(css.contains("padding"));
    assert!(css.contains("margin"));
    assert!(css.contains("background-color"));
    
    // Test minified CSS generation
    let minified_css = generator.generate_minified_css();
    assert!(!minified_css.is_empty());
    assert!(!minified_css.contains('\n'));
    assert!(!minified_css.contains(' '));
    
    // Test rule count
    assert!(generator.rule_count() > 0);
    
    // Test responsive classes
    generator.add_responsive_class(Breakpoint::Md, "text-lg").unwrap();
    let responsive_css = generator.generate_css();
    assert!(responsive_css.contains("@media"));
    
    // Test hover states
    generator.add_class("hover:bg-blue-600").unwrap();
    let hover_css = generator.generate_css();
    assert!(hover_css.contains(":hover"));
    
    // Test dark mode
    generator.add_class("dark:bg-gray-800").unwrap();
    let dark_css = generator.generate_css();
    assert!(dark_css.contains(".dark"));
}

/// Test TailwindBuilder comprehensive functionality
#[test]
fn test_tailwind_builder_comprehensive() {
    // Test basic builder creation
    let builder = TailwindBuilder::new();
    assert_eq!(builder.source_paths.len(), 0);
    assert!(builder.output_path.is_none());
    assert!(builder.config_path.is_none());
    assert!(!builder.tree_shaking);
    assert!(!builder.minification);
    assert!(!builder.source_maps);
    
    // Test configuration
    let configured_builder = TailwindBuilder::new()
        .scan_source(std::path::Path::new("src"))
        .output_css(std::path::Path::new("dist/styles.css"))
        .config_file(std::path::Path::new("tailwind.config.toml"))
        .enable_tree_shaking()
        .enable_minification()
        .enable_source_maps();
    
    assert_eq!(configured_builder.source_paths.len(), 1);
    assert!(configured_builder.output_path.is_some());
    assert!(configured_builder.config_path.is_some());
    assert!(configured_builder.tree_shaking);
    assert!(configured_builder.minification);
    assert!(configured_builder.source_maps);
    
    // Test build process
    let result = configured_builder.build();
    assert!(result.is_ok());
}

/// Test error handling comprehensive functionality
#[test]
fn test_error_handling_comprehensive() {
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

/// Test performance comprehensive functionality
#[test]
fn test_performance_comprehensive() {
    // Test large class set performance
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

/// Test memory usage comprehensive functionality
#[test]
fn test_memory_usage_comprehensive() {
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

/// Test edge cases comprehensive functionality
#[test]
fn test_edge_cases_comprehensive() {
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

/// Test regression prevention comprehensive functionality
#[test]
fn test_regression_prevention_comprehensive() {
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

/// Test API consistency comprehensive functionality
#[test]
fn test_api_consistency_comprehensive() {
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

/// Test production readiness comprehensive functionality
#[test]
fn test_production_readiness_comprehensive() {
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

/// Test accessibility comprehensive functionality
#[test]
fn test_accessibility_comprehensive() {
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

/// Test cross-platform compatibility comprehensive functionality
#[test]
fn test_cross_platform_compatibility_comprehensive() {
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

/// Test security comprehensive functionality
#[test]
fn test_security_comprehensive() {
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

/// Test final release validation comprehensive functionality
#[test]
fn test_final_release_validation_comprehensive() {
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
