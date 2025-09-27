//! Font Stretch Utilities
//!
//! This module provides parsing logic for font-stretch related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse font-stretch classes
pub fn parse_font_stretch_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "font-ultra-condensed" => Some(vec![CssProperty {
            name: "font-stretch".to_string(),
            value: "ultra-condensed".to_string(),
            important: false,
        }]),
        "font-extra-condensed" => Some(vec![CssProperty {
            name: "font-stretch".to_string(),
            value: "extra-condensed".to_string(),
            important: false,
        }]),
        "font-condensed" => Some(vec![CssProperty {
            name: "font-stretch".to_string(),
            value: "condensed".to_string(),
            important: false,
        }]),
        "font-semi-condensed" => Some(vec![CssProperty {
            name: "font-stretch".to_string(),
            value: "semi-condensed".to_string(),
            important: false,
        }]),
        "font-normal" => Some(vec![CssProperty {
            name: "font-stretch".to_string(),
            value: "normal".to_string(),
            important: false,
        }]),
        "font-semi-expanded" => Some(vec![CssProperty {
            name: "font-stretch".to_string(),
            value: "semi-expanded".to_string(),
            important: false,
        }]),
        "font-expanded" => Some(vec![CssProperty {
            name: "font-stretch".to_string(),
            value: "expanded".to_string(),
            important: false,
        }]),
        "font-extra-expanded" => Some(vec![CssProperty {
            name: "font-stretch".to_string(),
            value: "extra-expanded".to_string(),
            important: false,
        }]),
        "font-ultra-expanded" => Some(vec![CssProperty {
            name: "font-stretch".to_string(),
            value: "ultra-expanded".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
