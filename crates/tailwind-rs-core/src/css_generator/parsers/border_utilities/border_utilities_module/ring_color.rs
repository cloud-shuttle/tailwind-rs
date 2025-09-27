//! Ring Color Parser
//!
//! This module provides parsing logic for ring-color related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse ring-color classes
pub fn parse_ring_color_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "ring-black" => Some(vec![CssProperty {
            name: "--ring-color".to_string(),
            value: "#000000".to_string(),
            important: false,
        }]),
        "ring-white" => Some(vec![CssProperty {
            name: "--ring-color".to_string(),
            value: "#ffffff".to_string(),
            important: false,
        }]),
        "ring-gray-500" => Some(vec![CssProperty {
            name: "--ring-color".to_string(),
            value: "#6b7280".to_string(),
            important: false,
        }]),
        "ring-blue-500" => Some(vec![CssProperty {
            name: "--ring-color".to_string(),
            value: "#3b82f6".to_string(),
            important: false,
        }]),
        "ring-red-500" => Some(vec![CssProperty {
            name: "--ring-color".to_string(),
            value: "#ef4444".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Ring color parser
pub struct RingColorParser;

impl RingColorParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_ring_color_class(class)
    }
}
