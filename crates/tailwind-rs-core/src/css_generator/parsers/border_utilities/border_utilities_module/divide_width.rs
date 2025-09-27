//! Divide Width Parser
//!
//! This module provides parsing logic for divide-width related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse divide-width classes
pub fn parse_divide_width_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "divide-x-0" => Some(vec![CssProperty {
            name: "border-right-width".to_string(),
            value: "0px".to_string(),
            important: false,
        }]),
        "divide-x" => Some(vec![CssProperty {
            name: "border-right-width".to_string(),
            value: "1px".to_string(),
            important: false,
        }]),
        "divide-y-0" => Some(vec![CssProperty {
            name: "border-top-width".to_string(),
            value: "0px".to_string(),
            important: false,
        }]),
        "divide-y" => Some(vec![CssProperty {
            name: "border-top-width".to_string(),
            value: "1px".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Divide width parser
pub struct DivideWidthParser;

impl DivideWidthParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_divide_width_class(class)
    }
}
