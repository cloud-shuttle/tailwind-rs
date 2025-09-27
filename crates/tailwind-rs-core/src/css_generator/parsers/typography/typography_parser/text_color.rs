//! Text Color Utilities
//!
//! This module provides parsing logic for text-color related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse text-color classes
pub fn parse_text_color_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "text-black" => Some(vec![CssProperty {
            name: "color".to_string(),
            value: "#000000".to_string(),
            important: false,
        }]),
        "text-white" => Some(vec![CssProperty {
            name: "color".to_string(),
            value: "#ffffff".to_string(),
            important: false,
        }]),
        "text-gray-500" => Some(vec![CssProperty {
            name: "color".to_string(),
            value: "#6b7280".to_string(),
            important: false,
        }]),
        "text-blue-500" => Some(vec![CssProperty {
            name: "color".to_string(),
            value: "#3b82f6".to_string(),
            important: false,
        }]),
        "text-red-500" => Some(vec![CssProperty {
            name: "color".to_string(),
            value: "#ef4444".to_string(),
            important: false,
        }]),
        "text-green-500" => Some(vec![CssProperty {
            name: "color".to_string(),
            value: "#22c55e".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
