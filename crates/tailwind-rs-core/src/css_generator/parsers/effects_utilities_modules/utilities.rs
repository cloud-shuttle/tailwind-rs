//! Utility functions and helpers for effects parsing

use crate::css_generator::types::CssProperty;

/// Class parser for effects utilities
pub struct EffectClassParser;

impl EffectClassParser {
    /// Parse opacity values like "opacity-50" -> "0.5"
    pub fn parse_opacity_value(class: &str) -> Option<String> {
        if !class.starts_with("opacity-") {
            return None;
        }

        let value = &class[8..]; // Remove "opacity-" prefix

        match value {
            "0" => Some("0".to_string()),
            "5" => Some("0.05".to_string()),
            "10" => Some("0.1".to_string()),
            "20" => Some("0.2".to_string()),
            "25" => Some("0.25".to_string()),
            "30" => Some("0.3".to_string()),
            "40" => Some("0.4".to_string()),
            "50" => Some("0.5".to_string()),
            "60" => Some("0.6".to_string()),
            "70" => Some("0.7".to_string()),
            "75" => Some("0.75".to_string()),
            "80" => Some("0.8".to_string()),
            "90" => Some("0.9".to_string()),
            "95" => Some("0.95".to_string()),
            "100" => Some("1".to_string()),
            _ => None,
        }
    }

    /// Parse blend mode values
    pub fn parse_blend_mode_value(class: &str) -> Option<String> {
        if class.starts_with("mix-blend-") {
            let mode = &class[10..]; // Remove "mix-blend-" prefix
            Self::get_blend_mode_value(mode)
        } else if class.starts_with("bg-blend-") {
            let mode = &class[9..]; // Remove "bg-blend-" prefix
            Self::get_blend_mode_value(mode)
        } else {
            None
        }
    }

    /// Get blend mode CSS value
    fn get_blend_mode_value(mode: &str) -> Option<String> {
        match mode {
            "normal" => Some("normal".to_string()),
            "multiply" => Some("multiply".to_string()),
            "screen" => Some("screen".to_string()),
            "overlay" => Some("overlay".to_string()),
            "darken" => Some("darken".to_string()),
            "lighten" => Some("lighten".to_string()),
            "color-dodge" => Some("color-dodge".to_string()),
            "color-burn" => Some("color-burn".to_string()),
            "hard-light" => Some("hard-light".to_string()),
            "soft-light" => Some("soft-light".to_string()),
            "difference" => Some("difference".to_string()),
            "exclusion" => Some("exclusion".to_string()),
            "hue" => Some("hue".to_string()),
            "saturation" => Some("saturation".to_string()),
            "color" => Some("color".to_string()),
            "luminosity" => Some("luminosity".to_string()),
            _ => None,
        }
    }
}

/// CSS generator for effects utilities
pub struct EffectCssGenerator;

impl EffectCssGenerator {
    /// Generate filter CSS property
    pub fn generate_filter_property(value: &str) -> CssProperty {
        CssProperty::new("filter".to_string(), value.to_string())
    }

    /// Generate backdrop-filter CSS property
    pub fn generate_backdrop_filter_property(value: &str) -> CssProperty {
        CssProperty::new("backdrop-filter".to_string(), value.to_string())
    }
}

/// Validator for effects utilities
pub struct EffectValidator;

impl EffectValidator {
    /// Validate opacity class
    pub fn is_valid_opacity_class(class: &str) -> bool {
        EffectClassParser::parse_opacity_value(class).is_some()
    }

    /// Validate blend mode class
    pub fn is_valid_blend_mode_class(class: &str) -> bool {
        EffectClassParser::parse_blend_mode_value(class).is_some()
    }

    /// Validate backdrop blur class
    pub fn is_valid_backdrop_blur_class(class: &str) -> bool {
        matches!(class,
            "backdrop-blur" | "backdrop-blur-none" | "backdrop-blur-sm" |
            "backdrop-blur" | "backdrop-blur-md" | "backdrop-blur-lg" |
            "backdrop-blur-xl" | "backdrop-blur-2xl" | "backdrop-blur-3xl"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opacity_value() {
        assert_eq!(EffectClassParser::parse_opacity_value("opacity-50"), Some("0.5".to_string()));
        assert_eq!(EffectClassParser::parse_opacity_value("opacity-100"), Some("1".to_string()));
        assert_eq!(EffectClassParser::parse_opacity_value("opacity-invalid"), None);
    }

    #[test]
    fn test_parse_blend_mode_value() {
        assert_eq!(EffectClassParser::parse_blend_mode_value("mix-blend-multiply"), Some("multiply".to_string()));
        assert_eq!(EffectClassParser::parse_blend_mode_value("bg-blend-screen"), Some("screen".to_string()));
        assert_eq!(EffectClassParser::parse_blend_mode_value("invalid-blend"), None);
    }

    #[test]
    fn test_effect_css_generator() {
        let filter_prop = EffectCssGenerator::generate_filter_property("blur(4px)");
        assert_eq!(filter_prop.name, "filter");
        assert_eq!(filter_prop.value, "blur(4px)");

        let backdrop_prop = EffectCssGenerator::generate_backdrop_filter_property("blur(8px)");
        assert_eq!(backdrop_prop.name, "backdrop-filter");
        assert_eq!(backdrop_prop.value, "blur(8px)");
    }

    #[test]
    fn test_effect_validator() {
        assert!(EffectValidator::is_valid_opacity_class("opacity-50"));
        assert!(!EffectValidator::is_valid_opacity_class("opacity-invalid"));

        assert!(EffectValidator::is_valid_blend_mode_class("mix-blend-multiply"));
        assert!(!EffectValidator::is_valid_blend_mode_class("mix-blend-invalid"));

        assert!(EffectValidator::is_valid_backdrop_blur_class("backdrop-blur-xl"));
        assert!(!EffectValidator::is_valid_backdrop_blur_class("backdrop-blur-invalid"));
    }
}
