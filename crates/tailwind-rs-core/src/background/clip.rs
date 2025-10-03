//! Background Clip Parser Module
//!
//! Handles parsing of background-clip utilities:
//! - bg-clip-border
//! - bg-clip-padding
//! - bg-clip-content
//! - bg-clip-text

use crate::css_generator::types::CssProperty;

/// Background clip parser
#[derive(Debug, Clone)]
pub struct BackgroundClipParser;

impl BackgroundClipParser {
    /// Create a new background clip parser
    pub fn new() -> Self {
        Self
    }

    /// Parse background-clip classes
    pub fn parse_clip_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-clip-border" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "border-box".to_string(),
                important: false,
            }]),
            "bg-clip-padding" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "padding-box".to_string(),
                important: false,
            }]),
            "bg-clip-content" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "content-box".to_string(),
                important: false,
            }]),
            "bg-clip-text" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "text".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Get supported clip patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "bg-clip-border",
            "bg-clip-padding",
            "bg-clip-content",
            "bg-clip-text",
        ]
    }
}

impl Default for BackgroundClipParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_parsing() {
        let parser = BackgroundClipParser::new();

        // Test bg-clip-border
        let result = parser.parse_clip_class("bg-clip-border");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "background-clip");
        assert_eq!(properties[0].value, "border-box");
        assert!(!properties[0].important);

        // Test bg-clip-padding
        let result = parser.parse_clip_class("bg-clip-padding");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "background-clip");
        assert_eq!(properties[0].value, "padding-box");

        // Test bg-clip-content
        let result = parser.parse_clip_class("bg-clip-content");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "background-clip");
        assert_eq!(properties[0].value, "content-box");

        // Test bg-clip-text
        let result = parser.parse_clip_class("bg-clip-text");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "background-clip");
        assert_eq!(properties[0].value, "text");

        // Test invalid class
        let result = parser.parse_clip_class("bg-clip-invalid");
        assert!(result.is_none());
    }

    #[test]
    fn supported_patterns() {
        let parser = BackgroundClipParser::new();
        let patterns = parser.supported_patterns();

        assert_eq!(patterns.len(), 4);
        assert!(patterns.contains(&"bg-clip-border"));
        assert!(patterns.contains(&"bg-clip-padding"));
        assert!(patterns.contains(&"bg-clip-content"));
        assert!(patterns.contains(&"bg-clip-text"));
    }

    #[test]
    fn comprehensive_clip_test() {
        let parser = BackgroundClipParser::new();
        let test_cases = vec![
            ("bg-clip-border", Some("border-box")),
            ("bg-clip-padding", Some("padding-box")),
            ("bg-clip-content", Some("content-box")),
            ("bg-clip-text", Some("text")),
            ("bg-fixed", None),
            ("bg-cover", None),
            ("invalid", None),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_clip_class(class);
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
