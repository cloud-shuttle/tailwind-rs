//! Font Style Utilities
//!
//! This module provides parsing logic for font-style related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse font-style classes
pub fn parse_font_style_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "italic" => Some(vec![CssProperty {
            name: "font-style".to_string(),
            value: "italic".to_string(),
            important: false,
        }]),
        "not-italic" => Some(vec![CssProperty {
            name: "font-style".to_string(),
            value: "normal".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
