//! Font Family Utilities
//!
//! This module provides parsing logic for font-family related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse font-family classes
pub fn parse_font_family_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "font-sans" => Some(vec![CssProperty {
            name: "font-family".to_string(),
            value: "var(--font-sans)".to_string(),
            important: false,
        }]),
        "font-serif" => Some(vec![CssProperty {
            name: "font-family".to_string(),
            value: "var(--font-serif)".to_string(),
            important: false,
        }]),
        "font-mono" => Some(vec![CssProperty {
            name: "font-family".to_string(),
            value: "ui-monospace, SFMono-Regular, 'SF Mono', Consolas, 'Liberation Mono', Menlo, monospace".to_string(),
            important: false,
        }]),
        _ => {
            // Custom properties for font-family
            if let Some(value) = class.strip_prefix("font-(") {
                if let Some(value) = value.strip_suffix(")") {
                    return Some(vec![CssProperty {
                        name: "font-family".to_string(),
                        value: format!("var({})", value),
                        important: false,
                    }]);
                }
            }

            // Arbitrary values for font-family
            if let Some(value) = class.strip_prefix("font-[") {
                if let Some(value) = value.strip_suffix("]") {
                    return Some(vec![CssProperty {
                        name: "font-family".to_string(),
                        value: value.to_string(),
                        important: false,
                    }]);
                }
            }

            None
        }
    }
}
