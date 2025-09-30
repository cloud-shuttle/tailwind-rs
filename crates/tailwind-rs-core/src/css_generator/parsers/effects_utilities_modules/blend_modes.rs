//! Blend Mode Utilities Module
//!
//! Handles parsing of blend mode utilities:
//! - mix-blend-* (mix-blend-mode)
//! - bg-blend-* (background-blend-mode)

use crate::css_generator::types::CssProperty;

/// Blend mode utilities parser
#[derive(Debug, Clone)]
pub struct BlendModeParser;

impl BlendModeParser {
    /// Create a new blend mode parser
    pub fn new() -> Self {
        Self
    }

    /// Parse mix-blend-mode classes
    pub fn parse_mix_blend_mode_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let blend_mode = match class {
            "mix-blend-normal" => "normal",
            "mix-blend-multiply" => "multiply",
            "mix-blend-screen" => "screen",
            "mix-blend-overlay" => "overlay",
            "mix-blend-darken" => "darken",
            "mix-blend-lighten" => "lighten",
            "mix-blend-color-dodge" => "color-dodge",
            "mix-blend-color-burn" => "color-burn",
            "mix-blend-hard-light" => "hard-light",
            "mix-blend-soft-light" => "soft-light",
            "mix-blend-difference" => "difference",
            "mix-blend-exclusion" => "exclusion",
            "mix-blend-hue" => "hue",
            "mix-blend-saturation" => "saturation",
            "mix-blend-color" => "color",
            "mix-blend-luminosity" => "luminosity",
            _ => return None,
        };

        Some(vec![CssProperty {
            name: "mix-blend-mode".to_string(),
            value: blend_mode.to_string(),
            important: false,
        }])
    }

    /// Parse background-blend-mode classes
    pub fn parse_background_blend_mode_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let blend_mode = match class {
            "bg-blend-normal" => "normal",
            "bg-blend-multiply" => "multiply",
            "bg-blend-screen" => "screen",
            "bg-blend-overlay" => "overlay",
            "bg-blend-darken" => "darken",
            "bg-blend-lighten" => "lighten",
            "bg-blend-color-dodge" => "color-dodge",
            "bg-blend-color-burn" => "color-burn",
            "bg-blend-hard-light" => "hard-light",
            "bg-blend-soft-light" => "soft-light",
            "bg-blend-difference" => "difference",
            "bg-blend-exclusion" => "exclusion",
            "bg-blend-hue" => "hue",
            "bg-blend-saturation" => "saturation",
            "bg-blend-color" => "color",
            "bg-blend-luminosity" => "luminosity",
            _ => return None,
        };

        Some(vec![CssProperty {
            name: "background-blend-mode".to_string(),
            value: blend_mode.to_string(),
            important: false,
        }])
    }

    /// Get supported blend mode patterns
    pub fn supported_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();

        let blend_modes = [
            "normal", "multiply", "screen", "overlay", "darken", "lighten",
            "color-dodge", "color-burn", "hard-light", "soft-light",
            "difference", "exclusion", "hue", "saturation", "color", "luminosity"
        ];

        // Mix blend modes
        for mode in &blend_modes {
            patterns.push(format!("mix-blend-{}", mode));
        }

        // Background blend modes
        for mode in &blend_modes {
            patterns.push(format!("bg-blend-{}", mode));
        }

        patterns
    }
}

impl Default for BlendModeParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_blend_mode_parsing() {
        let parser = BlendModeParser::new();

        let result = parser.parse_mix_blend_mode_class("mix-blend-multiply");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "mix-blend-mode");
        assert_eq!(properties[0].value, "multiply");

        let result = parser.parse_mix_blend_mode_class("mix-blend-normal");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "normal");
    }

    #[test]
    fn background_blend_mode_parsing() {
        let parser = BlendModeParser::new();

        let result = parser.parse_background_blend_mode_class("bg-blend-screen");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "background-blend-mode");
        assert_eq!(properties[0].value, "screen");

        let result = parser.parse_background_blend_mode_class("bg-blend-overlay");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "overlay");
    }

    #[test]
    fn invalid_blend_mode_classes() {
        let parser = BlendModeParser::new();

        assert!(parser.parse_mix_blend_mode_class("mix-blend-invalid").is_none());
        assert!(parser.parse_background_blend_mode_class("bg-blend-invalid").is_none());
    }

    #[test]
    fn supported_patterns_includes_all_blend_modes() {
        let parser = BlendModeParser::new();
        let patterns = parser.supported_patterns();

        // Should include both mix-blend and bg-blend patterns
        assert!(patterns.contains(&"mix-blend-normal".to_string()));
        assert!(patterns.contains(&"mix-blend-multiply".to_string()));
        assert!(patterns.contains(&"bg-blend-normal".to_string()));
        assert!(patterns.contains(&"bg-blend-screen".to_string()));

        // Should have 32 patterns total (16 mix + 16 bg)
        assert_eq!(patterns.len(), 32);
    }
}
