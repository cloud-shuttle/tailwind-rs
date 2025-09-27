//! Mask value parsing logic for mask utilities
//!
//! This module contains parsing logic for mask value-related utilities
//! including axis mask values, radial mask values, and conic mask values.

use crate::css_generator::types::CssProperty;

/// Parse axis mask "to" value
pub fn parse_axis_mask_to_value(axis: &str, value: &str) -> Option<Vec<CssProperty>> {
    if value.ends_with("%") {
        let percentage = value.trim_end_matches('%');
        if percentage.parse::<f32>().is_ok() {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("linear-gradient(to {}, transparent, black {})", axis, value),
                important: false,
            }]);
        }
    }

    if value.parse::<f32>().is_ok() {
        return Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "linear-gradient(to {}, transparent, black calc(var(--spacing) * {}))",
                axis, value
            ),
            important: false,
        }]);
    }

    None
}

/// Parse radial mask "to" value
pub fn parse_radial_mask_to_value(value: &str) -> Option<Vec<CssProperty>> {
    if value.ends_with("%") {
        let percentage = value.trim_end_matches('%');
        if percentage.parse::<f32>().is_ok() {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("radial-gradient(circle, transparent, black {})", value),
                important: false,
            }]);
        }
    }

    if value.parse::<f32>().is_ok() {
        return Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "radial-gradient(circle, transparent, black calc(var(--spacing) * {}))",
                value
            ),
            important: false,
        }]);
    }

    None
}

/// Parse conic mask "to" value
pub fn parse_conic_mask_to_value(value: &str) -> Option<Vec<CssProperty>> {
    if value.ends_with("%") {
        let percentage = value.trim_end_matches('%');
        if percentage.parse::<f32>().is_ok() {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("conic-gradient(from 0deg, transparent, black {})", value),
                important: false,
            }]);
        }
    }

    if value.parse::<f32>().is_ok() {
        return Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "conic-gradient(from 0deg, transparent, black calc(var(--spacing) * {}))",
                value
            ),
            important: false,
        }]);
    }

    None
}

/// Parse conic angle value
pub fn parse_conic_angle_value(angle: &str) -> Option<Vec<CssProperty>> {
    if let Ok(angle_deg) = angle.parse::<f32>() {
        return Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!("conic-gradient(from {}deg, black, transparent)", angle_deg),
            important: false,
        }]);
    }

    None
}
