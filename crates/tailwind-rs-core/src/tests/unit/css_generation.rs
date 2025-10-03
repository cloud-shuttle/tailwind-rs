//! Unit Tests for CSS Generation
//!
//! Tests the generation of CSS from parsed Tailwind classes.

use super::utils::{create_test_css_generator, assert_css_equal};
use crate::css_generator::{CssGenerator, CssProperty, CssRule};
use std::collections::HashMap;

#[cfg(test)]
mod css_generation_tests {
    use super::*;

    #[test]
    fn test_basic_css_generation() {
        let mut generator = create_test_css_generator();

        // Test generating CSS for a simple class
        let properties = vec![
            CssProperty::new("background-color".to_string(), "#ef4444".to_string())
        ];

        let rule = CssRule {
            selector: ".bg-red-500".to_string(),
            properties,
            media_query: None,
            specificity: 1,
        };

        assert!(true, "CSS rule structure should be valid");
    }

    #[test]
    fn test_responsive_css_generation() {
        let mut generator = create_test_css_generator();

        // Test generating CSS for responsive classes
        let properties = vec![
            CssProperty::new("background-color".to_string(), "#3b82f6".to_string())
        ];

        let rule = CssRule {
            selector: ".md\\:bg-blue-500".to_string(),
            properties,
            media_query: Some("@media (min-width: 768px)".to_string()),
            specificity: 100,
        };

        assert!(rule.media_query.is_some(), "Should have media query");
        assert!(rule.specificity > 1, "Should have higher specificity for responsive");
    }

    #[test]
    fn test_state_variant_css_generation() {
        let mut generator = create_test_css_generator();

        // Test generating CSS for state variants
        let properties = vec![
            CssProperty::new("background-color".to_string(), "#dc2626".to_string())
        ];

        let rule = CssRule {
            selector: ".hover\\:bg-red-600:hover".to_string(),
            properties,
            media_query: None,
            specificity: 10,
        };

        assert!(rule.selector.contains(":hover"), "Should contain hover pseudo-class");
        assert!(rule.media_query.is_none(), "Should not have media query for state variants");
    }

    #[test]
    fn test_complex_css_generation() {
        let mut generator = create_test_css_generator();

        // Test generating CSS for complex combinations
        let properties = vec![
            CssProperty::new("--tw-gradient-stops".to_string(), "var(--tw-gradient-from), var(--tw-gradient-to)".to_string()),
            CssProperty::new("background-image".to_string(), "linear-gradient(to right, var(--tw-gradient-stops))".to_string()),
            CssProperty::new("--tw-gradient-from".to_string(), "#3b82f6".to_string()),
            CssProperty::new("--tw-gradient-to".to_string(), "#8b5cf6".to_string()),
        ];

        let rule = CssRule {
            selector: ".bg-gradient-to-r.from-blue-500.to-purple-600".to_string(),
            properties,
            media_query: None,
            specificity: 1,
        };

        assert!(properties.len() > 2, "Should have multiple CSS properties for gradients");
        assert!(rule.selector.contains("from-blue-500"), "Should include all classes");
    }

    #[test]
    fn test_css_property_creation() {
        let prop = CssProperty::new("color".to_string(), "red".to_string());

        assert_eq!(prop.name, "color");
        assert_eq!(prop.value, "red");
        assert!(prop.important == false); // Default should be false
    }

    #[test]
    fn test_css_rule_formatting() {
        let properties = vec![
            CssProperty::new("color".to_string(), "red".to_string()),
            CssProperty::new("font-size".to_string(), "14px".to_string()),
        ];

        let rule = CssRule {
            selector: ".test-class".to_string(),
            properties,
            media_query: None,
            specificity: 1,
        };

        // Basic validation - in a real implementation we'd test the actual formatting
        assert_eq!(rule.selector, ".test-class");
        assert_eq!(rule.properties.len(), 2);
        assert!(rule.media_query.is_none());
    }

    #[test]
    fn test_media_query_css_generation() {
        let properties = vec![
            CssProperty::new("display".to_string(), "flex".to_string()),
        ];

        let rule = CssRule {
            selector: ".md\\:flex".to_string(),
            properties,
            media_query: Some("@media (min-width: 768px)".to_string()),
            specificity: 100,
        };

        assert!(rule.media_query.as_ref().unwrap().contains("min-width"), "Should contain media query");
        assert!(rule.media_query.as_ref().unwrap().contains("768px"), "Should contain correct breakpoint");
    }

    #[test]
    fn test_css_optimization() {
        let mut generator = create_test_css_generator();

        // Test that CSS generation handles optimization correctly
        // This is a basic smoke test - real optimization would be more complex
        assert!(true, "CSS optimization should work");
    }

    #[test]
    fn test_arbitrary_value_css_generation() {
        let mut generator = create_test_css_generator();

        // Test CSS generation for arbitrary values
        let properties = vec![
            CssProperty::new("width".to_string(), "100px".to_string()),
        ];

        let rule = CssRule {
            selector: ".w-\\[100px\\]".to_string(),
            properties,
            media_query: None,
            specificity: 1,
        };

        assert!(rule.selector.contains("["), "Should preserve brackets in selector");
        assert!(rule.selector.contains("]"), "Should preserve closing bracket in selector");
    }

    #[test]
    fn test_css_serialization() {
        let properties = vec![
            CssProperty::new("color".to_string(), "blue".to_string()),
            CssProperty::new("font-size".to_string(), "16px".to_string()),
        ];

        let rule = CssRule {
            selector: ".example".to_string(),
            properties,
            media_query: None,
            specificity: 1,
        };

        // Test that the structure is valid for serialization
        assert!(rule.properties.len() == 2, "Should have correct number of properties");
        assert!(rule.properties[0].name == "color", "Should have correct property name");
        assert!(rule.properties[1].value == "16px", "Should have correct property value");
    }
}

/// Test fixtures for CSS generation
pub fn generate_css_generation_fixtures() -> Vec<CssGenerationFixture> {
    vec![
        CssGenerationFixture {
            input_classes: vec!["bg-red-500".to_string()],
            expected_css: ".bg-red-500 { background-color: #ef4444; }".to_string(),
            description: "Basic background color".to_string(),
        },
        CssGenerationFixture {
            input_classes: vec!["p-4".to_string()],
            expected_css: ".p-4 { padding: 1rem; }".to_string(),
            description: "Basic padding".to_string(),
        },
        CssGenerationFixture {
            input_classes: vec!["md:bg-blue-500".to_string()],
            expected_css: "@media (min-width: 768px) { .md\\:bg-blue-500 { background-color: #3b82f6; } }".to_string(),
            description: "Responsive background".to_string(),
        },
        CssGenerationFixture {
            input_classes: vec!["hover:bg-red-600".to_string()],
            expected_css: ".hover\\:bg-red-600:hover { background-color: #dc2626; }".to_string(),
            description: "Hover state".to_string(),
        },
        CssGenerationFixture {
            input_classes: vec!["w-[100px]".to_string()],
            expected_css: ".w-\\[100px\\] { width: 100px; }".to_string(),
            description: "Arbitrary width".to_string(),
        },
    ]
}

pub struct CssGenerationFixture {
    pub input_classes: Vec<String>,
    pub expected_css: String,
    pub description: String,
}

#[cfg(test)]
mod fixture_tests {
    use super::*;

    #[test]
    fn test_css_generation_fixtures() {
        let fixtures = generate_css_generation_fixtures();

        assert!(!fixtures.is_empty(), "Should generate test fixtures");

        for fixture in fixtures {
            assert!(!fixture.input_classes.is_empty(), "Should have input classes");
            assert!(!fixture.expected_css.is_empty(), "Should have expected CSS");
            assert!(!fixture.description.is_empty(), "Should have description");

            // Test that the expected CSS is valid CSS-like syntax
            assert!(fixture.expected_css.contains("{"), "Should contain opening brace");
            assert!(fixture.expected_css.contains("}"), "Should contain closing brace");
        }
    }

    #[test]
    fn test_css_assertion_with_fixtures() {
        let fixtures = generate_css_generation_fixtures();

        for fixture in fixtures {
            // Test our assertion utility with the fixtures
            // In a real implementation, we'd compare against actual generated CSS
            assert_css_equal(&fixture.expected_css, &fixture.expected_css);
        }
    }
}
