//! Letter Spacing Utilities
//!
//! This module provides parsing logic for letter-spacing related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse letter-spacing classes
pub fn parse_letter_spacing_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "tracking-tighter" => Some(vec![CssProperty {
            name: "letter-spacing".to_string(),
            value: "-0.05em".to_string(),
            important: false,
        }]),
        "tracking-tight" => Some(vec![CssProperty {
            name: "letter-spacing".to_string(),
            value: "-0.025em".to_string(),
            important: false,
        }]),
        "tracking-normal" => Some(vec![CssProperty {
            name: "letter-spacing".to_string(),
            value: "0em".to_string(),
            important: false,
        }]),
        "tracking-wide" => Some(vec![CssProperty {
            name: "letter-spacing".to_string(),
            value: "0.025em".to_string(),
            important: false,
        }]),
        "tracking-wider" => Some(vec![CssProperty {
            name: "letter-spacing".to_string(),
            value: "0.05em".to_string(),
            important: false,
        }]),
        "tracking-widest" => Some(vec![CssProperty {
            name: "letter-spacing".to_string(),
            value: "0.1em".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
