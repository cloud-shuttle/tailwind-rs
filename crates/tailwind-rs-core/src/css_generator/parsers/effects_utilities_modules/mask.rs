//! Mask Utilities Module
//!
//! Handles parsing of mask utilities:
//! - mask-clip-* (mask clipping)
//! - mask-* (mask compositing)

use crate::css_generator::types::CssProperty;

/// Mask utilities parser
#[derive(Debug, Clone)]
pub struct MaskParser;

impl MaskParser {
    /// Create a new mask parser
    pub fn new() -> Self {
        Self
    }

    /// Parse mask-clip classes
    pub fn parse_mask_clip_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let clip_value = match class {
            "mask-clip-border" => "border-box",
            "mask-clip-padding" => "padding-box",
            "mask-clip-content" => "content-box",
            "mask-clip-fill" => "fill-box",
            "mask-clip-stroke" => "stroke-box",
            "mask-clip-view" => "view-box",
            "mask-no-clip" => "no-clip",
            _ => return None,
        };

        Some(vec![CssProperty {
            name: "mask-clip".to_string(),
            value: clip_value.to_string(),
            important: false,
        }])
    }

    /// Parse mask-composite classes
    pub fn parse_mask_composite_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let composite_value = match class {
            "mask-add" => "add",
            "mask-subtract" => "subtract",
            "mask-intersect" => "intersect",
            "mask-exclude" => "exclude",
            _ => return None,
        };

        Some(vec![CssProperty {
            name: "mask-composite".to_string(),
            value: composite_value.to_string(),
            important: false,
        }])
    }

    /// Get supported mask patterns
    pub fn supported_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();

        // Mask clip patterns
        let clip_values = [
            "border", "padding", "content", "fill", "stroke", "view"
        ];
        for value in &clip_values {
            patterns.push(format!("mask-clip-{}", value));
        }
        patterns.push("mask-no-clip".to_string());

        // Mask composite patterns
        let composite_values = ["add", "subtract", "intersect", "exclude"];
        for value in &composite_values {
            patterns.push(format!("mask-{}", value));
        }

        patterns
    }
}

impl Default for MaskParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_clip_parsing() {
        let parser = MaskParser::new();

        let result = parser.parse_mask_clip_class("mask-clip-border");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "mask-clip");
        assert_eq!(properties[0].value, "border-box");

        let result = parser.parse_mask_clip_class("mask-no-clip");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "no-clip");
    }

    #[test]
    fn mask_composite_parsing() {
        let parser = MaskParser::new();

        let result = parser.parse_mask_composite_class("mask-add");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "mask-composite");
        assert_eq!(properties[0].value, "add");

        let result = parser.parse_mask_composite_class("mask-subtract");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "subtract");
    }

    #[test]
    fn invalid_mask_classes() {
        let parser = MaskParser::new();

        assert!(parser.parse_mask_clip_class("mask-clip-invalid").is_none());
        assert!(parser.parse_mask_composite_class("mask-invalid").is_none());
    }

    #[test]
    fn supported_patterns_includes_all_mask_types() {
        let parser = MaskParser::new();
        let patterns = parser.supported_patterns();

        // Should include mask clip patterns
        assert!(patterns.contains(&"mask-clip-border".to_string()));
        assert!(patterns.contains(&"mask-no-clip".to_string()));

        // Should include mask composite patterns
        assert!(patterns.contains(&"mask-add".to_string()));
        assert!(patterns.contains(&"mask-subtract".to_string()));

        // Should have 10 patterns total (6 clip + 4 composite)
        assert_eq!(patterns.len(), 10);
    }
}
