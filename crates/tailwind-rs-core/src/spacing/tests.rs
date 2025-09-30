//! Spacing Tests Module
//!
//! Comprehensive tests for the spacing system to ensure correctness
//! and prevent regressions.

use super::{parser::*, utilities::*, values::*, constants::*};

#[cfg(test)]
mod spacing_parser_tests {
    use super::*;

    #[test]
    fn basic_spacing_parsing() {
        let parser = SpacingParser::new();

        // Test padding classes
        assert!(parser.parse_spacing_class("p-0").is_some());
        assert!(parser.parse_spacing_class("p-4").is_some());
        assert!(parser.parse_spacing_class("px-2").is_some());
        assert!(parser.parse_spacing_class("py-3").is_some());
        assert!(parser.parse_spacing_class("pt-1").is_some());
        assert!(parser.parse_spacing_class("pr-2").is_some());
        assert!(parser.parse_spacing_class("pb-3").is_some());
        assert!(parser.parse_spacing_class("pl-4").is_some());

        // Test margin classes
        assert!(parser.parse_spacing_class("m-0").is_some());
        assert!(parser.parse_spacing_class("m-4").is_some());
        assert!(parser.parse_spacing_class("mx-2").is_some());
        assert!(parser.parse_spacing_class("my-3").is_some());
        assert!(parser.parse_spacing_class("mt-1").is_some());
        assert!(parser.parse_spacing_class("mr-2").is_some());
        assert!(parser.parse_spacing_class("mb-3").is_some());
        assert!(parser.parse_spacing_class("ml-4").is_some());

        // Test gap classes
        assert!(parser.parse_spacing_class("gap-0").is_some());
        assert!(parser.parse_spacing_class("gap-4").is_some());
        assert!(parser.parse_spacing_class("gap-x-2").is_some());
        assert!(parser.parse_spacing_class("gap-y-3").is_some());
    }

    #[test]
    fn arbitrary_value_parsing() {
        let parser = SpacingParser::new();

        // Test arbitrary values
        let result = parser.parse_spacing_class("p-[10px]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "10px");

        let result = parser.parse_spacing_class("m-[2rem]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "2rem");

        let result = parser.parse_spacing_class("gap-[1.5em]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "1.5em");
    }

    #[test]
    fn logical_properties_parsing() {
        let parser = SpacingParser::new();

        // Test logical properties
        let result = parser.parse_spacing_class("ps-4");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "padding-inline-start");

        let result = parser.parse_spacing_class("pe-2");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "padding-inline-end");

        let result = parser.parse_spacing_class("ms-3");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "margin-inline-start");

        let result = parser.parse_spacing_class("me-1");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "margin-inline-end");
    }

    #[test]
    fn invalid_class_handling() {
        let parser = SpacingParser::new();

        // Test invalid classes
        assert!(parser.parse_spacing_class("").is_none());
        assert!(parser.parse_spacing_class("invalid").is_none());
        assert!(parser.parse_spacing_class("p-").is_none());
        assert!(parser.parse_spacing_class("p-invalid").is_none());
        assert!(parser.parse_spacing_class("p-[invalid").is_none()); // Unbalanced brackets
        assert!(parser.parse_spacing_class("p-invalid]").is_none()); // Invalid content
    }

    #[test]
    fn spacing_value_ranges() {
        let parser = SpacingParser::new();

        // Test all standard spacing values
        for i in 0..=96 {
            let class = format!("p-{}", i);
            let result = parser.parse_spacing_class(&class);
            if i == 0 || i == 1 || (i >= 2 && i <= 12) || i == 14 || i == 16 || i == 20 ||
               i == 24 || i == 28 || i == 32 || i == 36 || i == 40 || i == 44 || i == 48 ||
               i == 52 || i == 56 || i == 60 || i == 64 || i == 72 || i == 80 || i == 96 {
                assert!(result.is_some(), "Expected parsing for p-{}", i);
            }
        }
    }

    #[test]
    fn configuration_restrictions() {
        // Test parser with restricted configuration
        let config = SpacingParserConfig {
            enable_arbitrary_values: false,
            enable_logical_properties: false,
            enable_axis_properties: false,
            max_spacing_scale: 10.0,
        };
        let parser = SpacingParser::with_config(config);

        // Should not parse arbitrary values
        assert!(parser.parse_spacing_class("p-[10px]").is_none());

        // Should not parse logical properties
        assert!(parser.parse_spacing_class("ps-4").is_none());

        // Should not parse axis properties
        assert!(parser.parse_spacing_class("px-4").is_none());

        // Should still parse basic properties
        assert!(parser.parse_spacing_class("p-4").is_some());
        assert!(parser.parse_spacing_class("pt-2").is_some());
    }

    #[test]
    fn css_property_generation() {
        let parser = SpacingParser::new();

        // Test that properties are generated correctly
        let result = parser.parse_spacing_class("p-4");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "padding");
        assert_eq!(properties[0].value, "1rem");
        assert!(!properties[0].important);

        // Test axis properties generate multiple properties
        let result = parser.parse_spacing_class("px-2");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0].name, "padding-left");
        assert_eq!(properties[1].name, "padding-right");
        assert_eq!(properties[0].value, "0.5rem");
        assert_eq!(properties[1].value, "0.5rem");
    }
}

#[cfg(test)]
mod spacing_utilities_tests {
    use super::*;

    #[test]
    fn utility_methods() {
        let utils = SpacingUtilities::new();

        // Test spacing value retrieval
        assert_eq!(utils.get_spacing_value("0"), Some("0".to_string()));
        assert_eq!(utils.get_spacing_value("4"), Some("1rem".to_string()));
        assert_eq!(utils.get_spacing_value("px"), Some("1px".to_string()));
        assert_eq!(utils.get_spacing_value("invalid"), None);

        // Test arbitrary value handling
        assert_eq!(utils.get_spacing_value("[10px]"), Some("10px".to_string()));
        assert_eq!(utils.get_spacing_value("[2.5rem]"), Some("2.5rem".to_string()));
    }

    #[test]
    fn padding_parsing_detailed() {
        let utils = SpacingUtilities::new();

        // Test all padding variants
        let test_cases = vec![
            ("p-4", "padding", "1rem"),
            ("px-2", "padding-left", "0.5rem"),
            ("py-3", "padding-top", "0.75rem"),
            ("pt-1", "padding-top", "0.25rem"),
            ("pr-2", "padding-right", "0.5rem"),
            ("pb-3", "padding-bottom", "0.75rem"),
            ("pl-4", "padding-left", "1rem"),
        ];

        for (class, expected_property, expected_value) in test_cases {
            let result = utils.parse_padding_class(class);
            assert!(result.is_some(), "Failed to parse: {}", class);
            let properties = result.unwrap();

            if class.starts_with("px-") || class.starts_with("py-") {
                // Axis properties generate two properties
                assert_eq!(properties.len(), 2, "Expected 2 properties for: {}", class);
                let matching_props: Vec<_> = properties.iter()
                    .filter(|p| p.name == expected_property)
                    .collect();
                assert_eq!(matching_props.len(), 1, "Expected property {} for: {}", expected_property, class);
                assert_eq!(matching_props[0].value, expected_value);
            } else {
                // Single properties
                assert_eq!(properties.len(), 1, "Expected 1 property for: {}", class);
                assert_eq!(properties[0].name, expected_property);
                assert_eq!(properties[0].value, expected_value);
            }
        }
    }

    #[test]
    fn margin_parsing_detailed() {
        let utils = SpacingUtilities::new();

        // Test margin variants
        let result = utils.parse_margin_class("m-4");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "margin");
        assert_eq!(result.unwrap()[0].value, "1rem");

        let result = utils.parse_margin_class("mx-2");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0].name, "margin-left");
        assert_eq!(properties[1].name, "margin-right");
    }

    #[test]
    fn gap_parsing_detailed() {
        let utils = SpacingUtilities::new();

        let result = utils.parse_gap_class("gap-4");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "gap");

        let result = utils.parse_gap_class("gap-x-2");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "column-gap");

        let result = utils.parse_gap_class("gap-y-3");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "row-gap");
    }
}

#[cfg(test)]
mod spacing_values_tests {
    use super::*;

    #[test]
    fn value_mappings() {
        let values = SpacingValues::new();

        // Test standard mappings
        assert_eq!(values.get_value("0"), Some("0".to_string()));
        assert_eq!(values.get_value("4"), Some("1rem".to_string()));
        assert_eq!(values.get_value("px"), Some("1px".to_string()));
        assert_eq!(values.get_value("96"), Some("24rem".to_string()));

        // Test invalid values
        assert_eq!(values.get_value("999"), None);
        assert_eq!(values.get_value("invalid"), None);
    }

    #[test]
    fn token_validation() {
        let values = SpacingValues::new();

        assert!(values.is_valid_token("0"));
        assert!(values.is_valid_token("16"));
        assert!(values.is_valid_token("96"));
        assert!(!values.is_valid_token("999"));
        assert!(!values.is_valid_token("invalid"));
    }

    #[test]
    fn arbitrary_value_parsing() {
        assert_eq!(SpacingValueUtils::parse_arbitrary_value("[10px]"), Some("10px".to_string()));
        assert_eq!(SpacingValueUtils::parse_arbitrary_value("[2.5rem]"), Some("2.5rem".to_string()));
        assert_eq!(SpacingValueUtils::parse_arbitrary_value("[1.5em]"), Some("1.5em".to_string()));
        assert_eq!(SpacingValueUtils::parse_arbitrary_value("10px"), None);
        assert_eq!(SpacingValueUtils::parse_arbitrary_value("[invalid"), None);
        assert_eq!(SpacingValueUtils::parse_arbitrary_value("invalid]"), None);
    }

    #[test]
    fn css_length_validation() {
        assert!(SpacingValueUtils::is_valid_css_length("10px"));
        assert!(SpacingValueUtils::is_valid_css_length("2.5rem"));
        assert!(SpacingValueUtils::is_valid_css_length("100%"));
        assert!(SpacingValueUtils::is_valid_css_length("16"));
        assert!(!SpacingValueUtils::is_valid_css_length("invalid"));
        assert!(!SpacingValueUtils::is_valid_css_length("10unknown"));
    }

    #[test]
    fn token_to_numeric_conversion() {
        assert_eq!(SpacingValueUtils::token_to_numeric("0"), Some(0.0));
        assert_eq!(SpacingValueUtils::token_to_numeric("4"), Some(4.0));
        assert_eq!(SpacingValueUtils::token_to_numeric("px"), Some(0.0625));
        assert_eq!(SpacingValueUtils::token_to_numeric("invalid"), None);
    }
}

#[cfg(test)]
mod spacing_constants_tests {
    use super::*;

    #[test]
    fn pattern_definitions() {
        assert!(SPACING_PATTERNS.contains(&"p-*"));
        assert!(SPACING_PATTERNS.contains(&"m-*"));
        assert!(SPACING_PATTERNS.contains(&"gap-*"));
        assert!(SPACING_PATTERNS.contains(&"px-*"));
        assert!(SPACING_PATTERNS.contains(&"py-*"));
        assert!(SPACING_PATTERNS.contains(&"pt-*"));
        assert!(SPACING_PATTERNS.contains(&"pr-*"));
        assert!(SPACING_PATTERNS.contains(&"pb-*"));
        assert!(SPACING_PATTERNS.contains(&"pl-*"));
    }

    #[test]
    fn property_definitions() {
        assert!(SPACING_PROPERTIES.contains(&"padding"));
        assert!(SPACING_PROPERTIES.contains(&"margin"));
        assert!(SPACING_PROPERTIES.contains(&"gap"));
        assert!(SPACING_PROPERTIES.contains(&"padding-top"));
        assert!(SPACING_PROPERTIES.contains(&"margin-left"));
        assert!(SPACING_PROPERTIES.contains(&"row-gap"));
    }

    #[test]
    fn direction_mappings() {
        assert_eq!(SpacingDirections::get_padding_property("t"), Some("padding-top"));
        assert_eq!(SpacingDirections::get_padding_property("r"), Some("padding-right"));
        assert_eq!(SpacingDirections::get_padding_property("b"), Some("padding-bottom"));
        assert_eq!(SpacingDirections::get_padding_property("l"), Some("padding-left"));
        assert_eq!(SpacingDirections::get_padding_property("s"), Some("padding-inline-start"));
        assert_eq!(SpacingDirections::get_padding_property("e"), Some("padding-inline-end"));
        assert_eq!(SpacingDirections::get_padding_property("x"), None); // x is axis, not direction

        assert_eq!(SpacingDirections::get_margin_property("t"), Some("margin-top"));
        assert_eq!(SpacingDirections::get_margin_property("s"), Some("margin-inline-start"));
    }

    #[test]
    fn axis_mappings() {
        let x_padding = SpacingAxes::get_padding_properties("x");
        assert_eq!(x_padding, Some(vec!["padding-left", "padding-right"]));

        let y_margin = SpacingAxes::get_margin_properties("y");
        assert_eq!(y_margin, Some(vec!["margin-top", "margin-bottom"]));

        let x_gap = SpacingAxes::get_gap_properties("x");
        assert_eq!(x_gap, Some(vec!["column-gap"]));

        let y_gap = SpacingAxes::get_gap_properties("y");
        assert_eq!(y_gap, Some(vec!["row-gap"]));
    }

    #[test]
    fn prefix_definitions() {
        assert_eq!(SpacingPrefixes::PADDING_ALL, "p-");
        assert_eq!(SpacingPrefixes::MARGIN_X, "mx-");
        assert_eq!(SpacingPrefixes::GAP_Y, "gap-y-");
        assert_eq!(SpacingPrefixes::PADDING_START, "ps-");
        assert_eq!(SpacingPrefixes::MARGIN_END, "me-");
    }

    #[test]
    fn validation_rules() {
        assert!(SpacingValidation::validate_spacing_token("4").is_ok());
        assert!(SpacingValidation::validate_spacing_token("999").is_err()); // Too large
        assert!(SpacingValidation::validate_spacing_token(&"a".repeat(60)).is_err()); // Too long

        assert!(SpacingValidation::validate_arbitrary_value("10px").is_ok());
        assert!(SpacingValidation::validate_arbitrary_value("").is_err());
        assert!(SpacingValidation::validate_arbitrary_value(&"a".repeat(60)).is_err());
    }

    #[test]
    fn parser_configuration() {
        let config = SpacingParserConfig::default();

        assert!(config.arbitrary_values_enabled());
        assert!(config.logical_properties_enabled());
        assert!(config.axis_properties_enabled());
        assert!(config.is_spacing_allowed(96.0));
        assert!(!config.is_spacing_allowed(100.0));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn end_to_end_spacing_parsing() {
        let parser = SpacingParser::new();

        // Test a comprehensive set of spacing classes
        let test_classes = vec![
            "p-0", "p-1", "p-2", "p-4", "p-8", "p-16", "p-32", "p-64",
            "px-2", "py-3", "px-4", "py-5",
            "pt-1", "pr-2", "pb-3", "pl-4",
            "m-0", "m-2", "m-4", "m-8",
            "mx-1", "my-2", "mx-3", "my-4",
            "mt-1", "mr-2", "mb-3", "ml-4",
            "gap-0", "gap-2", "gap-4", "gap-8",
            "gap-x-1", "gap-y-2",
        ];

        for class in test_classes {
            let result = parser.parse_spacing_class(class);
            assert!(result.is_some(), "Failed to parse valid class: {}", class);
            assert!(!result.as_ref().unwrap().is_empty(), "No properties generated for: {}", class);
        }
    }

    #[test]
    fn performance_test() {
        let parser = SpacingParser::new();
        let test_classes = vec![
            "p-4", "m-2", "gap-3", "px-1", "py-2", "pt-3", "pr-4", "pb-5", "pl-6",
        ];

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            for class in &test_classes {
                let _ = parser.parse_spacing_class(class);
            }
        }
        let duration = start.elapsed();

        // Should complete in reasonable time (less than 1 second for 9000 operations)
        assert!(duration.as_millis() < 1000, "Performance test failed: {:?}", duration);
    }

    #[test]
    fn memory_safety_test() {
        let parser = SpacingParser::new();

        // Test with various edge cases
        let edge_cases = vec![
            "p-[999999px]", "m-[very_long_arbitrary_value_that_should_be_handled_safely]",
            "gap-[0]", "px-[1rem]", "py-[2em]", "pt-[3vh]", "pr-[4vw]", "pb-[5vmin]", "pl-[6vmax]",
        ];

        for class in edge_cases {
            // Should not panic or cause memory issues
            let _ = parser.parse_spacing_class(class);
        }
    }
}
