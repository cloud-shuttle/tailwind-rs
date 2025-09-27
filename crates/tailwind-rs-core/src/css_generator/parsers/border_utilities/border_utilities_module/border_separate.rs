//! Border Separate Parser
//!
//! This module provides parsing logic for border-separate related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse border-separate classes
pub fn parse_border_separate_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "border-separate" => Some(vec![CssProperty {
            name: "border-collapse".to_string(),
            value: "separate".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Border separate parser
pub struct BorderSeparateParser;

impl BorderSeparateParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_border_separate_class(class)
    }
}
