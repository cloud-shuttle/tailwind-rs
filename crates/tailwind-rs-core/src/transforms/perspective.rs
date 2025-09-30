//! Perspective Parser Module
//!
//! Handles parsing of perspective utilities:
//! - perspective-none, perspective-dramatic, perspective-near, etc.
//! - perspective-250, perspective-500, etc.

use crate::css_generator::types::CssProperty;
use super::utils::TransformConversion;

/// Perspective parser
#[derive(Debug, Clone)]
pub struct PerspectiveParser;

impl PerspectiveParser {
    /// Create a new perspective parser
    pub fn new() -> Self {
        Self
    }

    /// Parse perspective classes
    pub fn parse_perspective_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            // Named perspective values
            "perspective-dramatic" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: TransformConversion::perspective_value_to_css("dramatic"),
                important: false,
            }]),
            "perspective-near" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: TransformConversion::perspective_value_to_css("near"),
                important: false,
            }]),
            "perspective-normal" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: TransformConversion::perspective_value_to_css("normal"),
                important: false,
            }]),
            "perspective-midrange" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: TransformConversion::perspective_value_to_css("midrange"),
                important: false,
            }]),
            "perspective-distant" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: TransformConversion::perspective_value_to_css("distant"),
                important: false,
            }]),
            "perspective-none" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: TransformConversion::perspective_value_to_css("none"),
                important: false,
            }]),
            // Numerical perspective values
            _ => {
                if let Some(value) = class.strip_prefix("perspective-") {
                    if let Ok(num) = value.parse::<u32>() {
                        return Some(vec![CssProperty {
                            name: "perspective".to_string(),
                            value: format!("{}px", num),
                            important: false,
                        }]);
                    }
                }
                None
            },
        }
    }

    /// Get supported perspective patterns
    pub fn supported_patterns(&self) -> Vec<String> {
        let mut patterns = vec![
            "perspective-none".to_string(),
            "perspective-dramatic".to_string(),
            "perspective-near".to_string(),
            "perspective-normal".to_string(),
            "perspective-midrange".to_string(),
            "perspective-distant".to_string(),
        ];

        // Add numerical patterns
        for num in [250, 500, 750, 1000] {
            patterns.push(format!("perspective-{}", num));
        }

        patterns
    }
}

impl Default for PerspectiveParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_perspective_parsing() {
        let parser = PerspectiveParser::new();

        // Test named perspectives
        let test_cases = vec![
            ("perspective-none", "none"),
            ("perspective-dramatic", "100px"),
            ("perspective-near", "300px"),
            ("perspective-normal", "500px"),
            ("perspective-midrange", "800px"),
            ("perspective-distant", "1200px"),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_perspective_class(class);
            assert!(result.is_some(), "Failed to parse: {}", class);
            let properties = result.unwrap();
            assert_eq!(properties.len(), 1);
            assert_eq!(properties[0].name, "perspective");
            assert_eq!(properties[0].value, expected_value);
            assert!(!properties[0].important);
        }
    }

    #[test]
    fn numerical_perspective_parsing() {
        let parser = PerspectiveParser::new();

        // Test numerical perspectives
        let result = parser.parse_perspective_class("perspective-500");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "perspective");
        assert_eq!(properties[0].value, "500px");

        let result = parser.parse_perspective_class("perspective-1000");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "1000px");
    }

    #[test]
    fn invalid_perspective_parsing() {
        let parser = PerspectiveParser::new();

        // Test invalid classes
        assert!(parser.parse_perspective_class("perspective-invalid").is_none());
        assert!(parser.parse_perspective_class("perspective-abc").is_none());
        assert!(parser.parse_perspective_class("perspective--100").is_none());
        assert!(parser.parse_perspective_class("perspective").is_none());
    }

    #[test]
    fn supported_patterns() {
        let parser = PerspectiveParser::new();
        let patterns = parser.supported_patterns();

        // Should contain named patterns
        assert!(patterns.contains(&"perspective-none".to_string()));
        assert!(patterns.contains(&"perspective-dramatic".to_string()));
        assert!(patterns.contains(&"perspective-normal".to_string()));

        // Should contain numerical patterns
        assert!(patterns.contains(&"perspective-250".to_string()));
        assert!(patterns.contains(&"perspective-500".to_string()));
        assert!(patterns.contains(&"perspective-1000".to_string()));
    }

    #[test]
    fn comprehensive_perspective_test() {
        let parser = PerspectiveParser::new();
        let test_cases = vec![
            ("perspective-none", Some("none")),
            ("perspective-dramatic", Some("100px")),
            ("perspective-near", Some("300px")),
            ("perspective-normal", Some("500px")),
            ("perspective-midrange", Some("800px")),
            ("perspective-distant", Some("1200px")),
            ("perspective-750", Some("750px")),
            ("perspective-1000", Some("1000px")),
            ("perspective-invalid", None),
            ("perspective-abc", None),
            ("invalid", None),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_perspective_class(class);
            match expected_value {
                Some(value) => {
                    assert!(result.is_some(), "Expected parsing for: {}", class);
                    assert_eq!(result.unwrap()[0].value, value, "Wrong value for: {}", class);
                }
                None => {
                    assert!(result.is_none(), "Expected no parsing for: {}", class);
                }
            }
        }
    }
}
