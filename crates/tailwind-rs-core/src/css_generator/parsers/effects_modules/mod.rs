//! Effects Parser Module
//!
//! Comprehensive effects utilities for Tailwind-RS:
//! - Box shadows: sm, md, lg, xl, 2xl, inner, none, arbitrary
//! - Opacity: 0-100 in steps, arbitrary values
//! - Mix-blend-mode: all CSS blend modes
//! - Mask utilities: basic mask support

// Re-export all effects components
pub mod types;
pub mod utilities;
pub mod parser;

// Re-export all types and utilities for easy access
pub use types::*;
pub use utilities::*;
pub use parser::*;

/// Main effects parser that implements UtilityParser trait
#[derive(Debug, Clone)]
pub struct EffectsParser;

impl EffectsParser {
    pub fn new() -> Self {
        Self
    }
}

impl super::super::UtilityParser for EffectsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<crate::css_generator::types::CssProperty>> {
        let parser = self::parser::EffectsParser::new();

        // Try shadow classes first
        if let Some(properties) = parser.parse_shadow_class(class) {
            return Some(properties);
        }

        // Try opacity classes
        if let Some(properties) = parser.parse_opacity_class(class) {
            return Some(properties);
        }

        // Try mix-blend-mode classes
        if let Some(properties) = parser.parse_mix_blend_class(class) {
            return Some(properties);
        }

        // Try drop shadow classes
        if let Some(properties) = parser.parse_drop_shadow_class(class) {
            return Some(properties);
        }

        // Try shadow color classes
        if let Some(properties) = parser.parse_shadow_color_class(class) {
            return Some(properties);
        }

        // Try backdrop blur classes
        if let Some(properties) = parser.parse_backdrop_blur_class(class) {
            return Some(properties);
        }

        // Try backdrop opacity classes
        if let Some(properties) = parser.parse_backdrop_opacity_class(class) {
            return Some(properties);
        }

        // Try mask classes
        if let Some(properties) = parser.parse_mask_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "shadow-", "opacity-", "mix-blend-", "mask-",
        ]
    }

    fn get_priority(&self) -> u32 {
        25 // Higher priority than basic utilities but lower than animations
    }

    fn get_category(&self) -> super::super::ParserCategory {
        super::super::ParserCategory::Effects
    }
}

impl Default for EffectsParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Effects registry for managing available effects
#[derive(Debug, Clone)]
pub struct EffectsRegistry {
    available_shadows: Vec<ShadowType>,
    available_opacities: Vec<OpacityLevel>,
    available_blend_modes: Vec<MixBlendMode>,
    enable_masks: bool,
}

impl EffectsRegistry {
    /// Create a new effects registry with all effects enabled
    pub fn new() -> Self {
        Self {
            available_shadows: vec![
                ShadowType::Sm, ShadowType::Default, ShadowType::Md,
                ShadowType::Lg, ShadowType::Xl, ShadowType::Xxl,
                ShadowType::Inner, ShadowType::None,
            ],
            available_opacities: OpacityLevel::all_levels(),
            available_blend_modes: vec![
                MixBlendMode::Normal, MixBlendMode::Multiply, MixBlendMode::Screen,
                MixBlendMode::Overlay, MixBlendMode::Darken, MixBlendMode::Lighten,
                MixBlendMode::ColorDodge, MixBlendMode::ColorBurn, MixBlendMode::HardLight,
                MixBlendMode::SoftLight, MixBlendMode::Difference, MixBlendMode::Exclusion,
                MixBlendMode::Hue, MixBlendMode::Saturation, MixBlendMode::Color, MixBlendMode::Luminosity,
            ],
            enable_masks: true,
        }
    }

    /// Check if a shadow type is available
    pub fn is_shadow_available(&self, shadow: &ShadowType) -> bool {
        self.available_shadows.contains(shadow)
    }

    /// Check if an opacity level is available
    pub fn is_opacity_available(&self, opacity: &OpacityLevel) -> bool {
        self.available_opacities.contains(opacity)
    }

    /// Check if a blend mode is available
    pub fn is_blend_mode_available(&self, blend_mode: &MixBlendMode) -> bool {
        self.available_blend_modes.contains(blend_mode)
    }

    /// Check if masks are enabled
    pub fn masks_enabled(&self) -> bool {
        self.enable_masks
    }

    /// Disable a specific shadow type
    pub fn disable_shadow(&mut self, shadow: ShadowType) -> &mut Self {
        self.available_shadows.retain(|s| s != &shadow);
        self
    }

    /// Disable masks
    pub fn disable_masks(&mut self, disabled: bool) -> &mut Self {
        self.enable_masks = !disabled;
        self
    }

    /// Get all available shadows
    pub fn available_shadows(&self) -> &[ShadowType] {
        &self.available_shadows
    }

    /// Get all available opacities
    pub fn available_opacities(&self) -> &[OpacityLevel] {
        &self.available_opacities
    }

    /// Get all available blend modes
    pub fn available_blend_modes(&self) -> &[MixBlendMode] {
        &self.available_blend_modes
    }
}

impl Default for EffectsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Accessibility utilities for effects
pub mod accessibility {
    use super::*;

    /// Check if an effect respects user's motion preferences
    pub fn respects_motion_preferences(effect_type: &EffectType) -> bool {
        // Effects that don't involve motion/animation are fine
        !matches!(effect_type, EffectType::Shadow | EffectType::BlendMode | EffectType::Mask)
    }

    /// Check if effect is essential for functionality
    pub fn is_essential_for_functionality(class: &str) -> bool {
        // Opacity changes for interactive states might be essential
        class.contains("opacity-") && (class.contains("hover") || class.contains("focus") || class.contains("active"))
    }

    /// Suggest reduced-motion alternatives
    pub fn suggest_reduced_motion_alternative(effect_type: &EffectType) -> Option<String> {
        match effect_type {
            EffectType::Shadow => Some("Consider using border instead of shadow for reduced motion".to_string()),
            EffectType::BlendMode => Some("Consider using opacity instead of blend modes".to_string()),
            EffectType::Mask => Some("Consider using clip-path instead of mask".to_string()),
            EffectType::Opacity => None, // Opacity is usually fine
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_generator::parsers::UtilityParser;

    #[test]
    fn effects_parser_trait() {
        let parser = EffectsParser::new();

        // Test supported patterns
        let patterns = parser.get_supported_patterns();
        assert!(patterns.contains(&"shadow-"));
        assert!(patterns.contains(&"opacity-"));
        assert!(patterns.contains(&"mix-blend-"));
        assert!(patterns.contains(&"mask-"));

        // Test priority
        assert_eq!(parser.get_priority(), 25);

        // Test category
        assert!(matches!(parser.get_category(), super::super::ParserCategory::Effects));
    }

    #[test]
    fn effects_registry_operations() {
        let registry = EffectsRegistry::new();

        assert!(registry.is_shadow_available(&ShadowType::Md));
        assert!(registry.is_opacity_available(&OpacityLevel::Fifty));
        assert!(registry.is_blend_mode_available(&MixBlendMode::Multiply));
        assert!(registry.masks_enabled());

        let mut modified_registry = registry.clone();
        modified_registry.disable_shadow(ShadowType::Md);
        assert!(!modified_registry.is_shadow_available(&ShadowType::Md));

        let mut no_masks_registry = registry.clone();
        no_masks_registry.disable_masks(true);
        assert!(!no_masks_registry.masks_enabled());
    }

    #[test]
    fn effects_registry_defaults() {
        let registry = EffectsRegistry::default();

        assert!(!registry.available_shadows().is_empty());
        assert!(!registry.available_opacities().is_empty());
        assert!(!registry.available_blend_modes().is_empty());
        assert!(registry.masks_enabled());
    }

    #[test]
    fn accessibility_utilities() {
        assert!(accessibility::respects_motion_preferences(&EffectType::Opacity));
        assert!(!accessibility::respects_motion_preferences(&EffectType::Shadow));

        assert!(accessibility::is_essential_for_functionality("hover:opacity-50"));
        assert!(!accessibility::is_essential_for_functionality("opacity-50"));

        assert!(accessibility::suggest_reduced_motion_alternative(&EffectType::Shadow).is_some());
        assert!(accessibility::suggest_reduced_motion_alternative(&EffectType::Opacity).is_none());
    }
}
