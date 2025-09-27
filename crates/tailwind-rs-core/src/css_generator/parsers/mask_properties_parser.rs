//! Mask Properties Parser for Tailwind CSS
//!
//! This module handles parsing of mask property utilities:
//! - Mask mode (mask-alpha, mask-luminance, mask-match)
//! - Mask origin (mask-origin-border, mask-origin-padding, etc.)
//! - Mask position (mask-top-left, mask-top, mask-center, etc.)
//! - Mask repeat (mask-repeat, mask-no-repeat, mask-repeat-x, etc.)
//! - Mask size (mask-auto, mask-cover, mask-contain, etc.)
//! - Mask type (mask-type-alpha, mask-type-luminance)

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

/// Parser for mask property utilities
#[derive(Debug, Clone)]
pub struct MaskPropertiesParser;

impl Default for MaskPropertiesParser {
    fn default() -> Self {
        Self::new()
    }
}

impl MaskPropertiesParser {
    /// Create a new mask properties parser
    pub fn new() -> Self {
        Self
    }

    /// Parse mask-mode classes
    pub fn parse_mask_mode_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
    pub fn parse_mask_origin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
    pub fn parse_mask_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
    pub fn parse_mask_repeat_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
    pub fn parse_mask_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
    pub fn parse_mask_type_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
}

impl UtilityParser for MaskPropertiesParser {
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

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "mask-alpha",
            "mask-luminance",
            "mask-match",
            "mask-origin-border",
            "mask-origin-padding",
            "mask-origin-content",
            "mask-origin-fill",
            "mask-origin-stroke",
            "mask-origin-view",
            "mask-top-left",
            "mask-top",
            "mask-top-right",
            "mask-left",
            "mask-center",
            "mask-right",
            "mask-bottom-left",
            "mask-bottom",
            "mask-bottom-right",
            "mask-position-(*)",
            "mask-position-[*]",
            "mask-repeat",
            "mask-no-repeat",
            "mask-repeat-x",
            "mask-repeat-y",
            "mask-repeat-space",
            "mask-repeat-round",
            "mask-auto",
            "mask-cover",
            "mask-contain",
            "mask-size-(*)",
            "mask-size-[*]",
            "mask-type-alpha",
            "mask-type-luminance",
        ]
    }

    fn get_priority(&self) -> u32 {
        95 // High priority for mask properties
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Effects
    }
}
