//! Mask image parsing logic for mask utilities
//!
//! This module contains parsing logic for mask image-related utilities
//! including arbitrary values, linear gradients, side-specific masks, and more.

use crate::css_generator::types::CssProperty;

/// Parse mask-image classes
pub fn parse_mask_image_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "mask-none" => Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: "none".to_string(),
            important: false,
        }]),
        _ => {
            // Try different mask types in order of specificity
            parse_arbitrary_mask_values(class)
                .or_else(|| parse_linear_gradient_masks(class))
                .or_else(|| parse_side_specific_masks(class))
                .or_else(|| parse_axis_specific_masks(class))
                .or_else(|| parse_radial_masks(class))
                .or_else(|| parse_conic_masks(class))
        }
    }
}

/// Parse arbitrary mask values (mask-[...] and mask-(...))
pub fn parse_arbitrary_mask_values(class: &str) -> Option<Vec<CssProperty>> {
    // Arbitrary values for mask image
    if let Some(value) = class.strip_prefix("mask-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: value.to_string(),
                important: false,
            }]);
        }
    }

    // Custom properties for mask image
    if let Some(value) = class.strip_prefix("mask-(") {
        if let Some(value) = value.strip_suffix(")") {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("var({})", value),
                important: false,
            }]);
        }
    }

    None
}

/// Parse linear gradient masks
pub fn parse_linear_gradient_masks(class: &str) -> Option<Vec<CssProperty>> {
    // Linear gradient masks
    if class.starts_with("mask-linear-") {
        if let Some(angle) = class.strip_prefix("mask-linear-") {
            if angle.parse::<f32>().is_ok() {
                return Some(vec![CssProperty {
                    name: "mask-image".to_string(),
                    value: format!("linear-gradient({}deg, black var(--tw-mask-linear-from)), transparent var(--tw-mask-linear-to))", angle),
                    important: false,
                }]);
            }
        }
    }

    // Negative linear gradient masks
    if class.starts_with("-mask-linear-") {
        if let Some(angle) = class.strip_prefix("-mask-linear-") {
            if angle.parse::<f32>().is_ok() {
                return Some(vec![CssProperty {
                    name: "mask-image".to_string(),
                    value: format!("linear-gradient(calc({}deg * -1), black var(--tw-mask-linear-from)), transparent var(--tw-mask-linear-to))", angle),
                    important: false,
                }]);
            }
        }
    }

    // Linear gradient from values
    if class.starts_with("mask-linear-from-") {
        if let Some(value) = class.strip_prefix("mask-linear-from-") {
            return parse_linear_from_value(value);
        }
    }

    None
}

/// Parse side-specific masks
pub fn parse_side_specific_masks(class: &str) -> Option<Vec<CssProperty>> {
    if class.starts_with("mask-t-from-") {
        if let Some(value) = class.strip_prefix("mask-t-from-") {
            return parse_side_mask_value("top", value);
        }
    }

    if class.starts_with("mask-r-from-") {
        if let Some(value) = class.strip_prefix("mask-r-from-") {
            return parse_side_mask_value("right", value);
        }
    }

    if class.starts_with("mask-b-from-") {
        if let Some(value) = class.strip_prefix("mask-b-from-") {
            return parse_side_mask_value("bottom", value);
        }
    }

    if class.starts_with("mask-l-from-") {
        if let Some(value) = class.strip_prefix("mask-l-from-") {
            return parse_side_mask_value("left", value);
        }
    }

    None
}

/// Parse axis-specific masks
pub fn parse_axis_specific_masks(class: &str) -> Option<Vec<CssProperty>> {
    if class.starts_with("mask-x-from-") {
        if let Some(value) = class.strip_prefix("mask-x-from-") {
            return parse_axis_mask_value("x", value);
        }
    }

    if class.starts_with("mask-y-from-") {
        if let Some(value) = class.strip_prefix("mask-y-from-") {
            return parse_axis_mask_value("y", value);
        }
    }

    if class.starts_with("mask-x-to-") {
        if let Some(value) = class.strip_prefix("mask-x-to-") {
            return parse_axis_mask_to_value("x", value);
        }
    }

    if class.starts_with("mask-y-to-") {
        if let Some(value) = class.strip_prefix("mask-y-to-") {
            return parse_axis_mask_to_value("y", value);
        }
    }

    None
}

/// Parse radial masks
pub fn parse_radial_masks(class: &str) -> Option<Vec<CssProperty>> {
    if class.starts_with("mask-radial-from-") {
        if let Some(value) = class.strip_prefix("mask-radial-from-") {
            return parse_radial_mask_value(value);
        }
    }

    if class.starts_with("mask-radial-to-") {
        if let Some(value) = class.strip_prefix("mask-radial-to-") {
            return parse_radial_mask_to_value(value);
        }
    }

    None
}

/// Parse conic masks
pub fn parse_conic_masks(class: &str) -> Option<Vec<CssProperty>> {
    if class.starts_with("mask-conic-from-") {
        if let Some(value) = class.strip_prefix("mask-conic-from-") {
            return parse_conic_mask_value(value);
        }
    }

    if class.starts_with("mask-conic-to-") {
        if let Some(value) = class.strip_prefix("mask-conic-to-") {
            return parse_conic_mask_to_value(value);
        }
    }

    if class.starts_with("mask-conic-") {
        if let Some(angle) = class.strip_prefix("mask-conic-") {
            return parse_conic_angle_value(angle);
        }
    }

    None
}

/// Parse linear from value
pub fn parse_linear_from_value(value: &str) -> Option<Vec<CssProperty>> {
    if value.ends_with("%") {
        let percentage = value.trim_end_matches('%');
        if percentage.parse::<f32>().is_ok() {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("linear-gradient(var(--tw-mask-linear-position), black {}, transparent var(--tw-mask-linear-to))", value),
                important: false,
            }]);
        }
    }

    if value.parse::<f32>().is_ok() {
        return Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!("linear-gradient(var(--tw-mask-linear-position), black calc(var(--spacing) * {}), transparent var(--tw-mask-linear-to))", value),
            important: false,
        }]);
    }

    // Custom properties for linear from
    if let Some(custom_value) = value.strip_prefix("(") {
        if let Some(custom_value) = custom_value.strip_suffix(")") {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("linear-gradient(var(--tw-mask-linear-position), black {}, transparent var(--tw-mask-linear-to))", custom_value),
                important: false,
            }]);
        }
    }

    // Arbitrary values for linear from
    if let Some(arbitrary_value) = value.strip_prefix("[") {
        if let Some(arbitrary_value) = arbitrary_value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("linear-gradient(var(--tw-mask-linear-position), black {}, transparent var(--tw-mask-linear-to))", arbitrary_value),
                important: false,
            }]);
        }
    }

    None
}

/// Parse side mask value
pub fn parse_side_mask_value(side: &str, value: &str) -> Option<Vec<CssProperty>> {
    if value.ends_with("%") {
        let percentage = value.trim_end_matches('%');
        if percentage.parse::<f32>().is_ok() {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("linear-gradient(to {}, black {}, transparent)", side, value),
                important: false,
            }]);
        }
    }

    if value.parse::<f32>().is_ok() {
        return Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "linear-gradient(to {}, black calc(var(--spacing) * {}), transparent)",
                side, value
            ),
            important: false,
        }]);
    }

    None
}

/// Parse axis mask value
pub fn parse_axis_mask_value(axis: &str, value: &str) -> Option<Vec<CssProperty>> {
    if value.ends_with("%") {
        let percentage = value.trim_end_matches('%');
        if percentage.parse::<f32>().is_ok() {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("linear-gradient(to {}, black {}, transparent)", axis, value),
                important: false,
            }]);
        }
    }

    if value.parse::<f32>().is_ok() {
        return Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "linear-gradient(to {}, black calc(var(--spacing) * {}), transparent)",
                axis, value
            ),
            important: false,
        }]);
    }

    None
}

/// Parse radial mask value
pub fn parse_radial_mask_value(value: &str) -> Option<Vec<CssProperty>> {
    if value.ends_with("%") {
        let percentage = value.trim_end_matches('%');
        if percentage.parse::<f32>().is_ok() {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("radial-gradient(circle, black {}, transparent)", value),
                important: false,
            }]);
        }
    }

    if value.parse::<f32>().is_ok() {
        return Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "radial-gradient(circle, black calc(var(--spacing) * {}), transparent)",
                value
            ),
            important: false,
        }]);
    }

    None
}

/// Parse conic mask value
pub fn parse_conic_mask_value(value: &str) -> Option<Vec<CssProperty>> {
    if value.ends_with("%") {
        let percentage = value.trim_end_matches('%');
        if percentage.parse::<f32>().is_ok() {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("conic-gradient(from 0deg, black {}, transparent)", value),
                important: false,
            }]);
        }
    }

    if value.parse::<f32>().is_ok() {
        return Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "conic-gradient(from 0deg, black calc(var(--spacing) * {}), transparent)",
                value
            ),
            important: false,
        }]);
    }

    None
}
