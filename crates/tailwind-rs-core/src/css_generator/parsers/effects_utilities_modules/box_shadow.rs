//! Box Shadow Utilities Module
//!
//! Handles parsing of box-shadow utilities:
//! - shadow-* (standard shadow sizes)
//! - shadow-color-* (colored shadows)
//! - shadow-*/opacity (shadows with opacity)
//! - inset-shadow-* (inset shadows)
//! - shadow-[...] (arbitrary shadows)

use crate::css_generator::types::CssProperty;

/// Box shadow utilities parser
#[derive(Debug, Clone)]
pub struct BoxShadowParser;

impl BoxShadowParser {
    /// Create a new box shadow parser
    pub fn new() -> Self {
        Self
    }

    /// Parse box-shadow classes
    pub fn parse_box_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "shadow-2xs" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-2xs)".to_string(),
                important: false,
            }]),
            "shadow-xs" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-xs)".to_string(),
                important: false,
            }]),
            "shadow-sm" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-sm)".to_string(),
                important: false,
            }]),
            "shadow" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-md)".to_string(),
                important: false,
            }]),
            "shadow-md" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-md)".to_string(),
                important: false,
            }]),
            "shadow-lg" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-lg)".to_string(),
                important: false,
            }]),
            "shadow-xl" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-xl)".to_string(),
                important: false,
            }]),
            "shadow-2xl" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-2xl)".to_string(),
                important: false,
            }]),
            "shadow-none" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 0 #0000".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for box shadow
                if let Some(value) = class.strip_prefix("shadow-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "box-shadow".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for box shadow
                if let Some(value) = class.strip_prefix("shadow-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "box-shadow".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                // Shadow with opacity modifier (e.g., shadow-xl/20)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_shadow = parts[0];
                        let opacity = parts[1];
                        if let Some(shadow_value) = self.get_shadow_value(base_shadow) {
                            return Some(vec![CssProperty {
                                name: "box-shadow".to_string(),
                                value: format!("{}/{}", shadow_value, opacity),
                                important: false,
                            }]);
                        }
                    }
                }

                // Shadow with color (e.g., shadow-indigo-500, shadow-indigo-500/50)
                if let Some(color_part) = class.strip_prefix("shadow-") {
                    if let Some(color_value) = self.get_color_value(color_part) {
                        return Some(vec![CssProperty {
                            name: "box-shadow".to_string(),
                            value: color_value,
                            important: false,
                        }]);
                    }
                }

                // Inset shadows
                if class.starts_with("inset-shadow-") {
                    if let Some(shadow_type) = class.strip_prefix("inset-shadow-") {
                        if let Some(shadow_value) = self.get_inset_shadow_value(shadow_type) {
                            return Some(vec![CssProperty {
                                name: "box-shadow".to_string(),
                                value: shadow_value,
                                important: false,
                            }]);
                        }
                    }
                }

                None
            }
        }
    }

    /// Get standard shadow value for a shadow type
    fn get_shadow_value(&self, shadow_type: &str) -> Option<String> {
        match shadow_type.strip_prefix("shadow-") {
            Some("2xs") => Some("var(--shadow-2xs)".to_string()),
            Some("xs") => Some("var(--shadow-xs)".to_string()),
            Some("sm") => Some("var(--shadow-sm)".to_string()),
            Some("md") | Some("") => Some("var(--shadow-md)".to_string()),
            Some("lg") => Some("var(--shadow-lg)".to_string()),
            Some("xl") => Some("var(--shadow-xl)".to_string()),
            Some("2xl") => Some("var(--shadow-2xl)".to_string()),
            _ => None,
        }
    }

    /// Get inset shadow value for a shadow type
    fn get_inset_shadow_value(&self, shadow_type: &str) -> Option<String> {
        match shadow_type {
            "2xs" => Some("inset var(--shadow-2xs)".to_string()),
            "xs" => Some("inset var(--shadow-xs)".to_string()),
            "sm" => Some("inset var(--shadow-sm)".to_string()),
            "md" => Some("inset var(--shadow-md)".to_string()),
            "lg" => Some("inset var(--shadow-lg)".to_string()),
            "xl" => Some("inset var(--shadow-xl)".to_string()),
            "2xl" => Some("inset var(--shadow-2xl)".to_string()),
            _ => None,
        }
    }

    /// Get color value for shadow with color
    fn get_color_value(&self, class: &str) -> Option<String> {
        // This would typically delegate to a color parser
        // For now, return None as this would need color parsing logic
        // that should be handled by a dedicated color module
        None
    }

    /// Get supported box shadow patterns
    pub fn supported_patterns(&self) -> Vec<String> {
        let mut patterns = vec![
            "shadow-2xs".to_string(),
            "shadow-xs".to_string(),
            "shadow-sm".to_string(),
            "shadow".to_string(),
            "shadow-md".to_string(),
            "shadow-lg".to_string(),
            "shadow-xl".to_string(),
            "shadow-2xl".to_string(),
            "shadow-none".to_string(),
        ];

        // Add inset shadow patterns
        for size in &["2xs", "xs", "sm", "md", "lg", "xl", "2xl"] {
            patterns.push(format!("inset-shadow-{}", size));
        }

        patterns
    }
}

impl Default for BoxShadowParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_shadow_standard_sizes() {
        let parser = BoxShadowParser::new();

        let result = parser.parse_box_shadow_class("shadow-md");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "box-shadow");
        assert_eq!(properties[0].value, "var(--shadow-md)");

        let result = parser.parse_box_shadow_class("shadow-none");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "0 0 #0000");
    }

    #[test]
    fn box_shadow_arbitrary_values() {
        let parser = BoxShadowParser::new();

        let result = parser.parse_box_shadow_class("shadow-[0_4px_6px_-1px_rgba(0,0,0,0.1)]");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "box-shadow");
        assert_eq!(properties[0].value, "0_4px_6px_-1px_rgba(0,0,0,0.1)");
    }

    #[test]
    fn inset_shadow_parsing() {
        let parser = BoxShadowParser::new();

        let result = parser.parse_box_shadow_class("inset-shadow-lg");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "box-shadow");
        assert_eq!(properties[0].value, "inset var(--shadow-lg)");
    }

    #[test]
    fn invalid_box_shadow_classes() {
        let parser = BoxShadowParser::new();

        assert!(parser.parse_box_shadow_class("invalid-shadow").is_none());
        assert!(parser.parse_box_shadow_class("shadow-invalid").is_none());
    }
}
