//! Line Clamp Utilities
//!
//! This module provides parsing logic for line-clamp related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse line-clamp classes
pub fn parse_line_clamp_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "line-clamp-1" => Some(vec![
            CssProperty {
                name: "overflow".to_string(),
                value: "hidden".to_string(),
                important: false,
            },
            CssProperty {
                name: "display".to_string(),
                value: "-webkit-box".to_string(),
                important: false,
            },
            CssProperty {
                name: "-webkit-box-orient".to_string(),
                value: "vertical".to_string(),
                important: false,
            },
            CssProperty {
                name: "-webkit-line-clamp".to_string(),
                value: "1".to_string(),
                important: false,
            },
        ]),
        "line-clamp-2" => Some(vec![
            CssProperty {
                name: "overflow".to_string(),
                value: "hidden".to_string(),
                important: false,
            },
            CssProperty {
                name: "display".to_string(),
                value: "-webkit-box".to_string(),
                important: false,
            },
            CssProperty {
                name: "-webkit-box-orient".to_string(),
                value: "vertical".to_string(),
                important: false,
            },
            CssProperty {
                name: "-webkit-line-clamp".to_string(),
                value: "2".to_string(),
                important: false,
            },
        ]),
        "line-clamp-3" => Some(vec![
            CssProperty {
                name: "overflow".to_string(),
                value: "hidden".to_string(),
                important: false,
            },
            CssProperty {
                name: "display".to_string(),
                value: "-webkit-box".to_string(),
                important: false,
            },
            CssProperty {
                name: "-webkit-box-orient".to_string(),
                value: "vertical".to_string(),
                important: false,
            },
            CssProperty {
                name: "-webkit-line-clamp".to_string(),
                value: "3".to_string(),
                important: false,
            },
        ]),
        "line-clamp-none" => Some(vec![
            CssProperty {
                name: "overflow".to_string(),
                value: "visible".to_string(),
                important: false,
            },
            CssProperty {
                name: "display".to_string(),
                value: "block".to_string(),
                important: false,
            },
            CssProperty {
                name: "-webkit-box-orient".to_string(),
                value: "unset".to_string(),
                important: false,
            },
            CssProperty {
                name: "-webkit-line-clamp".to_string(),
                value: "unset".to_string(),
                important: false,
            },
        ]),
        _ => None,
    }
}
