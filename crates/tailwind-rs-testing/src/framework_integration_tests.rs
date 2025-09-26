//! Property-based tests for framework integrations
//!
//! This module provides comprehensive property-based testing for
//! framework-specific integrations like Yew, Leptos, and Dioxus.

use proptest::prelude::*;

use tailwind_rs_core::{classes::ClassBuilder, responsive::Breakpoint};

/// Generate valid Tailwind CSS class names for framework testing
pub fn valid_framework_class() -> impl Strategy<Value = String> {
    prop_oneof![
        // Layout classes
        prop_oneof![
            Just("block".to_string()),
            Just("inline".to_string()),
            Just("flex".to_string()),
            Just("grid".to_string()),
            Just("hidden".to_string()),
        ],
        // Spacing classes
        (0..=12u8).prop_map(|n| format!("p-{}", n)),
        (0..=12u8).prop_map(|n| format!("m-{}", n)),
        (0..=12u8).prop_map(|n| format!("px-{}", n)),
        (0..=12u8).prop_map(|n| format!("py-{}", n)),
        (0..=12u8).prop_map(|n| format!("mx-{}", n)),
        (0..=12u8).prop_map(|n| format!("my-{}", n)),
        // Color classes
        prop_oneof![
            Just("bg-blue-500".to_string()),
            Just("bg-red-500".to_string()),
            Just("bg-green-500".to_string()),
            Just("text-white".to_string()),
            Just("text-black".to_string()),
            Just("border-gray-300".to_string()),
        ],
        // Size classes
        prop_oneof![
            Just("w-full".to_string()),
            Just("h-full".to_string()),
            Just("w-1/2".to_string()),
            Just("h-1/2".to_string()),
        ],
        // Responsive classes
        prop_oneof![
            Just("sm:block".to_string()),
            Just("md:flex".to_string()),
            Just("lg:grid".to_string()),
            Just("xl:hidden".to_string()),
        ],
        // State classes
        prop_oneof![
            Just("hover:bg-blue-600".to_string()),
            Just("focus:ring-2".to_string()),
            Just("active:scale-95".to_string()),
            Just("disabled:opacity-50".to_string()),
        ],
    ]
}

/// Generate multiple valid Tailwind classes
pub fn valid_framework_classes() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(valid_framework_class(), 1..=10)
}

/// Generate valid component props for testing
pub fn valid_component_props() -> impl Strategy<Value = (String, String, bool)> {
    (
        valid_framework_class(),
        valid_framework_class(),
        prop::bool::ANY,
    )
}

/// Property-based tests for Yew integration
#[cfg(test)]
mod yew_integration_property_tests {
    use super::*;

    proptest! {
        /// Test that Yew class generation maintains invariants
        #[test]
        fn test_yew_class_generation_properties(
            base_classes in valid_framework_classes(),
            variant_classes in valid_framework_classes(),
            size_classes in valid_framework_classes()
        ) {
            // Create a ClassBuilder with the generated classes
            let mut builder = ClassBuilder::new();

            // Add base classes
            for class in &base_classes {
                builder = builder.class(class);
            }

            // Add variant classes
            for class in &variant_classes {
                builder = builder.class(class);
            }

            // Add size classes
            for class in &size_classes {
                builder = builder.class(class);
            }

            let class_set = builder.build();
            let class_string = class_set.to_css_classes();

            // Property: All input classes should be present in output
            for class in base_classes.iter().chain(variant_classes.iter()).chain(size_classes.iter()) {
                assert!(class_string.contains(class),
                    "Generated classes should contain: {}", class);
            }

            // Property: Class string should not be empty
            assert!(!class_string.is_empty(), "Class string should not be empty");

            // Property: Class string should be valid CSS classes
            let classes: Vec<&str> = class_string.split_whitespace().collect();
            assert!(!classes.is_empty(), "Should have at least one class");
        }

        /// Test that Yew responsive class generation works correctly
        #[test]
        fn test_yew_responsive_class_properties(
            base_classes in valid_framework_classes(),
            responsive_classes in prop::collection::vec(
                (prop_oneof![
                    Just(Breakpoint::Sm),
                    Just(Breakpoint::Md),
                    Just(Breakpoint::Lg),
                    Just(Breakpoint::Xl),
                ], valid_framework_class()),
                1..=5
            )
        ) {
            let mut builder = ClassBuilder::new();

            // Add base classes
            for class in &base_classes {
                builder = builder.class(class);
            }

            // Add responsive classes
            for (breakpoint, class) in &responsive_classes {
                let responsive_class = format!("{}{}", breakpoint.prefix(), class);
                builder = builder.class(&responsive_class);
            }

            let class_set = builder.build();
            let class_string = class_set.to_css_classes();

            // Property: All base classes should be present
            for class in &base_classes {
                assert!(class_string.contains(class),
                    "Base classes should be present: {}", class);
            }

            // Property: All responsive classes should be present
            for (breakpoint, class) in &responsive_classes {
                let responsive_class = format!("{}{}", breakpoint.prefix(), class);
                assert!(class_string.contains(&responsive_class),
                    "Responsive classes should be present: {}", responsive_class);
            }
        }

        /// Test that Yew component props maintain consistency
        #[test]
        fn test_yew_component_props_properties(
            props in prop::collection::vec(valid_component_props(), 1..=5)
        ) {
            // Test that component props can be processed consistently
            for (variant_class, size_class, disabled) in &props {
                let mut builder = ClassBuilder::new();
                builder = builder.class(variant_class);
                builder = builder.class(size_class);

                if *disabled {
                    builder = builder.class("disabled:opacity-50");
                }

                let class_set = builder.build();
                let class_string = class_set.to_css_classes();

                // Property: Props should generate valid classes
                assert!(class_string.contains(variant_class),
                    "Variant class should be present: {}", variant_class);
                assert!(class_string.contains(size_class),
                    "Size class should be present: {}", size_class);

                if *disabled {
                    assert!(class_string.contains("disabled:opacity-50"),
                        "Disabled class should be present");
                }
            }
        }

        /// Test that Yew class caching works correctly
        #[test]
        fn test_yew_class_caching_properties(
            classes in valid_framework_classes()
        ) {
            // Simulate class caching behavior
            let mut builder1 = ClassBuilder::new();
            let mut builder2 = ClassBuilder::new();

            for class in &classes {
                builder1 = builder1.class(class);
                builder2 = builder2.class(class);
            }

            let class_set1 = builder1.build();
            let class_set2 = builder2.build();

            let class_string1 = class_set1.to_css_classes();
            let class_string2 = class_set2.to_css_classes();

            // Property: Same input should produce same output
            assert_eq!(class_string1, class_string2,
                "Same input should produce same output");

            // Property: Output should be deterministic
            assert!(!class_string1.is_empty(), "Output should not be empty");
        }
    }
}

/// Property-based tests for framework integration edge cases
#[cfg(test)]
mod framework_edge_case_tests {
    use super::*;

    proptest! {
        /// Test that framework integrations handle edge cases correctly
        #[test]
        fn test_framework_edge_cases(
            empty_classes in prop::collection::vec(valid_framework_class(), 0..=0),
            single_class in valid_framework_class(),
            many_classes in prop::collection::vec(valid_framework_class(), 10..=20)
        ) {
            // Test empty classes
            let mut builder = ClassBuilder::new();
            for class in &empty_classes {
                builder = builder.class(class);
            }
            let class_set = builder.build();
            let class_string = class_set.to_css_classes();

            // Property: Empty input should produce empty output
            assert!(class_string.is_empty() || class_string.trim().is_empty(),
                "Empty input should produce empty output");

            // Test single class
            let mut builder = ClassBuilder::new();
            builder = builder.class(&single_class);
            let class_set = builder.build();
            let class_string = class_set.to_css_classes();

            // Property: Single class should be preserved
            assert!(class_string.contains(&single_class),
                "Single class should be preserved: {}", single_class);

            // Test many classes
            let mut builder = ClassBuilder::new();
            for class in &many_classes {
                builder = builder.class(class);
            }
            let class_set = builder.build();
            let class_string = class_set.to_css_classes();

            // Property: Many classes should all be present
            for class in &many_classes {
                assert!(class_string.contains(class),
                    "Many classes should all be present: {}", class);
            }
        }

        /// Test that framework integrations handle conflicting classes
        #[test]
        fn test_framework_conflicting_classes(
            conflicting_pairs in prop::collection::vec(
                (valid_framework_class(), valid_framework_class()),
                1..=5
            )
        ) {
            // Test that conflicting classes are handled gracefully
            for (class1, class2) in &conflicting_pairs {
                let mut builder = ClassBuilder::new();
                builder = builder.class(class1);
                builder = builder.class(class2);

                let class_set = builder.build();
                let class_string = class_set.to_css_classes();

                // Property: Both classes should be present (framework handles conflicts)
                assert!(class_string.contains(class1),
                    "First class should be present: {}", class1);
                assert!(class_string.contains(class2),
                    "Second class should be present: {}", class2);
            }
        }

        /// Test that framework integrations maintain performance properties
        #[test]
        fn test_framework_performance_properties(
            classes in prop::collection::vec(valid_framework_class(), 1..=100)
        ) {
            // Test that large numbers of classes don't break the system
            let mut builder = ClassBuilder::new();

            for class in &classes {
                builder = builder.class(class);
            }

            let class_set = builder.build();
            let class_string = class_set.to_css_classes();

            // Property: Large input should produce valid output
            assert!(!class_string.is_empty(), "Large input should produce valid output");

            // Property: Output should contain all input classes
            for class in &classes {
                assert!(class_string.contains(class),
                    "Output should contain all input classes: {}", class);
            }

            // Property: Output should be reasonable length
            assert!(class_string.len() < 10000, "Output should be reasonable length");
        }
    }
}

/// Property-based tests for framework integration compatibility
#[cfg(test)]
mod framework_compatibility_tests {
    use super::*;

    proptest! {
        /// Test that framework integrations are compatible with each other
        #[test]
        fn test_framework_compatibility(
            yew_classes in valid_framework_classes(),
            leptos_classes in valid_framework_classes(),
            dioxus_classes in valid_framework_classes()
        ) {
            // Test that classes work across different frameworks
            let mut yew_builder = ClassBuilder::new();
            let mut leptos_builder = ClassBuilder::new();
            let mut dioxus_builder = ClassBuilder::new();

            for class in &yew_classes {
                yew_builder = yew_builder.class(class);
            }

            for class in &leptos_classes {
                leptos_builder = leptos_builder.class(class);
            }

            for class in &dioxus_classes {
                dioxus_builder = dioxus_builder.class(class);
            }

            let yew_result = yew_builder.build().to_css_classes();
            let leptos_result = leptos_builder.build().to_css_classes();
            let dioxus_result = dioxus_builder.build().to_css_classes();

            // Property: Same classes should produce same results across frameworks
            if yew_classes == leptos_classes {
                assert_eq!(yew_result, leptos_result,
                    "Same classes should produce same results across frameworks");
            }

            if leptos_classes == dioxus_classes {
                assert_eq!(leptos_result, dioxus_result,
                    "Same classes should produce same results across frameworks");
            }

            if yew_classes == dioxus_classes {
                assert_eq!(yew_result, dioxus_result,
                    "Same classes should produce same results across frameworks");
            }
        }

        /// Test that framework integrations maintain API stability
        #[test]
        fn test_framework_api_stability(
            classes in valid_framework_classes()
        ) {
            // Test that the API remains stable across different usage patterns
            let mut builder1 = ClassBuilder::new();
            let mut builder2 = ClassBuilder::new();

            // Add classes in different orders
            for (i, class) in classes.iter().enumerate() {
                if i % 2 == 0 {
                    builder1 = builder1.class(class);
                } else {
                    builder2 = builder2.class(class);
                }
            }

            // Add remaining classes
            for (i, class) in classes.iter().enumerate() {
                if i % 2 == 1 {
                    builder1 = builder1.class(class);
                } else {
                    builder2 = builder2.class(class);
                }
            }

            let result1 = builder1.build().to_css_classes();
            let result2 = builder2.build().to_css_classes();

            // Property: Different order should produce same result
            assert_eq!(result1, result2,
                "Different order should produce same result");
        }
    }
}
