//! Font Variant Numeric Utilities
//!
//! This module provides parsing logic for font-variant-numeric related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse font-variant-numeric classes
pub fn parse_font_variant_numeric_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "normal-nums" => Some(vec![CssProperty {
            name: "font-variant-numeric".to_string(),
            value: "normal".to_string(),
            important: false,
        }]),
        "ordinal" => Some(vec![CssProperty {
            name: "font-variant-numeric".to_string(),
            value: "ordinal".to_string(),
            important: false,
        }]),
        "slashed-zero" => Some(vec![CssProperty {
            name: "font-variant-numeric".to_string(),
            value: "slashed-zero".to_string(),
            important: false,
        }]),
        "lining-nums" => Some(vec![CssProperty {
            name: "font-variant-numeric".to_string(),
            value: "lining-nums".to_string(),
            important: false,
        }]),
        "oldstyle-nums" => Some(vec![CssProperty {
            name: "font-variant-numeric".to_string(),
            value: "oldstyle-nums".to_string(),
            important: false,
        }]),
        "proportional-nums" => Some(vec![CssProperty {
            name: "font-variant-numeric".to_string(),
            value: "proportional-nums".to_string(),
            important: false,
        }]),
        "tabular-nums" => Some(vec![CssProperty {
            name: "font-variant-numeric".to_string(),
            value: "tabular-nums".to_string(),
            important: false,
        }]),
        "diagonal-fractions" => Some(vec![CssProperty {
            name: "font-variant-numeric".to_string(),
            value: "diagonal-fractions".to_string(),
            important: false,
        }]),
        "stacked-fractions" => Some(vec![CssProperty {
            name: "font-variant-numeric".to_string(),
            value: "stacked-fractions".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
