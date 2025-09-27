//! Border Collapse Parser
//!
//! This module provides parsing logic for border-collapse related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse border-collapse classes
pub fn parse_border_collapse_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "border-collapse" => Some(vec![CssProperty {
            name: "border-collapse".to_string(),
            value: "collapse".to_string(),
            important: false,
        }]),
        "border-separate" => Some(vec![CssProperty {
            name: "border-collapse".to_string(),
            value: "separate".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Border collapse parser
pub struct BorderCollapseParser;

impl BorderCollapseParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_border_collapse_class(class)
    }
}
