//! Text Shadow Utilities Module
//!
//! Handles parsing of text-shadow utilities:
//! - text-shadow-* (standard text shadow sizes)
//! - text-shadow-*/opacity (text shadows with opacity)
//! - text-shadow-color-* (colored text shadows)
//! - text-shadow-[...] (arbitrary text shadows)

use crate::css_generator::types::CssProperty;

/// Text shadow utilities parser
#[derive(Debug, Clone)]
pub struct TextShadowParser;

impl TextShadowParser {
    /// Create a new text shadow parser
    pub fn new() -> Self {
        Self
    }

    /// Parse text-shadow classes
    pub fn parse_text_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "text-shadow-2xs" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "var(--text-shadow-2xs)".to_string(),
                important: false,
            }]),
            "text-shadow-xs" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "var(--text-shadow-xs)".to_string(),
                important: false,
            }]),
            "text-shadow-sm" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "var(--text-shadow-sm)".to_string(),
                important: false,
            }]),
            "text-shadow-md" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "var(--text-shadow-md)".to_string(),
                important: false,
            }]),
            "text-shadow-lg" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "var(--text-shadow-lg)".to_string(),
                important: false,
            }]),
            "text-shadow-none" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for text shadow
                if let Some(value) = class.strip_prefix("text-shadow-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "text-shadow".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for text shadow
                if let Some(value) = class.strip_prefix("text-shadow-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "text-shadow".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                // Text shadow with opacity modifier (e.g., text-shadow-lg/20)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_shadow = parts[0];
                        let opacity = parts[1];
                        if let Some(shadow_value) = self.get_text_shadow_value(base_shadow) {
                            return Some(vec![CssProperty {
                                name: "text-shadow".to_string(),
                                value: format!("{}/{}", shadow_value, opacity),
                                important: false,
                            }]);
                        }
                    }
                }

                // Text shadow with color (e.g., text-shadow-indigo-500)
                if let Some(color_value) = self.get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "text-shadow".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }

                None
            }
        }
    }

    /// Get standard text shadow value for a shadow type
    fn get_text_shadow_value(&self, shadow_type: &str) -> Option<String> {
        match shadow_type.strip_prefix("text-shadow-") {
            Some("2xs") => Some("var(--text-shadow-2xs)".to_string()),
            Some("xs") => Some("var(--text-shadow-xs)".to_string()),
            Some("sm") => Some("var(--text-shadow-sm)".to_string()),
            Some("md") => Some("var(--text-shadow-md)".to_string()),
            Some("lg") => Some("var(--text-shadow-lg)".to_string()),
            _ => None,
        }
    }

    /// Get color value for text shadow with color
    fn get_color_value(&self, class: &str) -> Option<String> {
        // This would typically delegate to a color parser
        // For now, return None as this would need color parsing logic
        // that should be handled by a dedicated color module
        None
    }

    /// Get supported text shadow patterns
    pub fn supported_patterns(&self) -> Vec<String> {
        vec![
            "text-shadow-2xs".to_string(),
            "text-shadow-xs".to_string(),
            "text-shadow-sm".to_string(),
            "text-shadow-md".to_string(),
            "text-shadow-lg".to_string(),
            "text-shadow-none".to_string(),
        ]
    }
}

impl Default for TextShadowParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_shadow_standard_sizes() {
        let parser = TextShadowParser::new();

        let result = parser.parse_text_shadow_class("text-shadow-md");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "text-shadow");
        assert_eq!(properties[0].value, "var(--text-shadow-md)");

        let result = parser.parse_text_shadow_class("text-shadow-none");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "none");
    }

    #[test]
    fn text_shadow_arbitrary_values() {
        let parser = TextShadowParser::new();

        let result = parser.parse_text_shadow_class("text-shadow-[2px_2px_4px_rgba(0,0,0,0.5)]");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "text-shadow");
        assert_eq!(properties[0].value, "2px_2px_4px_rgba(0,0,0,0.5)");
    }

    #[test]
    fn invalid_text_shadow_classes() {
        let parser = TextShadowParser::new();

        assert!(parser.parse_text_shadow_class("invalid-shadow").is_none());
        assert!(parser.parse_text_shadow_class("text-shadow-invalid").is_none());
    }
}
