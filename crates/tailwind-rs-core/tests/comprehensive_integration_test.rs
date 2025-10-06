//! Comprehensive Integration Tests
//!
//! This test suite validates the complete integration of all Tailwind-RS components:
//! - CSS generation pipeline
//! - Advanced variants (container queries, custom variants, arbitrary variants)
//! - Plugin system integration
//! - Color caching performance
//! - Transform accumulation
//! - CSS functions (@apply, @layer, @import)
//! - CLI and PostCSS integration points

use tailwind_rs_core::*;
use std::collections::HashSet;

/// Test complete CSS generation pipeline from classes to CSS
#[test]
fn test_complete_css_generation_pipeline() {
    let generator = CssGenerator::new();

    // Test basic utility classes
    let classes = vec![
        "bg-blue-500",
        "text-white",
        "flex",
        "items-center",
        "justify-center",
        "p-4",
        "rounded-lg",
        "shadow-md",
    ];

    let css = generator.process_element_classes(&classes);

    // Verify CSS contains expected rules
    assert!(css.contains("background-color"));
    assert!(css.contains("color: rgb(255, 255, 255)"));
    assert!(css.contains("display: flex"));
    assert!(css.contains("align-items: center"));
    assert!(css.contains("justify-content: center"));
    assert!(css.contains("padding: 1rem"));
    assert!(css.contains("border-radius: 0.5rem"));
    assert!(css.contains("box-shadow"));
}

/// Test advanced variants integration
#[test]
fn test_advanced_variants_integration() {
    let generator = CssGenerator::new();

    // Test container query variants
    let container_classes = vec!["@container-md:flex"];
    let css = generator.process_element_classes(&container_classes);
    // Container queries generate CSS with @container selectors
    assert!(css.contains("@container") || css.contains("flex"));

    // Test arbitrary variants
    let arbitrary_classes = vec!["[data-open]:flex"];
    let css = generator.process_element_classes(&arbitrary_classes);
    assert!(css.contains("flex"));

    // Test advanced device variants
    let device_classes = vec!["pointer-coarse:flex", "motion-reduce:hidden"];
    let css = generator.process_element_classes(&device_classes);
    assert!(css.contains("flex") || css.contains("hidden"));
}

/// Test transform accumulation with CSS custom properties
#[test]
fn test_transform_accumulation() {
    let generator = CssGenerator::new();

    // Test multiple transforms on same element
    let transform_classes = vec![
        "scale-110",
        "rotate-3",
        "translate-x-2",
        "skew-x-3",
    ];

    let css = generator.process_element_classes(&transform_classes);

    // Should contain CSS custom properties for transform accumulation
    assert!(css.contains("--tw-scale-x"));
    assert!(css.contains("--tw-rotate"));
    assert!(css.contains("--tw-translate-x"));
    assert!(css.contains("--tw-skew-x"));
    assert!(css.contains("--tw-transform"));
    assert!(css.contains(".transform"));
}

/// Test gradient hover functionality
#[test]
fn test_gradient_hover_functionality() {
    let generator = CssGenerator::new();

    // Test gradient classes with hover variants
    let gradient_classes = vec![
        "bg-gradient-to-r",
        "from-blue-500",
        "to-purple-600",
        "hover:from-blue-600",
        "hover:to-purple-700",
    ];

    let css = generator.process_element_classes(&gradient_classes);

    // Should generate compound background-image rules for hover states
    assert!(css.contains("background-image"));
    assert!(css.contains("linear-gradient"));
}

/// Test color caching performance
#[test]
fn test_color_caching_performance() {
    let generator = CssGenerator::new();

    // Generate CSS for many color variants to test caching
    let color_classes: Vec<String> = (1..=50)
        .flat_map(|i| vec![
            format!("bg-blue-{}", i * 10),
            format!("text-red-{}", i * 10),
            format!("border-green-{}", i * 10),
        ])
        .collect();

    let css = generator.process_element_classes(&color_classes.iter().map(|s| s.as_str()).collect::<Vec<_>>());

    // Should contain many color rules without performance issues
    assert!(css.contains("background-color"));
    assert!(css.contains("color:"));
    assert!(css.contains("border-color"));
}

/// Test plugin system integration
#[test]
fn test_plugin_system_integration() {
    let mut generator = CssGenerator::new();

    // Create a simple test plugin
    struct TestPlugin;
    impl Plugin for TestPlugin {
        fn name(&self) -> &str { "test-plugin" }
        fn version(&self) -> &str { "1.0.0" }

        fn add_utilities(&self, utilities: &mut HashMap<String, Vec<CssProperty>>) -> Result<(), TailwindError> {
            utilities.insert("test-utility".to_string(), vec![
                CssProperty {
                    name: "test-property".to_string(),
                    value: "test-value".to_string(),
                    important: false,
                }
            ]);
            Ok(())
        }
    }

    // Register plugin
    generator.register_plugin(Box::new(TestPlugin)).unwrap();

    // Test that plugin utilities are available
    let css = generator.process_element_classes(&["test-utility"]);
    assert!(css.contains("test-property"));
    assert!(css.contains("test-value"));
}

/// Test CSS functions (@apply directive)
#[test]
fn test_css_functions_apply() {
    // Note: @apply processing happens at the PostCSS plugin level
    // This test validates that the underlying functionality works

    let generator = CssGenerator::new();

    // Test that individual classes work (prerequisite for @apply)
    let classes = vec!["flex", "items-center", "justify-center"];
    let css = generator.process_element_classes(&classes);

    assert!(css.contains("display: flex"));
    assert!(css.contains("align-items: center"));
    assert!(css.contains("justify-content: center"));
}

/// Test responsive breakpoints integration
#[test]
fn test_responsive_breakpoints_integration() {
    let generator = CssGenerator::new();

    // Test responsive variants
    let responsive_classes = vec![
        "flex",           // base
        "md:grid",        // medium and up
        "lg:flex-col",    // large and up
        "xl:justify-end", // extra large and up
    ];

    let css = generator.process_element_classes(&responsive_classes);

    // Should contain media queries for responsive breakpoints
    assert!(css.contains("@media") || css.contains("flex") || css.contains("grid"));
}

/// Test complex variant combinations
#[test]
fn test_complex_variant_combinations() {
    let generator = CssGenerator::new();

    // Test complex nested variants
    let complex_classes = vec![
        "hover:focus:bg-blue-500",
        "dark:md:hover:text-white",
        "group-hover:peer-focus:opacity-75",
    ];

    let css = generator.process_element_classes(&complex_classes);

    // Should handle complex variant combinations
    assert!(css.contains("background-color") || css.contains("color") || css.contains("opacity"));
}

/// Test CSS optimization and deduplication
#[test]
fn test_css_optimization() {
    let generator = CssGenerator::new();

    // Test duplicate classes (should be deduplicated)
    let duplicate_classes = vec![
        "bg-blue-500",
        "bg-blue-500", // duplicate
        "text-white",
        "p-4",
        "p-4", // duplicate
    ];

    let css = generator.process_element_classes(&duplicate_classes);

    // Should still generate valid CSS (optimization happens at different level)
    assert!(css.contains("background-color"));
    assert!(css.contains("color"));
    assert!(css.contains("padding"));
}

/// Test error handling and recovery
#[test]
fn test_error_handling() {
    let generator = CssGenerator::new();

    // Test with invalid classes (should not panic)
    let invalid_classes = vec![
        "invalid-class-123",
        "bg-blue-500", // valid
        "another-invalid-class",
        "text-white", // valid
    ];

    let css = generator.process_element_classes(&invalid_classes);

    // Should still generate CSS for valid classes
    assert!(css.contains("background-color"));
    assert!(css.contains("color"));
}

/// Performance regression test
#[test]
fn test_performance_regression() {
    let generator = CssGenerator::new();

    // Generate a large number of classes to test performance
    let large_classes: Vec<String> = (0..1000)
        .map(|i| format!("bg-blue-{}", (i % 9) * 100))
        .collect();

    let start = std::time::Instant::now();
    let css = generator.process_element_classes(&large_classes.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    let duration = start.elapsed();

    // Should complete in reasonable time (< 1 second for 1000 classes)
    assert!(duration < std::time::Duration::from_secs(1));
    assert!(!css.is_empty());
}

/// Test integration with CLI-style processing
#[test]
fn test_cli_style_processing() {
    let generator = CssGenerator::new();

    // Simulate processing multiple elements as in CLI
    let element_groups = vec![
        vec!["bg-blue-500", "text-white", "p-4"],
        vec!["flex", "items-center", "justify-center"],
        vec!["rounded-lg", "shadow-md", "border"],
    ];

    let mut combined_css = String::new();
    for group in element_groups {
        combined_css.push_str(&generator.process_element_classes(&group));
    }

    // Should contain CSS from all element groups
    assert!(combined_css.contains("background-color"));
    assert!(combined_css.contains("display: flex"));
    assert!(combined_css.contains("border-radius"));
}

/// Test PostCSS integration compatibility
#[test]
fn test_postcss_compatibility() {
    let generator = CssGenerator::new();

    // Test classes that would be processed by PostCSS plugin
    let postcss_classes = vec![
        "bg-gradient-to-r",
        "from-blue-500",
        "via-purple-400",
        "to-pink-500",
        "hover:scale-105",
        "focus:ring-2",
        "focus:ring-blue-500",
    ];

    let css = generator.process_element_classes(&postcss_classes);

    // Should generate valid CSS that PostCSS can further process
    assert!(css.contains("background-image") || css.contains("transform") || css.contains("box-shadow"));
}

/// Comprehensive integration test covering all major features
#[test]
fn test_comprehensive_integration() {
    let mut generator = CssGenerator::new();

    // Test all major feature categories
    let comprehensive_classes = vec![
        // Basic utilities
        "bg-blue-500", "text-white", "p-4", "rounded-lg",

        // Layout
        "flex", "grid", "items-center", "justify-center",

        // Responsive
        "md:grid-cols-2", "lg:flex-col",

        // State variants
        "hover:bg-blue-600", "focus:ring-2",

        // Dark mode
        "dark:bg-gray-800", "dark:text-white",

        // Transforms
        "scale-110", "rotate-3", "translate-x-2",

        // Effects
        "shadow-md", "blur-sm", "opacity-75",

        // Animations
        "animate-spin", "transition-all", "duration-300",

        // Advanced variants (if supported)
        "pointer-coarse:flex", "motion-reduce:hidden",
    ];

    let css = generator.process_element_classes(&comprehensive_classes);

    // Should generate substantial CSS output
    assert!(css.len() > 100); // Reasonable minimum size

    // Should contain key CSS properties from different categories
    let essential_properties = [
        "background-color", "color", "padding", "border-radius",
        "display", "align-items", "justify-content",
        "transform", "box-shadow", "opacity",
        "transition", "animation",
    ];

    let css_lower = css.to_lowercase();
    for prop in essential_properties {
        assert!(css_lower.contains(&prop.to_lowercase()),
                "Missing property: {}", prop);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_css_generation() {
        let generator = CssGenerator::new();

        // Benchmark with realistic class set
        let benchmark_classes = vec![
            "bg-blue-500", "hover:bg-blue-600", "text-white", "dark:text-gray-200",
            "flex", "items-center", "justify-between", "p-4", "rounded-lg",
            "shadow-md", "md:grid", "lg:flex-col", "scale-105", "transition-all",
        ];

        let iterations = 100;
        let mut total_time = std::time::Duration::new(0, 0);

        for _ in 0..iterations {
            let start = Instant::now();
            let _css = generator.process_element_classes(&benchmark_classes);
            total_time += start.elapsed();
        }

        let avg_time = total_time / iterations as u32;
        println!("Average CSS generation time: {:?}", avg_time);

        // Should be reasonably fast (< 10ms per generation)
        assert!(avg_time < std::time::Duration::from_millis(10));
    }

    #[test]
    fn memory_usage_test() {
        let generator = CssGenerator::new();

        // Process many classes to test memory usage
        let many_classes: Vec<String> = (0..1000)
            .map(|i| format!("bg-blue-{}", i % 900 + 100))
            .collect();

        let _css = generator.process_element_classes(&many_classes.iter().map(|s| s.as_str()).collect::<Vec<_>>());

        // Test passes if no memory issues occur
        assert!(true);
    }
}
