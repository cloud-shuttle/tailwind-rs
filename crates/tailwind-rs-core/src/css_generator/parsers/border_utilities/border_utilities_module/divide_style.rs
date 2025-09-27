//! Divide Style Parser
//!
//! This module provides parsing logic for divide-style related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse divide-style classes
pub fn parse_divide_style_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "divide-solid" => Some(vec![CssProperty {
            name: "border-style".to_string(),
            value: "solid".to_string(),
            important: false,
        }]),
        "divide-dashed" => Some(vec![CssProperty {
            name: "border-style".to_string(),
            value: "dashed".to_string(),
            important: false,
        }]),
        "divide-dotted" => Some(vec![CssProperty {
            name: "border-style".to_string(),
            value: "dotted".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Divide style parser
pub struct DivideStyleParser;

impl DivideStyleParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_divide_style_class(class)
    }
}
