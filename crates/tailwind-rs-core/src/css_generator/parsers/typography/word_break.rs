//! Word Break Utilities
//!
//! This module provides parsing logic for word-break related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse word-break classes
pub fn parse_word_break_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "break-normal" => Some(vec![CssProperty {
            name: "word-break".to_string(),
            value: "normal".to_string(),
            important: false,
        }]),
        "break-words" => Some(vec![CssProperty {
            name: "word-break".to_string(),
            value: "break-word".to_string(),
            important: false,
        }]),
        "break-all" => Some(vec![CssProperty {
            name: "word-break".to_string(),
            value: "break-all".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
