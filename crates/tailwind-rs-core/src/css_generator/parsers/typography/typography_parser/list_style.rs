//! List Style Utilities
//!
//! This module provides parsing logic for list-style related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse list-style-image classes
pub fn parse_list_style_image_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "list-image-none" => Some(vec![CssProperty {
            name: "list-style-image".to_string(),
            value: "none".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Parse list-style-position classes
pub fn parse_list_style_position_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "list-inside" => Some(vec![CssProperty {
            name: "list-style-position".to_string(),
            value: "inside".to_string(),
            important: false,
        }]),
        "list-outside" => Some(vec![CssProperty {
            name: "list-style-position".to_string(),
            value: "outside".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Parse list-style-type classes
pub fn parse_list_style_type_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "list-none" => Some(vec![CssProperty {
            name: "list-style-type".to_string(),
            value: "none".to_string(),
            important: false,
        }]),
        "list-disc" => Some(vec![CssProperty {
            name: "list-style-type".to_string(),
            value: "disc".to_string(),
            important: false,
        }]),
        "list-decimal" => Some(vec![CssProperty {
            name: "list-style-type".to_string(),
            value: "decimal".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
