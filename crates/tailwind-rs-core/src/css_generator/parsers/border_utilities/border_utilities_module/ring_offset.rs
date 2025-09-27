//! Ring Offset Parser
//!
//! This module provides parsing logic for ring-offset related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse ring-offset classes
pub fn parse_ring_offset_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "ring-offset-0" => Some(vec![CssProperty {
            name: "box-shadow".to_string(),
            value: "0 0 0 0 var(--ring-offset-color)".to_string(),
            important: false,
        }]),
        "ring-offset-1" => Some(vec![CssProperty {
            name: "box-shadow".to_string(),
            value: "0 0 0 1px var(--ring-offset-color)".to_string(),
            important: false,
        }]),
        "ring-offset-2" => Some(vec![CssProperty {
            name: "box-shadow".to_string(),
            value: "0 0 0 2px var(--ring-offset-color)".to_string(),
            important: false,
        }]),
        "ring-offset-4" => Some(vec![CssProperty {
            name: "box-shadow".to_string(),
            value: "0 0 0 4px var(--ring-offset-color)".to_string(),
            important: false,
        }]),
        "ring-offset-8" => Some(vec![CssProperty {
            name: "box-shadow".to_string(),
            value: "0 0 0 8px var(--ring-offset-color)".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Ring offset parser
pub struct RingOffsetParser;

impl RingOffsetParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_ring_offset_class(class)
    }
}
