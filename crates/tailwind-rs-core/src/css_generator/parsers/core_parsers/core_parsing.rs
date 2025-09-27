//! Core parsing logic for CSS generation
//!
//! This module contains the main parsing logic and core functionality
//! for converting Tailwind classes to CSS properties.

use super::super::types::CssProperty;
use crate::error::{Result, TailwindError};

/// Core parsing functionality for CSS generator
pub trait CoreParsing {
    /// Convert a class name to CSS properties using core parsers
    fn parse_core_properties(&self, base_class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse advanced color properties
    fn parse_advanced_color(&self, base_class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse advanced spacing properties
    fn parse_advanced_spacing(&self, base_class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse typography properties
    fn parse_typography(&self, base_class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse positioning properties
    fn parse_positioning(&self, base_class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse flexbox properties
    fn parse_flexbox(&self, base_class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse sizing properties
    fn parse_sizing(&self, base_class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse border properties
    fn parse_border(&self, base_class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse effects properties
    fn parse_effects(&self, base_class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse transform properties
    fn parse_transforms(&self, base_class: &str) -> Option<Vec<CssProperty>>;
}

impl CoreParsing for super::super::super::CssGenerator {
    fn parse_core_properties(&self, base_class: &str) -> Option<Vec<CssProperty>> {
        // Try advanced parsers first (higher priority)
        if let Some(properties) = self.parse_advanced_color(base_class) {
            return Some(properties);
        }

        if let Some(properties) = self.parse_advanced_spacing(base_class) {
            return Some(properties);
        }

        if let Some(properties) = self.parse_typography(base_class) {
            return Some(properties);
        }

        if let Some(properties) = self.parse_positioning(base_class) {
            return Some(properties);
        }

        if let Some(properties) = self.parse_flexbox(base_class) {
            return Some(properties);
        }

        if let Some(properties) = self.parse_sizing(base_class) {
            return Some(properties);
        }

        if let Some(properties) = self.parse_border(base_class) {
            return Some(properties);
        }

        if let Some(properties) = self.parse_effects(base_class) {
            return Some(properties);
        }

        if let Some(properties) = self.parse_transforms(base_class) {
            return Some(properties);
        }

        None
    }

    fn parse_advanced_color(&self, base_class: &str) -> Option<Vec<CssProperty>> {
        self.advanced_color_parser.parse_class(base_class)
    }

    fn parse_advanced_spacing(&self, base_class: &str) -> Option<Vec<CssProperty>> {
        self.advanced_spacing_parser.parse_class(base_class)
    }

    fn parse_typography(&self, base_class: &str) -> Option<Vec<CssProperty>> {
        self.typography_parser.parse_class(base_class)
    }

    fn parse_positioning(&self, base_class: &str) -> Option<Vec<CssProperty>> {
        self.positioning_parser.parse_class(base_class)
    }

    fn parse_flexbox(&self, base_class: &str) -> Option<Vec<CssProperty>> {
        self.flexbox_parser.parse_class(base_class)
    }

    fn parse_sizing(&self, base_class: &str) -> Option<Vec<CssProperty>> {
        self.sizing_parser.parse_class(base_class)
    }

    fn parse_border(&self, base_class: &str) -> Option<Vec<CssProperty>> {
        self.advanced_border_parser.parse_class(base_class)
    }

    fn parse_effects(&self, base_class: &str) -> Option<Vec<CssProperty>> {
        self.ring_parser.parse_class(base_class)
            .or_else(|| self.transition_parser.parse_class(base_class))
            .or_else(|| self.shadow_parser.parse_class(base_class))
    }

    fn parse_transforms(&self, base_class: &str) -> Option<Vec<CssProperty>> {
        self.basic_transforms_parser.parse_class(base_class)
            .or_else(|| self.scale_parser.parse_class(base_class))
            .or_else(|| self.transform_parser.parse_class(base_class))
    }
}
