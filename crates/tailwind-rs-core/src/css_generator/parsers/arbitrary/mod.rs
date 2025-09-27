//! Arbitrary Values Parser Module
//!
//! This module provides parsing logic for arbitrary value classes in Tailwind CSS,
//! such as `size-[38px]`, `top-[4px]`, `left-[7px]`, `drop-shadow-[0_3px_1px_rgba(0,0,0,.15)]`.
//! The implementation has been modularized into separate files for better maintainability.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

pub mod size_parsers;
pub mod positioning_parsers;
pub mod transform_parsers;

// Re-export the parsing functions
use size_parsers::*;
use positioning_parsers::*;
use transform_parsers::*;

#[derive(Debug, Clone)]
pub struct ArbitraryParser;

impl ArbitraryParser {
    pub fn new() -> Self {
        Self
    }
}

impl UtilityParser for ArbitraryParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = parse_arbitrary_size_class(class) {
            return Some(properties);
        }
        if let Some(properties) = parse_standard_size_class(class) {
            return Some(properties);
        }
        if let Some(properties) = parse_arbitrary_position_class(class) {
            return Some(properties);
        }
        if let Some(properties) = parse_arbitrary_background_class(class) {
            return Some(properties);
        }
        if let Some(properties) = parse_arbitrary_filter_class(class) {
            return Some(properties);
        }
        if let Some(properties) = parse_arbitrary_transform_class(class) {
            return Some(properties);
        }
        if let Some(properties) = parse_arbitrary_mask_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "size-[*]",
            "size-0",
            "size-1",
            "size-2",
            "size-3",
            "size-4",
            "size-5",
            "size-6",
            "size-8",
            "size-10",
            "size-12",
            "size-16",
            "size-20",
            "size-24",
            "size-32",
            "size-40",
            "size-48",
            "size-56",
            "size-64",
            "size-72",
            "size-80",
            "size-96",
            "size-auto",
            "size-full",
            "size-screen",
            "size-min",
            "size-max",
            "size-fit",
            "w-[*]",
            "h-[*]",
            "top-[*]",
            "left-[*]",
            "right-[*]",
            "bottom-[*]",
            "bg-[*]",
            "drop-shadow-[*]",
            "blur-[*]",
            "brightness-[*]",
            "contrast-[*]",
            "grayscale-[*]",
            "hue-rotate-[*]",
            "invert-[*]",
            "opacity-[*]",
            "saturate-[*]",
            "sepia-[*]",
            "translate-x-[*]",
            "translate-y-[*]",
            "-translate-x-[*]",
            "-translate-y-[*]",
            "rotate-[*]",
            "scale-[*]",
            "mask-[*]",
        ]
    }

    fn get_priority(&self) -> u32 {
        95
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

impl Default for ArbitraryParser {
    fn default() -> Self {
        Self::new()
    }
}
