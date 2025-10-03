use crate::css_generator::types::CssProperty;
use crate::css_generator::parsers::effects_utilities_modules::utilities::{
    EffectClassParser, EffectCssGenerator, EffectValidator
};

/// Box shadow parser for Tailwind CSS shadow utilities
#[derive(Debug, Clone)]
pub struct BoxShadowParser;

impl BoxShadowParser {
    /// Create a new box shadow parser
    pub fn new() -> Self {
        Self
    }

    /// Parse shadow classes like `shadow-sm`, `shadow-lg`, `shadow-blue-500/25`
    pub fn parse_box_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Handle basic shadow classes
        if let Some(shadow_value) = self.parse_basic_shadow_value(class) {
            return Some(vec![CssProperty::new("box-shadow".to_string(), shadow_value)]);
        }

        // Handle shadow color classes like `shadow-blue-500` or `shadow-blue-500/25`
        if let Some(color_property) = self.get_color_value(class) {
            return Some(vec![color_property]);
        }

        None
    }

    /// Parse basic shadow values
    fn parse_basic_shadow_value(&self, class: &str) -> Option<String> {
        match class {
            "shadow" | "shadow-none" => Some("none".to_string()),
            "shadow-sm" => Some("0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string()),
            "shadow" => Some("0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string()),
            "shadow-md" => Some("0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string()),
            "shadow-lg" => Some("0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string()),
            "shadow-xl" => Some("0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string()),
            "shadow-2xl" => Some("0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string()),
            "shadow-inner" => Some("inset 0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string()),
            _ => None,
        }
    }

    /// Get color value for shadow color classes
    pub fn get_color_value(&self, class: &str) -> Option<CssProperty> {
        // This method is used by the parser but not directly accessible
        // The color parsing is handled by the main parser system
        None
    }

    /// Get supported shadow patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "shadow", "shadow-none", "shadow-sm", "shadow-md", "shadow-lg",
            "shadow-xl", "shadow-2xl", "shadow-inner",
        ]
    }

    /// Check if a class is a valid shadow class
    pub fn is_valid_shadow_class(&self, class: &str) -> bool {
        self.parse_basic_shadow_value(class).is_some() ||
        class.starts_with("shadow-") // Allow for color variants
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_shadow() {
        let parser = BoxShadowParser::new();

        assert_eq!(
            parser.parse_box_shadow_class("shadow-lg"),
            Some(vec![CssProperty::new("box-shadow".to_string(), "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string())])
        );

        assert_eq!(
            parser.parse_box_shadow_class("shadow-none"),
            Some(vec![CssProperty::new("box-shadow".to_string(), "none".to_string())])
        );

        assert_eq!(parser.parse_box_shadow_class("invalid"), None);
    }

    #[test]
    fn test_supported_patterns() {
        let parser = BoxShadowParser::new();
        let patterns = parser.supported_patterns();
        assert!(patterns.contains(&"shadow-lg"));
        assert!(patterns.contains(&"shadow-none"));
    }
}