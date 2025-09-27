//! Ring Opacity Parser
//!
//! This module provides parsing logic for ring-opacity related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse ring-opacity classes
pub fn parse_ring_opacity_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "ring-opacity-0" => Some(vec![CssProperty {
            name: "--ring-opacity".to_string(),
            value: "0".to_string(),
            important: false,
        }]),
        "ring-opacity-25" => Some(vec![CssProperty {
            name: "--ring-opacity".to_string(),
            value: "0.25".to_string(),
            important: false,
        }]),
        "ring-opacity-50" => Some(vec![CssProperty {
            name: "--ring-opacity".to_string(),
            value: "0.5".to_string(),
            important: false,
        }]),
        "ring-opacity-75" => Some(vec![CssProperty {
            name: "--ring-opacity".to_string(),
            value: "0.75".to_string(),
            important: false,
        }]),
        "ring-opacity-100" => Some(vec![CssProperty {
            name: "--ring-opacity".to_string(),
            value: "1".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Ring opacity parser
pub struct RingOpacityParser;

impl RingOpacityParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_ring_opacity_class(class)
    }
}
