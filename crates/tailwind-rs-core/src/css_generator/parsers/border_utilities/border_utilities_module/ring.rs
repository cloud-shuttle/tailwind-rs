//! Ring Parser
//!
//! This module provides parsing logic for ring related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse ring classes
pub fn parse_ring_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "ring-0" => Some(vec![CssProperty {
            name: "box-shadow".to_string(),
            value: "none".to_string(),
            important: false,
        }]),
        "ring-1" => Some(vec![CssProperty {
            name: "box-shadow".to_string(),
            value: "0 0 0 1px var(--ring-color)".to_string(),
            important: false,
        }]),
        "ring-2" => Some(vec![CssProperty {
            name: "box-shadow".to_string(),
            value: "0 0 0 2px var(--ring-color)".to_string(),
            important: false,
        }]),
        "ring-4" => Some(vec![CssProperty {
            name: "box-shadow".to_string(),
            value: "0 0 0 4px var(--ring-color)".to_string(),
            important: false,
        }]),
        "ring-8" => Some(vec![CssProperty {
            name: "box-shadow".to_string(),
            value: "0 0 0 8px var(--ring-color)".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Ring parser
pub struct RingParser;

impl RingParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_ring_class(class)
    }
}
