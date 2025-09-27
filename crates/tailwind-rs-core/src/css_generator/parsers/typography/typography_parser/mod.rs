//! Typography Utilities Parser
//!
//! This module provides parsing logic for typography-related Tailwind CSS utilities,
//! including font-family, font-size, font-smoothing, font-style, font-weight, font-stretch, and font-variant-numeric.

use crate::css_generator::parsers::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

pub mod font_family;
pub mod font_size;
pub mod font_smoothing;
pub mod font_style;
pub mod font_weight;
pub mod font_stretch;
pub mod font_variant_numeric;
pub mod letter_spacing;
pub mod line_clamp;
pub mod line_height;
pub mod list_style;
pub mod text_align;
pub mod text_color;
pub mod text_decoration;
pub mod text_transform;
pub mod text_overflow;
pub mod text_wrap;
pub mod text_indent;
pub mod vertical_align;
pub mod white_space;
pub mod word_break;
pub mod overflow_wrap;
pub mod hyphens;
pub mod content;
pub mod utils;

#[derive(Debug, Clone)]
pub struct TypographyParser;

impl Default for TypographyParser {
    fn default() -> Self {
        Self::new()
    }
}

impl TypographyParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse typography classes
    pub fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try font family first
        if let Some(result) = font_family::parse_font_family_class(class) {
            return Some(result);
        }

        // Try font size
        if let Some(result) = font_size::parse_font_size_class(class) {
            return Some(result);
        }

        // Try font smoothing
        if let Some(result) = font_smoothing::parse_font_smoothing_class(class) {
            return Some(result);
        }

        // Try font style
        if let Some(result) = font_style::parse_font_style_class(class) {
            return Some(result);
        }

        // Try font weight
        if let Some(result) = font_weight::parse_font_weight_class(class) {
            return Some(result);
        }

        // Try font stretch
        if let Some(result) = font_stretch::parse_font_stretch_class(class) {
            return Some(result);
        }

        // Try font variant numeric
        if let Some(result) = font_variant_numeric::parse_font_variant_numeric_class(class) {
            return Some(result);
        }

        // Try letter spacing
        if let Some(result) = letter_spacing::parse_letter_spacing_class(class) {
            return Some(result);
        }

        // Try line clamp
        if let Some(result) = line_clamp::parse_line_clamp_class(class) {
            return Some(result);
        }

        // Try line height
        if let Some(result) = line_height::parse_line_height_class(class) {
            return Some(result);
        }

        // Try list style image
        if let Some(result) = list_style::parse_list_style_image_class(class) {
            return Some(result);
        }

        // Try list style position
        if let Some(result) = list_style::parse_list_style_position_class(class) {
            return Some(result);
        }

        // Try list style type
        if let Some(result) = list_style::parse_list_style_type_class(class) {
            return Some(result);
        }

        // Try text align
        if let Some(result) = text_align::parse_text_align_class(class) {
            return Some(result);
        }

        // Try text color
        if let Some(result) = text_color::parse_text_color_class(class) {
            return Some(result);
        }

        // Try text decoration line
        if let Some(result) = text_decoration::parse_text_decoration_line_class(class) {
            return Some(result);
        }

        // Try text decoration color
        if let Some(result) = text_decoration::parse_text_decoration_color_class(class) {
            return Some(result);
        }

        // Try text decoration style
        if let Some(result) = text_decoration::parse_text_decoration_style_class(class) {
            return Some(result);
        }

        // Try text decoration thickness
        if let Some(result) = text_decoration::parse_text_decoration_thickness_class(class) {
            return Some(result);
        }

        // Try text underline offset
        if let Some(result) = text_decoration::parse_text_underline_offset_class(class) {
            return Some(result);
        }

        // Try text transform
        if let Some(result) = text_transform::parse_text_transform_class(class) {
            return Some(result);
        }

        // Try text overflow
        if let Some(result) = text_overflow::parse_text_overflow_class(class) {
            return Some(result);
        }

        // Try text wrap
        if let Some(result) = text_wrap::parse_text_wrap_class(class) {
            return Some(result);
        }

        // Try text indent
        if let Some(result) = text_indent::parse_text_indent_class(class) {
            return Some(result);
        }

        // Try vertical align
        if let Some(result) = vertical_align::parse_vertical_align_class(class) {
            return Some(result);
        }

        // Try white space
        if let Some(result) = white_space::parse_white_space_class(class) {
            return Some(result);
        }

        // Try word break
        if let Some(result) = word_break::parse_word_break_class(class) {
            return Some(result);
        }

        // Try overflow wrap
        if let Some(result) = overflow_wrap::parse_overflow_wrap_class(class) {
            return Some(result);
        }

        // Try hyphens
        if let Some(result) = hyphens::parse_hyphens_class(class) {
            return Some(result);
        }

        // Try content
        if let Some(result) = content::parse_content_class(class) {
            return Some(result);
        }

        None
    }

    /// Get supported patterns for this parser
    pub fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "font-*",
            "text-*",
            "leading-*",
            "tracking-*",
            "line-clamp-*",
            "list-*",
            "align-*",
            "decoration-*",
            "underline-*",
            "uppercase",
            "lowercase",
            "capitalize",
            "truncate",
            "text-ellipsis",
            "text-clip",
            "whitespace-*",
            "break-*",
            "overflow-wrap-*",
            "hyphens-*",
            "content-*",
        ]
    }

    /// Get parser priority
    pub fn get_priority(&self) -> u32 {
        100
    }

    /// Get parser category
    pub fn get_category(&self) -> ParserCategory {
        ParserCategory::Typography
    }
}

impl UtilityParser for TypographyParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        self.get_supported_patterns()
    }

    fn get_priority(&self) -> u32 {
        self.get_priority()
    }

    fn get_category(&self) -> ParserCategory {
        self.get_category()
    }
}
