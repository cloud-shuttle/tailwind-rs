//! Effects Utilities Helper Functions Module
//!
//! Utility functions and helpers for effects parsing:
//! - Value parsing and validation
//! - CSS property generation
//! - Effects-specific utilities

use super::types::*;

/// Helper functions for parsing effect values
pub struct EffectValueParser;

impl EffectValueParser {
    /// Parse opacity value from class suffix
    pub fn parse_opacity_value(suffix: &str) -> Option<String> {
        match suffix {
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

    /// Parse mix-blend-mode value from class suffix
    pub fn parse_blend_mode_value(suffix: &str) -> Option<String> {
        match suffix {
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

    /// Parse arbitrary values for effects
    pub fn parse_arbitrary_value(class: &str, prefix: &str) -> Option<String> {
        if let Some(stripped) = class.strip_prefix(&format!("{}-[", prefix)) {
            if let Some(inner) = stripped.strip_suffix(']') {
                return Some(inner.to_string());
            }
        }
        None
    }

    /// Parse custom property values for effects
    pub fn parse_custom_property_value(class: &str, prefix: &str) -> Option<String> {
        if let Some(stripped) = class.strip_prefix(&format!("{}-(", prefix)) {
            if let Some(inner) = stripped.strip_suffix(')') {
                return Some(format!("var({})", inner));
            }
        }
        None
    }
}

/// CSS property generation utilities
pub struct EffectCssGenerator;

impl EffectCssGenerator {
    /// Generate CSS property for box-shadow
    pub fn generate_shadow_property(value: &str) -> crate::css_generator::types::CssProperty {
        crate::css_generator::types::CssProperty {
            name: "box-shadow".to_string(),
            value: value.to_string(),
            important: false,
        }
    }

    /// Generate CSS property for opacity
    pub fn generate_opacity_property(value: &str) -> crate::css_generator::types::CssProperty {
        crate::css_generator::types::CssProperty {
            name: "opacity".to_string(),
            value: value.to_string(),
            important: false,
        }
    }

    /// Generate CSS property for mix-blend-mode
    pub fn generate_blend_mode_property(value: &str) -> crate::css_generator::types::CssProperty {
        crate::css_generator::types::CssProperty {
            name: "mix-blend-mode".to_string(),
            value: value.to_string(),
            important: false,
        }
    }

    /// Generate CSS property for filter
    pub fn generate_filter_property(value: &str) -> crate::css_generator::types::CssProperty {
        crate::css_generator::types::CssProperty {
            name: "filter".to_string(),
            value: value.to_string(),
            important: false,
        }
    }

    /// Generate CSS property for --tw-shadow-color
    pub fn generate_shadow_color_property(value: &str) -> crate::css_generator::types::CssProperty {
        crate::css_generator::types::CssProperty {
            name: "--tw-shadow-color".to_string(),
            value: value.to_string(),
            important: false,
        }
    }

    /// Generate CSS property for backdrop-filter
    pub fn generate_backdrop_filter_property(value: &str) -> crate::css_generator::types::CssProperty {
        crate::css_generator::types::CssProperty {
            name: "backdrop-filter".to_string(),
            value: value.to_string(),
            important: false,
        }
    }

    /// Generate CSS property for --tw-backdrop-opacity
    pub fn generate_backdrop_opacity_property(value: &str) -> crate::css_generator::types::CssProperty {
        crate::css_generator::types::CssProperty {
            name: "--tw-backdrop-opacity".to_string(),
            value: value.to_string(),
            important: false,
        }
    }

    /// Generate CSS property for mask
    pub fn generate_mask_property(value: &str) -> crate::css_generator::types::CssProperty {
        crate::css_generator::types::CssProperty {
            name: "mask".to_string(),
            value: value.to_string(),
            important: false,
        }
    }
}

/// Effect validation utilities
pub struct EffectValidator;

impl EffectValidator {
    /// Validate opacity value (0.0 to 1.0)
    pub fn is_valid_opacity(value: f32) -> bool {
        value >= 0.0 && value <= 1.0
    }

    /// Validate shadow class name
    pub fn is_valid_shadow_class(class: &str) -> bool {
        matches!(class, "shadow-sm" | "shadow" | "shadow-md" | "shadow-lg" | "shadow-xl" | "shadow-2xl" | "shadow-inner" | "shadow-none")
    }

    /// Validate opacity class name
    pub fn is_valid_opacity_class(class: &str) -> bool {
        class.starts_with("opacity-") && class.len() > 8 // "opacity-".len() = 8
    }

    /// Validate blend mode class name
    pub fn is_valid_blend_mode_class(class: &str) -> bool {
        class.starts_with("mix-blend-") && class.len() > 10 // "mix-blend-".len() = 10
    }

    /// Check if class represents a none effect
    pub fn is_none_effect(class: &str) -> bool {
        matches!(class, "shadow-none" | "opacity-0")
    }
}

/// Effect class parsing utilities
pub struct EffectClassParser;

impl EffectClassParser {
    /// Parse shadow type from class name
    pub fn parse_shadow_type(class: &str) -> Option<ShadowType> {
        match class {
            "shadow-sm" => Some(ShadowType::Sm),
            "shadow" => Some(ShadowType::Default),
            "shadow-md" => Some(ShadowType::Md),
            "shadow-lg" => Some(ShadowType::Lg),
            "shadow-xl" => Some(ShadowType::Xl),
            "shadow-2xl" => Some(ShadowType::Xxl),
            "shadow-inner" => Some(ShadowType::Inner),
            "shadow-none" => Some(ShadowType::None),
            _ => None,
        }
    }

    /// Parse opacity level from class name
    pub fn parse_opacity_level(class: &str) -> Option<OpacityLevel> {
        if let Some(suffix) = class.strip_prefix("opacity-") {
            match suffix {
                "0" => Some(OpacityLevel::Zero),
                "5" => Some(OpacityLevel::Five),
                "10" => Some(OpacityLevel::Ten),
                "20" => Some(OpacityLevel::Twenty),
                "25" => Some(OpacityLevel::TwentyFive),
                "30" => Some(OpacityLevel::Thirty),
                "40" => Some(OpacityLevel::Forty),
                "50" => Some(OpacityLevel::Fifty),
                "60" => Some(OpacityLevel::Sixty),
                "70" => Some(OpacityLevel::Seventy),
                "75" => Some(OpacityLevel::SeventyFive),
                "80" => Some(OpacityLevel::Eighty),
                "90" => Some(OpacityLevel::Ninety),
                "95" => Some(OpacityLevel::NinetyFive),
                "100" => Some(OpacityLevel::Hundred),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Parse blend mode from class name
    pub fn parse_blend_mode(class: &str) -> Option<MixBlendMode> {
        if let Some(suffix) = class.strip_prefix("mix-blend-") {
            match suffix {
                "normal" => Some(MixBlendMode::Normal),
                "multiply" => Some(MixBlendMode::Multiply),
                "screen" => Some(MixBlendMode::Screen),
                "overlay" => Some(MixBlendMode::Overlay),
                "darken" => Some(MixBlendMode::Darken),
                "lighten" => Some(MixBlendMode::Lighten),
                "color-dodge" => Some(MixBlendMode::ColorDodge),
                "color-burn" => Some(MixBlendMode::ColorBurn),
                "hard-light" => Some(MixBlendMode::HardLight),
                "soft-light" => Some(MixBlendMode::SoftLight),
                "difference" => Some(MixBlendMode::Difference),
                "exclusion" => Some(MixBlendMode::Exclusion),
                "hue" => Some(MixBlendMode::Hue),
                "saturation" => Some(MixBlendMode::Saturation),
                "color" => Some(MixBlendMode::Color),
                "luminosity" => Some(MixBlendMode::Luminosity),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Extract effect value from class name
    pub fn extract_effect_value(class: &str, effect_type: &EffectType) -> Option<String> {
        let prefix = match effect_type {
            EffectType::Shadow => "shadow",
            EffectType::Opacity => "opacity",
            EffectType::BlendMode => "mix-blend",
            EffectType::Mask => "mask",
        };

        class.strip_prefix(&format!("{}-", prefix)).map(|s| s.to_string())
    }
}

/// Performance utilities for effects
pub mod performance {
    use super::*;

    /// Estimate performance cost of an effect
    pub fn estimate_performance_cost(effect_type: &EffectType) -> u8 {
        match effect_type {
            EffectType::Shadow => 3, // Box shadows can be expensive
            EffectType::Opacity => 1, // Opacity is usually cheap
            EffectType::BlendMode => 4, // Blend modes are expensive
            EffectType::Mask => 5, // Masks are very expensive
        }
    }

    /// Check if effect might cause performance issues on mobile
    pub fn may_cause_mobile_performance_issues(effect_type: &EffectType, value: &str) -> bool {
        let cost = estimate_performance_cost(effect_type);

        // High-cost effects or complex values
        cost >= 4 || value.contains("blur") || value.contains("multiple")
    }

    /// Suggest performance alternatives
    pub fn suggest_performance_alternative(effect_type: &EffectType) -> Option<EffectType> {
        match effect_type {
            EffectType::BlendMode => Some(EffectType::Opacity),
            EffectType::Mask => Some(EffectType::Opacity),
            EffectType::Shadow => Some(EffectType::Opacity),
            EffectType::Opacity => None, // Already optimal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_value_parser_opacity() {
        assert_eq!(EffectValueParser::parse_opacity_value("0"), Some("0".to_string()));
        assert_eq!(EffectValueParser::parse_opacity_value("50"), Some("0.5".to_string()));
        assert_eq!(EffectValueParser::parse_opacity_value("100"), Some("1".to_string()));
        assert_eq!(EffectValueParser::parse_opacity_value("invalid"), None);
    }

    #[test]
    fn effect_value_parser_blend_mode() {
        assert_eq!(EffectValueParser::parse_blend_mode_value("multiply"), Some("multiply".to_string()));
        assert_eq!(EffectValueParser::parse_blend_mode_value("overlay"), Some("overlay".to_string()));
        assert_eq!(EffectValueParser::parse_blend_mode_value("invalid"), None);
    }

    #[test]
    fn effect_value_parser_arbitrary() {
        assert_eq!(EffectValueParser::parse_arbitrary_value("opacity-[0.75]", "opacity"), Some("0.75".to_string()));
        assert_eq!(EffectValueParser::parse_arbitrary_value("shadow-[custom]", "shadow"), Some("custom".to_string()));
        assert_eq!(EffectValueParser::parse_arbitrary_value("invalid", "opacity"), None);
    }

    #[test]
    fn effect_css_generator() {
        let shadow_prop = EffectCssGenerator::generate_shadow_property("0 1px 2px black");
        assert_eq!(shadow_prop.name, "box-shadow");
        assert_eq!(shadow_prop.value, "0 1px 2px black");

        let opacity_prop = EffectCssGenerator::generate_opacity_property("0.5");
        assert_eq!(opacity_prop.name, "opacity");
        assert_eq!(opacity_prop.value, "0.5");

        let blend_prop = EffectCssGenerator::generate_blend_mode_property("multiply");
        assert_eq!(blend_prop.name, "mix-blend-mode");
        assert_eq!(blend_prop.value, "multiply");
    }

    #[test]
    fn effect_validator() {
        assert!(EffectValidator::is_valid_opacity(0.5));
        assert!(!EffectValidator::is_valid_opacity(1.5));

        assert!(EffectValidator::is_valid_shadow_class("shadow-md"));
        assert!(!EffectValidator::is_valid_shadow_class("shadow-invalid"));

        assert!(EffectValidator::is_valid_opacity_class("opacity-50"));
        assert!(!EffectValidator::is_valid_opacity_class("invalid"));

        assert!(EffectValidator::is_none_effect("shadow-none"));
        assert!(EffectValidator::is_none_effect("opacity-0"));
        assert!(!EffectValidator::is_none_effect("opacity-50"));
    }

    #[test]
    fn effect_class_parser() {
        assert_eq!(EffectClassParser::parse_shadow_type("shadow-lg"), Some(ShadowType::Lg));
        assert_eq!(EffectClassParser::parse_shadow_type("shadow-invalid"), None);

        assert_eq!(EffectClassParser::parse_opacity_level("opacity-25"), Some(OpacityLevel::TwentyFive));
        assert_eq!(EffectClassParser::parse_opacity_level("opacity-invalid"), None);

        assert_eq!(EffectClassParser::parse_blend_mode("mix-blend-multiply"), Some(MixBlendMode::Multiply));
        assert_eq!(EffectClassParser::parse_blend_mode("mix-blend-invalid"), None);
    }

    #[test]
    fn performance_utilities() {
        assert_eq!(performance::estimate_performance_cost(&EffectType::Shadow), 3);
        assert_eq!(performance::estimate_performance_cost(&EffectType::Opacity), 1);
        assert_eq!(performance::estimate_performance_cost(&EffectType::BlendMode), 4);

        assert!(performance::may_cause_mobile_performance_issues(&EffectType::BlendMode, "multiply"));
        assert!(!performance::may_cause_mobile_performance_issues(&EffectType::Opacity, "0.5"));

        assert_eq!(performance::suggest_performance_alternative(&EffectType::BlendMode), Some(EffectType::Opacity));
        assert_eq!(performance::suggest_performance_alternative(&EffectType::Opacity), None);
    }
}
