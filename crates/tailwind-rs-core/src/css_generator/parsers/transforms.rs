//! Transform Utilities Parser
//!
//! This module provides parsing logic for transform-related Tailwind CSS utilities,
//! including transform origin, scale, rotate, and translate utilities.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct TransformParser;

impl TransformParser {
    pub fn new() -> Self { Self }

    /// Parse basic transform classes
    fn parse_basic_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "transform" => Some(vec![CssProperty { name: "transform".to_string(), value: "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))".to_string(), important: false }]),
            "transform-gpu" => Some(vec![CssProperty { name: "transform".to_string(), value: "translate3d(var(--tw-translate-x), var(--tw-translate-y), 0) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))".to_string(), important: false }]),
            "transform-none" => Some(vec![CssProperty { name: "transform".to_string(), value: "none".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse transform origin classes
    fn parse_origin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "origin-center" => Some(vec![CssProperty { name: "transform-origin".to_string(), value: "center".to_string(), important: false }]),
            "origin-top" => Some(vec![CssProperty { name: "transform-origin".to_string(), value: "top".to_string(), important: false }]),
            "origin-top-right" => Some(vec![CssProperty { name: "transform-origin".to_string(), value: "top right".to_string(), important: false }]),
            "origin-right" => Some(vec![CssProperty { name: "transform-origin".to_string(), value: "right".to_string(), important: false }]),
            "origin-bottom-right" => Some(vec![CssProperty { name: "transform-origin".to_string(), value: "bottom right".to_string(), important: false }]),
            "origin-bottom" => Some(vec![CssProperty { name: "transform-origin".to_string(), value: "bottom".to_string(), important: false }]),
            "origin-bottom-left" => Some(vec![CssProperty { name: "transform-origin".to_string(), value: "bottom left".to_string(), important: false }]),
            "origin-left" => Some(vec![CssProperty { name: "transform-origin".to_string(), value: "left".to_string(), important: false }]),
            "origin-top-left" => Some(vec![CssProperty { name: "transform-origin".to_string(), value: "top left".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse scale classes
    fn parse_scale_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "scale-0" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0)".to_string(), important: false }]),
            "scale-50" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.5)".to_string(), important: false }]),
            "scale-75" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.75)".to_string(), important: false }]),
            "scale-90" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.9)".to_string(), important: false }]),
            "scale-95" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.95)".to_string(), important: false }]),
            "scale-100" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1)".to_string(), important: false }]),
            "scale-105" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.05)".to_string(), important: false }]),
            "scale-110" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.1)".to_string(), important: false }]),
            "scale-125" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.25)".to_string(), important: false }]),
            "scale-150" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.5)".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse rotate classes
    fn parse_rotate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "rotate-0" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(0deg)".to_string(), important: false }]),
            "rotate-1" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(1deg)".to_string(), important: false }]),
            "rotate-2" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(2deg)".to_string(), important: false }]),
            "rotate-3" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(3deg)".to_string(), important: false }]),
            "rotate-6" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(6deg)".to_string(), important: false }]),
            "rotate-12" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(12deg)".to_string(), important: false }]),
            "rotate-45" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(45deg)".to_string(), important: false }]),
            "rotate-90" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(90deg)".to_string(), important: false }]),
            "rotate-180" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(180deg)".to_string(), important: false }]),
            "-rotate-1" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(-1deg)".to_string(), important: false }]),
            "-rotate-2" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(-2deg)".to_string(), important: false }]),
            "-rotate-3" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(-3deg)".to_string(), important: false }]),
            "-rotate-6" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(-6deg)".to_string(), important: false }]),
            "-rotate-12" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(-12deg)".to_string(), important: false }]),
            "-rotate-45" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(-45deg)".to_string(), important: false }]),
            "-rotate-90" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(-90deg)".to_string(), important: false }]),
            "-rotate-180" => Some(vec![CssProperty { name: "transform".to_string(), value: "rotate(-180deg)".to_string(), important: false }]),
            _ => None,
        }
    }
}

impl UtilityParser for TransformParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_basic_transform_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_origin_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_scale_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_rotate_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "transform", "transform-gpu", "transform-none",
            "origin-*", "scale-*", "rotate-*", "-rotate-*"
        ]
    }

    fn get_priority(&self) -> u32 { 80 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Transforms }
}

impl Default for TransformParser {
    fn default() -> Self { Self::new() }
}