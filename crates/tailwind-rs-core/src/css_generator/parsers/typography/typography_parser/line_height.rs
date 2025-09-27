//! Line Height Utilities
//!
//! This module provides parsing logic for line-height related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse line-height classes
pub fn parse_line_height_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "leading-3" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "0.75rem".to_string(),
            important: false,
        }]),
        "leading-4" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "1rem".to_string(),
            important: false,
        }]),
        "leading-5" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "1.25rem".to_string(),
            important: false,
        }]),
        "leading-6" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "1.5rem".to_string(),
            important: false,
        }]),
        "leading-7" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "1.75rem".to_string(),
            important: false,
        }]),
        "leading-8" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "2rem".to_string(),
            important: false,
        }]),
        "leading-9" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "2.25rem".to_string(),
            important: false,
        }]),
        "leading-10" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "2.5rem".to_string(),
            important: false,
        }]),
        "leading-none" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "1".to_string(),
            important: false,
        }]),
        "leading-tight" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "1.25".to_string(),
            important: false,
        }]),
        "leading-snug" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "1.375".to_string(),
            important: false,
        }]),
        "leading-normal" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "1.5".to_string(),
            important: false,
        }]),
        "leading-relaxed" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "1.625".to_string(),
            important: false,
        }]),
        "leading-loose" => Some(vec![CssProperty {
            name: "line-height".to_string(),
            value: "2".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
