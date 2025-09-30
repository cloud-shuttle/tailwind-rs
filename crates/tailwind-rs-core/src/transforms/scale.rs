//! Scale Parser Module
//!
//! Handles parsing of scale transform utilities:
//! - scale-0, scale-50, scale-75, scale-100, etc.
//! - scale-x-*, scale-y-*

use crate::css_generator::types::CssProperty;
use super::utils::{TransformConversion, TransformValidation};

/// Scale parser
#[derive(Debug, Clone)]
pub struct ScaleParser;

impl ScaleParser {
    /// Create a new scale parser
    pub fn new() -> Self {
        Self
    }

    /// Parse scale classes
    pub fn parse_scale_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Handle uniform scaling (scale-*)
        if let Some(value) = class.strip_prefix("scale-") {
            if TransformValidation::is_valid_scale_value(value) {
                let scale_value = TransformConversion::scale_percentage_to_decimal(value);
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: format!("scale({})", scale_value),
                    important: false,
                }]);
            }
        }

        // Handle X-axis scaling (scale-x-*)
        if let Some(value) = class.strip_prefix("scale-x-") {
            if TransformValidation::is_valid_scale_value(value) {
                let scale_value = TransformConversion::scale_percentage_to_decimal(value);
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: format!("scaleX({})", scale_value),
                    important: false,
                }]);
            }
        }

        // Handle Y-axis scaling (scale-y-*)
        if let Some(value) = class.strip_prefix("scale-y-") {
            if TransformValidation::is_valid_scale_value(value) {
                let scale_value = TransformConversion::scale_percentage_to_decimal(value);
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: format!("scaleY({})", scale_value),
                    important: false,
                }]);
            }
        }

        None
    }

    /// Get supported scale patterns
    pub fn supported_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        let scale_values = ["0", "50", "75", "90", "95", "100", "105", "110", "125", "150"];

        // Uniform scaling
        for value in &scale_values {
            patterns.push(format!("scale-{}", value));
        }

        // X-axis scaling
        for value in &scale_values {
            patterns.push(format!("scale-x-{}", value));
        }

        // Y-axis scaling
        for value in &scale_values {
            patterns.push(format!("scale-y-{}", value));
        }

        patterns
    }
}

impl Default for ScaleParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_scale_parsing() {
        let parser = ScaleParser::new();

        // Test uniform scaling
        let test_cases = vec![
            ("scale-0", "scale(0)"),
            ("scale-50", "scale(0.5)"),
            ("scale-75", "scale(0.75)"),
            ("scale-100", "scale(1)"),
            ("scale-150", "scale(1.5)"),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_scale_class(class);
            assert!(result.is_some(), "Failed to parse: {}", class);
            let properties = result.unwrap();
            assert_eq!(properties.len(), 1);
            assert_eq!(properties[0].name, "transform");
            assert_eq!(properties[0].value, expected_value);
            assert!(!properties[0].important);
        }
    }

    #[test]
    fn x_axis_scale_parsing() {
        let parser = ScaleParser::new();

        // Test X-axis scaling
        let result = parser.parse_scale_class("scale-x-50");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "scaleX(0.5)");

        let result = parser.parse_scale_class("scale-x-125");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleX(1.25)");
    }

    #[test]
    fn y_axis_scale_parsing() {
        let parser = ScaleParser::new();

        // Test Y-axis scaling
        let result = parser.parse_scale_class("scale-y-75");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "scaleY(0.75)");

        let result = parser.parse_scale_class("scale-y-90");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleY(0.9)");
    }

    #[test]
    fn invalid_scale_parsing() {
        let parser = ScaleParser::new();

        // Test invalid classes
        assert!(parser.parse_scale_class("scale-200").is_none());
        assert!(parser.parse_scale_class("scale-invalid").is_none());
        assert!(parser.parse_scale_class("scale-x-abc").is_none());
        assert!(parser.parse_scale_class("scale").is_none());
    }

    #[test]
    fn supported_patterns() {
        let parser = ScaleParser::new();
        let patterns = parser.supported_patterns();

        // Should contain uniform scaling
        assert!(patterns.contains(&"scale-0".to_string()));
        assert!(patterns.contains(&"scale-100".to_string()));
        assert!(patterns.contains(&"scale-150".to_string()));

        // Should contain X-axis scaling
        assert!(patterns.contains(&"scale-x-50".to_string()));
        assert!(patterns.contains(&"scale-x-125".to_string()));

        // Should contain Y-axis scaling
        assert!(patterns.contains(&"scale-y-75".to_string()));
        assert!(patterns.contains(&"scale-y-110".to_string()));

        // Should have all expected patterns
        assert!(patterns.len() > 20); // At least 10 uniform + 10 x + 10 y
    }

    #[test]
    fn comprehensive_scale_test() {
        let parser = ScaleParser::new();
        let test_cases = vec![
            ("scale-0", Some("scale(0)")),
            ("scale-50", Some("scale(0.5)")),
            ("scale-100", Some("scale(1)")),
            ("scale-150", Some("scale(1.5)")),
            ("scale-x-75", Some("scaleX(0.75)")),
            ("scale-x-110", Some("scaleX(1.1)")),
            ("scale-y-90", Some("scaleY(0.9)")),
            ("scale-y-125", Some("scaleY(1.25)")),
            ("scale-200", None),
            ("scale-invalid", None),
            ("scale-x-abc", None),
            ("invalid", None),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_scale_class(class);
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
