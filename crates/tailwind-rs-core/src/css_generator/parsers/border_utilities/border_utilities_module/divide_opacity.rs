//! Divide Opacity Parser
//!
//! This module provides parsing logic for divide-opacity related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse divide-opacity classes
pub fn parse_divide_opacity_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "divide-opacity-0" => Some(vec![CssProperty {
            name: "border-opacity".to_string(),
            value: "0".to_string(),
            important: false,
        }]),
        "divide-opacity-25" => Some(vec![CssProperty {
            name: "border-opacity".to_string(),
            value: "0.25".to_string(),
            important: false,
        }]),
        "divide-opacity-50" => Some(vec![CssProperty {
            name: "border-opacity".to_string(),
            value: "0.5".to_string(),
            important: false,
        }]),
        "divide-opacity-75" => Some(vec![CssProperty {
            name: "border-opacity".to_string(),
            value: "0.75".to_string(),
            important: false,
        }]),
        "divide-opacity-100" => Some(vec![CssProperty {
            name: "border-opacity".to_string(),
            value: "1".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Divide opacity parser
pub struct DivideOpacityParser;

impl DivideOpacityParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_divide_opacity_class(class)
    }
}
