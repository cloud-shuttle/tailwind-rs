use super::utilities::{EffectClassParser, EffectCssGenerator, EffectValidator};
use super::box_shadow::BoxShadowParser;
use crate::css_generator::types::CssProperty;
use crate::css_generator::parsers::{UtilityParser, ParserCategory};

/// Effects parser for Tailwind CSS effect utilities (shadows, opacity, etc.)
#[derive(Debug, Clone)]
pub struct EffectsParser {
    box_shadow_parser: BoxShadowParser,
}

impl EffectsParser {
    /// Create a new effects parser
    pub fn new() -> Self {
        Self {
            box_shadow_parser: BoxShadowParser::new(),
        }
    }

    /// Parse effect classes
    pub fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try box shadow parsing first
        if let Some(properties) = self.box_shadow_parser.parse_box_shadow_class(class) {
            return Some(properties);
        }

        // Try other effect parsing
        self.parse_other_effects(class)
    }

    /// Parse other effect classes (opacity, blend modes, etc.)
    fn parse_other_effects(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Handle opacity classes
        if let Some(opacity_value) = EffectClassParser::parse_opacity_value(class) {
            return Some(vec![CssProperty::new("opacity".to_string(), opacity_value)]);
        }

        // Handle blend mode classes
        if let Some(blend_value) = EffectClassParser::parse_blend_mode_value(class) {
            let property_name = if class.starts_with("bg-blend-") {
                "background-blend-mode"
            } else if class.starts_with("mix-blend-") {
                "mix-blend-mode"
            } else {
                return None;
            };
            return Some(vec![CssProperty::new(property_name.to_string(), blend_value)]);
        }

        // Handle backdrop blur classes
        if class.starts_with("backdrop-blur") {
            return self.parse_backdrop_blur_class(class);
        }

        None
    }

    /// Parse backdrop blur classes
    fn parse_backdrop_blur_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let blur_value = match class {
            "backdrop-blur" | "backdrop-blur-none" => "none",
            "backdrop-blur-sm" => "blur(4px)",
            "backdrop-blur" => "blur(8px)",
            "backdrop-blur-md" => "blur(12px)",
            "backdrop-blur-lg" => "blur(16px)",
            "backdrop-blur-xl" => "blur(24px)",
            "backdrop-blur-2xl" => "blur(40px)",
            "backdrop-blur-3xl" => "blur(64px)",
            _ => return None,
        };

        Some(vec![CssProperty::new("backdrop-filter".to_string(), format!("blur({})", blur_value))])
    }

    /// Get supported patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        let mut patterns = self.box_shadow_parser.supported_patterns();

        // Add other effect patterns
        patterns.extend_from_slice(&[
            "opacity-0", "opacity-5", "opacity-10", "opacity-20", "opacity-25", "opacity-30",
            "opacity-40", "opacity-50", "opacity-60", "opacity-70", "opacity-75", "opacity-80",
            "opacity-90", "opacity-95", "opacity-100",
            "mix-blend-normal", "mix-blend-multiply", "mix-blend-screen", "mix-blend-overlay",
            "mix-blend-darken", "mix-blend-lighten", "mix-blend-color-dodge", "mix-blend-color-burn",
            "mix-blend-hard-light", "mix-blend-soft-light", "mix-blend-difference", "mix-blend-exclusion",
            "mix-blend-hue", "mix-blend-saturation", "mix-blend-color", "mix-blend-luminosity",
            "bg-blend-normal", "bg-blend-multiply", "bg-blend-screen", "bg-blend-overlay",
            "bg-blend-darken", "bg-blend-lighten", "bg-blend-color-dodge", "bg-blend-color-burn",
            "bg-blend-hard-light", "bg-blend-soft-light", "bg-blend-difference", "bg-blend-exclusion",
            "bg-blend-hue", "bg-blend-saturation", "bg-blend-color", "bg-blend-luminosity",
            "backdrop-blur", "backdrop-blur-none", "backdrop-blur-sm", "backdrop-blur-md",
            "backdrop-blur-lg", "backdrop-blur-xl", "backdrop-blur-2xl", "backdrop-blur-3xl",
        ]);

        patterns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shadow_class() {
        let parser = EffectsParser::new();

        let result = parser.parse_class("shadow-lg");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "box-shadow");
    }

    #[test]
    fn test_parse_opacity_class() {
        let parser = EffectsParser::new();

        let result = parser.parse_class("opacity-50");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "opacity");
        assert_eq!(properties[0].value, "0.5");
    }

    #[test]
    fn test_parse_backdrop_blur() {
        let parser = EffectsParser::new();

        let result = parser.parse_class("backdrop-blur-xl");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "backdrop-filter");
    }

    #[test]
    fn test_invalid_class() {
        let parser = EffectsParser::new();
        assert!(parser.parse_class("invalid-class").is_none());
    }
}

impl UtilityParser for EffectsParser {
    fn get_priority(&self) -> u32 {
        70
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Effects
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "shadow-*",
            "opacity-*",
            "mix-blend-*",
            "bg-blend-*",
            "backdrop-blur-*",
            "backdrop-brightness-*",
            "backdrop-contrast-*",
            "backdrop-grayscale*",
            "backdrop-hue-rotate-*",
            "backdrop-invert*",
            "backdrop-opacity-*",
            "backdrop-saturate-*",
            "backdrop-sepia*",
        ]
    }

    fn parse_class(&self, class: &str) -> Option<Vec<crate::css_generator::types::CssProperty>> {
        EffectsParser::parse_class(self, class)
    }
}
