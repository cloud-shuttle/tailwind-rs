use crate::css_generator::types::CssProperty;
use crate::css_generator::trie::ParserType;
use crate::css_generator::parsers::{UtilityParser, ParserCategory};

/// Parser for CSS `field-sizing` utilities
/// https://developer.mozilla.org/en-US/docs/Web/CSS/field-sizing
#[derive(Debug, Clone)]
pub struct FieldSizingParser;

impl FieldSizingParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse field-sizing utility classes
    pub fn parse_field_sizing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "field-sizing-content" => Some(vec![CssProperty {
                name: "field-sizing".to_string(),
                value: "content".to_string(),
                important: false,
            }]),
            "field-sizing-fixed" => Some(vec![CssProperty {
                name: "field-sizing".to_string(),
                value: "fixed".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Get supported field-sizing patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "field-sizing-content",
            "field-sizing-fixed",
        ]
    }
}

impl UtilityParser for FieldSizingParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_field_sizing_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        self.supported_patterns()
    }

    fn get_priority(&self) -> u32 {
        80 // Standard priority
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_sizing_content() {
        let parser = FieldSizingParser::new();
        let result = parser.parse_field_sizing_class("field-sizing-content");

        assert!(result.is_some());
        let props = result.unwrap();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].name, "field-sizing");
        assert_eq!(props[0].value, "content");
        assert!(!props[0].important);
    }

    #[test]
    fn test_field_sizing_fixed() {
        let parser = FieldSizingParser::new();
        let result = parser.parse_field_sizing_class("field-sizing-fixed");

        assert!(result.is_some());
        let props = result.unwrap();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].name, "field-sizing");
        assert_eq!(props[0].value, "fixed");
        assert!(!props[0].important);
    }

    #[test]
    fn test_invalid_field_sizing() {
        let parser = FieldSizingParser::new();
        let result = parser.parse_field_sizing_class("field-sizing-invalid");
        assert!(result.is_none());
    }

    #[test]
    fn test_supported_patterns() {
        let parser = FieldSizingParser::new();
        let patterns = parser.supported_patterns();
        assert_eq!(patterns.len(), 2);
        assert!(patterns.contains(&"field-sizing-content"));
        assert!(patterns.contains(&"field-sizing-fixed"));
    }

    #[test]
    fn test_utility_parser_trait() {
        let parser = FieldSizingParser::new();

        // Test parse_class method
        let result = parser.parse_class("field-sizing-content");
        assert!(result.is_some());

        // Test supported patterns
        let patterns = parser.get_supported_patterns();
        assert!(!patterns.is_empty());

        // Test priority
        assert_eq!(parser.get_priority(), 80);

        // Test category
        assert!(matches!(parser.get_category(), ParserCategory::Layout));
    }
}
