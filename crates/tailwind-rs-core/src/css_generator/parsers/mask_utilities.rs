use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

/// Parser for mask utilities
#[derive(Debug, Clone)]
pub struct MaskUtilitiesParser;

impl MaskUtilitiesParser {
    /// Create a new MaskUtilitiesParser
    pub fn new() -> Self {
        Self
    }

    /// Parse mask-image classes
    fn parse_mask_image_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mask-none" => Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => {
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

                // Linear gradient masks
                if class.starts_with("mask-linear-") {
                    if let Some(angle) = class.strip_prefix("mask-linear-") {
                        if let Ok(_) = angle.parse::<f32>() {
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
                        if let Ok(_) = angle.parse::<f32>() {
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
                        if let Some(properties) = self.parse_linear_from_value(value) {
                            return Some(properties);
                        }
                    }
                }

                // Side-specific masks
                if class.starts_with("mask-t-from-") {
                    if let Some(value) = class.strip_prefix("mask-t-from-") {
                        if let Some(properties) = self.parse_side_mask_value("top", value) {
                            return Some(properties);
                        }
                    }
                }

                if class.starts_with("mask-r-from-") {
                    if let Some(value) = class.strip_prefix("mask-r-from-") {
                        if let Some(properties) = self.parse_side_mask_value("right", value) {
                            return Some(properties);
                        }
                    }
                }

                if class.starts_with("mask-b-from-") {
                    if let Some(value) = class.strip_prefix("mask-b-from-") {
                        if let Some(properties) = self.parse_side_mask_value("bottom", value) {
                            return Some(properties);
                        }
                    }
                }

                if class.starts_with("mask-l-from-") {
                    if let Some(value) = class.strip_prefix("mask-l-from-") {
                        if let Some(properties) = self.parse_side_mask_value("left", value) {
                            return Some(properties);
                        }
                    }
                }

                // Axis-specific masks
                if class.starts_with("mask-x-from-") {
                    if let Some(value) = class.strip_prefix("mask-x-from-") {
                        if let Some(properties) = self.parse_axis_mask_value("x", value) {
                            return Some(properties);
                        }
                    }
                }

                if class.starts_with("mask-y-from-") {
                    if let Some(value) = class.strip_prefix("mask-y-from-") {
                        if let Some(properties) = self.parse_axis_mask_value("y", value) {
                            return Some(properties);
                        }
                    }
                }

                // Axis-specific "to" masks
                if class.starts_with("mask-x-to-") {
                    if let Some(value) = class.strip_prefix("mask-x-to-") {
                        if let Some(properties) = self.parse_axis_mask_to_value("x", value) {
                            return Some(properties);
                        }
                    }
                }

                if class.starts_with("mask-y-to-") {
                    if let Some(value) = class.strip_prefix("mask-y-to-") {
                        if let Some(properties) = self.parse_axis_mask_to_value("y", value) {
                            return Some(properties);
                        }
                    }
                }

                // Radial masks
                if class.starts_with("mask-radial-from-") {
                    if let Some(value) = class.strip_prefix("mask-radial-from-") {
                        if let Some(properties) = self.parse_radial_mask_value(value) {
                            return Some(properties);
                        }
                    }
                }

                // Radial "to" masks
                if class.starts_with("mask-radial-to-") {
                    if let Some(value) = class.strip_prefix("mask-radial-to-") {
                        if let Some(properties) = self.parse_radial_mask_to_value(value) {
                            return Some(properties);
                        }
                    }
                }

                // Conic masks
                if class.starts_with("mask-conic-from-") {
                    if let Some(value) = class.strip_prefix("mask-conic-from-") {
                        if let Some(properties) = self.parse_conic_mask_value(value) {
                            return Some(properties);
                        }
                    }
                }

                // Conic "to" masks
                if class.starts_with("mask-conic-to-") {
                    if let Some(value) = class.strip_prefix("mask-conic-to-") {
                        if let Some(properties) = self.parse_conic_mask_to_value(value) {
                            return Some(properties);
                        }
                    }
                }

                // Conic angle masks
                if class.starts_with("mask-conic-") {
                    if let Some(angle) = class.strip_prefix("mask-conic-") {
                        if let Some(properties) = self.parse_conic_angle_value(angle) {
                            return Some(properties);
                        }
                    }
                }

                None
            }
        }
    }

    /// Parse mask-mode classes
    fn parse_mask_mode_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mask-alpha" => Some(vec![CssProperty {
                name: "mask-mode".to_string(),
                value: "alpha".to_string(),
                important: false,
            }]),
            "mask-luminance" => Some(vec![CssProperty {
                name: "mask-mode".to_string(),
                value: "luminance".to_string(),
                important: false,
            }]),
            "mask-match" => Some(vec![CssProperty {
                name: "mask-mode".to_string(),
                value: "match-source".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse mask-origin classes
    fn parse_mask_origin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mask-origin-border" => Some(vec![CssProperty {
                name: "mask-origin".to_string(),
                value: "border-box".to_string(),
                important: false,
            }]),
            "mask-origin-padding" => Some(vec![CssProperty {
                name: "mask-origin".to_string(),
                value: "padding-box".to_string(),
                important: false,
            }]),
            "mask-origin-content" => Some(vec![CssProperty {
                name: "mask-origin".to_string(),
                value: "content-box".to_string(),
                important: false,
            }]),
            "mask-origin-fill" => Some(vec![CssProperty {
                name: "mask-origin".to_string(),
                value: "fill-box".to_string(),
                important: false,
            }]),
            "mask-origin-stroke" => Some(vec![CssProperty {
                name: "mask-origin".to_string(),
                value: "stroke-box".to_string(),
                important: false,
            }]),
            "mask-origin-view" => Some(vec![CssProperty {
                name: "mask-origin".to_string(),
                value: "view-box".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse mask-position classes
    fn parse_mask_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mask-top-left" => Some(vec![CssProperty {
                name: "mask-position".to_string(),
                value: "top left".to_string(),
                important: false,
            }]),
            "mask-top" => Some(vec![CssProperty {
                name: "mask-position".to_string(),
                value: "top".to_string(),
                important: false,
            }]),
            "mask-top-right" => Some(vec![CssProperty {
                name: "mask-position".to_string(),
                value: "top right".to_string(),
                important: false,
            }]),
            "mask-left" => Some(vec![CssProperty {
                name: "mask-position".to_string(),
                value: "left".to_string(),
                important: false,
            }]),
            "mask-center" => Some(vec![CssProperty {
                name: "mask-position".to_string(),
                value: "center".to_string(),
                important: false,
            }]),
            "mask-right" => Some(vec![CssProperty {
                name: "mask-position".to_string(),
                value: "right".to_string(),
                important: false,
            }]),
            "mask-bottom-left" => Some(vec![CssProperty {
                name: "mask-position".to_string(),
                value: "bottom left".to_string(),
                important: false,
            }]),
            "mask-bottom" => Some(vec![CssProperty {
                name: "mask-position".to_string(),
                value: "bottom".to_string(),
                important: false,
            }]),
            "mask-bottom-right" => Some(vec![CssProperty {
                name: "mask-position".to_string(),
                value: "bottom right".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for mask position
                if let Some(value) = class.strip_prefix("mask-position-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "mask-position".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for mask position
                if let Some(value) = class.strip_prefix("mask-position-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "mask-position".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                None
            }
        }
    }

    /// Parse mask-repeat classes
    fn parse_mask_repeat_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mask-repeat" => Some(vec![CssProperty {
                name: "mask-repeat".to_string(),
                value: "repeat".to_string(),
                important: false,
            }]),
            "mask-no-repeat" => Some(vec![CssProperty {
                name: "mask-repeat".to_string(),
                value: "no-repeat".to_string(),
                important: false,
            }]),
            "mask-repeat-x" => Some(vec![CssProperty {
                name: "mask-repeat".to_string(),
                value: "repeat-x".to_string(),
                important: false,
            }]),
            "mask-repeat-y" => Some(vec![CssProperty {
                name: "mask-repeat".to_string(),
                value: "repeat-y".to_string(),
                important: false,
            }]),
            "mask-repeat-space" => Some(vec![CssProperty {
                name: "mask-repeat".to_string(),
                value: "space".to_string(),
                important: false,
            }]),
            "mask-repeat-round" => Some(vec![CssProperty {
                name: "mask-repeat".to_string(),
                value: "round".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse mask-size classes
    fn parse_mask_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mask-auto" => Some(vec![CssProperty {
                name: "mask-size".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "mask-cover" => Some(vec![CssProperty {
                name: "mask-size".to_string(),
                value: "cover".to_string(),
                important: false,
            }]),
            "mask-contain" => Some(vec![CssProperty {
                name: "mask-size".to_string(),
                value: "contain".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for mask size
                if let Some(value) = class.strip_prefix("mask-size-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "mask-size".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for mask size
                if let Some(value) = class.strip_prefix("mask-size-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "mask-size".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                None
            }
        }
    }

    /// Parse mask-type classes
    fn parse_mask_type_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mask-type-alpha" => Some(vec![CssProperty {
                name: "mask-type".to_string(),
                value: "alpha".to_string(),
                important: false,
            }]),
            "mask-type-luminance" => Some(vec![CssProperty {
                name: "mask-type".to_string(),
                value: "luminance".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse linear from value
    fn parse_linear_from_value(&self, value: &str) -> Option<Vec<CssProperty>> {
        if value.ends_with("%") {
            let percentage = value.trim_end_matches('%');
            if let Ok(_) = percentage.parse::<f32>() {
                return Some(vec![CssProperty {
                    name: "mask-image".to_string(),
                    value: format!("linear-gradient(var(--tw-mask-linear-position), black {}, transparent var(--tw-mask-linear-to))", value),
                    important: false,
                }]);
            }
        }

        if let Ok(_) = value.parse::<f32>() {
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
    fn parse_side_mask_value(&self, side: &str, value: &str) -> Option<Vec<CssProperty>> {
        if value.ends_with("%") {
            let percentage = value.trim_end_matches('%');
            if let Ok(_) = percentage.parse::<f32>() {
                return Some(vec![CssProperty {
                    name: "mask-image".to_string(),
                    value: format!("linear-gradient(to {}, black {}, transparent)", side, value),
                    important: false,
                }]);
            }
        }

        if let Ok(_) = value.parse::<f32>() {
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
    fn parse_axis_mask_value(&self, axis: &str, value: &str) -> Option<Vec<CssProperty>> {
        if value.ends_with("%") {
            let percentage = value.trim_end_matches('%');
            if let Ok(_) = percentage.parse::<f32>() {
                return Some(vec![CssProperty {
                    name: "mask-image".to_string(),
                    value: format!("linear-gradient(to {}, black {}, transparent)", axis, value),
                    important: false,
                }]);
            }
        }

        if let Ok(_) = value.parse::<f32>() {
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
    fn parse_radial_mask_value(&self, value: &str) -> Option<Vec<CssProperty>> {
        if value.ends_with("%") {
            let percentage = value.trim_end_matches('%');
            if let Ok(_) = percentage.parse::<f32>() {
                return Some(vec![CssProperty {
                    name: "mask-image".to_string(),
                    value: format!("radial-gradient(circle, black {}, transparent)", value),
                    important: false,
                }]);
            }
        }

        if let Ok(_) = value.parse::<f32>() {
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
    fn parse_conic_mask_value(&self, value: &str) -> Option<Vec<CssProperty>> {
        if value.ends_with("%") {
            let percentage = value.trim_end_matches('%');
            if let Ok(_) = percentage.parse::<f32>() {
                return Some(vec![CssProperty {
                    name: "mask-image".to_string(),
                    value: format!("conic-gradient(from 0deg, black {}, transparent)", value),
                    important: false,
                }]);
            }
        }

        if let Ok(_) = value.parse::<f32>() {
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

    /// Parse axis mask "to" value
    fn parse_axis_mask_to_value(&self, axis: &str, value: &str) -> Option<Vec<CssProperty>> {
        if value.ends_with("%") {
            let percentage = value.trim_end_matches('%');
            if let Ok(_) = percentage.parse::<f32>() {
                return Some(vec![CssProperty {
                    name: "mask-image".to_string(),
                    value: format!("linear-gradient(to {}, transparent, black {})", axis, value),
                    important: false,
                }]);
            }
        }

        if let Ok(_) = value.parse::<f32>() {
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
    fn parse_radial_mask_to_value(&self, value: &str) -> Option<Vec<CssProperty>> {
        if value.ends_with("%") {
            let percentage = value.trim_end_matches('%');
            if let Ok(_) = percentage.parse::<f32>() {
                return Some(vec![CssProperty {
                    name: "mask-image".to_string(),
                    value: format!("radial-gradient(circle, transparent, black {})", value),
                    important: false,
                }]);
            }
        }

        if let Ok(_) = value.parse::<f32>() {
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
    fn parse_conic_mask_to_value(&self, value: &str) -> Option<Vec<CssProperty>> {
        if value.ends_with("%") {
            let percentage = value.trim_end_matches('%');
            if let Ok(_) = percentage.parse::<f32>() {
                return Some(vec![CssProperty {
                    name: "mask-image".to_string(),
                    value: format!("conic-gradient(from 0deg, transparent, black {})", value),
                    important: false,
                }]);
            }
        }

        if let Ok(_) = value.parse::<f32>() {
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
    fn parse_conic_angle_value(&self, angle: &str) -> Option<Vec<CssProperty>> {
        if let Ok(angle_deg) = angle.parse::<f32>() {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: format!("conic-gradient(from {}deg, black, transparent)", angle_deg),
                important: false,
            }]);
        }

        None
    }
}

impl UtilityParser for MaskUtilitiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity

        // Mask type (most specific)
        if let Some(properties) = self.parse_mask_type_class(class) {
            return Some(properties);
        }

        // Mask size
        if let Some(properties) = self.parse_mask_size_class(class) {
            return Some(properties);
        }

        // Mask repeat
        if let Some(properties) = self.parse_mask_repeat_class(class) {
            return Some(properties);
        }

        // Mask position
        if let Some(properties) = self.parse_mask_position_class(class) {
            return Some(properties);
        }

        // Mask origin
        if let Some(properties) = self.parse_mask_origin_class(class) {
            return Some(properties);
        }

        // Mask mode
        if let Some(properties) = self.parse_mask_mode_class(class) {
            return Some(properties);
        }

        // Mask image (least specific)
        if let Some(properties) = self.parse_mask_image_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "mask-*",
            "mask-image-*",
            "mask-mode-*",
            "mask-origin-*",
            "mask-position-*",
            "mask-repeat-*",
            "mask-size-*",
            "mask-type-*",
            "mask-linear-*",
            "mask-radial-*",
            "mask-conic-*",
            "mask-t-*",
            "mask-r-*",
            "mask-b-*",
            "mask-l-*",
            "mask-x-*",
            "mask-y-*",
            "mask-x-to-*",
            "mask-y-to-*",
            "mask-radial-to-*",
            "mask-conic-to-*",
        ]
    }

    fn get_priority(&self) -> u32 {
        95
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Effects
    }
}
