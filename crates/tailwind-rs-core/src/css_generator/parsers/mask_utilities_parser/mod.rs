//! Mask Utilities Parser Module
//!
//! This module provides parsing logic for mask utility classes in Tailwind CSS,
//! such as `mask-none`, `mask-cover`, `mask-linear-45`, `mask-radial-from-50%`.
//! The implementation has been modularized into separate files for better maintainability.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

pub mod mask_image_parsers;
pub mod mask_property_parsers;
pub mod mask_value_parsers;

// Re-export the parsing functions
use mask_image_parsers::*;
use mask_property_parsers::*;
use mask_value_parsers::*;

#[derive(Debug, Clone)]
pub struct MaskUtilitiesParser;

impl MaskUtilitiesParser {
    /// Create a new MaskUtilitiesParser
    pub fn new() -> Self {
        Self
    }
}

impl UtilityParser for MaskUtilitiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity

        // Mask type (most specific)
        if let Some(properties) = parse_mask_type_class(class) {
            return Some(properties);
        }

        // Mask size
        if let Some(properties) = parse_mask_size_class(class) {
            return Some(properties);
        }

        // Mask repeat
        if let Some(properties) = parse_mask_repeat_class(class) {
            return Some(properties);
        }

        // Mask position
        if let Some(properties) = parse_mask_position_class(class) {
            return Some(properties);
        }

        // Mask origin
        if let Some(properties) = parse_mask_origin_class(class) {
            return Some(properties);
        }

        // Mask mode
        if let Some(properties) = parse_mask_mode_class(class) {
            return Some(properties);
        }

        // Mask image (least specific)
        if let Some(properties) = parse_mask_image_class(class) {
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

impl Default for MaskUtilitiesParser {
    fn default() -> Self {
        Self::new()
    }
}
