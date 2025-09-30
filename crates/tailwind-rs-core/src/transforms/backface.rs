//! Backface Visibility Parser Module
//!
//! Handles parsing of backface-visibility utilities:
//! - backface-visible, backface-hidden

use crate::css_generator::types::CssProperty;

/// Backface visibility parser
#[derive(Debug, Clone)]
pub struct BackfaceVisibilityParser;

impl BackfaceVisibilityParser {
    /// Create a new backface visibility parser
    pub fn new() -> Self {
        Self
    }

    /// Parse backface-visibility classes
    pub fn parse_backface_visibility_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "backface-hidden" => Some(vec![CssProperty {
                name: "backface-visibility".to_string(),
                value: "hidden".to_string(),
                important: false,
            }]),
            "backface-visible" => Some(vec![CssProperty {
                name: "backface-visibility".to_string(),
                value: "visible".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Get supported backface visibility patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "backface-hidden",
            "backface-visible",
        ]
    }
}

impl Default for BackfaceVisibilityParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backface_visibility_parsing() {
        let parser = BackfaceVisibilityParser::new();

        // Test backface-hidden
        let result = parser.parse_backface_visibility_class("backface-hidden");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "backface-visibility");
        assert_eq!(properties[0].value, "hidden");
        assert!(!properties[0].important);

        // Test backface-visible
        let result = parser.parse_backface_visibility_class("backface-visible");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "backface-visibility");
        assert_eq!(properties[0].value, "visible");

        // Test invalid class
        let result = parser.parse_backface_visibility_class("backface-invalid");
        assert!(result.is_none());
    }

    #[test]
    fn supported_patterns() {
        let parser = BackfaceVisibilityParser::new();
        let patterns = parser.supported_patterns();

        assert_eq!(patterns.len(), 2);
        assert!(patterns.contains(&"backface-hidden"));
        assert!(patterns.contains(&"backface-visible"));
    }

    #[test]
    fn comprehensive_backface_test() {
        let parser = BackfaceVisibilityParser::new();
        let test_cases = vec![
            ("backface-hidden", Some("hidden")),
            ("backface-visible", Some("visible")),
            ("backface-auto", None),
            ("backface-inherit", None),
            ("invalid", None),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_backface_visibility_class(class);
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
