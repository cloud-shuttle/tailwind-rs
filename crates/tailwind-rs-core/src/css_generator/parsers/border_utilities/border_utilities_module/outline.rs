//! Outline Parser
//!
//! This module provides parsing logic for outline related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse outline classes
pub fn parse_outline_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "outline-none" => Some(vec![CssProperty {
            name: "outline".to_string(),
            value: "none".to_string(),
            important: false,
        }]),
        "outline" => Some(vec![CssProperty {
            name: "outline".to_string(),
            value: "2px solid".to_string(),
            important: false,
        }]),
        "outline-2" => Some(vec![CssProperty {
            name: "outline".to_string(),
            value: "2px solid".to_string(),
            important: false,
        }]),
        "outline-4" => Some(vec![CssProperty {
            name: "outline".to_string(),
            value: "4px solid".to_string(),
            important: false,
        }]),
        "outline-8" => Some(vec![CssProperty {
            name: "outline".to_string(),
            value: "8px solid".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Outline parser
pub struct OutlineParser;

impl OutlineParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_outline_class(class)
    }
}
