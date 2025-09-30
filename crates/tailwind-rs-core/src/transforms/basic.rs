//! Basic Transform Parser Module
//!
//! Handles parsing of basic transform utilities:
//! - transform, transform-gpu, transform-cpu, transform-none

use crate::css_generator::types::CssProperty;

/// Basic transform parser
#[derive(Debug, Clone)]
pub struct BasicTransformParser;

impl BasicTransformParser {
    /// Create a new basic transform parser
    pub fn new() -> Self {
        Self
    }

    /// Parse basic transform classes
    pub fn parse_basic_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "transform" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "translate(0px, 0px) rotate(0deg) skewX(0deg) skewY(0deg) scaleX(1) scaleY(1)".to_string(),
                important: false,
            }]),
            "transform-gpu" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "translate3d(0px, 0px, 0) rotate(0deg) skewX(0deg) skewY(0deg) scaleX(1) scaleY(1)".to_string(),
                important: false,
            }]),
            "transform-cpu" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "var(--tw-rotate-x) var(--tw-rotate-y) var(--tw-rotate-z) var(--tw-skew-x) var(--tw-skew-y)".to_string(),
                important: false,
            }]),
            "transform-none" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Get supported basic transform patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "transform",
            "transform-gpu",
            "transform-cpu",
            "transform-none",
        ]
    }
}

impl Default for BasicTransformParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_transform_parsing() {
        let parser = BasicTransformParser::new();

        // Test transform
        let result = parser.parse_basic_transform_class("transform");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert!(properties[0].value.contains("translate(0px, 0px)"));
        assert!(properties[0].value.contains("scaleX(1)"));
        assert!(!properties[0].important);

        // Test transform-gpu
        let result = parser.parse_basic_transform_class("transform-gpu");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert!(properties[0].value.contains("translate3d"));

        // Test transform-cpu
        let result = parser.parse_basic_transform_class("transform-cpu");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert!(properties[0].value.contains("var(--tw-rotate-x)"));

        // Test transform-none
        let result = parser.parse_basic_transform_class("transform-none");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "none");

        // Test invalid class
        let result = parser.parse_basic_transform_class("transform-invalid");
        assert!(result.is_none());
    }

    #[test]
    fn supported_patterns() {
        let parser = BasicTransformParser::new();
        let patterns = parser.supported_patterns();

        assert_eq!(patterns.len(), 4);
        assert!(patterns.contains(&"transform"));
        assert!(patterns.contains(&"transform-gpu"));
        assert!(patterns.contains(&"transform-cpu"));
        assert!(patterns.contains(&"transform-none"));
    }

    #[test]
    fn comprehensive_basic_transform_test() {
        let parser = BasicTransformParser::new();
        let test_cases = vec![
            ("transform", true),
            ("transform-gpu", true),
            ("transform-cpu", true),
            ("transform-none", true),
            ("transform-scale", false),
            ("transform-rotate", false),
            ("invalid", false),
        ];

        for (class, should_parse) in test_cases {
            let result = parser.parse_basic_transform_class(class);
            if should_parse {
                assert!(result.is_some(), "Expected parsing for: {}", class);
                assert_eq!(result.unwrap()[0].name, "transform");
            } else {
                assert!(result.is_none(), "Expected no parsing for: {}", class);
            }
        }
    }
}
