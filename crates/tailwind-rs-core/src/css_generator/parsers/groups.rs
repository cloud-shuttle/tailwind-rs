//! Group Utilities Parser
//!
//! This module provides parsing logic for group-related Tailwind CSS utilities,
//! including group classes and group hover states.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct GroupParser;

impl GroupParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse group class
    fn parse_group_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "group" => Some(vec![CssProperty {
                name: "position".to_string(),
                value: "relative".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse group hover classes
    fn parse_group_hover_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(property_part) = class.strip_prefix("group-hover:") {
            // This will be handled by the variant parser, but we can add specific logic here
            // For now, we'll return None and let the variant system handle it
            None
        } else {
            None
        }
    }
}

impl UtilityParser for GroupParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_group_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_group_hover_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["group", "group-hover:*"]
    }

    fn get_priority(&self) -> u32 {
        60
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

impl Default for GroupParser {
    fn default() -> Self {
        Self::new()
    }
}
