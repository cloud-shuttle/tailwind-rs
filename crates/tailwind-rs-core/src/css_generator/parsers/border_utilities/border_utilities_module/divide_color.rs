//! Divide Color Parser
//!
//! This module provides parsing logic for divide-color related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse divide-color classes
pub fn parse_divide_color_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "divide-black" => Some(vec![CssProperty {
            name: "border-color".to_string(),
            value: "#000000".to_string(),
            important: false,
        }]),
        "divide-white" => Some(vec![CssProperty {
            name: "border-color".to_string(),
            value: "#ffffff".to_string(),
            important: false,
        }]),
        "divide-gray-500" => Some(vec![CssProperty {
            name: "border-color".to_string(),
            value: "#6b7280".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Divide color parser
pub struct DivideColorParser;

impl DivideColorParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_divide_color_class(class)
    }
}
