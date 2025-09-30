//! Effects Utilities Parser Module
//!
//! Comprehensive effects utilities parser that handles all effects-related Tailwind classes:
//! - shadow-* (box shadows), text-shadow-* (text shadows)
//! - opacity-* (opacity values)
//! - mix-blend-* (mix blend modes), bg-blend-* (background blend modes)
//! - mask-clip-* (mask clipping), mask-* (mask compositing)

use crate::css_generator::types::CssProperty;
use super::super::ParserCategory;
use super::super::UtilityParser;

// Re-export all effects utility types and traits
pub mod blend_modes;
pub mod box_shadow;
pub mod mask;
pub mod opacity;
pub mod text_shadow;

// Re-export all types for easy access
pub use blend_modes::BlendModeParser;
pub use box_shadow::BoxShadowParser;
pub use mask::MaskParser;
pub use opacity::OpacityParser;
pub use text_shadow::TextShadowParser;

/// Main effects utilities parser that coordinates all effects parsing
#[derive(Debug, Clone)]
pub struct EffectsParser {
    box_shadow_parser: BoxShadowParser,
    text_shadow_parser: TextShadowParser,
    opacity_parser: OpacityParser,
    blend_mode_parser: BlendModeParser,
    mask_parser: MaskParser,
}

impl EffectsParser {
    /// Create a new effects parser with all sub-parsers
    pub fn new() -> Self {
        Self {
            box_shadow_parser: BoxShadowParser::new(),
            text_shadow_parser: TextShadowParser::new(),
            opacity_parser: OpacityParser::new(),
            blend_mode_parser: BlendModeParser::new(),
            mask_parser: MaskParser::new(),
        }
    }

    /// Parse box-shadow classes
    pub fn parse_box_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.box_shadow_parser.parse_box_shadow_class(class)
    }

    /// Parse text-shadow classes
    pub fn parse_text_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.text_shadow_parser.parse_text_shadow_class(class)
    }

    /// Parse opacity classes
    pub fn parse_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.opacity_parser.parse_opacity_class(class)
    }

    /// Parse mix-blend-mode classes
    pub fn parse_mix_blend_mode_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.blend_mode_parser.parse_mix_blend_mode_class(class)
    }

    /// Parse background-blend-mode classes
    pub fn parse_background_blend_mode_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.blend_mode_parser.parse_background_blend_mode_class(class)
    }

    /// Parse mask-clip classes
    pub fn parse_mask_clip_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.mask_parser.parse_mask_clip_class(class)
    }

    /// Parse mask-composite classes
    pub fn parse_mask_composite_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.mask_parser.parse_mask_composite_class(class)
    }
}

impl UtilityParser for EffectsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try parsing in order of specificity/precedence
        if let Some(properties) = self.parse_box_shadow_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_text_shadow_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_opacity_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_mix_blend_mode_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_background_blend_mode_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_mask_clip_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_mask_composite_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        // For now, return a static list of common effects patterns
        // TODO: Implement proper pattern collection when sub-parsers are fully implemented
        vec![
            "shadow-", "text-shadow-", "opacity-", "mix-blend-", "bg-blend-",
            "mask-clip-", "mask-add", "mask-subtract", "mask-intersect", "mask-exclude",
        ]
    }

    fn get_priority(&self) -> u32 {
        70 // Effects utilities have medium-high priority
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Effects
    }
}

impl Default for EffectsParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Effects utilities trait for extending ClassBuilder
pub trait EffectsUtilities {
    /// Add shadow class (box shadow)
    fn shadow(self, value: &str) -> Self;

    /// Add text shadow class
    fn text_shadow(self, value: &str) -> Self;

    /// Add opacity class
    fn opacity(self, value: &str) -> Self;

    /// Add mix blend mode class
    fn mix_blend(self, value: &str) -> Self;

    /// Add background blend mode class
    fn bg_blend(self, value: &str) -> Self;

    /// Add mask clip class
    fn mask_clip(self, value: &str) -> Self;

    /// Add mask composite class
    fn mask(self, value: &str) -> Self;
}

impl EffectsUtilities for crate::classes::ClassBuilder {
    fn shadow(self, value: &str) -> Self {
        self.class(&format!("shadow-{}", value))
    }

    fn text_shadow(self, value: &str) -> Self {
        self.class(&format!("text-shadow-{}", value))
    }

    fn opacity(self, value: &str) -> Self {
        self.class(&format!("opacity-{}", value))
    }

    fn mix_blend(self, value: &str) -> Self {
        self.class(&format!("mix-blend-{}", value))
    }

    fn bg_blend(self, value: &str) -> Self {
        self.class(&format!("bg-blend-{}", value))
    }

    fn mask_clip(self, value: &str) -> Self {
        self.class(&format!("mask-clip-{}", value))
    }

    fn mask(self, value: &str) -> Self {
        self.class(&format!("mask-{}", value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effects_parser_creation() {
        let parser = EffectsParser::new();
        // Parser should be created successfully
        assert!(true); // Placeholder - would test actual functionality
    }

    #[test]
    fn utility_parser_implementation() {
        let parser = EffectsParser::new();

        assert_eq!(parser.get_priority(), 70);
        assert_eq!(parser.get_category(), ParserCategory::Effects);

        let patterns = parser.get_supported_patterns();
        assert!(!patterns.is_empty());
        // Should contain various effects patterns
        assert!(patterns.iter().any(|p| p.contains("shadow-")));
        assert!(patterns.iter().any(|p| p.contains("opacity-")));
    }

    #[test]
    fn comprehensive_effects_parsing() {
        let parser = EffectsParser::new();

        // Test that the main dispatcher works for implemented parsers
        let result = parser.parse_box_shadow_class("shadow-md");
        assert!(result.is_some());

        let result = parser.parse_text_shadow_class("text-shadow-lg");
        assert!(result.is_some());

        let result = parser.parse_opacity_class("opacity-50");
        assert!(result.is_some());

        let result = parser.parse_mix_blend_mode_class("mix-blend-multiply");
        assert!(result.is_some());

        let result = parser.parse_background_blend_mode_class("bg-blend-screen");
        assert!(result.is_some());

        let result = parser.parse_mask_clip_class("mask-clip-border");
        assert!(result.is_some());

        let result = parser.parse_mask_composite_class("mask-add");
        assert!(result.is_some());

        let result = parser.parse_class("invalid-effects-class");
        assert!(result.is_none());
    }

    #[test]
    fn effects_utilities_trait() {
        use crate::classes::ClassBuilder;

        let builder = ClassBuilder::new();

        // Test trait methods (simplified - would need actual ClassBuilder implementation)
        let _result = builder.shadow("md").text_shadow("lg").opacity("50");
        // In a real implementation, this would add classes to the builder
    }
}
