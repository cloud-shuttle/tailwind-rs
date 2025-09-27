//! Text Decoration Utilities
//!
//! This module provides parsing logic for text-decoration related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse text-decoration-line classes
pub fn parse_text_decoration_line_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "underline" => Some(vec![CssProperty {
            name: "text-decoration-line".to_string(),
            value: "underline".to_string(),
            important: false,
        }]),
        "overline" => Some(vec![CssProperty {
            name: "text-decoration-line".to_string(),
            value: "overline".to_string(),
            important: false,
        }]),
        "line-through" => Some(vec![CssProperty {
            name: "text-decoration-line".to_string(),
            value: "line-through".to_string(),
            important: false,
        }]),
        "no-underline" => Some(vec![CssProperty {
            name: "text-decoration-line".to_string(),
            value: "none".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Parse text-decoration-color classes
pub fn parse_text_decoration_color_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "decoration-black" => Some(vec![CssProperty {
            name: "text-decoration-color".to_string(),
            value: "#000000".to_string(),
            important: false,
        }]),
        "decoration-white" => Some(vec![CssProperty {
            name: "text-decoration-color".to_string(),
            value: "#ffffff".to_string(),
            important: false,
        }]),
        "decoration-current" => Some(vec![CssProperty {
            name: "text-decoration-color".to_string(),
            value: "currentColor".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Parse text-decoration-style classes
pub fn parse_text_decoration_style_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "decoration-solid" => Some(vec![CssProperty {
            name: "text-decoration-style".to_string(),
            value: "solid".to_string(),
            important: false,
        }]),
        "decoration-double" => Some(vec![CssProperty {
            name: "text-decoration-style".to_string(),
            value: "double".to_string(),
            important: false,
        }]),
        "decoration-dotted" => Some(vec![CssProperty {
            name: "text-decoration-style".to_string(),
            value: "dotted".to_string(),
            important: false,
        }]),
        "decoration-dashed" => Some(vec![CssProperty {
            name: "text-decoration-style".to_string(),
            value: "dashed".to_string(),
            important: false,
        }]),
        "decoration-wavy" => Some(vec![CssProperty {
            name: "text-decoration-style".to_string(),
            value: "wavy".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Parse text-decoration-thickness classes
pub fn parse_text_decoration_thickness_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "decoration-0" => Some(vec![CssProperty {
            name: "text-decoration-thickness".to_string(),
            value: "0px".to_string(),
            important: false,
        }]),
        "decoration-1" => Some(vec![CssProperty {
            name: "text-decoration-thickness".to_string(),
            value: "1px".to_string(),
            important: false,
        }]),
        "decoration-2" => Some(vec![CssProperty {
            name: "text-decoration-thickness".to_string(),
            value: "2px".to_string(),
            important: false,
        }]),
        "decoration-4" => Some(vec![CssProperty {
            name: "text-decoration-thickness".to_string(),
            value: "4px".to_string(),
            important: false,
        }]),
        "decoration-8" => Some(vec![CssProperty {
            name: "text-decoration-thickness".to_string(),
            value: "8px".to_string(),
            important: false,
        }]),
        "decoration-auto" => Some(vec![CssProperty {
            name: "text-decoration-thickness".to_string(),
            value: "auto".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Parse text-underline-offset classes
pub fn parse_text_underline_offset_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "underline-offset-0" => Some(vec![CssProperty {
            name: "text-underline-offset".to_string(),
            value: "0px".to_string(),
            important: false,
        }]),
        "underline-offset-1" => Some(vec![CssProperty {
            name: "text-underline-offset".to_string(),
            value: "1px".to_string(),
            important: false,
        }]),
        "underline-offset-2" => Some(vec![CssProperty {
            name: "text-underline-offset".to_string(),
            value: "2px".to_string(),
            important: false,
        }]),
        "underline-offset-4" => Some(vec![CssProperty {
            name: "text-underline-offset".to_string(),
            value: "4px".to_string(),
            important: false,
        }]),
        "underline-offset-8" => Some(vec![CssProperty {
            name: "text-underline-offset".to_string(),
            value: "8px".to_string(),
            important: false,
        }]),
        "underline-offset-auto" => Some(vec![CssProperty {
            name: "text-underline-offset".to_string(),
            value: "auto".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
