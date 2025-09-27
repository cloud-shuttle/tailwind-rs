//! Border Radius Parser
//!
//! This module provides parsing logic for border-radius related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse border-radius classes
pub fn parse_border_radius_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "rounded-xs" => Some(vec![CssProperty {
            name: "border-radius".to_string(),
            value: "var(--radius-xs)".to_string(),
            important: false,
        }]),
        "rounded-sm" => Some(vec![CssProperty {
            name: "border-radius".to_string(),
            value: "var(--radius-sm)".to_string(),
            important: false,
        }]),
        "rounded" => Some(vec![CssProperty {
            name: "border-radius".to_string(),
            value: "var(--radius-md)".to_string(),
            important: false,
        }]),
        "rounded-md" => Some(vec![CssProperty {
            name: "border-radius".to_string(),
            value: "var(--radius-md)".to_string(),
            important: false,
        }]),
        "rounded-lg" => Some(vec![CssProperty {
            name: "border-radius".to_string(),
            value: "var(--radius-lg)".to_string(),
            important: false,
        }]),
        "rounded-xl" => Some(vec![CssProperty {
            name: "border-radius".to_string(),
            value: "var(--radius-xl)".to_string(),
            important: false,
        }]),
        "rounded-2xl" => Some(vec![CssProperty {
            name: "border-radius".to_string(),
            value: "var(--radius-2xl)".to_string(),
            important: false,
        }]),
        "rounded-3xl" => Some(vec![CssProperty {
            name: "border-radius".to_string(),
            value: "var(--radius-3xl)".to_string(),
            important: false,
        }]),
        "rounded-full" => Some(vec![CssProperty {
            name: "border-radius".to_string(),
            value: "9999px".to_string(),
            important: false,
        }]),
        "rounded-none" => Some(vec![CssProperty {
            name: "border-radius".to_string(),
            value: "0px".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Parse border-radius with direction classes
pub fn parse_border_radius_direction_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "rounded-t" => Some(vec![CssProperty {
            name: "border-top-left-radius".to_string(),
            value: "var(--radius-md)".to_string(),
            important: false,
        }, CssProperty {
            name: "border-top-right-radius".to_string(),
            value: "var(--radius-md)".to_string(),
            important: false,
        }]),
        "rounded-r" => Some(vec![CssProperty {
            name: "border-top-right-radius".to_string(),
            value: "var(--radius-md)".to_string(),
            important: false,
        }, CssProperty {
            name: "border-bottom-right-radius".to_string(),
            value: "var(--radius-md)".to_string(),
            important: false,
        }]),
        "rounded-b" => Some(vec![CssProperty {
            name: "border-bottom-left-radius".to_string(),
            value: "var(--radius-md)".to_string(),
            important: false,
        }, CssProperty {
            name: "border-bottom-right-radius".to_string(),
            value: "var(--radius-md)".to_string(),
            important: false,
        }]),
        "rounded-l" => Some(vec![CssProperty {
            name: "border-top-left-radius".to_string(),
            value: "var(--radius-md)".to_string(),
            important: false,
        }, CssProperty {
            name: "border-bottom-left-radius".to_string(),
            value: "var(--radius-md)".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Border radius parser
pub struct BorderRadiusParser;

impl BorderRadiusParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_border_radius_class(class)
            .or_else(|| parse_border_radius_direction_class(class))
    }
}
