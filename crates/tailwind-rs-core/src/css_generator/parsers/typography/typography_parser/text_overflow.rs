//! Text Overflow Utilities
//!
//! This module provides parsing logic for text-overflow related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse text-overflow classes
pub fn parse_text_overflow_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "truncate" => Some(vec![
            CssProperty {
                name: "overflow".to_string(),
                value: "hidden".to_string(),
                important: false,
            },
            CssProperty {
                name: "text-overflow".to_string(),
                value: "ellipsis".to_string(),
                important: false,
            },
            CssProperty {
                name: "white-space".to_string(),
                value: "nowrap".to_string(),
                important: false,
            },
        ]),
        "text-ellipsis" => Some(vec![CssProperty {
            name: "text-overflow".to_string(),
            value: "ellipsis".to_string(),
            important: false,
        }]),
        "text-clip" => Some(vec![CssProperty {
            name: "text-overflow".to_string(),
            value: "clip".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
