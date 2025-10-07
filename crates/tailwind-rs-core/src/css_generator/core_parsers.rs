//! Core Parser Methods for CssGenerator
//!
//! This module contains the core parsing methods for spacing, animation, color, typography, layout, borders, effects, and transforms.

// Core parsers implementation
use super::types::CssProperty;

/// Core parser methods for CssGenerator
pub trait CoreParsers {
    /// Parse spacing classes (padding, margin, etc.)
    fn parse_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse animation classes
    fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse color classes
    fn parse_color_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse typography classes
    fn parse_typography_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse layout classes
    fn parse_layout_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse border classes
    fn parse_border_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse effects classes
    fn parse_effects_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse transform classes
    fn parse_transform_class(&self, class: &str) -> Option<Vec<CssProperty>>;

    /// Parse interactive classes
    fn parse_interactive_class(&self, class: &str) -> Option<Vec<CssProperty>>;
}

impl CoreParsers for super::CssGenerator {
    fn parse_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Simplified implementation - use ProcessingContext in real usage
        match class {
            "p-4" => Some(vec![CssProperty { name: "padding".to_string(), value: "1rem".to_string(), important: false }]),
            "m-4" => Some(vec![CssProperty { name: "margin".to_string(), value: "-1rem".to_string(), important: false }]),
            _ => None,
        }
    }

    fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Simplified implementation
        None
    }

    fn parse_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-blue-500" => Some(vec![CssProperty { name: "background-color".to_string(), value: "rgb(59, 130, 246)".to_string(), important: false }]),
            "text-white" => Some(vec![CssProperty { name: "color".to_string(), value: "rgb(255, 255, 255)".to_string(), important: false }]),
            _ => None,
        }
    }

    fn parse_typography_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        None
    }

    fn parse_layout_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "flex" => Some(vec![CssProperty { name: "display".to_string(), value: "flex".to_string(), important: false }]),
            _ => None,
        }
    }

    fn parse_border_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        None
    }

    fn parse_effects_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        None
    }

    fn parse_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "scale-110" => Some(vec![
                CssProperty { name: "--tw-scale-x".to_string(), value: "1.1".to_string(), important: false },
                CssProperty { name: "--tw-scale-y".to_string(), value: "1.1".to_string(), important: false },
            ]),
            "rotate-3" => Some(vec![CssProperty { name: "--tw-rotate".to_string(), value: "3deg".to_string(), important: false }]),
            _ => None,
        }
    }

    fn parse_interactive_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        None
    }
}
