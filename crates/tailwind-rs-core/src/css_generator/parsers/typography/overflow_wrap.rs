//! Overflow Wrap Utilities
//!
//! This module provides parsing logic for overflow-wrap related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse overflow-wrap classes
pub fn parse_overflow_wrap_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "break-normal" => Some(vec![CssProperty {
            name: "overflow-wrap".to_string(),
            value: "normal".to_string(),
            important: false,
        }]),
        "break-words" => Some(vec![CssProperty {
            name: "overflow-wrap".to_string(),
            value: "break-word".to_string(),
            important: false,
        }]),
        "break-all" => Some(vec![CssProperty {
            name: "overflow-wrap".to_string(),
            value: "anywhere".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
