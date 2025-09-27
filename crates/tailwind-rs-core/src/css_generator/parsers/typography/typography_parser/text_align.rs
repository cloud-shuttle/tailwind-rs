//! Text Align Utilities
//!
//! This module provides parsing logic for text-align related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse text-align classes
pub fn parse_text_align_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "text-left" => Some(vec![CssProperty {
            name: "text-align".to_string(),
            value: "left".to_string(),
            important: false,
        }]),
        "text-center" => Some(vec![CssProperty {
            name: "text-align".to_string(),
            value: "center".to_string(),
            important: false,
        }]),
        "text-right" => Some(vec![CssProperty {
            name: "text-align".to_string(),
            value: "right".to_string(),
            important: false,
        }]),
        "text-justify" => Some(vec![CssProperty {
            name: "text-align".to_string(),
            value: "justify".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
