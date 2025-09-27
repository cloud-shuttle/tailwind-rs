//! Font Weight Utilities
//!
//! This module provides parsing logic for font-weight related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse font-weight classes
pub fn parse_font_weight_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "font-thin" => Some(vec![CssProperty {
            name: "font-weight".to_string(),
            value: "100".to_string(),
            important: false,
        }]),
        "font-extralight" => Some(vec![CssProperty {
            name: "font-weight".to_string(),
            value: "200".to_string(),
            important: false,
        }]),
        "font-light" => Some(vec![CssProperty {
            name: "font-weight".to_string(),
            value: "300".to_string(),
            important: false,
        }]),
        "font-normal" => Some(vec![CssProperty {
            name: "font-weight".to_string(),
            value: "400".to_string(),
            important: false,
        }]),
        "font-medium" => Some(vec![CssProperty {
            name: "font-weight".to_string(),
            value: "500".to_string(),
            important: false,
        }]),
        "font-semibold" => Some(vec![CssProperty {
            name: "font-weight".to_string(),
            value: "600".to_string(),
            important: false,
        }]),
        "font-bold" => Some(vec![CssProperty {
            name: "font-weight".to_string(),
            value: "700".to_string(),
            important: false,
        }]),
        "font-extrabold" => Some(vec![CssProperty {
            name: "font-weight".to_string(),
            value: "800".to_string(),
            important: false,
        }]),
        "font-black" => Some(vec![CssProperty {
            name: "font-weight".to_string(),
            value: "900".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
