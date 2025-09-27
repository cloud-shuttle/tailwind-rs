//! White Space Utilities
//!
//! This module provides parsing logic for white-space related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse white-space classes
pub fn parse_white_space_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "whitespace-normal" => Some(vec![CssProperty {
            name: "white-space".to_string(),
            value: "normal".to_string(),
            important: false,
        }]),
        "whitespace-nowrap" => Some(vec![CssProperty {
            name: "white-space".to_string(),
            value: "nowrap".to_string(),
            important: false,
        }]),
        "whitespace-pre" => Some(vec![CssProperty {
            name: "white-space".to_string(),
            value: "pre".to_string(),
            important: false,
        }]),
        "whitespace-pre-line" => Some(vec![CssProperty {
            name: "white-space".to_string(),
            value: "pre-line".to_string(),
            important: false,
        }]),
        "whitespace-pre-wrap" => Some(vec![CssProperty {
            name: "white-space".to_string(),
            value: "pre-wrap".to_string(),
            important: false,
        }]),
        "whitespace-break-spaces" => Some(vec![CssProperty {
            name: "white-space".to_string(),
            value: "break-spaces".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
