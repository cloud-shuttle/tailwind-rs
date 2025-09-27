//! Font Smoothing Utilities
//!
//! This module provides parsing logic for font-smoothing related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse font-smoothing classes
pub fn parse_font_smoothing_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "antialiased" => Some(vec![CssProperty {
            name: "-webkit-font-smoothing".to_string(),
            value: "antialiased".to_string(),
            important: false,
        }]),
        "subpixel-antialiased" => Some(vec![CssProperty {
            name: "-webkit-font-smoothing".to_string(),
            value: "auto".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
