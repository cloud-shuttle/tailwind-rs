//! Hyphens Utilities
//!
//! This module provides parsing logic for hyphens related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse hyphens classes
pub fn parse_hyphens_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "hyphens-none" => Some(vec![CssProperty {
            name: "hyphens".to_string(),
            value: "none".to_string(),
            important: false,
        }]),
        "hyphens-manual" => Some(vec![CssProperty {
            name: "hyphens".to_string(),
            value: "manual".to_string(),
            important: false,
        }]),
        "hyphens-auto" => Some(vec![CssProperty {
            name: "hyphens".to_string(),
            value: "auto".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
