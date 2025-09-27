//! Border Width Parser
//!
//! This module provides parsing logic for border-width related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse border-width classes
pub fn parse_border_width_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "border-0" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "0px".to_string(),
            important: false,
        }]),
        "border" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "1px".to_string(),
            important: false,
        }]),
        "border-2" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "2px".to_string(),
            important: false,
        }]),
        "border-4" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "4px".to_string(),
            important: false,
        }]),
        "border-8" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "8px".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Border width parser
pub struct BorderWidthParser;

impl BorderWidthParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_border_width_class(class)
    }
}
