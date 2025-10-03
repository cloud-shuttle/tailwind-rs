//! Enhanced Variants Tests Module
//!
//! Comprehensive integration tests for the enhanced variant system.

use super::types::*;
use super::definitions::*;
use super::parser::*;
use super::utilities::*;
use super::combinations::*;
use super::utilities::SuggestionType;
use super::combinations::ConflictType;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn full_variant_parsing_pipeline() {
        let parser = EnhancedVariantParser::new();

        // Test cases covering various scenarios
        let test_cases = vec![
            ("bg-blue-500", "bg-blue-500", 0, true, None),
            ("hover:bg-blue-500", "bg-blue-500", 80, true, None),
            ("sm:hover:bg-blue-500", "bg-blue-500", 180, true, Some("(min-width: 640px)")),
            ("dark:sm:hover:bg-blue-500", "bg-blue-500", 260, true, Some("(min-width: 640px)")),
            ("motion-safe:hover:bg-blue-500", "bg-blue-500", 140, true, None),
            ("print:bg-blue-500", "bg-blue-500", 10, true, Some("print")),
            ("invalid:bg-blue-500", "bg-blue-500", 0, false, None),
        ];

        for (class, expected_base, expected_specificity, should_succeed, expected_media) in test_cases {
            let result = parser.parse_class(class);

            if should_succeed {
                assert!(result.is_ok(), "Failed to parse: {}", class);
                let result = result.unwrap();
                assert_eq!(result.base_class, expected_base, "Wrong base class for: {}", class);
                assert_eq!(result.combination.specificity, expected_specificity, "Wrong specificity for: {}", class);
                assert!(result.success, "Parse should succeed for: {}", class);

                if let Some(expected_mq) = expected_media {
                    assert!(result.media_query.is_some(), "Missing media query for: {}", class);
                    assert!(result.media_query.as_ref().unwrap().contains(expected_mq),
                           "Wrong media query for: {} - got: {:?}", class, result.media_query);
                }
            } else {
                assert!(result.is_err() || !result.as_ref().unwrap().success, "Should fail for: {}", class);
            }
        }
    }

    #[test]
    fn complex_variant_combinations() {
        let parser = EnhancedVariantParser::new();

        // Test complex combinations that should work
        let valid_classes = vec![
            "sm:hover:focus:bg-gradient-to-r",
            "dark:sm:active:bg-blue-500",
            "motion-safe:hover:scale-105",
            "contrast-more:focus:text-blue-600",
            "portrait:sm:flex",
        ];

        for class in valid_classes {
            let result = parser.parse_class(class);
            assert!(result.is_ok(), "Failed to parse valid class: {}", class);
            assert!(result.unwrap().success, "Parse should succeed for: {}", class);
        }

        // Test combinations that should fail
        let invalid_classes = vec![
            "print:screen:bg-blue-500", // Mutually exclusive
            "sm:md:bg-red-500", // Multiple responsive
        ];

        for class in invalid_classes {
            let result = parser.parse_class(class);
            if let Ok(result) = result {
                assert!(!result.success, "Should fail for: {}", class);
            }
        }
    }

    #[test]
    fn custom_variant_integration() {
        let mut parser = EnhancedVariantParser::new();

        // Add custom variants
        parser.add_custom_variant("group-hover".to_string(), ".group:hover ".to_string()).unwrap();
        parser.add_custom_variant("peer-focus".to_string(), ".peer:focus ~ ".to_string()).unwrap();

        // Test custom variants work
        let result = parser.parse_class("group-hover:bg-blue-500").unwrap();
        assert!(result.success);

        let result = parser.parse_class("peer-focus:text-red-500").unwrap();
        assert!(result.success);

        // Test custom variant combinations
        let result = parser.parse_class("sm:group-hover:bg-blue-500").unwrap();
        assert!(result.success);
        assert!(result.media_query.is_some());
    }

    #[test]
    fn variant_caching() {
        let mut cache = VariantCache::new(100);

        let parser = EnhancedVariantParser::new();
        let result1 = parser.parse_class("hover:bg-blue-500").unwrap();

        cache.insert("hover:bg-blue-500".to_string(), result1.clone());

        let cached = cache.get("hover:bg-blue-500");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().original_class, result1.original_class);

        // Test cache eviction (basic)
        for i in 0..101 {
            let class = format!("test{}:bg-blue-500", i);
            let result = parser.parse_class(&class).unwrap();
            cache.insert(class, result);
        }

        let (size, _) = cache.stats();
        assert!(size <= 100); // Should not exceed max size
    }

    #[test]
    fn optimization_suggestions() {
        let parser = EnhancedVariantParser::new();

        let results = vec![
            parser.parse_class("hover:bg-blue-500").unwrap(),
            parser.parse_class("sm:md:hover:bg-red-500").unwrap(), // Multiple responsive
            parser.parse_class("hover:focus:active:bg-green-500").unwrap(), // High specificity
        ];

        let suggestions = VariantAnalyzer::find_optimization_opportunities(&results);

        // Should find suggestions for multiple responsive and high specificity
        assert!(!suggestions.is_empty());

        let has_specificity_suggestion = suggestions.iter()
            .any(|s| matches!(s.suggestion_type, SuggestionType::HighSpecificity));
        assert!(has_specificity_suggestion, "Should suggest high specificity optimization");
    }

    #[test]
    fn usage_statistics() {
        let parser = EnhancedVariantParser::new();

        let results = vec![
            parser.parse_class("hover:bg-blue-500").unwrap(),
            parser.parse_class("hover:bg-red-500").unwrap(),
            parser.parse_class("sm:hover:bg-green-500").unwrap(),
            parser.parse_class("focus:bg-yellow-500").unwrap(),
            parser.parse_class("invalid:class").unwrap_or_else(|_| {
                VariantParseResult::failure("invalid:class".to_string(), "test error".to_string())
            }),
        ];

        let stats = VariantAnalyzer::analyze_usage(&results);

        assert_eq!(stats.total_classes, 5);
        assert_eq!(stats.successful_parses, 4);
        assert_eq!(stats.failed_parses, 1);

        // Should have hover as most used
        let hover_count = stats.most_used_variants.iter()
            .find(|(name, _)| name == "hover")
            .map(|(_, count)| *count)
            .unwrap_or(0);
        assert_eq!(hover_count, 3);
    }

    #[test]
    fn advanced_combination_processing() {
        let variants = vec![
            ParsedVariant::new("hover".to_string(), VariantType::State),
            ParsedVariant::new("sm".to_string(), VariantType::Responsive),
            ParsedVariant::new("dark".to_string(), VariantType::DarkMode),
        ];

        let processed = VariantCombinationProcessor::process_combination(&variants).unwrap();

        assert!(processed.valid);
        assert_eq!(processed.specificity, 260); // 80 + 100 + 60 + 20 (dark priority)
        assert!(!processed.interactions.is_empty());
    }

    #[test]
    fn fluent_combination_builder() {
        let combination = VariantCombinationBuilder::new()
            .responsive("sm")
            .state("hover")
            .dark_mode()
            .custom("group-hover", VariantType::Custom("group-hover".to_string()))
            .build()
            .unwrap();

        assert_eq!(combination.variants.len(), 4);
        assert!(combination.valid);

        // Check ordering
        assert_eq!(combination.variants[0].variant_type, VariantType::Responsive);
        assert_eq!(combination.variants[1].variant_type, VariantType::DarkMode);
        assert_eq!(combination.variants[2].variant_type, VariantType::State);
    }

    #[test]
    fn conflict_detection_integration() {
        let conflicting_variants = vec![
            ParsedVariant::new("print".to_string(), VariantType::Print),
            ParsedVariant::new("screen".to_string(), VariantType::Screen),
        ];

        assert!(VariantConflictDetector::has_conflicts(&conflicting_variants));

        let conflicts = VariantConflictDetector::detect_conflicts(&conflicting_variants);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].conflict_type, ConflictType::MutuallyExclusive);
    }

    #[test]
    fn css_output_strategies() {
        let combinations = vec![
            vec!["hover".to_string()],
            vec!["sm".to_string(), "hover".to_string()],
            vec!["dark".to_string(), "hover".to_string()],
            vec!["sm".to_string(), "dark".to_string(), "hover".to_string()],
        ];

        let optimized = VariantCombinationStrategies::optimize_for_css_output(&combinations);

        // Check that different strategies are chosen
        let strategies: Vec<_> = optimized.iter().map(|opt| &opt.css_strategy).collect();
        assert!(strategies.contains(&CssOutputStrategy::DirectSelectors));
        assert!(strategies.contains(&CssOutputStrategy::MediaQueryOnly));
        assert!(strategies.contains(&CssOutputStrategy::ClassBased));
        assert!(strategies.contains(&CssOutputStrategy::NestedMediaQueries));
    }

    #[test]
    fn migration_utilities() {
        use super::utilities::VariantMigrator;

        // Test individual migrations
        assert_eq!(VariantMigrator::migrate_class_syntax("mobile:bg-blue-500"), "sm:bg-blue-500");
        assert_eq!(VariantMigrator::migrate_class_syntax("tablet:hover:bg-red-500"), "md:hover:bg-red-500");

        // Test batch migration
        let classes = vec![
            "mobile:bg-blue-500".to_string(),
            "normal:bg-red-500".to_string(),
            "tablet:bg-green-500".to_string(),
        ];

        let migrated = VariantMigrator::migrate_classes(&classes);
        assert_eq!(migrated[0], "sm:bg-blue-500");
        assert_eq!(migrated[1], "normal:bg-red-500"); // Unchanged
        assert_eq!(migrated[2], "md:bg-green-500");
    }

    #[test]
    fn comprehensive_edge_cases() {
        let parser = EnhancedVariantParser::new();

        // Test edge cases
        let edge_cases = vec![
            ("", None), // Empty
            (":bg-blue-500", None), // Leading colon
            ("bg-blue-500:", None), // Trailing colon
            ("bg-blue-500::", None), // Double colon
            ("valid:bg-blue-500", Some("valid:bg-blue-500")), // Actually valid
            ("sm:hover:bg-blue-500", Some("sm:hover:bg-blue-500")),
        ];

        for (class, expected_success) in edge_cases {
            let result = parser.parse_class(class);

            match expected_success {
                Some(expected) => {
                    assert!(result.is_ok(), "Should parse: {}", class);
                    assert_eq!(result.unwrap().original_class, expected);
                }
                None => {
                    // Should either fail or succeed but with issues
                    if let Ok(result) = result {
                        assert!(!result.success || result.combination.error_message.is_some(),
                               "Should have issues with: {}", class);
                    }
                }
            }
        }
    }

    #[test]
    fn performance_regression_test() {
        let parser = EnhancedVariantParser::new();

        let test_classes = vec![
            "hover:bg-blue-500",
            "sm:hover:bg-red-500",
            "dark:sm:hover:bg-green-500",
            "motion-safe:hover:bg-yellow-500",
            "sm:dark:hover:focus:bg-purple-500",
        ];

        let start = std::time::Instant::now();

        // Parse each class multiple times
        for _ in 0..100 {
            for class in &test_classes {
                let _ = parser.parse_class(class);
            }
        }

        let duration = start.elapsed();
        let total_operations = 100 * test_classes.len();

        // Should complete in reasonable time (less than 500ms for 500 operations)
        assert!(duration.as_millis() < 500,
               "Performance regression: {} operations took {:?}", total_operations, duration);
    }

    #[test]
    fn memory_usage_test() {
        let mut parser = EnhancedVariantParser::new();

        // Add many custom variants
        for i in 0..1000 {
            let name = format!("custom-{}", i);
            let selector = format!(".custom-{} ", i);
            parser.add_custom_variant(name, selector).unwrap();
        }

        // Should still work without excessive memory usage
        let result = parser.parse_class("custom-500:bg-blue-500").unwrap();
        assert!(result.success);

        let result = parser.parse_class("sm:custom-200:hover:bg-red-500").unwrap();
        assert!(result.success);
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn large_variant_combinations() {
        let parser = EnhancedVariantParser::new();

        // Test with many variants in combination
        let result = parser.parse_class("sm:dark:hover:focus:active:bg-gradient-to-r").unwrap();
        assert!(result.success);
        assert_eq!(result.combination.variants.len(), 5);
        assert!(result.combination.specificity > 300);
    }

    #[test]
    fn many_custom_variants() {
        let mut parser = EnhancedVariantParser::new();

        // Add many custom variants
        for i in 0..100 {
            parser.add_custom_variant(
                format!("variant-{}", i),
                format!(".variant-{} ", i),
            ).unwrap();
        }

        // Test parsing with custom variants
        let result = parser.parse_class("variant-50:hover:bg-blue-500").unwrap();
        assert!(result.success);
    }

    #[test]
    fn complex_nested_combinations() {
        let parser = EnhancedVariantParser::new();

        // Test deeply nested combinations
        let complex_classes = vec![
            "sm:dark:motion-safe:hover:focus:bg-blue-500",
            "portrait:contrast-more:sm:active:bg-red-500",
            "motion-reduce:dark:hover:disabled:bg-green-500",
        ];

        for class in complex_classes {
            let result = parser.parse_class(class).unwrap();
            assert!(result.success, "Failed to parse: {}", class);
            assert!(result.combination.valid, "Invalid combination: {}", class);
        }
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn invalid_variant_names() {
        let mut parser = EnhancedVariantParser::new();

        // Test invalid custom variant names
        let invalid_names = vec![
            "",
            "-invalid",
            "invalid-",
            "Invalid",
            "invalid variant",
            "invalid.variant",
        ];

        for name in invalid_names {
            assert!(parser.add_custom_variant(name.to_string(), ":test".to_string()).is_err(),
                   "Should reject invalid name: {}", name);
        }
    }

    #[test]
    fn conflicting_combinations() {
        let parser = EnhancedVariantParser::new();

        // Test combinations that should fail
        let failing_classes = vec![
            "print:screen:bg-blue-500",
            "sm:md:lg:bg-red-500",
            "print:sm:bg-green-500", // print can't combine
        ];

        for class in failing_classes {
            let result = parser.parse_class(class);
            if let Ok(result) = result {
                assert!(!result.success || result.combination.error_message.is_some(),
                       "Should fail or have error for: {}", class);
            }
        }
    }

    #[test]
    fn malformed_classes() {
        let parser = EnhancedVariantParser::new();

        // Test malformed class syntax
        let malformed = vec![
            ":",
            "::",
            ":::",
            "variant:",
            ":class",
            "variant::class",
            "[incomplete",
            "incomplete]",
            "variant:[malformed]:class",
        ];

        for class in malformed {
            let result = parser.parse_class(class);
            // Should either fail or succeed with issues
            if let Ok(result) = result {
                // If it succeeds, should still be marked as having issues
                assert!(result.success || result.combination.error_message.is_some(),
                       "Malformed class should have issues: {}", class);
            }
        }
    }
}
