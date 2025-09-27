//! Border Utilities Parser
//!
//! This module provides parsing logic for general border utilities.

use crate::css_generator::types::CssProperty;

/// Parse border utilities
pub fn parse_border_utilities_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "border" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "1px".to_string(),
            important: false,
        }, CssProperty {
            name: "border-style".to_string(),
            value: "solid".to_string(),
            important: false,
        }]),
        "border-0" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "0px".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Border utilities parser
#[derive(Debug, Clone)]
pub struct BorderUtilitiesParser;

impl BorderUtilitiesParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_border_utilities_class(class)
    }

    pub fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse(class)
    }
}
