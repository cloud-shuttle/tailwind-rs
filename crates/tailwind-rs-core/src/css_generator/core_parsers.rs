//! Core Parser Methods for CssGenerator
//!
//! This module contains the core parsing methods for spacing, animation, color, typography, layout, borders, effects, and transforms.

// Removed unused imports
use super::parsers::{
    BorderParser, ColorParser, EffectsParser, InteractiveParser, LayoutParser,
    TypographyParser,
};
use crate::transforms::TransformParser;
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
        // Delegate to class_processor in the new architecture
        self.class_processor.process_class(class, &mut self.clone()).ok()
    }

    fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Delegate to class_processor in the new architecture
        self.class_processor.process_class(class, &mut self.clone()).ok()
    }

    fn parse_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = ColorParser::new();
        parser.parse_class(class)
    }

    fn parse_typography_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = TypographyParser::new();
        parser.parse_class(class)
    }

    fn parse_layout_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = LayoutParser::new();
        parser.parse_class(class)
    }

    fn parse_border_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = BorderParser::new();
        parser.parse_class(class)
    }

    fn parse_effects_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = EffectsParser::new();
        parser.parse_class(class)
    }

    fn parse_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = TransformParser::new();
        parser.parse_class(class)
    }

    fn parse_interactive_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let parser = InteractiveParser::new();
        parser.parse_class(class)
    }
}
