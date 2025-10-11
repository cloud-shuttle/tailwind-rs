//! Translate Parser Module
//!
//! Handles parsing of translate transform utilities:
//! - translate-x-0, translate-x-1, translate-x-2, ..., translate-x-px, translate-x-full, etc.
//! - translate-y-0, translate-y-1, translate-y-2, ..., translate-y-px, translate-y-full, etc.
//! - -translate-x-*, -translate-y-* (negative translations)

use crate::css_generator::types::CssProperty;

/// Translate parser
#[derive(Debug, Clone)]
pub struct TranslateParser;

impl TranslateParser {
    /// Create a new translate parser
    pub fn new() -> Self {
        Self
    }

    /// Parse translate classes
    pub fn parse_translate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Handle X-axis translations (both positive and negative)
        if class.starts_with("translate-x-") || (class.starts_with("-translate-x-") && class.len() > 12) {
            let value_part = if class.starts_with("-translate-x-") {
                class.strip_prefix("-translate-x-").unwrap()
            } else {
                class.strip_prefix("translate-x-").unwrap()
            };
            if let Some(css_value) = self.convert_translate_value(value_part) {
                let final_value = if class.starts_with("-") {
                    format!("translateX(-{})", css_value)
                } else {
                    format!("translateX({})", css_value)
                };
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: final_value,
                    important: false,
                }]);
            }
        }

        // Handle Y-axis translations (both positive and negative)
        if class.starts_with("translate-y-") || (class.starts_with("-translate-y-") && class.len() > 12) {
            let value_part = if class.starts_with("-translate-y-") {
                class.strip_prefix("-translate-y-").unwrap()
            } else {
                class.strip_prefix("translate-y-").unwrap()
            };
            if let Some(css_value) = self.convert_translate_value(value_part) {
                let final_value = if class.starts_with("-") {
                    format!("translateY(-{})", css_value)
                } else {
                    format!("translateY({})", css_value)
                };
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: final_value,
                    important: false,
                }]);
            }
        }

        None
    }

    /// Parse X-axis translate classes specifically
    pub fn parse_translate_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("translate-x-") {
            if let Some(css_value) = self.convert_translate_value(value) {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: format!("translateX({})", css_value),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse Y-axis translate classes specifically
    pub fn parse_translate_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("translate-y-") {
            if let Some(css_value) = self.convert_translate_value(value) {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: format!("translateY({})", css_value),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Convert translate value to CSS value
    fn convert_translate_value(&self, value: &str) -> Option<String> {
        self.value_to_css(value)
    }

    /// Convert a single value to CSS
    fn value_to_css(&self, value: &str) -> Option<String> {
        match value {
            "0" => Some("0px".to_string()),
            "px" => Some("1px".to_string()),
            "0.5" => Some("0.125rem".to_string()), // 2px
            "1" => Some("0.25rem".to_string()),   // 4px
            "1.5" => Some("0.375rem".to_string()), // 6px
            "2" => Some("0.5rem".to_string()),    // 8px
            "2.5" => Some("0.625rem".to_string()), // 10px
            "3" => Some("0.75rem".to_string()),   // 12px
            "3.5" => Some("0.875rem".to_string()), // 14px
            "4" => Some("1rem".to_string()),      // 16px
            "5" => Some("1.25rem".to_string()),   // 20px
            "6" => Some("1.5rem".to_string()),    // 24px
            "7" => Some("1.75rem".to_string()),   // 28px
            "8" => Some("2rem".to_string()),      // 32px
            "9" => Some("2.25rem".to_string()),   // 36px
            "10" => Some("2.5rem".to_string()),   // 40px
            "11" => Some("2.75rem".to_string()),  // 44px
            "12" => Some("3rem".to_string()),     // 48px
            "14" => Some("3.5rem".to_string()),   // 56px
            "16" => Some("4rem".to_string()),     // 64px
            "20" => Some("5rem".to_string()),     // 80px
            "24" => Some("6rem".to_string()),     // 96px
            "28" => Some("7rem".to_string()),     // 112px
            "32" => Some("8rem".to_string()),     // 128px
            "36" => Some("9rem".to_string()),     // 144px
            "40" => Some("10rem".to_string()),    // 160px
            "44" => Some("11rem".to_string()),    // 176px
            "48" => Some("12rem".to_string()),    // 192px
            "52" => Some("13rem".to_string()),    // 208px
            "56" => Some("14rem".to_string()),    // 224px
            "60" => Some("15rem".to_string()),    // 240px
            "64" => Some("16rem".to_string()),    // 256px
            "72" => Some("18rem".to_string()),    // 288px
            "80" => Some("20rem".to_string()),    // 320px
            "96" => Some("24rem".to_string()),    // 384px
            "full" => Some("100%".to_string()),
            "1/2" => Some("50%".to_string()),
            "1/3" => Some("33.333333%".to_string()),
            "2/3" => Some("66.666667%".to_string()),
            "1/4" => Some("25%".to_string()),
            "2/4" => Some("50%".to_string()),
            "3/4" => Some("75%".to_string()),
            _ => None,
        }
    }

    /// Get supported translate patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "translate-x-0", "translate-x-1", "translate-x-2", "translate-x-4", "translate-x-6",
            "translate-x-8", "translate-x-10", "translate-x-12", "translate-x-16", "translate-x-20",
            "translate-x-24", "translate-x-32", "translate-x-40", "translate-x-48", "translate-x-56",
            "translate-x-64", "translate-x-px", "translate-x-full",
            "-translate-x-1", "-translate-x-2", "-translate-x-4", "-translate-x-6", "-translate-x-8",
            "translate-y-0", "translate-y-1", "translate-y-2", "translate-y-4", "translate-y-6",
            "translate-y-8", "translate-y-10", "translate-y-12", "translate-y-16", "translate-y-20",
            "translate-y-24", "translate-y-32", "translate-y-40", "translate-y-48", "translate-y-56",
            "translate-y-64", "translate-y-px", "translate-y-full",
            "-translate-y-1", "-translate-y-2", "-translate-y-4", "-translate-y-6", "-translate-y-8",
        ]
    }
}

impl Default for TranslateParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_x_parsing() {
        let parser = TranslateParser::new();

        // Test positive X translations
        let result = parser.parse_translate_x_class("translate-x-4");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "translateX(1rem)");
        assert!(!properties[0].important);

        // Test negative X translations
        let result = parser.parse_translate_x_class("-translate-x-2");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "translateX(-0.5rem)");

        // Test full X translation
        let result = parser.parse_translate_x_class("translate-x-full");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "translateX(100%)");
    }

    #[test]
    fn translate_y_parsing() {
        let parser = TranslateParser::new();

        // Test positive Y translations
        let result = parser.parse_translate_y_class("translate-y-8");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "transform");
        assert_eq!(properties[0].value, "translateY(2rem)");

        // Test negative Y translations
        let result = parser.parse_translate_y_class("-translate-y-1");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "translateY(-0.25rem)");
    }

    #[test]
    fn translate_combined_parsing() {
        let parser = TranslateParser::new();

        // Test that the combined method works
        let result = parser.parse_translate_class("translate-x-6");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "translateX(1.5rem)");

        let result = parser.parse_translate_class("translate-y-12");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "translateY(3rem)");
    }

    #[test]
    fn invalid_translate_parsing() {
        let parser = TranslateParser::new();

        // Test invalid classes
        assert!(parser.parse_translate_class("translate-x-invalid").is_none());
        assert!(parser.parse_translate_class("translate-z-4").is_none());
        assert!(parser.parse_translate_class("translate-4").is_none());
        assert!(parser.parse_translate_class("invalid").is_none());
    }

    #[test]
    fn supported_patterns() {
        let parser = TranslateParser::new();
        let patterns = parser.supported_patterns();

        // Should contain various translate patterns
        assert!(patterns.contains(&"translate-x-4"));
        assert!(patterns.contains(&"translate-y-8"));
        assert!(patterns.contains(&"-translate-x-2"));
        assert!(patterns.contains(&"translate-x-full"));
    }
}
