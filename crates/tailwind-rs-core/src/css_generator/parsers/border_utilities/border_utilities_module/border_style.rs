//! Border Style Parser
//!
//! This module provides parsing logic for border-style related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse border-style classes
pub fn parse_border_style_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "border-solid" => Some(vec![CssProperty {
            name: "border-style".to_string(),
            value: "solid".to_string(),
            important: false,
        }]),
        "border-dashed" => Some(vec![CssProperty {
            name: "border-style".to_string(),
            value: "dashed".to_string(),
            important: false,
        }]),
        "border-dotted" => Some(vec![CssProperty {
            name: "border-style".to_string(),
            value: "dotted".to_string(),
            important: false,
        }]),
        "border-double" => Some(vec![CssProperty {
            name: "border-style".to_string(),
            value: "double".to_string(),
            important: false,
        }]),
        "border-none" => Some(vec![CssProperty {
            name: "border-style".to_string(),
            value: "none".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Border style parser
pub struct BorderStyleParser;

impl BorderStyleParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_border_style_class(class)
    }
}
