//! Background Attachment Parser Module
//!
//! Handles parsing of background-attachment utilities:
//! - bg-fixed
//! - bg-local
//! - bg-scroll

use crate::css_generator::types::CssProperty;

/// Background attachment parser
pub struct BackgroundAttachmentParser;

impl BackgroundAttachmentParser {
    /// Create a new background attachment parser
    pub fn new() -> Self {
        Self
    }

    /// Parse background-attachment classes
    pub fn parse_attachment_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-fixed" => Some(vec![CssProperty {
                name: "background-attachment".to_string(),
                value: "fixed".to_string(),
                important: false,
            }]),
            "bg-local" => Some(vec![CssProperty {
                name: "background-attachment".to_string(),
                value: "local".to_string(),
                important: false,
            }]),
            "bg-scroll" => Some(vec![CssProperty {
                name: "background-attachment".to_string(),
                value: "scroll".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Get supported attachment patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "bg-fixed",
            "bg-local",
            "bg-scroll",
        ]
    }
}

impl Default for BackgroundAttachmentParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attachment_parsing() {
        let parser = BackgroundAttachmentParser::new();

        // Test bg-fixed
        let result = parser.parse_attachment_class("bg-fixed");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "background-attachment");
        assert_eq!(properties[0].value, "fixed");
        assert!(!properties[0].important);

        // Test bg-local
        let result = parser.parse_attachment_class("bg-local");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "background-attachment");
        assert_eq!(properties[0].value, "local");

        // Test bg-scroll
        let result = parser.parse_attachment_class("bg-scroll");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "background-attachment");
        assert_eq!(properties[0].value, "scroll");

        // Test invalid class
        let result = parser.parse_attachment_class("bg-invalid");
        assert!(result.is_none());
    }

    #[test]
    fn supported_patterns() {
        let parser = BackgroundAttachmentParser::new();
        let patterns = parser.supported_patterns();

        assert_eq!(patterns.len(), 3);
        assert!(patterns.contains(&"bg-fixed"));
        assert!(patterns.contains(&"bg-local"));
        assert!(patterns.contains(&"bg-scroll"));
    }

    #[test]
    fn comprehensive_attachment_test() {
        let parser = BackgroundAttachmentParser::new();
        let test_cases = vec![
            ("bg-fixed", Some("fixed")),
            ("bg-local", Some("local")),
            ("bg-scroll", Some("scroll")),
            ("bg-center", None),
            ("bg-cover", None),
            ("invalid", None),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_attachment_class(class);
            match expected_value {
                Some(value) => {
                    assert!(result.is_some(), "Expected parsing for: {}", class);
                    assert_eq!(result.unwrap()[0].value, value);
                }
                None => {
                    assert!(result.is_none(), "Expected no parsing for: {}", class);
                }
            }
        }
    }
}
