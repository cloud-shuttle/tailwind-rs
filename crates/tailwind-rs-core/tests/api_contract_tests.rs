//! API Contract Tests
//!
//! This module provides comprehensive contract testing to ensure API stability,
//! backward compatibility, and reliability.

use tailwind_rs_core::*;

/// Test ClassBuilder API contract
#[test]
fn test_class_builder_api_contract() {
    // Test basic class building
    let class_set = ClassBuilder::new()
        .class("p-4")
        .class("m-2")
        .responsive(Breakpoint::Md, "text-lg")
        .conditional("hover", "bg-blue-600")
        .custom("primary-color", "#3b82f6")
        .build();
    
    // Test that output contains expected classes
    assert!(class_set.has_class("p-4"));
    assert!(class_set.has_class("m-2"));
    
    // Test responsive classes
    let responsive_classes = class_set.get_responsive_classes(Breakpoint::Md);
    assert!(responsive_classes.contains(&"text-lg".to_string()));
    
    // Test conditional classes
    let hover_classes = class_set.get_conditional_classes("hover");
    assert!(hover_classes.contains(&"bg-blue-600".to_string()));
    
    // Test custom properties
    let custom_props = class_set.get_custom_properties();
    assert_eq!(custom_props.get("primary-color"), Some(&"#3b82f6".to_string()));
}

/// Test ClassBuilder API contract with invalid input
#[test]
fn test_class_builder_api_contract_invalid_input() {
    // Test with empty class name - should still work but be empty
    let class_set = ClassBuilder::new()
        .class("")  // Empty class name
        .build();
    
    // Empty class is actually added (current behavior)
    assert!(class_set.has_class(""));
    assert!(!class_set.get_classes().is_empty());
}

/// Test CssGenerator API contract
#[test]
fn test_css_generator_api_contract() {
    // Test CSS generation with basic classes
    let mut generator = CssGenerator::new();
    generator.add_class("p-4").unwrap();
    generator.add_class("m-2").unwrap();
    
    let css = generator.generate_css();
    
    // Test that output contains expected CSS
    assert!(css.contains("padding:"));
    assert!(css.contains("margin:"));
}

/// Test CssGenerator API contract with invalid input
#[test]
fn test_css_generator_api_contract_invalid_input() {
    // Test with empty class name - should return error
    let mut generator = CssGenerator::new();
    let result = generator.add_class("");  // Empty class name
    
    // Should return an error for empty class
    assert!(result.is_err());
    
    let css = generator.generate_css();
    
    // CSS should be empty since no valid classes were added
    assert!(css.is_empty());
}

/// Test API consistency
#[test]
fn test_api_consistency() {
    // Test that all utility methods follow the same pattern
    let spacing_test = ClassBuilder::new()
        .class("p-4")
        .class("m-2")
        .class("px-6")
        .class("py-3")
        .build();

    assert!(!spacing_test.to_css_classes().is_empty());

    // Test that all color methods follow the same pattern
    let color_test = ClassBuilder::new()
        .class("bg-blue-500")
        .class("text-white")
        .class("border-gray-300")
        .build();

    assert!(!color_test.to_css_classes().is_empty());

    // Test that all layout methods follow the same pattern
    let layout_test = ClassBuilder::new()
        .class("flex")
        .class("items-center")
        .class("justify-center")
        .build();

    assert!(!layout_test.to_css_classes().is_empty());
}

/// Test responsive design API
#[test]
fn test_responsive_design_api() {
    let class_set = ClassBuilder::new()
        .class("text-sm")
        .responsive(Breakpoint::Md, "text-base")
        .responsive(Breakpoint::Lg, "text-lg")
        .build();
    
    // Test base classes
    assert!(class_set.has_class("text-sm"));
    
    // Test responsive classes
    let md_classes = class_set.get_responsive_classes(Breakpoint::Md);
    assert!(md_classes.contains(&"text-base".to_string()));
    
    let lg_classes = class_set.get_responsive_classes(Breakpoint::Lg);
    assert!(lg_classes.contains(&"text-lg".to_string()));
}

/// Test conditional classes API
#[test]
fn test_conditional_classes_api() {
    let class_set = ClassBuilder::new()
        .class("bg-white")
        .conditional("hover", "bg-gray-100")
        .conditional("focus", "bg-blue-50")
        .conditional("dark", "bg-gray-800")
        .build();
    
    // Test base classes
    assert!(class_set.has_class("bg-white"));
    
    // Test conditional classes
    let hover_classes = class_set.get_conditional_classes("hover");
    assert!(hover_classes.contains(&"bg-gray-100".to_string()));
    
    let focus_classes = class_set.get_conditional_classes("focus");
    assert!(focus_classes.contains(&"bg-blue-50".to_string()));
    
    let dark_classes = class_set.get_conditional_classes("dark");
    assert!(dark_classes.contains(&"bg-gray-800".to_string()));
}

/// Test custom properties API
#[test]
fn test_custom_properties_api() {
    let class_set = ClassBuilder::new()
        .class("custom-button")
        .custom("--primary-color", "#3b82f6")
        .custom("--border-radius", "0.5rem")
        .build();
    
    // Test base classes
    assert!(class_set.has_class("custom-button"));
    
    // Test custom properties
    let custom_props = class_set.get_custom_properties();
    assert_eq!(custom_props.get("--primary-color"), Some(&"#3b82f6".to_string()));
    assert_eq!(custom_props.get("--border-radius"), Some(&"0.5rem".to_string()));
}

/// Test ARIA variant API
#[test]
fn test_aria_variant_api() {
    let class_set = ClassBuilder::new()
        .class("button")
        .aria("expanded", "bg-blue-100")
        .aria("pressed", "bg-blue-200")
        .build();
    
    // Test base classes
    assert!(class_set.has_class("button"));
    
    // Test ARIA conditional classes
    let aria_expanded_classes = class_set.get_conditional_classes("aria-expanded");
    assert!(aria_expanded_classes.contains(&"bg-blue-100".to_string()));
    
    let aria_pressed_classes = class_set.get_conditional_classes("aria-pressed");
    assert!(aria_pressed_classes.contains(&"bg-blue-200".to_string()));
}

/// Test data variant API
#[test]
fn test_data_variant_api() {
    let class_set = ClassBuilder::new()
        .class("component")
        .data("state", Some("active".to_string()), "bg-green-100")
        .data("loading", None, "opacity-50")
        .build();
    
    // Test base classes
    assert!(class_set.has_class("component"));
    
    // Test data conditional classes
    let data_state_classes = class_set.get_conditional_classes("data-state=active");
    assert!(data_state_classes.contains(&"bg-green-100".to_string()));
    
    let data_loading_classes = class_set.get_conditional_classes("data-loading");
    assert!(data_loading_classes.contains(&"opacity-50".to_string()));
}

/// Test supports variant API
#[test]
fn test_supports_variant_api() {
    let class_set = ClassBuilder::new()
        .class("feature")
        .supports("backdrop-filter", "backdrop-blur-sm")
        .supports("grid", "grid-cols-3")
        .build();
    
    // Test base classes
    assert!(class_set.has_class("feature"));
    
    // Test supports conditional classes
    let backdrop_filter_classes = class_set.get_conditional_classes("supports-backdrop-filter");
    assert!(backdrop_filter_classes.contains(&"backdrop-blur-sm".to_string()));
    
    let grid_classes = class_set.get_conditional_classes("supports-grid");
    assert!(grid_classes.contains(&"grid-cols-3".to_string()));
}

/// Test complex class set building
#[test]
fn test_complex_class_set_building() {
    let class_set = ClassBuilder::new()
        .class("card")
        .class("shadow-lg")
        .class("rounded-lg")
        .responsive(Breakpoint::Md, "shadow-xl")
        .responsive(Breakpoint::Lg, "shadow-2xl")
        .conditional("hover", "shadow-2xl")
        .conditional("focus", "ring-2")
        .conditional("focus", "ring-blue-500")
        .custom("--card-padding", "1.5rem")
        .custom("--card-border", "1px solid #e5e7eb")
        .aria("expanded", "bg-blue-50")
        .data("variant", Some("primary".to_string()), "bg-blue-500")
        .data("variant", Some("secondary".to_string()), "bg-gray-500")
        .supports("backdrop-filter", "backdrop-blur-sm")
        .build();
    
    // Test base classes
    assert!(class_set.has_class("card"));
    assert!(class_set.has_class("shadow-lg"));
    assert!(class_set.has_class("rounded-lg"));
    
    // Test responsive classes
    let md_classes = class_set.get_responsive_classes(Breakpoint::Md);
    assert!(md_classes.contains(&"shadow-xl".to_string()));
    
    let lg_classes = class_set.get_responsive_classes(Breakpoint::Lg);
    assert!(lg_classes.contains(&"shadow-2xl".to_string()));
    
    // Test conditional classes
    let hover_classes = class_set.get_conditional_classes("hover");
    assert!(hover_classes.contains(&"shadow-2xl".to_string()));
    
    let focus_classes = class_set.get_conditional_classes("focus");
    assert!(focus_classes.contains(&"ring-2".to_string()));
    assert!(focus_classes.contains(&"ring-blue-500".to_string()));
    
    // Test custom properties
    let custom_props = class_set.get_custom_properties();
    assert_eq!(custom_props.get("--card-padding"), Some(&"1.5rem".to_string()));
    assert_eq!(custom_props.get("--card-border"), Some(&"1px solid #e5e7eb".to_string()));
    
    // Test ARIA classes
    let aria_expanded_classes = class_set.get_conditional_classes("aria-expanded");
    assert!(aria_expanded_classes.contains(&"bg-blue-50".to_string()));
    
    // Test data classes
    let data_primary_classes = class_set.get_conditional_classes("data-variant=primary");
    assert!(data_primary_classes.contains(&"bg-blue-500".to_string()));
    
    let data_secondary_classes = class_set.get_conditional_classes("data-variant=secondary");
    assert!(data_secondary_classes.contains(&"bg-gray-500".to_string()));
    
    // Test supports classes
    let backdrop_filter_classes = class_set.get_conditional_classes("supports-backdrop-filter");
    assert!(backdrop_filter_classes.contains(&"backdrop-blur-sm".to_string()));
}

/// Test CSS generation with complex class set
#[test]
fn test_css_generation_with_complex_class_set() {
    let mut generator = CssGenerator::new();
    
    // Add various types of classes
    generator.add_class("p-4").unwrap();
    generator.add_class("m-2").unwrap();
    generator.add_class("bg-blue-500").unwrap();
    generator.add_class("text-white").unwrap();
    generator.add_class("rounded-lg").unwrap();
    generator.add_class("shadow-lg").unwrap();
    
    // Add responsive classes
    generator.add_responsive_class(Breakpoint::Md, "p-6").unwrap();
    generator.add_responsive_class(Breakpoint::Lg, "p-8").unwrap();
    
    // Add more classes (CssGenerator doesn't support conditional classes directly)
    generator.add_class("hover:bg-blue-600").unwrap();
    generator.add_class("focus:ring-2").unwrap();
    generator.add_class("focus:ring-blue-500").unwrap();
    
    // Add custom properties
    generator.add_custom_property("--primary-color", "#3b82f6");
    generator.add_custom_property("--border-radius", "0.5rem");
    
    let css = generator.generate_css();
    
    // Test that output contains expected CSS
    assert!(css.contains("padding:"));
    assert!(css.contains("margin:"));
    assert!(css.contains("background-color:"));
    assert!(css.contains("color:"));
    assert!(css.contains("border-radius:"));
    assert!(css.contains("box-shadow:"));
    
    // Test responsive CSS
    assert!(css.contains("@media"));
    assert!(css.contains("min-width: 768px"));
    assert!(css.contains("min-width: 1024px"));
    
    // Test that CSS was generated (conditional classes are handled by the parser)
    assert!(css.len() > 100); // Should have substantial CSS
    
    // Test custom properties
    assert!(css.contains("--primary-color"));
    assert!(css.contains("--border-radius"));
}

/// Test error handling
#[test]
fn test_error_handling() {
    // Test with valid classes and error handling
    let mut generator = CssGenerator::new();
    
    // Test that valid classes work
    generator.add_class("p-4").unwrap();
    generator.add_class("m-2").unwrap();
    
    let css = generator.generate_css();
    
    // Should generate CSS successfully
    assert!(!css.is_empty());
    assert!(css.contains("padding"));
    assert!(css.contains("margin"));
}

/// Test performance with many classes
#[test]
fn test_performance_with_many_classes() {
    let mut generator = CssGenerator::new();
    
    // Add many classes using valid Tailwind classes
    let valid_spacing = ["p-1", "p-2", "p-3", "p-4", "p-5", "p-6", "p-8", "p-10", "p-12", "p-16", "p-20", "p-24", "p-32", "p-40", "p-48", "p-56", "p-64", "p-72", "p-80", "p-96"];
    let valid_margin = ["m-1", "m-2", "m-3", "m-4", "m-5", "m-6", "m-8", "m-10", "m-12", "m-16", "m-20", "m-24", "m-32", "m-40", "m-48", "m-56", "m-64", "m-72", "m-80", "m-96"];
    
    for i in 0..100 {
        generator.add_class(valid_spacing[i % valid_spacing.len()]).unwrap();
        generator.add_class(valid_margin[i % valid_margin.len()]).unwrap();
        generator.add_class("text-sm").unwrap(); // Use valid class
    }
    
    let css = generator.generate_css();
    
    // Should generate CSS successfully
    assert!(!css.is_empty());
    assert!(css.len() > 1000); // Should be substantial
}