//! Transform Utilities Parser
//!
//! This module provides parsing logic for transform-related Tailwind CSS utilities,
//! including transform origin, scale, rotate, and translate utilities.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct TransformParser;

impl TransformParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse basic transform classes
    fn parse_basic_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "transform" => Some(vec![CssProperty { name: "transform".to_string(), value: "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))".to_string(), important: false }]),
            "transform-gpu" => Some(vec![CssProperty { name: "transform".to_string(), value: "translate3d(var(--tw-translate-x), var(--tw-translate-y), 0) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))".to_string(), important: false }]),
            "transform-cpu" => Some(vec![CssProperty { name: "transform".to_string(), value: "var(--tw-rotate-x) var(--tw-rotate-y) var(--tw-rotate-z) var(--tw-skew-x) var(--tw-skew-y)".to_string(), important: false }]),
            "transform-none" => Some(vec![CssProperty { name: "transform".to_string(), value: "none".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse backface-visibility classes
    fn parse_backface_visibility_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "backface-hidden" => Some(vec![CssProperty {
                name: "backface-visibility".to_string(),
                value: "hidden".to_string(),
                important: false,
            }]),
            "backface-visible" => Some(vec![CssProperty {
                name: "backface-visibility".to_string(),
                value: "visible".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse perspective classes
    fn parse_perspective_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "perspective-dramatic" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: "var(--perspective-dramatic)".to_string(),
                important: false,
            }]),
            "perspective-near" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: "var(--perspective-near)".to_string(),
                important: false,
            }]),
            "perspective-normal" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: "var(--perspective-normal)".to_string(),
                important: false,
            }]),
            "perspective-midrange" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: "var(--perspective-midrange)".to_string(),
                important: false,
            }]),
            "perspective-distant" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: "var(--perspective-distant)".to_string(),
                important: false,
            }]),
            "perspective-none" => Some(vec![CssProperty {
                name: "perspective".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse perspective-origin classes
    fn parse_perspective_origin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "perspective-origin-center" => Some(vec![CssProperty {
                name: "perspective-origin".to_string(),
                value: "center".to_string(),
                important: false,
            }]),
            "perspective-origin-top" => Some(vec![CssProperty {
                name: "perspective-origin".to_string(),
                value: "top".to_string(),
                important: false,
            }]),
            "perspective-origin-top-right" => Some(vec![CssProperty {
                name: "perspective-origin".to_string(),
                value: "top right".to_string(),
                important: false,
            }]),
            "perspective-origin-right" => Some(vec![CssProperty {
                name: "perspective-origin".to_string(),
                value: "right".to_string(),
                important: false,
            }]),
            "perspective-origin-bottom-right" => Some(vec![CssProperty {
                name: "perspective-origin".to_string(),
                value: "bottom right".to_string(),
                important: false,
            }]),
            "perspective-origin-bottom" => Some(vec![CssProperty {
                name: "perspective-origin".to_string(),
                value: "bottom".to_string(),
                important: false,
            }]),
            "perspective-origin-bottom-left" => Some(vec![CssProperty {
                name: "perspective-origin".to_string(),
                value: "bottom left".to_string(),
                important: false,
            }]),
            "perspective-origin-left" => Some(vec![CssProperty {
                name: "perspective-origin".to_string(),
                value: "left".to_string(),
                important: false,
            }]),
            "perspective-origin-top-left" => Some(vec![CssProperty {
                name: "perspective-origin".to_string(),
                value: "top left".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse transform-style classes
    fn parse_transform_style_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "transform-3d" => Some(vec![CssProperty {
                name: "transform-style".to_string(),
                value: "preserve-3d".to_string(),
                important: false,
            }]),
            "transform-flat" => Some(vec![CssProperty {
                name: "transform-style".to_string(),
                value: "flat".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse transform origin classes
    fn parse_origin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "origin-center" => Some(vec![CssProperty {
                name: "transform-origin".to_string(),
                value: "center".to_string(),
                important: false,
            }]),
            "origin-top" => Some(vec![CssProperty {
                name: "transform-origin".to_string(),
                value: "top".to_string(),
                important: false,
            }]),
            "origin-top-right" => Some(vec![CssProperty {
                name: "transform-origin".to_string(),
                value: "top right".to_string(),
                important: false,
            }]),
            "origin-right" => Some(vec![CssProperty {
                name: "transform-origin".to_string(),
                value: "right".to_string(),
                important: false,
            }]),
            "origin-bottom-right" => Some(vec![CssProperty {
                name: "transform-origin".to_string(),
                value: "bottom right".to_string(),
                important: false,
            }]),
            "origin-bottom" => Some(vec![CssProperty {
                name: "transform-origin".to_string(),
                value: "bottom".to_string(),
                important: false,
            }]),
            "origin-bottom-left" => Some(vec![CssProperty {
                name: "transform-origin".to_string(),
                value: "bottom left".to_string(),
                important: false,
            }]),
            "origin-left" => Some(vec![CssProperty {
                name: "transform-origin".to_string(),
                value: "left".to_string(),
                important: false,
            }]),
            "origin-top-left" => Some(vec![CssProperty {
                name: "transform-origin".to_string(),
                value: "top left".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse scale classes
    fn parse_scale_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "scale-0" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(0)".to_string(),
                important: false,
            }]),
            "scale-50" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(0.5)".to_string(),
                important: false,
            }]),
            "scale-75" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(0.75)".to_string(),
                important: false,
            }]),
            "scale-90" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(0.9)".to_string(),
                important: false,
            }]),
            "scale-95" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(0.95)".to_string(),
                important: false,
            }]),
            "scale-100" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(1)".to_string(),
                important: false,
            }]),
            "scale-105" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(1.05)".to_string(),
                important: false,
            }]),
            "scale-110" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(1.1)".to_string(),
                important: false,
            }]),
            "scale-125" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(1.25)".to_string(),
                important: false,
            }]),
            "scale-150" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(1.5)".to_string(),
                important: false,
            }]),
            "-scale-75" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(-0.75)".to_string(),
                important: false,
            }]),
            "-scale-125" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "scale(-1.25)".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse rotate classes
    fn parse_rotate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "rotate-none" => Some(vec![CssProperty {
                name: "rotate".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            "rotate-0" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(0deg)".to_string(),
                important: false,
            }]),
            "rotate-1" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(1deg)".to_string(),
                important: false,
            }]),
            "rotate-2" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(2deg)".to_string(),
                important: false,
            }]),
            "rotate-3" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(3deg)".to_string(),
                important: false,
            }]),
            "rotate-6" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(6deg)".to_string(),
                important: false,
            }]),
            "rotate-12" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(12deg)".to_string(),
                important: false,
            }]),
            "rotate-45" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(45deg)".to_string(),
                important: false,
            }]),
            "rotate-90" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(90deg)".to_string(),
                important: false,
            }]),
            "rotate-180" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(180deg)".to_string(),
                important: false,
            }]),
            "-rotate-1" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(-1deg)".to_string(),
                important: false,
            }]),
            "-rotate-2" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(-2deg)".to_string(),
                important: false,
            }]),
            "-rotate-3" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(-3deg)".to_string(),
                important: false,
            }]),
            "-rotate-6" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(-6deg)".to_string(),
                important: false,
            }]),
            "-rotate-12" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(-12deg)".to_string(),
                important: false,
            }]),
            "-rotate-45" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(-45deg)".to_string(),
                important: false,
            }]),
            "-rotate-90" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(-90deg)".to_string(),
                important: false,
            }]),
            "-rotate-180" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "rotate(-180deg)".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse skew classes
    fn parse_skew_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "skew-0" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(0deg) skewY(0deg)".to_string(),
                important: false,
            }]),
            "skew-1" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(1deg) skewY(1deg)".to_string(),
                important: false,
            }]),
            "skew-2" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(2deg) skewY(2deg)".to_string(),
                important: false,
            }]),
            "skew-3" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(3deg) skewY(3deg)".to_string(),
                important: false,
            }]),
            "skew-6" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(6deg) skewY(6deg)".to_string(),
                important: false,
            }]),
            "skew-12" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(12deg) skewY(12deg)".to_string(),
                important: false,
            }]),
            "-skew-1" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(-1deg) skewY(-1deg)".to_string(),
                important: false,
            }]),
            "-skew-2" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(-2deg) skewY(-2deg)".to_string(),
                important: false,
            }]),
            "-skew-3" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(-3deg) skewY(-3deg)".to_string(),
                important: false,
            }]),
            "-skew-6" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(-6deg) skewY(-6deg)".to_string(),
                important: false,
            }]),
            "-skew-12" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(-12deg) skewY(-12deg)".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse skew-x classes
    fn parse_skew_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "skew-x-0" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(0deg)".to_string(),
                important: false,
            }]),
            "skew-x-1" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(1deg)".to_string(),
                important: false,
            }]),
            "skew-x-2" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(2deg)".to_string(),
                important: false,
            }]),
            "skew-x-3" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(3deg)".to_string(),
                important: false,
            }]),
            "skew-x-6" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(6deg)".to_string(),
                important: false,
            }]),
            "skew-x-12" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(12deg)".to_string(),
                important: false,
            }]),
            "-skew-x-1" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(-1deg)".to_string(),
                important: false,
            }]),
            "-skew-x-2" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(-2deg)".to_string(),
                important: false,
            }]),
            "-skew-x-3" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(-3deg)".to_string(),
                important: false,
            }]),
            "-skew-x-6" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(-6deg)".to_string(),
                important: false,
            }]),
            "-skew-x-12" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewX(-12deg)".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse skew-y classes
    fn parse_skew_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "skew-y-0" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(0deg)".to_string(),
                important: false,
            }]),
            "skew-y-1" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(1deg)".to_string(),
                important: false,
            }]),
            "skew-y-2" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(2deg)".to_string(),
                important: false,
            }]),
            "skew-y-3" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(3deg)".to_string(),
                important: false,
            }]),
            "skew-y-6" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(6deg)".to_string(),
                important: false,
            }]),
            "skew-y-12" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(12deg)".to_string(),
                important: false,
            }]),
            "-skew-y-1" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(-1deg)".to_string(),
                important: false,
            }]),
            "-skew-y-2" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(-2deg)".to_string(),
                important: false,
            }]),
            "-skew-y-3" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(-3deg)".to_string(),
                important: false,
            }]),
            "-skew-y-6" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(-6deg)".to_string(),
                important: false,
            }]),
            "-skew-y-12" => Some(vec![CssProperty {
                name: "transform".to_string(),
                value: "skewY(-12deg)".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }
}

impl UtilityParser for TransformParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_basic_transform_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_backface_visibility_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_perspective_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_perspective_origin_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_transform_style_class(class) {
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
        if let Some(properties) = self.parse_skew_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_skew_x_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_skew_y_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "transform",
            "transform-gpu",
            "transform-cpu",
            "transform-none",
            "backface-*",
            "perspective-*",
            "perspective-origin-*",
            "transform-*",
            "origin-*",
            "scale-*",
            "rotate-*",
            "-rotate-*",
            "skew-*",
            "-skew-*",
            "skew-x-*",
            "-skew-x-*",
            "skew-y-*",
            "-skew-y-*",
        ]
    }

    fn get_priority(&self) -> u32 {
        80
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Transforms
    }
}

impl Default for TransformParser {
    fn default() -> Self {
        Self::new()
    }
}
