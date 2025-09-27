//! Text Wrap Utilities
//!
//! This module provides parsing logic for text-wrap related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse text-wrap classes
pub fn parse_text_wrap_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "text-wrap" => Some(vec![CssProperty {
            name: "text-wrap".to_string(),
            value: "wrap".to_string(),
            important: false,
        }]),
        "text-nowrap" => Some(vec![CssProperty {
            name: "text-wrap".to_string(),
            value: "nowrap".to_string(),
            important: false,
        }]),
        "text-balance" => Some(vec![CssProperty {
            name: "text-wrap".to_string(),
            value: "balance".to_string(),
            important: false,
        }]),
        "text-pretty" => Some(vec![CssProperty {
            name: "text-wrap".to_string(),
            value: "pretty".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
