//! Background Color Parser Module
//!
//! Handles parsing of background-color utilities:
//! - bg-white, bg-black, bg-transparent, etc.
//! - bg-red-500, bg-blue-600, etc.
//! - bg-[arbitrary], bg-(custom)
//! - Colors with opacity: bg-blue-500/50

use crate::css_generator::types::CssProperty;
use super::utils::BackgroundColorUtils;

/// Background color parser
pub struct BackgroundColorParser;

impl BackgroundColorParser {
    /// Create a new background color parser
    pub fn new() -> Self {
        Self
    }

    /// Parse background-color classes
    pub fn parse_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-inherit" => Some(vec![CssProperty {
                name: "background-color".to_string(),
                value: "inherit".to_string(),
                important: false,
            }]),
            "bg-current" => Some(vec![CssProperty {
                name: "background-color".to_string(),
                value: "currentColor".to_string(),
                important: false,
            }]),
            "bg-transparent" => Some(vec![CssProperty {
                name: "background-color".to_string(),
                value: "transparent".to_string(),
                important: false,
            }]),
            "bg-black" => Some(vec![CssProperty {
                name: "background-color".to_string(),
                value: "#000000".to_string(),
                important: false,
            }]),
            "bg-white" => Some(vec![CssProperty {
                name: "background-color".to_string(),
                value: "#ffffff".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for background color
                if let Some(value) = class.strip_prefix("bg-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "background-color".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for background color
                if let Some(value) = class.strip_prefix("bg-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "background-color".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                // Color with opacity modifier (e.g., bg-blue-600/50)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_color = parts[0];
                        let opacity = parts[1];
                        // Handle basic colors like bg-white, bg-black, bg-transparent
                        let color_value = match base_color {
                            "bg-white" => Some("#ffffff".to_string()),
                            "bg-black" => Some("#000000".to_string()),
                            "bg-transparent" => Some("transparent".to_string()),
                            "bg-current" => Some("currentColor".to_string()),
                            "bg-inherit" => Some("inherit".to_string()),
                            _ => BackgroundColorUtils::get_color_value(base_color),
                        };
                        if let Some(color_value) = color_value {
                            // Convert to rgba with opacity
                            if let Some(rgba_value) = Self::convert_hex_to_rgba(&color_value, opacity) {
                                return Some(vec![CssProperty {
                                    name: "background-color".to_string(),
                                    value: rgba_value,
                                    important: false,
                                }]);
                            }
                        }
                    }
                }

                // Standard color classes (bg-red-500, bg-blue-600, etc.)
                if let Some(color_value) = BackgroundColorUtils::get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "background-color".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }

                None
            }
        }
    }

    /// Get supported color patterns (simplified - would be comprehensive in real implementation)
    pub fn supported_patterns(&self) -> Vec<String> {
        let mut patterns = vec![
            "bg-inherit".to_string(),
            "bg-current".to_string(),
            "bg-transparent".to_string(),
            "bg-black".to_string(),
            "bg-white".to_string(),
        ];

        // Add color scales
        let colors = ["red", "blue", "green", "gray"];
        let intensities = ["50", "100", "200", "300", "400", "500", "600", "700", "800", "900", "950"];

        for color in &colors {
            for intensity in &intensities {
                patterns.push(format!("bg-{}-{}", color, intensity));
            }
        }

        // Add arbitrary value patterns
        patterns.push("bg-(*)".to_string());
        patterns.push("bg-[*]".to_string());

        // Add opacity patterns
        patterns.push("bg-*/**".to_string());

        patterns
    }

    /// Convert hex color to rgba with opacity
    fn convert_hex_to_rgba(hex_color: &str, opacity: &str) -> Option<String> {
        if hex_color.starts_with('#') && hex_color.len() == 7 {
            let r = u8::from_str_radix(&hex_color[1..3], 16).ok()?;
            let g = u8::from_str_radix(&hex_color[3..5], 16).ok()?;
            let b = u8::from_str_radix(&hex_color[5..7], 16).ok()?;
            let a = match opacity {
                "0" => 0.0,
                "5" => 0.05,
                "10" => 0.1,
                "20" => 0.2,
                "25" => 0.25,
                "30" => 0.3,
                "40" => 0.4,
                "50" => 0.5,
                "60" => 0.6,
                "70" => 0.7,
                "75" => 0.75,
                "80" => 0.8,
                "90" => 0.9,
                "95" => 0.95,
                "100" => 1.0,
                _ => return None,
            };
            Some(format!("rgba({}, {}, {}, {})", r, g, b, a))
        } else if hex_color == "transparent" {
            Some("transparent".to_string())
        } else if hex_color == "currentColor" {
            Some("currentColor".to_string())
        } else if hex_color == "inherit" {
            Some("inherit".to_string())
        } else {
            None
        }
    }
}

impl Default for BackgroundColorParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_color_parsing() {
        let parser = BackgroundColorParser::new();

        // Test basic colors
        let result = parser.parse_color_class("bg-white");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "#ffffff");

        let result = parser.parse_color_class("bg-black");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "#000000");

        let result = parser.parse_color_class("bg-transparent");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "transparent");
    }

    #[test]
    fn scale_color_parsing() {
        let parser = BackgroundColorParser::new();

        // Test color scales
        let result = parser.parse_color_class("bg-red-500");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "#ef4444");

        let result = parser.parse_color_class("bg-blue-600");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "#2563eb");

        let result = parser.parse_color_class("bg-green-400");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "#4ade80");
    }

    #[test]
    fn arbitrary_value_parsing() {
        let parser = BackgroundColorParser::new();

        // Test arbitrary values
        let result = parser.parse_color_class("bg-[#ff0000]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "#ff0000");

        let result = parser.parse_color_class("bg-(--custom-color)");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "var(--custom-color)");
    }

    #[test]
    fn opacity_parsing() {
        let parser = BackgroundColorParser::new();

        // Test colors with opacity
        let result = parser.parse_color_class("bg-blue-500/50");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "rgba(59, 130, 246, 0.5)");

        let result = parser.parse_color_class("bg-red-600/25");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "rgba(220, 38, 38, 0.25)");

        let result = parser.parse_color_class("bg-white/80");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "rgba(255, 255, 255, 0.8)");
    }

    #[test]
    fn invalid_color_handling() {
        let parser = BackgroundColorParser::new();

        // Test invalid classes
        assert!(parser.parse_color_class("bg-invalid").is_none());
        assert!(parser.parse_color_class("bg-red-999").is_none());
        assert!(parser.parse_color_class("bg-blue-500/150").is_none());
        assert!(parser.parse_color_class("bg-[#invalid]").is_none());
    }

    #[test]
    fn rgba_conversion() {
        // Test the rgba conversion function directly
        assert_eq!(
            BackgroundColorParser::convert_hex_to_rgba("#ff0000", "50"),
            Some("rgba(255, 0, 0, 0.5)".to_string())
        );
        assert_eq!(
            BackgroundColorParser::convert_hex_to_rgba("#00ff00", "100"),
            Some("rgba(0, 255, 0, 1)".to_string())
        );
        assert_eq!(
            BackgroundColorParser::convert_hex_to_rgba("#0000ff", "0"),
            Some("rgba(0, 0, 255, 0)".to_string())
        );
        assert_eq!(
            BackgroundColorParser::convert_hex_to_rgba("transparent", "50"),
            Some("transparent".to_string())
        );
        assert!(BackgroundColorParser::convert_hex_to_rgba("#invalid", "50").is_none());
        assert!(BackgroundColorParser::convert_hex_to_rgba("#ff0000", "invalid").is_none());
    }

    #[test]
    fn supported_patterns() {
        let parser = BackgroundColorParser::new();
        let patterns = parser.supported_patterns();

        // Check that basic patterns are included
        assert!(patterns.contains(&"bg-white".to_string()));
        assert!(patterns.contains(&"bg-black".to_string()));
        assert!(patterns.contains(&"bg-transparent".to_string()));

        // Check that color scales are included
        assert!(patterns.contains(&"bg-red-500".to_string()));
        assert!(patterns.contains(&"bg-blue-600".to_string()));
        assert!(patterns.contains(&"bg-green-400".to_string()));

        // Check arbitrary patterns
        assert!(patterns.contains(&"bg-(*)".to_string()));
        assert!(patterns.contains(&"bg-[*]".to_string()));
        assert!(patterns.contains(&"bg-*/**".to_string()));
    }

    #[test]
    fn comprehensive_color_test() {
        let parser = BackgroundColorParser::new();
        let test_cases = vec![
            ("bg-inherit", Some("inherit")),
            ("bg-current", Some("currentColor")),
            ("bg-transparent", Some("transparent")),
            ("bg-black", Some("#000000")),
            ("bg-white", Some("#ffffff")),
            ("bg-red-500", Some("#ef4444")),
            ("bg-blue-600", Some("#2563eb")),
            ("bg-green-400", Some("#4ade80")),
            ("bg-gray-700", Some("#374151")),
            ("bg-[#ff0000]", Some("#ff0000")),
            ("bg-(--custom)", Some("var(--custom)")),
            ("bg-blue-500/50", Some("rgba(59, 130, 246, 0.5)")),
            ("bg-white/80", Some("rgba(255, 255, 255, 0.8)")),
            ("bg-invalid", None),
            ("bg-red-999", None),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_color_class(class);
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
