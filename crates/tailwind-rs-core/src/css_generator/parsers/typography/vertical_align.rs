//! Vertical Align Utilities
//!
//! This module provides parsing logic for vertical-align related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse vertical-align classes
pub fn parse_vertical_align_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "align-baseline" => Some(vec![CssProperty {
            name: "vertical-align".to_string(),
            value: "baseline".to_string(),
            important: false,
        }]),
        "align-top" => Some(vec![CssProperty {
            name: "vertical-align".to_string(),
            value: "top".to_string(),
            important: false,
        }]),
        "align-middle" => Some(vec![CssProperty {
            name: "vertical-align".to_string(),
            value: "middle".to_string(),
            important: false,
        }]),
        "align-bottom" => Some(vec![CssProperty {
            name: "vertical-align".to_string(),
            value: "bottom".to_string(),
            important: false,
        }]),
        "align-text-top" => Some(vec![CssProperty {
            name: "vertical-align".to_string(),
            value: "text-top".to_string(),
            important: false,
        }]),
        "align-text-bottom" => Some(vec![CssProperty {
            name: "vertical-align".to_string(),
            value: "text-bottom".to_string(),
            important: false,
        }]),
        "align-sub" => Some(vec![CssProperty {
            name: "vertical-align".to_string(),
            value: "sub".to_string(),
            important: false,
        }]),
        "align-super" => Some(vec![CssProperty {
            name: "vertical-align".to_string(),
            value: "super".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
