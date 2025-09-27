//! Content Utilities
//!
//! This module provides parsing logic for content related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse content classes
pub fn parse_content_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "content-none" => Some(vec![CssProperty {
            name: "content".to_string(),
            value: "none".to_string(),
            important: false,
        }]),
        "content-['']" => Some(vec![CssProperty {
            name: "content".to_string(),
            value: "''".to_string(),
            important: false,
        }]),
        "content-['*']" => Some(vec![CssProperty {
            name: "content".to_string(),
            value: "'*'".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
