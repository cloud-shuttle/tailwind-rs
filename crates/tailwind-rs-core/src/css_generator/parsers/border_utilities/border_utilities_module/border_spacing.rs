//! Border Spacing Parser
//!
//! This module provides parsing logic for border-spacing related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse border-spacing classes
pub fn parse_border_spacing_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "border-spacing-0" => Some(vec![CssProperty {
            name: "border-spacing".to_string(),
            value: "0px".to_string(),
            important: false,
        }]),
        "border-spacing-1" => Some(vec![CssProperty {
            name: "border-spacing".to_string(),
            value: "0.25rem".to_string(),
            important: false,
        }]),
        "border-spacing-2" => Some(vec![CssProperty {
            name: "border-spacing".to_string(),
            value: "0.5rem".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Border spacing parser
pub struct BorderSpacingParser;

impl BorderSpacingParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_border_spacing_class(class)
    }
}
