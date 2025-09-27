//! Text Indent Utilities
//!
//! This module provides parsing logic for text-indent related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse text-indent classes
pub fn parse_text_indent_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "indent-0" => Some(vec![CssProperty {
            name: "text-indent".to_string(),
            value: "0px".to_string(),
            important: false,
        }]),
        "indent-px" => Some(vec![CssProperty {
            name: "text-indent".to_string(),
            value: "1px".to_string(),
            important: false,
        }]),
        "indent-0.5" => Some(vec![CssProperty {
            name: "text-indent".to_string(),
            value: "0.125rem".to_string(),
            important: false,
        }]),
        "indent-1" => Some(vec![CssProperty {
            name: "text-indent".to_string(),
            value: "0.25rem".to_string(),
            important: false,
        }]),
        "indent-1.5" => Some(vec![CssProperty {
            name: "text-indent".to_string(),
            value: "0.375rem".to_string(),
            important: false,
        }]),
        "indent-2" => Some(vec![CssProperty {
            name: "text-indent".to_string(),
            value: "0.5rem".to_string(),
            important: false,
        }]),
        "indent-2.5" => Some(vec![CssProperty {
            name: "text-indent".to_string(),
            value: "0.625rem".to_string(),
            important: false,
        }]),
        "indent-3" => Some(vec![CssProperty {
            name: "text-indent".to_string(),
            value: "0.75rem".to_string(),
            important: false,
        }]),
        "indent-3.5" => Some(vec![CssProperty {
            name: "text-indent".to_string(),
            value: "0.875rem".to_string(),
            important: false,
        }]),
        "indent-4" => Some(vec![CssProperty {
            name: "text-indent".to_string(),
            value: "1rem".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
