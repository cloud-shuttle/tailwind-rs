//! Background Gradient Parser Module
//!
//! Handles parsing of background gradient direction utilities:
//! - bg-gradient-to-t, bg-gradient-to-tr, bg-gradient-to-r, etc.

use crate::css_generator::types::CssProperty;

/// Background gradient parser
#[derive(Debug, Clone)]
pub struct BackgroundGradientParser;

impl BackgroundGradientParser {
    /// Create a new background gradient parser
    pub fn new() -> Self {
        Self
    }

    /// Parse background gradient direction classes
    pub fn parse_gradient_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let direction = match class {
            "bg-gradient-to-t" => "to top",
            "bg-gradient-to-tr" => "to top right",
            "bg-gradient-to-r" => "to right",
            "bg-gradient-to-br" => "to bottom right",
            "bg-gradient-to-b" => "to bottom",
            "bg-gradient-to-bl" => "to bottom left",
            "bg-gradient-to-l" => "to left",
            "bg-gradient-to-tl" => "to top left",
            _ => return None,
        };

        // Set up CSS variables for gradient stops and generate background-image using them
        Some(vec![
            CssProperty {
                name: "--tw-gradient-stops".to_string(),
                value: "var(--tw-gradient-from), var(--tw-gradient-via), var(--tw-gradient-to, transparent)".to_string(),
                important: false,
            },
            CssProperty {
                name: "background-image".to_string(),
                value: format!("linear-gradient({}, var(--tw-gradient-stops))", direction),
                important: false,
            },
        ])
    }

    /// Get supported gradient direction patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "bg-gradient-to-t",
            "bg-gradient-to-tr",
            "bg-gradient-to-r",
            "bg-gradient-to-br",
            "bg-gradient-to-b",
            "bg-gradient-to-bl",
            "bg-gradient-to-l",
            "bg-gradient-to-tl",
        ]
    }
}

impl Default for BackgroundGradientParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_direction_parsing() {
        let parser = BackgroundGradientParser::new();

        // Test bg-gradient-to-r
        let result = parser.parse_gradient_class("bg-gradient-to-r");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 2);

        // Check gradient-stops variable
        assert_eq!(properties[0].name, "--tw-gradient-stops");
        assert!(properties[0].value.contains("var(--tw-gradient-from)"));
        assert!(properties[0].value.contains("var(--tw-gradient-via)"));
        assert!(properties[0].value.contains("var(--tw-gradient-to"));

        // Check background-image
        assert_eq!(properties[1].name, "background-image");
        assert!(properties[1].value.contains("linear-gradient(to right"));
        assert!(properties[1].value.contains("var(--tw-gradient-stops)"));
    }

    #[test]
    fn all_gradient_directions() {
        let parser = BackgroundGradientParser::new();
        let directions = vec![
            ("bg-gradient-to-t", "to top"),
            ("bg-gradient-to-tr", "to top right"),
            ("bg-gradient-to-r", "to right"),
            ("bg-gradient-to-br", "to bottom right"),
            ("bg-gradient-to-b", "to bottom"),
            ("bg-gradient-to-bl", "to bottom left"),
            ("bg-gradient-to-l", "to left"),
            ("bg-gradient-to-tl", "to top left"),
        ];

        for (class, expected_direction) in directions {
            let result = parser.parse_gradient_class(class);
            assert!(result.is_some(), "Failed to parse: {}", class);
            let properties = result.unwrap();
            assert_eq!(properties.len(), 2);
            assert!(properties[1].value.contains(expected_direction),
                   "Wrong direction for {}: {}", class, properties[1].value);
        }
    }

    #[test]
    fn invalid_gradient_class() {
        let parser = BackgroundGradientParser::new();

        assert!(parser.parse_gradient_class("bg-gradient-to-x").is_none());
        assert!(parser.parse_gradient_class("bg-gradient").is_none());
        assert!(parser.parse_gradient_class("bg-fixed").is_none());
    }

    #[test]
    fn supported_patterns() {
        let parser = BackgroundGradientParser::new();
        let patterns = parser.supported_patterns();

        assert_eq!(patterns.len(), 8);
        assert!(patterns.contains(&"bg-gradient-to-t"));
        assert!(patterns.contains(&"bg-gradient-to-r"));
        assert!(patterns.contains(&"bg-gradient-to-b"));
        assert!(patterns.contains(&"bg-gradient-to-l"));
    }

    #[test]
    fn css_property_structure() {
        let parser = BackgroundGradientParser::new();
        let result = parser.parse_gradient_class("bg-gradient-to-r").unwrap();

        // Both properties should not be important
        assert!(!result[0].important);
        assert!(!result[1].important);

        // First property should be the CSS variable
        assert!(result[0].name.starts_with("--"));
        assert!(result[0].value.contains("var("));

        // Second property should be background-image
        assert_eq!(result[1].name, "background-image");
        assert!(result[1].value.starts_with("linear-gradient("));
    }
}
