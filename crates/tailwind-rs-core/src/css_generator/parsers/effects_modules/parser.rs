//! Effects Parser Module
//!
//! Main parsing logic for effects utilities:
//! - EffectsParser: Core parser implementation
//! - Individual effect type parsers

use super::types::*;
use super::utilities::*;
use crate::css_generator::types::CssProperty;

/// Core effects parser
#[derive(Debug, Clone)]
pub struct EffectsParser;

impl EffectsParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse shadow classes
    pub fn parse_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try predefined shadow types first
        if let Some(shadow_type) = EffectClassParser::parse_shadow_type(class) {
            return Some(vec![EffectCssGenerator::generate_shadow_property(shadow_type.css_value())]);
        }

        // Try arbitrary values
        if let Some(value) = EffectValueParser::parse_arbitrary_value(class, "shadow") {
            return Some(vec![EffectCssGenerator::generate_shadow_property(&value)]);
        }

        // Try custom properties
        if let Some(value) = EffectValueParser::parse_custom_property_value(class, "shadow") {
            return Some(vec![EffectCssGenerator::generate_shadow_property(&value)]);
        }

        None
    }

    /// Parse drop shadow classes
    pub fn parse_drop_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "drop-shadow-sm" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))")]),
            "drop-shadow" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))")]),
            "drop-shadow-md" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))")]),
            "drop-shadow-lg" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))")]),
            "drop-shadow-xl" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))")]),
            "drop-shadow-2xl" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))")]),
            "drop-shadow-none" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 0 #0000)")]),
            _ => None,
        }
    }

    /// Parse shadow color classes
    pub fn parse_shadow_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(color_part) = class.strip_prefix("shadow-") {
            // Handle opacity modifiers (e.g., shadow-purple-500/25)
            if let Some((color_name, opacity)) = color_part.split_once('/') {
                let color_value = self.get_shadow_color_value(color_name)?;
                let final_color = self.convert_hex_to_rgba(&color_value, opacity)?;
                return Some(vec![EffectCssGenerator::generate_shadow_color_property(&final_color)]);
            } else {
                // Handle regular shadow colors (e.g., shadow-purple-500)
                let color_value = self.get_shadow_color_value(color_part)?;
                return Some(vec![EffectCssGenerator::generate_shadow_color_property(&color_value)]);
            }
        }
        None
    }

    /// Parse backdrop blur classes
    pub fn parse_backdrop_blur_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(blur_value) = class.strip_prefix("backdrop-blur") {
            let value = match blur_value {
                "" | "-none" => "none",
                "-sm" => "blur(4px)",
                "" => "blur(8px)",
                "-md" => "blur(12px)",
                "-lg" => "blur(16px)",
                "-xl" => "blur(24px)",
                "-2xl" => "blur(40px)",
                "-3xl" => "blur(64px)",
                _ => return None,
            };
            return Some(vec![EffectCssGenerator::generate_backdrop_filter_property(value)]);
        }
        None
    }

    /// Parse backdrop opacity classes
    pub fn parse_backdrop_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(opacity_part) = class.strip_prefix("backdrop-opacity-") {
            let opacity_value = EffectValueParser::parse_opacity_value(opacity_part)?;
            return Some(vec![EffectCssGenerator::generate_backdrop_opacity_property(&opacity_value)]);
        }
        None
    }

    /// Get shadow color value for a given color name
    pub fn get_shadow_color_value(&self, color_name: &str) -> Option<String> {
        // This would need to be implemented with the actual color mappings
        // For now, return None to indicate this functionality needs to be added
        None
    }

    /// Convert hex color to RGBA with opacity
    pub fn convert_hex_to_rgba(&self, hex_color: &str, opacity: &str) -> Option<String> {
        // This would need to be implemented with hex to rgba conversion
        // For now, return None to indicate this functionality needs to be added
        None
    }

    /// Parse opacity classes
    pub fn parse_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try predefined opacity levels first
        if let Some(opacity_level) = EffectClassParser::parse_opacity_level(class) {
            return Some(vec![EffectCssGenerator::generate_opacity_property(&opacity_level.to_css_value())]);
        }

        // Try arbitrary values
        if let Some(value) = EffectValueParser::parse_arbitrary_value(class, "opacity") {
            // Validate that it's a valid opacity value
            if let Ok(parsed) = value.parse::<f32>() {
                if EffectValidator::is_valid_opacity(parsed) {
                    return Some(vec![EffectCssGenerator::generate_opacity_property(&value)]);
                }
            }
            // If it's not a number, use it as-is (for CSS variables, etc.)
            return Some(vec![EffectCssGenerator::generate_opacity_property(&value)]);
        }

        // Try custom properties
        if let Some(value) = EffectValueParser::parse_custom_property_value(class, "opacity") {
            return Some(vec![EffectCssGenerator::generate_opacity_property(&value)]);
        }

        None
    }

    /// Parse mix-blend-mode classes
    pub fn parse_mix_blend_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try predefined blend modes first
        if let Some(blend_mode) = EffectClassParser::parse_blend_mode(class) {
            return Some(vec![EffectCssGenerator::generate_blend_mode_property(blend_mode.css_value())]);
        }

        // Try arbitrary values
        if let Some(value) = EffectValueParser::parse_arbitrary_value(class, "mix-blend") {
            return Some(vec![EffectCssGenerator::generate_blend_mode_property(&value)]);
        }

        // Try custom properties
        if let Some(value) = EffectValueParser::parse_custom_property_value(class, "mix-blend") {
            return Some(vec![EffectCssGenerator::generate_blend_mode_property(&value)]);
        }

        None
    }

    /// Parse mask classes
    pub fn parse_mask_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class == "mask-none" {
            return Some(vec![EffectCssGenerator::generate_mask_property("none")]);
        }

        // Try arbitrary values
        if let Some(value) = EffectValueParser::parse_arbitrary_value(class, "mask") {
            return Some(vec![EffectCssGenerator::generate_mask_property(&value)]);
        }

        // Try custom properties
        if let Some(value) = EffectValueParser::parse_custom_property_value(class, "mask") {
            return Some(vec![EffectCssGenerator::generate_mask_property(&value)]);
        }

        None
    }

    /// Main parsing method that dispatches to specific effect parsers
    pub fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Dispatch to specific effect parsers based on prefix
        if class.starts_with("shadow-") {
            self.parse_shadow_class(class)
        } else if class.starts_with("opacity-") {
            self.parse_opacity_class(class)
        } else if class.starts_with("mix-blend-") {
            self.parse_mix_blend_class(class)
        } else if class.starts_with("mask-") {
            self.parse_mask_class(class)
        } else {
            None
        }
    }
}

impl Default for EffectsParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Effect builder for complex effect combinations
#[derive(Debug, Clone)]
pub struct EffectBuilder {
    effects: Vec<EffectConfig>,
}

impl EffectBuilder {
    /// Create a new effect builder
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    /// Add a shadow effect
    pub fn shadow(mut self, value: String) -> Self {
        self.effects.push(EffectConfig::new(EffectType::Shadow, value));
        self
    }

    /// Add an opacity effect
    pub fn opacity(mut self, value: String) -> Self {
        self.effects.push(EffectConfig::new(EffectType::Opacity, value));
        self
    }

    /// Add a blend mode effect
    pub fn blend_mode(mut self, value: String) -> Self {
        self.effects.push(EffectConfig::new(EffectType::BlendMode, value));
        self
    }

    /// Add a mask effect
    pub fn mask(mut self, value: String) -> Self {
        self.effects.push(EffectConfig::new(EffectType::Mask, value));
        self
    }

    /// Build all CSS properties
    pub fn build(&self) -> Vec<CssProperty> {
        self.effects.iter().map(|config| config.to_css_property()).collect()
    }

    /// Check if the effect combination might have performance issues
    pub fn has_performance_concerns(&self) -> bool {
        self.effects.iter().any(|config| {
            performance::may_cause_mobile_performance_issues(&config.effect_type, &config.value)
        })
    }

    /// Get performance cost estimate
    pub fn performance_cost(&self) -> u32 {
        self.effects.iter()
            .map(|config| performance::estimate_performance_cost(&config.effect_type) as u32)
            .sum()
    }
}

impl Default for EffectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Preset effect combinations
pub mod presets {
    use super::*;

    /// Create a subtle shadow effect
    pub fn subtle_shadow() -> EffectBuilder {
        EffectBuilder::new()
            .shadow("0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string())
            .opacity("1".to_string())
    }

    /// Create a card-like effect
    pub fn card_effect() -> EffectBuilder {
        EffectBuilder::new()
            .shadow("0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string())
            .opacity("1".to_string())
    }

    /// Create a modal backdrop effect
    pub fn modal_backdrop() -> EffectBuilder {
        EffectBuilder::new()
            .opacity("0.5".to_string())
            .blend_mode("multiply".to_string())
    }

    /// Create a glass morphism effect
    pub fn glass_morphism() -> EffectBuilder {
        EffectBuilder::new()
            .opacity("0.25".to_string())
            .blend_mode("overlay".to_string())
            .shadow("0 8px 32px 0 rgba(31, 38, 135, 0.37)".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effects_parser_creation() {
        let parser = EffectsParser::new();
        assert!(parser.parse_class("shadow-sm").is_some());
    }

    #[test]
    fn parse_shadow_classes() {
        let parser = EffectsParser::new();

        let result = parser.parse_shadow_class("shadow-md");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "box-shadow");

        let result = parser.parse_shadow_class("shadow-[custom]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "custom");
    }

    #[test]
    fn parse_opacity_classes() {
        let parser = EffectsParser::new();

        let result = parser.parse_opacity_class("opacity-50");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "0.50");

        let result = parser.parse_opacity_class("opacity-[0.75]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "0.75");
    }

    #[test]
    fn parse_mix_blend_classes() {
        let parser = EffectsParser::new();

        let result = parser.parse_mix_blend_class("mix-blend-multiply");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "multiply");

        let result = parser.parse_mix_blend_class("mix-blend-[screen]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "screen");
    }

    #[test]
    fn parse_mask_classes() {
        let parser = EffectsParser::new();

        let result = parser.parse_mask_class("mask-none");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "none");

        let result = parser.parse_mask_class("mask-[circle]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "circle");
    }

    #[test]
    fn effect_builder_operations() {
        let builder = EffectBuilder::new()
            .shadow("0 1px 2px black".to_string())
            .opacity("0.8".to_string())
            .blend_mode("multiply".to_string());

        let properties = builder.build();
        assert_eq!(properties.len(), 3);

        let shadow_prop = &properties[0];
        assert_eq!(shadow_prop.name, "box-shadow");
        assert_eq!(shadow_prop.value, "0 1px 2px black");

        let opacity_prop = &properties[1];
        assert_eq!(opacity_prop.name, "opacity");
        assert_eq!(opacity_prop.value, "0.8");

        let blend_prop = &properties[2];
        assert_eq!(blend_prop.name, "mix-blend-mode");
        assert_eq!(blend_prop.value, "multiply");
    }

    #[test]
    fn effect_builder_performance() {
        let simple_builder = EffectBuilder::new().opacity("0.5".to_string());
        assert!(!simple_builder.has_performance_concerns());
        assert_eq!(simple_builder.performance_cost(), 1);

        let complex_builder = EffectBuilder::new()
            .blend_mode("multiply".to_string())
            .mask("circle".to_string());
        assert!(complex_builder.has_performance_concerns());
        assert_eq!(complex_builder.performance_cost(), 9); // 4 + 5
    }

    #[test]
    fn effect_presets() {
        let card = presets::card_effect();
        let properties = card.build();
        assert!(properties.len() >= 2); // shadow + opacity

        let glass = presets::glass_morphism();
        let properties = glass.build();
        assert_eq!(properties.len(), 3); // opacity + blend_mode + shadow

        assert!(glass.has_performance_concerns());
    }
}
