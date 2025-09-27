//! Border Color Parser
//!
//! This module provides parsing logic for border-color related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse border-color classes
pub fn parse_border_color_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "border-black" => Some(vec![CssProperty {
            name: "border-color".to_string(),
            value: "#000000".to_string(),
            important: false,
        }]),
        "border-white" => Some(vec![CssProperty {
            name: "border-color".to_string(),
            value: "#ffffff".to_string(),
            important: false,
        }]),
        "border-gray-500" => Some(vec![CssProperty {
            name: "border-color".to_string(),
            value: "#6b7280".to_string(),
            important: false,
        }]),
        "border-blue-500" => Some(vec![CssProperty {
            name: "border-color".to_string(),
            value: "#3b82f6".to_string(),
            important: false,
        }]),
        "border-red-500" => Some(vec![CssProperty {
            name: "border-color".to_string(),
            value: "#ef4444".to_string(),
            important: false,
        }]),
        "border-green-500" => Some(vec![CssProperty {
            name: "border-color".to_string(),
            value: "#22c55e".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Border color parser
pub struct BorderColorParser;

impl BorderColorParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_border_color_class(class)
    }
}
