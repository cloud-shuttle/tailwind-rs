//! Opacity Utilities Module
//!
//! Handles parsing of opacity utilities:
//! - opacity-* (opacity percentages)
//! - opacity-[...] (arbitrary opacity values)

use crate::css_generator::types::CssProperty;

/// Opacity utilities parser
#[derive(Debug, Clone)]
pub struct OpacityParser;

impl OpacityParser {
    /// Create a new opacity parser
    pub fn new() -> Self {
        Self
    }

    /// Parse opacity classes
    pub fn parse_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(number) = class.strip_prefix("opacity-") {
            if number.parse::<u32>().is_ok() {
                return Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: format!("{}%", number),
                    important: false,
                }]);
            }
        }

        // Custom properties for opacity
        if let Some(value) = class.strip_prefix("opacity-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }

        // Arbitrary values for opacity
        if let Some(value) = class.strip_prefix("opacity-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }

        None
    }

    /// Get supported opacity patterns
    pub fn supported_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();

        // Standard opacity values (0-100)
        for i in 0..=100 {
            patterns.push(format!("opacity-{}", i));
        }

        patterns
    }
}

impl Default for OpacityParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opacity_percentage_values() {
        let parser = OpacityParser::new();

        let result = parser.parse_opacity_class("opacity-50");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "opacity");
        assert_eq!(properties[0].value, "50%");

        let result = parser.parse_opacity_class("opacity-0");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "0%");

        let result = parser.parse_opacity_class("opacity-100");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "100%");
    }

    #[test]
    fn opacity_arbitrary_values() {
        let parser = OpacityParser::new();

        let result = parser.parse_opacity_class("opacity-[0.75]");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "opacity");
        assert_eq!(properties[0].value, "0.75");
    }

    #[test]
    fn invalid_opacity_classes() {
        let parser = OpacityParser::new();

        assert!(parser.parse_opacity_class("opacity-invalid").is_none());
        assert!(parser.parse_opacity_class("opacity-150").is_none()); // Invalid percentage
    }

    #[test]
    fn supported_patterns_includes_standard_opacities() {
        let parser = OpacityParser::new();
        let patterns = parser.supported_patterns();

        assert!(patterns.contains(&"opacity-0".to_string()));
        assert!(patterns.contains(&"opacity-50".to_string()));
        assert!(patterns.contains(&"opacity-100".to_string()));

        // Should have 101 patterns (0-100)
        assert_eq!(patterns.len(), 101);
    }
}
