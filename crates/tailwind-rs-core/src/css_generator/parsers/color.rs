//! Color Utilities Parser
//! 
//! This module handles parsing of color-related utilities.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::core::CssProperty;

/// Parser for color utilities
#[derive(Debug, Clone)]
pub struct ColorParser;

impl ColorParser {
    pub fn new() -> Self {
        Self
    }
}

impl UtilityParser for ColorParser {
    fn parse_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        // TODO: Implement color parsing
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["bg-*", "text-*", "border-*"]
    }

    fn get_priority(&self) -> u32 {
        90
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Color
    }
}

impl Default for ColorParser {
    fn default() -> Self {
        Self::new()
    }
}
