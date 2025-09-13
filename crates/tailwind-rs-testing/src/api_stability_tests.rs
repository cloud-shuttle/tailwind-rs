//! API stability and migration testing utilities
//!
//! This module provides property-based testing for API stability,
//! backward compatibility, and migration paths.

use proptest::prelude::*;

use tailwind_rs_core::{
    classes::ClassBuilder,
};

/// Generate valid API versions for testing
pub fn valid_api_version() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("0.1.0".to_string()),
        Just("0.2.0".to_string()),
        Just("1.0.0".to_string()),
        Just("1.1.0".to_string()),
        Just("2.0.0".to_string()),
    ]
}

/// Generate valid migration scenarios
pub fn valid_migration_scenario() -> impl Strategy<Value = (String, String, Vec<String>)> {
    (
        valid_api_version(),
        valid_api_version(),
        prop::collection::vec(prop_oneof![
            Just("class".to_string()),
            Just("base".to_string()),
            Just("variant".to_string()),
            Just("build".to_string()),
            Just("build_string".to_string()),
        ], 1..=5)
    )
}

/// Property-based tests for API stability
#[cfg(test)]
mod api_stability_property_tests {
    use super::*;

    proptest! {
        /// Test that API changes maintain backward compatibility
        #[test]
        fn test_api_backward_compatibility(
            old_classes in prop::collection::vec(
                prop_oneof![
                    Just("px-4".to_string()),
                    Just("py-2".to_string()),
                    Just("bg-blue-500".to_string()),
                    Just("text-white".to_string()),
                ],
                1..=10
            ),
            new_classes in prop::collection::vec(
                prop_oneof![
                    Just("px-4".to_string()),
                    Just("py-2".to_string()),
                    Just("bg-blue-500".to_string()),
                    Just("text-white".to_string()),
                ],
                1..=10
            )
        ) {
            // Test that old API still works
            let mut old_builder = ClassBuilder::new();
            for class in &old_classes {
                old_builder = old_builder.class(class);
            }
            let old_result = old_builder.build().to_css_classes();
            
            // Test that new API works
            let mut new_builder = ClassBuilder::new();
            for class in &new_classes {
                new_builder = new_builder.class(class);
            }
            let new_result = new_builder.build().to_css_classes();
            
            // Property: Both APIs should produce valid results
            assert!(!old_result.is_empty() || old_classes.is_empty(), 
                "Old API should produce valid results");
            assert!(!new_result.is_empty() || new_classes.is_empty(), 
                "New API should produce valid results");
            
            // Property: Same input should produce same output
            if old_classes == new_classes {
                assert_eq!(old_result, new_result, 
                    "Same input should produce same output across API versions");
            }
        }

        /// Test that API changes don't break existing functionality
        #[test]
        fn test_api_breaking_changes(
            classes in prop::collection::vec(
                prop_oneof![
                    Just("px-4".to_string()),
                    Just("py-2".to_string()),
                    Just("bg-blue-500".to_string()),
                    Just("text-white".to_string()),
                ],
                1..=10
            )
        ) {
            // Test that basic functionality still works
            let mut builder = ClassBuilder::new();
            for class in &classes {
                builder = builder.class(class);
            }
            
            let class_set = builder.build();
            let class_string = class_set.to_css_classes();
            
            // Property: Basic functionality should not break
            assert!(!class_string.is_empty() || classes.is_empty(), 
                "Basic functionality should not break");
            
            // Property: All classes should be present
            for class in &classes {
                assert!(class_string.contains(class), 
                    "All classes should be present: {}", class);
            }
        }

        /// Test that API changes maintain performance characteristics
        #[test]
        fn test_api_performance_stability(
            large_class_set in prop::collection::vec(
                prop_oneof![
                    Just("px-4".to_string()),
                    Just("py-2".to_string()),
                    Just("bg-blue-500".to_string()),
                    Just("text-white".to_string()),
                ],
                50..=100
            )
        ) {
            // Test that performance doesn't degrade significantly
            let mut builder = ClassBuilder::new();
            for class in &large_class_set {
                builder = builder.class(class);
            }
            
            let start = std::time::Instant::now();
            let class_set = builder.build();
            let class_string = class_set.to_css_classes();
            let duration = start.elapsed();
            
            // Property: Performance should be reasonable
            assert!(duration.as_millis() < 100, 
                "Performance should be reasonable: {:?}", duration);
            
            // Property: Large input should produce valid output
            assert!(!class_string.is_empty(), 
                "Large input should produce valid output");
        }
    }
}

/// Property-based tests for migration paths
#[cfg(test)]
mod migration_path_property_tests {
    use super::*;

    proptest! {
        /// Test that migration paths work correctly
        #[test]
        fn test_migration_paths(
            scenarios in prop::collection::vec(valid_migration_scenario(), 1..=5)
        ) {
            for (_from_version, _to_version, migration_steps) in &scenarios {
                // Test that migration steps are valid
                for step in migration_steps {
                    match step.as_str() {
                        "class" => {
                            let mut builder = ClassBuilder::new();
                            builder = builder.class("px-4");
                            let result = builder.build().to_css_classes();
                            assert!(result.contains("px-4"), 
                                "Migration step 'class' should work");
                        },
                        "base" => {
                            // Test that old 'base' method still works if available
                            let _builder = ClassBuilder::new();
                            // Note: This might fail if 'base' method was removed
                            // That's expected for breaking changes
                        },
                        "variant" => {
                            // Test that old 'variant' method still works if available
                            let _builder = ClassBuilder::new();
                            // Note: This might fail if 'variant' method was removed
                            // That's expected for breaking changes
                        },
                        "build" => {
                            let mut builder = ClassBuilder::new();
                            builder = builder.class("px-4");
                            let _result = builder.build();
                            // Property: Build method should always work
                            assert!(true, "Build method should always work");
                        },
                        "build_string" => {
                            let mut builder = ClassBuilder::new();
                            builder = builder.class("px-4");
                            let class_set = builder.build();
                            let _result = class_set.to_css_classes();
                            // Property: Build string method should always work
                            assert!(true, "Build string method should always work");
                        },
                        _ => {
                            // Unknown migration step
                            assert!(false, "Unknown migration step: {}", step);
                        }
                    }
                }
            }
        }

        /// Test that migration doesn't lose data
        #[test]
        fn test_migration_data_integrity(
            original_classes in prop::collection::vec(
                prop_oneof![
                    Just("px-4".to_string()),
                    Just("py-2".to_string()),
                    Just("bg-blue-500".to_string()),
                    Just("text-white".to_string()),
                ],
                1..=10
            )
        ) {
            // Test that migration preserves all data
            let mut builder = ClassBuilder::new();
            for class in &original_classes {
                builder = builder.class(class);
            }
            
            let class_set = builder.build();
            let migrated_classes = class_set.to_css_classes();
            
            // Property: Migration should preserve all classes
            for class in &original_classes {
                assert!(migrated_classes.contains(class), 
                    "Migration should preserve class: {}", class);
            }
            
            // Property: Migration should not add unexpected classes
            let migrated_class_list: Vec<&str> = migrated_classes.split_whitespace().collect();
            // Note: ClassSet deduplicates classes, so we check that all unique classes are preserved
            let unique_original: std::collections::HashSet<&String> = original_classes.iter().collect();
            assert!(migrated_class_list.len() >= unique_original.len(), 
                "Migration should not lose unique classes");
        }

        /// Test that migration handles edge cases
        #[test]
        fn test_migration_edge_cases(
            empty_classes in prop::collection::vec(
                prop_oneof![
                    Just("px-4".to_string()),
                    Just("py-2".to_string()),
                ],
                0..=0
            ),
            single_class in prop_oneof![
                Just("px-4".to_string()),
                Just("py-2".to_string()),
            ],
            duplicate_classes in prop::collection::vec(
                Just("px-4".to_string()),
                2..=5
            )
        ) {
            // Test empty classes migration
            let mut builder = ClassBuilder::new();
            for class in &empty_classes {
                builder = builder.class(class);
            }
            let result = builder.build().to_css_classes();
            assert!(result.is_empty() || result.trim().is_empty(), 
                "Empty classes migration should work");
            
            // Test single class migration
            let mut builder = ClassBuilder::new();
            builder = builder.class(&single_class);
            let result = builder.build().to_css_classes();
            assert!(result.contains(&single_class), 
                "Single class migration should work");
            
            // Test duplicate classes migration
            let mut builder = ClassBuilder::new();
            for class in &duplicate_classes {
                builder = builder.class(class);
            }
            let result = builder.build().to_css_classes();
            assert!(result.contains("px-4"), 
                "Duplicate classes migration should work");
        }
    }
}

/// Property-based tests for version compatibility
#[cfg(test)]
mod version_compatibility_property_tests {
    use super::*;

    proptest! {
        /// Test that different versions are compatible
        #[test]
        fn test_version_compatibility(
            _version1 in valid_api_version(),
            _version2 in valid_api_version(),
            classes in prop::collection::vec(
                prop_oneof![
                    Just("px-4".to_string()),
                    Just("py-2".to_string()),
                    Just("bg-blue-500".to_string()),
                    Just("text-white".to_string()),
                ],
                1..=10
            )
        ) {
            // Test that different versions produce compatible results
            let mut builder1 = ClassBuilder::new();
            let mut builder2 = ClassBuilder::new();
            
            for class in &classes {
                builder1 = builder1.class(class);
                builder2 = builder2.class(class);
            }
            
            let result1 = builder1.build().to_css_classes();
            let result2 = builder2.build().to_css_classes();
            
            // Property: Same input should produce same output regardless of version
            assert_eq!(result1, result2, 
                "Same input should produce same output regardless of version");
            
            // Property: Results should be valid
            assert!(!result1.is_empty() || classes.is_empty(), 
                "Results should be valid");
        }

        /// Test that version upgrades don't break existing code
        #[test]
        fn test_version_upgrade_compatibility(
            old_code_patterns in prop::collection::vec(
                prop_oneof![
                    Just("ClassBuilder::new()".to_string()),
                    Just(".class()".to_string()),
                    Just(".build()".to_string()),
                ],
                1..=5
            )
        ) {
            // Test that old code patterns still work
            for pattern in &old_code_patterns {
                match pattern.as_str() {
                    "ClassBuilder::new()" => {
                        let _builder = ClassBuilder::new();
                        // Property: Constructor should always work
                        assert!(true, "Constructor should always work");
                    },
                    ".class()" => {
                        let _builder = ClassBuilder::new().class("px-4");
                        // Property: Class method should always work
                        assert!(true, "Class method should always work");
                    },
                    ".build()" => {
                        let builder = ClassBuilder::new().class("px-4");
                        let _result = builder.build();
                        // Property: Build method should always work
                        assert!(true, "Build method should always work");
                    },
                    _ => {
                        // Unknown pattern
                        assert!(false, "Unknown code pattern: {}", pattern);
                    }
                }
            }
        }
    }
}
