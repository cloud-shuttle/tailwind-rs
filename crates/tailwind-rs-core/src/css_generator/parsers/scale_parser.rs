//! Scale Parser
//!
//! This module provides parsing logic for scale utilities,
//! including scale-x and scale-y classes that were previously missing.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScaleParser {
    scale_x_map: HashMap<String, String>,
    scale_y_map: HashMap<String, String>,
}

impl ScaleParser {
    pub fn new() -> Self {
        let mut scale_x_map = HashMap::new();
        let mut scale_y_map = HashMap::new();

        // Initialize scale-x values
        scale_x_map.insert("0".to_string(), "scaleX(0)".to_string());
        scale_x_map.insert("50".to_string(), "scaleX(0.5)".to_string());
        scale_x_map.insert("75".to_string(), "scaleX(0.75)".to_string());
        scale_x_map.insert("90".to_string(), "scaleX(0.9)".to_string());
        scale_x_map.insert("95".to_string(), "scaleX(0.95)".to_string());
        scale_x_map.insert("100".to_string(), "scaleX(1)".to_string());
        scale_x_map.insert("105".to_string(), "scaleX(1.05)".to_string());
        scale_x_map.insert("110".to_string(), "scaleX(1.1)".to_string());
        scale_x_map.insert("125".to_string(), "scaleX(1.25)".to_string());
        scale_x_map.insert("150".to_string(), "scaleX(1.5)".to_string());

        // Initialize scale-y values (same as scale-x)
        scale_y_map.insert("0".to_string(), "scaleY(0)".to_string());
        scale_y_map.insert("50".to_string(), "scaleY(0.5)".to_string());
        scale_y_map.insert("75".to_string(), "scaleY(0.75)".to_string());
        scale_y_map.insert("90".to_string(), "scaleY(0.9)".to_string());
        scale_y_map.insert("95".to_string(), "scaleY(0.95)".to_string());
        scale_y_map.insert("100".to_string(), "scaleY(1)".to_string());
        scale_y_map.insert("105".to_string(), "scaleY(1.05)".to_string());
        scale_y_map.insert("110".to_string(), "scaleY(1.1)".to_string());
        scale_y_map.insert("125".to_string(), "scaleY(1.25)".to_string());
        scale_y_map.insert("150".to_string(), "scaleY(1.5)".to_string());

        Self {
            scale_x_map,
            scale_y_map,
        }
    }

    /// Parse scale-x classes
    fn parse_scale_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("scale-x-") {
            if let Some(transform_value) = self.scale_x_map.get(value) {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: transform_value.clone(),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse scale-y classes
    fn parse_scale_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("scale-y-") {
            if let Some(transform_value) = self.scale_y_map.get(value) {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: transform_value.clone(),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for ScaleParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_scale_x_class(class)
            .or_else(|| self.parse_scale_y_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "scale-x-0", "scale-x-50", "scale-x-75", "scale-x-90", "scale-x-95",
            "scale-x-100", "scale-x-105", "scale-x-110", "scale-x-125", "scale-x-150",
            "scale-y-0", "scale-y-50", "scale-y-75", "scale-y-90", "scale-y-95",
            "scale-y-100", "scale-y-105", "scale-y-110", "scale-y-125", "scale-y-150",
        ]
    }

    fn get_priority(&self) -> u32 { 80 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Transforms }
}

impl Default for ScaleParser {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_parser_creation() {
        let parser = ScaleParser::new();
        assert!(!parser.scale_x_map.is_empty());
        assert!(!parser.scale_y_map.is_empty());
    }

    #[test]
    fn test_scale_x_parsing() {
        let parser = ScaleParser::new();

        // Test basic scale-x classes
        let result = parser.parse_class("scale-x-50");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "scaleX(0.5)");

        let result = parser.parse_class("scale-x-100");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleX(1)");

        let result = parser.parse_class("scale-x-150");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleX(1.5)");
    }

    #[test]
    fn test_scale_y_parsing() {
        let parser = ScaleParser::new();

        // Test basic scale-y classes
        let result = parser.parse_class("scale-y-50");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "scaleY(0.5)");

        let result = parser.parse_class("scale-y-100");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleY(1)");

        let result = parser.parse_class("scale-y-150");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleY(1.5)");
    }

    #[test]
    fn test_zero_scale_parsing() {
        let parser = ScaleParser::new();

        // Test zero scale values
        let result = parser.parse_class("scale-x-0");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleX(0)");

        let result = parser.parse_class("scale-y-0");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleY(0)");
    }

    #[test]
    fn test_fractional_scale_parsing() {
        let parser = ScaleParser::new();

        // Test fractional scale values
        let result = parser.parse_class("scale-x-75");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleX(0.75)");

        let result = parser.parse_class("scale-y-90");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleY(0.9)");

        let result = parser.parse_class("scale-x-95");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleX(0.95)");

        let result = parser.parse_class("scale-y-105");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleY(1.05)");

        let result = parser.parse_class("scale-x-110");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleX(1.1)");

        let result = parser.parse_class("scale-y-125");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "scaleY(1.25)");
    }

    #[test]
    fn test_invalid_scale_class_returns_none() {
        let parser = ScaleParser::new();

        // Test invalid classes
        assert!(parser.parse_class("invalid-class").is_none());
        assert!(parser.parse_class("scale-z-50").is_none());
        assert!(parser.parse_class("scale-x-invalid").is_none());
        assert!(parser.parse_class("scale-y-200").is_none()); // Value not in mapping
    }

    #[test]
    fn test_scale_supported_patterns() {
        let parser = ScaleParser::new();
        let patterns = parser.get_supported_patterns();

        // Should contain scale-x and scale-y patterns
        assert!(patterns.contains(&"scale-x-0"));
        assert!(patterns.contains(&"scale-x-50"));
        assert!(patterns.contains(&"scale-x-75"));
        assert!(patterns.contains(&"scale-x-90"));
        assert!(patterns.contains(&"scale-x-95"));
        assert!(patterns.contains(&"scale-x-100"));
        assert!(patterns.contains(&"scale-x-105"));
        assert!(patterns.contains(&"scale-x-110"));
        assert!(patterns.contains(&"scale-x-125"));
        assert!(patterns.contains(&"scale-x-150"));

        assert!(patterns.contains(&"scale-y-0"));
        assert!(patterns.contains(&"scale-y-50"));
        assert!(patterns.contains(&"scale-y-75"));
        assert!(patterns.contains(&"scale-y-90"));
        assert!(patterns.contains(&"scale-y-95"));
        assert!(patterns.contains(&"scale-y-100"));
        assert!(patterns.contains(&"scale-y-105"));
        assert!(patterns.contains(&"scale-y-110"));
        assert!(patterns.contains(&"scale-y-125"));
        assert!(patterns.contains(&"scale-y-150"));
    }

    #[test]
    fn test_scale_parser_priority_and_category() {
        let parser = ScaleParser::new();

        assert_eq!(parser.get_priority(), 80);
        assert!(matches!(parser.get_category(), ParserCategory::Transforms));
    }

    #[test]
    fn test_all_scale_values_parsing() {
        let parser = ScaleParser::new();

        // Test all scale-x values
        let scale_x_values = ["0", "50", "75", "90", "95", "100", "105", "110", "125", "150"];
        for &value in &scale_x_values {
            let class = format!("scale-x-{}", value);
            let result = parser.parse_class(&class);
            assert!(result.is_some(), "Failed to parse {}", class);
        }

        // Test all scale-y values
        let scale_y_values = ["0", "50", "75", "90", "95", "100", "105", "110", "125", "150"];
        for &value in &scale_y_values {
            let class = format!("scale-y-{}", value);
            let result = parser.parse_class(&class);
            assert!(result.is_some(), "Failed to parse {}", class);
        }
    }

    #[test]
    fn test_scale_css_values() {
        let parser = ScaleParser::new();

        // Test specific scale value mappings
        let test_cases = vec![
            ("scale-x-0", "scaleX(0)"),
            ("scale-x-50", "scaleX(0.5)"),
            ("scale-x-75", "scaleX(0.75)"),
            ("scale-x-90", "scaleX(0.9)"),
            ("scale-x-95", "scaleX(0.95)"),
            ("scale-x-100", "scaleX(1)"),
            ("scale-x-105", "scaleX(1.05)"),
            ("scale-x-110", "scaleX(1.1)"),
            ("scale-x-125", "scaleX(1.25)"),
            ("scale-x-150", "scaleX(1.5)"),
            ("scale-y-0", "scaleY(0)"),
            ("scale-y-50", "scaleY(0.5)"),
            ("scale-y-75", "scaleY(0.75)"),
            ("scale-y-90", "scaleY(0.9)"),
            ("scale-y-95", "scaleY(0.95)"),
            ("scale-y-100", "scaleY(1)"),
            ("scale-y-105", "scaleY(1.05)"),
            ("scale-y-110", "scaleY(1.1)"),
            ("scale-y-125", "scaleY(1.25)"),
            ("scale-y-150", "scaleY(1.5)"),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_class(class);
            assert!(result.is_some(), "Failed to parse {}", class);
            let properties = result.unwrap();
            assert_eq!(properties[0].value, expected_value);
        }
    }
}
