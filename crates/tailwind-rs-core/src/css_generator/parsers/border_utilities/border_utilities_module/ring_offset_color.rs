//! Ring Offset Color Parser
//!
//! This module provides parsing logic for ring-offset-color related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse ring-offset-color classes
pub fn parse_ring_offset_color_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "ring-offset-black" => Some(vec![CssProperty {
            name: "--ring-offset-color".to_string(),
            value: "#000000".to_string(),
            important: false,
        }]),
        "ring-offset-white" => Some(vec![CssProperty {
            name: "--ring-offset-color".to_string(),
            value: "#ffffff".to_string(),
            important: false,
        }]),
        "ring-offset-gray-500" => Some(vec![CssProperty {
            name: "--ring-offset-color".to_string(),
            value: "#6b7280".to_string(),
            important: false,
        }]),
        "ring-offset-blue-500" => Some(vec![CssProperty {
            name: "--ring-offset-color".to_string(),
            value: "#3b82f6".to_string(),
            important: false,
        }]),
        "ring-offset-red-500" => Some(vec![CssProperty {
            name: "--ring-offset-color".to_string(),
            value: "#ef4444".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Ring offset color parser
pub struct RingOffsetColorParser;

impl RingOffsetColorParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_ring_offset_color_class(class)
    }
}
