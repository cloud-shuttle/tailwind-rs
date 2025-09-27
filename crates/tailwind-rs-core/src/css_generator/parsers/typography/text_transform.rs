//! Text Transform Utilities
//!
//! This module provides parsing logic for text-transform related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse text-transform classes
pub fn parse_text_transform_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "uppercase" => Some(vec![CssProperty {
            name: "text-transform".to_string(),
            value: "uppercase".to_string(),
            important: false,
        }]),
        "lowercase" => Some(vec![CssProperty {
            name: "text-transform".to_string(),
            value: "lowercase".to_string(),
            important: false,
        }]),
        "capitalize" => Some(vec![CssProperty {
            name: "text-transform".to_string(),
            value: "capitalize".to_string(),
            important: false,
        }]),
        "normal-case" => Some(vec![CssProperty {
            name: "text-transform".to_string(),
            value: "none".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
