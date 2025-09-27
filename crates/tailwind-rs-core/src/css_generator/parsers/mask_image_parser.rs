//! Mask Image Parser for Tailwind CSS
//!
//! This module handles parsing of mask image utilities:
//! - Basic mask (mask-none)
//! - Arbitrary mask values (mask-[...], mask-(...))
//! - Linear gradient masks (mask-linear-*, -mask-linear-*)
//! - Side-specific masks (mask-t-from-*, mask-r-from-*, etc.)
//! - Axis-specific masks (mask-x-from-*, mask-y-from-*, etc.)
//! - Radial masks (mask-radial-from-*, mask-radial-to-*)
//! - Conic masks (mask-conic-from-*, mask-conic-to-*, mask-conic-*)

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

/// Parser for mask image utilities
#[derive(Debug, Clone)]
pub struct MaskImageParser;

impl Default for MaskImageParser {
    fn default() -> Self {
        Self::new()
    }
}

impl MaskImageParser {
    /// Create a new mask image parser
    pub fn new() -> Self {
        Self
    }

    /// Parse mask-image classes
    pub fn parse_mask_image_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mask-none" => Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => {
                // Try different mask types in order of specificity
                self.parse_arbitrary_mask_values(class)
                    .or_else(|| self.parse_linear_gradient_masks(class))
                    .or_else(|| self.parse_side_specific_masks(class))
                    .or_else(|| self.parse_axis_specific_masks(class))
                    .or_else(|| self.parse_radial_masks(class))
                    .or_else(|| self.parse_conic_masks(class))
            }
        }
    }

    /// Parse arbitrary mask values (mask-[...] and mask-(...))
    fn parse_arbitrary_mask_values(&self, class: &str) -> Option<Vec<CssProperty>> {
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
    fn parse_linear_gradient_masks(&self, class: &str) -> Option<Vec<CssProperty>> {
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
                return self.parse_linear_from_value(value);
            }
        }

        None
    }

    /// Parse side-specific masks
    fn parse_side_specific_masks(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("mask-t-from-") {
            if let Some(value) = class.strip_prefix("mask-t-from-") {
                return self.parse_side_mask_value("top", value);
            }
        }

        if class.starts_with("mask-r-from-") {
            if let Some(value) = class.strip_prefix("mask-r-from-") {
                return self.parse_side_mask_value("right", value);
            }
        }

        if class.starts_with("mask-b-from-") {
            if let Some(value) = class.strip_prefix("mask-b-from-") {
                return self.parse_side_mask_value("bottom", value);
            }
        }

        if class.starts_with("mask-l-from-") {
            if let Some(value) = class.strip_prefix("mask-l-from-") {
                return self.parse_side_mask_value("left", value);
            }
        }

        None
    }

    /// Parse axis-specific masks
    fn parse_axis_specific_masks(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("mask-x-from-") {
            if let Some(value) = class.strip_prefix("mask-x-from-") {
                return self.parse_axis_mask_value("x", value);
            }
        }

        if class.starts_with("mask-y-from-") {
            if let Some(value) = class.strip_prefix("mask-y-from-") {
                return self.parse_axis_mask_value("y", value);
            }
        }

        if class.starts_with("mask-x-to-") {
            if let Some(value) = class.strip_prefix("mask-x-to-") {
                return self.parse_axis_mask_to_value("x", value);
            }
        }

        if class.starts_with("mask-y-to-") {
            if let Some(value) = class.strip_prefix("mask-y-to-") {
                return self.parse_axis_mask_to_value("y", value);
            }
        }

        None
    }

    /// Parse radial masks
    fn parse_radial_masks(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("mask-radial-from-") {
            if let Some(value) = class.strip_prefix("mask-radial-from-") {
                return self.parse_radial_mask_value(value);
            }
        }

        if class.starts_with("mask-radial-to-") {
            if let Some(value) = class.strip_prefix("mask-radial-to-") {
                return self.parse_radial_mask_to_value(value);
            }
        }

        None
    }

    /// Parse conic masks
    fn parse_conic_masks(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("mask-conic-from-") {
            if let Some(value) = class.strip_prefix("mask-conic-from-") {
                return self.parse_conic_mask_value(value);
            }
        }

        if class.starts_with("mask-conic-to-") {
            if let Some(value) = class.strip_prefix("mask-conic-to-") {
                return self.parse_conic_mask_to_value(value);
            }
        }

        if class.starts_with("mask-conic-") {
            if let Some(angle) = class.strip_prefix("mask-conic-") {
                return self.parse_conic_angle_value(angle);
            }
        }

        None
    }

    /// Parse linear from value
    fn parse_linear_from_value(&self, value: &str) -> Option<Vec<CssProperty>> {
        // Implementation for linear from values
        Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!("linear-gradient(var(--tw-mask-linear-angle), black {}), transparent var(--tw-mask-linear-to))", value),
            important: false,
        }])
    }

    /// Parse side mask value
    fn parse_side_mask_value(&self, side: &str, value: &str) -> Option<Vec<CssProperty>> {
        // Implementation for side mask values
        Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "linear-gradient(to {}, black {}), transparent var(--tw-mask-linear-to))",
                side, value
            ),
            important: false,
        }])
    }

    /// Parse axis mask value
    fn parse_axis_mask_value(&self, axis: &str, value: &str) -> Option<Vec<CssProperty>> {
        // Implementation for axis mask values
        Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "linear-gradient(to {}, black {}), transparent var(--tw-mask-linear-to))",
                axis, value
            ),
            important: false,
        }])
    }

    /// Parse axis mask to value
    fn parse_axis_mask_to_value(&self, axis: &str, value: &str) -> Option<Vec<CssProperty>> {
        // Implementation for axis mask to values
        Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "linear-gradient(to {}, black var(--tw-mask-linear-from)), transparent {})",
                axis, value
            ),
            important: false,
        }])
    }

    /// Parse radial mask value
    fn parse_radial_mask_value(&self, value: &str) -> Option<Vec<CssProperty>> {
        // Implementation for radial mask values
        Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "radial-gradient(circle, black {}), transparent var(--tw-mask-radial-to))",
                value
            ),
            important: false,
        }])
    }

    /// Parse radial mask to value
    fn parse_radial_mask_to_value(&self, value: &str) -> Option<Vec<CssProperty>> {
        // Implementation for radial mask to values
        Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!(
                "radial-gradient(circle, black var(--tw-mask-radial-from)), transparent {})",
                value
            ),
            important: false,
        }])
    }

    /// Parse conic mask value
    fn parse_conic_mask_value(&self, value: &str) -> Option<Vec<CssProperty>> {
        // Implementation for conic mask values
        Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!("conic-gradient(from var(--tw-mask-conic-angle), black {}), transparent var(--tw-mask-conic-to))", value),
            important: false,
        }])
    }

    /// Parse conic mask to value
    fn parse_conic_mask_to_value(&self, value: &str) -> Option<Vec<CssProperty>> {
        // Implementation for conic mask to values
        Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!("conic-gradient(from var(--tw-mask-conic-angle), black var(--tw-mask-conic-from)), transparent {})", value),
            important: false,
        }])
    }

    /// Parse conic angle value
    fn parse_conic_angle_value(&self, angle: &str) -> Option<Vec<CssProperty>> {
        // Implementation for conic angle values
        Some(vec![CssProperty {
            name: "mask-image".to_string(),
            value: format!("conic-gradient(from {}deg, black var(--tw-mask-conic-from)), transparent var(--tw-mask-conic-to))", angle),
            important: false,
        }])
    }
}

impl UtilityParser for MaskImageParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_mask_image_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "mask-none",
            "mask-[*]",
            "mask-(*)",
            "mask-linear-*",
            "-mask-linear-*",
            "mask-linear-from-*",
            "mask-t-from-*",
            "mask-r-from-*",
            "mask-b-from-*",
            "mask-l-from-*",
            "mask-x-from-*",
            "mask-y-from-*",
            "mask-x-to-*",
            "mask-y-to-*",
            "mask-radial-from-*",
            "mask-radial-to-*",
            "mask-conic-from-*",
            "mask-conic-to-*",
            "mask-conic-*",
        ]
    }

    fn get_priority(&self) -> u32 {
        50 // Medium priority for mask image
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Effects
    }
}
