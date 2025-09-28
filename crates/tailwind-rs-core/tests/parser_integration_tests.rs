//! Parser Integration Tests
//!
//! This module contains integration tests for the various parsers
//! to ensure they work correctly with the CSS generator.

use tailwind_rs_core::css_generator::parsers::{basic_transforms::BasicTransformsParser, scale_parser::ScaleParser, UtilityParser};
use tailwind_rs_core::css_generator::types::CssProperty;

#[cfg(test)]
mod parser_integration_tests {
    use super::*;

    #[test]
    fn test_basic_transforms_translate_x_parser() {
        let parser = BasicTransformsParser::new();

        // Test translate-x-1 (4px)
        let result = parser.parse_class("translate-x-1");
        println!("translate-x-1 result: {:?}", result);
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "translateX(0.25rem)");

        // Test translate-x-px (1px)
        let result = parser.parse_class("translate-x-px");
        println!("translate-x-px result: {:?}", result);
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "translateX(1px)");

        // Test negative translate-x-2
        let result = parser.parse_class("-translate-x-2");
        println!("-translate-x-2 result: {:?}", result);
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "translateX(-0.5rem)");
    }

    #[test]
    fn test_basic_transforms_translate_y_parser() {
        let parser = BasicTransformsParser::new();

        // Test translate-y-1 (4px)
        let result = parser.parse_class("translate-y-1");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "translateY(0.25rem)");

        // Test translate-y-full (100%)
        let result = parser.parse_class("translate-y-full");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "translateY(100%)");

        // Test negative translate-y-4
        let result = parser.parse_class("-translate-y-4");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "translateY(-1rem)");
    }

    #[test]
    fn test_scale_parser() {
        let parser = ScaleParser::new();

        // Test scale-x-50 (0.5)
        let result = parser.parse_class("scale-x-50");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "scaleX(0.5)");

        // Test scale-y-75 (0.75)
        let result = parser.parse_class("scale-y-75");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "scaleY(0.75)");

        // Test scale-x-100 (1.0)
        let result = parser.parse_class("scale-x-100");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "scaleX(1)");

        // Test invalid scale value
        let result = parser.parse_class("scale-x-150");
        assert!(result.is_none());
    }

    #[test]
    fn test_parser_supported_patterns() {
        let basic_transforms = BasicTransformsParser::new();
        let scale_parser = ScaleParser::new();

        // Check supported patterns for basic transforms
        let patterns = basic_transforms.get_supported_patterns();
        assert!(patterns.contains(&"translate-x-1"));
        assert!(patterns.contains(&"translate-y-px"));
        assert!(patterns.contains(&"-translate-x-2"));

        // Check supported patterns for scale parser
        let patterns = scale_parser.get_supported_patterns();
        assert!(patterns.contains(&"scale-x-50"));
        assert!(patterns.contains(&"scale-y-100"));
    }

    #[test]
    fn test_parser_priorities() {
        let basic_transforms = BasicTransformsParser::new();
        let scale_parser = ScaleParser::new();

        // Both should have reasonable priorities
        assert!(basic_transforms.get_priority() > 0);
        assert!(scale_parser.get_priority() > 0);
        assert_eq!(basic_transforms.get_category(), scale_parser.get_category());
    }
}
