//! Spacing Parser Module
//!
//! This module implements the main spacing parser that coordinates
//! all spacing-related parsing functionality.

use crate::css_generator::types::CssProperty;
use super::{utilities::*, constants::*, values::*};
use super::super::css_generator::parsers::{ParserCategory, UtilityParser};

/// Main spacing parser implementation
#[derive(Debug, Clone)]
pub struct SpacingParser {
    utilities: SpacingUtilities,
}

impl SpacingParser {
    /// Create a new spacing parser with default configuration
    pub fn new() -> Self {
        Self {
            utilities: SpacingUtilities::new(),
        }
    }

    /// Create a spacing parser with custom configuration
    pub fn with_config(config: SpacingParserConfig) -> Self {
        Self {
            utilities: SpacingUtilities::with_config(config),
        }
    }

    /// Get spacing value for a token (convenience method)
    pub fn get_spacing_value(&self, token: &str) -> Option<String> {
        self.utilities.get_spacing_value(token)
    }

    /// Parse a spacing class and return CSS properties
    pub fn parse_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try padding first
        if let Some(properties) = self.utilities.parse_padding_class(class) {
            return Some(properties);
        }

        // Try margin
        if let Some(properties) = self.utilities.parse_margin_class(class) {
            return Some(properties);
        }

        // Try gap
        if let Some(properties) = self.utilities.parse_gap_class(class) {
            return Some(properties);
        }

        None
    }

    /// Get the spacing utilities instance
    pub fn utilities(&self) -> &SpacingUtilities {
        &self.utilities
    }

    /// Get the parser configuration
    pub fn config(&self) -> &SpacingParserConfig {
        self.utilities.config()
    }
}

impl UtilityParser for SpacingParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_spacing_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        SPACING_PATTERNS.iter().cloned().collect()
    }

    fn get_priority(&self) -> u32 {
        SpacingPriority::default().padding
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Spacing
    }
}

impl Default for SpacingParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Spacing parser builder for advanced configuration
pub struct SpacingParserBuilder {
    config: SpacingParserConfig,
}

impl SpacingParserBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: SpacingParserConfig::default(),
        }
    }

    /// Enable or disable arbitrary values
    pub fn arbitrary_values(mut self, enabled: bool) -> Self {
        self.config.enable_arbitrary_values = enabled;
        self
    }

    /// Enable or disable logical properties
    pub fn logical_properties(mut self, enabled: bool) -> Self {
        self.config.enable_logical_properties = enabled;
        self
    }

    /// Enable or disable axis properties
    pub fn axis_properties(mut self, enabled: bool) -> Self {
        self.config.enable_axis_properties = enabled;
        self
    }

    /// Set maximum spacing scale
    pub fn max_spacing_scale(mut self, scale: f32) -> Self {
        self.config.max_spacing_scale = scale;
        self
    }

    /// Build the spacing parser
    pub fn build(self) -> SpacingParser {
        SpacingParser::with_config(self.config)
    }
}

impl Default for SpacingParserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_parser_creation() {
        let parser = SpacingParser::new();
        assert_eq!(parser.get_spacing_value("4"), Some("1rem".to_string()));
    }

    #[test]
    fn spacing_parser_integration() {
        let parser = SpacingParser::new();

        // Test padding parsing
        let result = parser.parse_spacing_class("p-4");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "1rem");

        // Test margin parsing
        let result = parser.parse_spacing_class("m-2");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "0.5rem");

        // Test gap parsing
        let result = parser.parse_spacing_class("gap-3");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "0.75rem");

        // Test invalid class
        let result = parser.parse_spacing_class("invalid-class");
        assert!(result.is_none());
    }

    #[test]
    fn utility_parser_implementation() {
        let parser = SpacingParser::new();

        assert_eq!(parser.get_priority(), 100);
        assert_eq!(parser.get_category(), ParserCategory::Spacing);

        let patterns = parser.get_supported_patterns();
        assert!(patterns.contains(&"p-*"));
        assert!(patterns.contains(&"m-*"));
        assert!(patterns.contains(&"gap-*"));
    }

    #[test]
    fn spacing_parser_builder() {
        let parser = SpacingParserBuilder::new()
            .arbitrary_values(false)
            .logical_properties(false)
            .axis_properties(false)
            .max_spacing_scale(50.0)
            .build();

        let config = parser.config();
        assert!(!config.arbitrary_values_enabled());
        assert!(!config.logical_properties_enabled());
        assert!(!config.axis_properties_enabled());
        assert_eq!(config.max_spacing_scale, 50.0);

        // Should not parse arbitrary values
        assert_eq!(parser.get_spacing_value("[10px]"), None);
    }

    #[test]
    fn comprehensive_spacing_test() {
        let parser = SpacingParser::new();

        // Test various spacing classes
        let test_cases = vec![
            ("p-0", Some("padding"), "0"),
            ("p-4", Some("padding"), "1rem"),
            ("px-2", Some("padding-left"), "0.5rem"),
            ("py-3", Some("padding-top"), "0.75rem"),
            ("pt-1", Some("padding-top"), "0.25rem"),
            ("m-4", Some("margin"), "1rem"),
            ("mx-2", Some("margin-left"), "0.5rem"),
            ("mt-3", Some("margin-top"), "0.75rem"),
            ("gap-4", Some("gap"), "1rem"),
            ("gap-x-2", Some("column-gap"), "0.5rem"),
            ("invalid", None, ""),
        ];

        for (class, expected_property, expected_value) in test_cases {
            let result = parser.parse_spacing_class(class);

            match expected_property {
                Some(prop) => {
                    assert!(result.is_some(), "Expected parsing for class: {}", class);
                    let properties = result.unwrap();
                    assert!(!properties.is_empty(), "Expected properties for class: {}", class);

                    // Find the property with the expected name
                    let property = properties.iter().find(|p| p.name == prop);
                    assert!(property.is_some(), "Expected property '{}' for class: {}", prop, class);
                    assert_eq!(property.unwrap().value, expected_value);
                }
                None => {
                    assert!(result.is_none(), "Expected no parsing for class: {}", class);
                }
            }
        }
    }

    #[test]
    fn arbitrary_value_integration() {
        let parser = SpacingParser::new();

        // Test arbitrary values in classes
        let result = parser.parse_spacing_class("p-[10px]");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "padding");
        assert_eq!(properties[0].value, "10px");
    }

    #[test]
    fn axis_properties_integration() {
        let parser = SpacingParser::new();

        // Test axis padding
        let result = parser.parse_spacing_class("px-4");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0].name, "padding-left");
        assert_eq!(properties[1].name, "padding-right");
        assert_eq!(properties[0].value, "1rem");
        assert_eq!(properties[1].value, "1rem");
    }
}
