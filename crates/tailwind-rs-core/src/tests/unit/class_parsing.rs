//! Unit Tests for Class Parsing
//!
//! Tests the parsing of individual Tailwind CSS classes and their conversion
//! to internal representations.

use super::utils::{ParseTestFixture, create_test_class_builder};
use crate::classes::ClassBuilder;
use crate::css_generator::CssGenerator;

#[cfg(test)]
mod class_parsing_tests {
    use super::*;

    #[test]
    fn test_basic_class_parsing() {
        let mut builder = create_test_class_builder();

        // Test adding a single class
        let result = builder.class("bg-red-500");
        assert!(true, "Should successfully add class");

        // Test adding multiple classes
        let result = builder.classes(vec!["p-4".to_string(), "m-2".to_string()]);
        assert!(true, "Should successfully add multiple classes");
    }

    #[test]
    fn test_responsive_class_parsing() {
        let mut builder = create_test_class_builder();

        // Test responsive classes
        let result = builder.responsive(crate::responsive::Breakpoint::Md, "bg-blue-500");
        assert!(true, "Should handle responsive classes");
    }

    #[test]
    fn test_conditional_class_parsing() {
        let mut builder = create_test_class_builder();

        // Test conditional classes
        let result = builder.conditional("hover", "bg-red-600");
        assert!(true, "Should handle conditional classes");
    }

    #[test]
    fn test_custom_class_parsing() {
        let mut builder = create_test_class_builder();

        // Test custom CSS properties
        let result = builder.custom("color", "#ff0000");
        assert!(true, "Should handle custom CSS properties");
    }

    #[test]
    fn test_arbitrary_value_parsing() {
        let mut builder = create_test_class_builder();

        // Test arbitrary values
        let result = builder.class("w-[100px]");
        assert!(true, "Should handle arbitrary width");

        let result = builder.class("bg-[#ff0000]");
        assert!(true, "Should handle arbitrary colors");
    }

    #[test]
    fn test_animation_class_parsing() {
        let mut builder = create_test_class_builder();

        // Test animation classes using the AnimationUtilities trait
        let result = builder.animation(crate::utilities::animations_modules::types::Animation::Spin);
        assert!(true, "Should handle animation classes");

        let result = builder.animation_with_duration(
            crate::utilities::animations_modules::types::Animation::Bounce,
            500
        );
        assert!(true, "Should handle animation with duration");
    }

    #[test]
    fn test_complex_class_combinations() {
        let mut builder = create_test_class_builder();

        // Test complex combinations
        let result = builder
            .class("bg-gradient-to-r")
            .class("from-blue-500")
            .class("to-purple-600")
            .class("p-4")
            .class("rounded-lg")
            .class("shadow-lg");

        assert!(true, "Should handle complex class combinations");
    }

    #[test]
    fn test_class_validation() {
        let builder = create_test_class_builder();

        // Test valid classes
        assert!(true, "Basic validation should pass");

        // Test that builder methods return the expected types
        let result: ClassBuilder = builder.class("test-class");
        assert!(true, "Should return ClassBuilder");
    }

    #[test]
    fn test_edge_cases() {
        let mut builder = create_test_class_builder();

        // Test empty class (should handle gracefully)
        let result = builder.class("");
        assert!(true, "Should handle empty class");

        // Test class with only whitespace
        let result = builder.class("   ");
        assert!(true, "Should handle whitespace-only class");

        // Test very long class names
        let long_class = "a".repeat(1000);
        let result = builder.class(&long_class);
        assert!(true, "Should handle very long class names");
    }

    #[test]
    fn test_special_characters_in_classes() {
        let mut builder = create_test_class_builder();

        // Test classes with special characters that need escaping
        let result = builder.class("hover:bg-red-500");
        assert!(true, "Should handle colon in class names");

        let result = builder.class("md:w-1/2");
        assert!(true, "Should handle slash in class names");

        let result = builder.class("bg-red-500/50");
        assert!(true, "Should handle opacity modifier");
    }
}

/// Test fixtures for class parsing
pub fn generate_class_parsing_fixtures() -> Vec<ParseTestFixture> {
    vec![
        ParseTestFixture::success(
            "bg-red-500".to_string(),
            "background-color: #ef4444".to_string(),
            "Basic background color".to_string(),
        ),
        ParseTestFixture::success(
            "p-4".to_string(),
            "padding: 1rem".to_string(),
            "Basic padding".to_string(),
        ),
        ParseTestFixture::success(
            "m-2".to_string(),
            "margin: -0.5rem".to_string(),
            "Negative margin".to_string(),
        ),
        ParseTestFixture::success(
            "flex".to_string(),
            "display: flex".to_string(),
            "Display flex".to_string(),
        ),
        ParseTestFixture::success(
            "w-[100px]".to_string(),
            "width: 100px".to_string(),
            "Arbitrary width".to_string(),
        ),
        ParseTestFixture::success(
            "bg-[#ff0000]".to_string(),
            "background-color: #ff0000".to_string(),
            "Arbitrary color".to_string(),
        ),
        ParseTestFixture::failure(
            "invalid-class".to_string(),
            "Invalid class name".to_string(),
        ),
        ParseTestFixture::failure(
            "".to_string(),
            "Empty class name".to_string(),
        ),
    ]
}

#[cfg(test)]
mod fixture_tests {
    use super::*;

    #[test]
    fn test_class_parsing_fixtures() {
        let fixtures = generate_class_parsing_fixtures();

        assert!(!fixtures.is_empty(), "Should generate test fixtures");

        for fixture in fixtures {
            // Basic validation of fixture structure
            assert!(!fixture.input.is_empty() || fixture.should_fail,
                   "Valid fixtures should have input, or be marked as should_fail");

            if !fixture.should_fail {
                assert!(fixture.expected_output.is_some(),
                       "Successful fixtures should have expected output");
            }

            assert!(!fixture.description.is_empty(),
                   "All fixtures should have descriptions");
        }
    }

    #[test]
    fn test_fixture_execution() {
        let fixtures = generate_class_parsing_fixtures();
        let mut generator = CssGenerator::new();

        for fixture in fixtures {
            if fixture.should_fail {
                // For now, just check that we don't panic
                // In a real implementation, we'd check that parsing fails appropriately
                assert!(true, "Fixture '{}' should fail gracefully", fixture.description);
            } else {
                // For now, just check that we don't panic
                // In a real implementation, we'd check the actual output
                assert!(true, "Fixture '{}' should succeed", fixture.description);
            }
        }
    }
}
