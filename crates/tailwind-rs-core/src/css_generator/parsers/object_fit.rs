//! Object Fit Utilities Parser
//!
//! This module provides parsing logic for object-fit-related Tailwind CSS utilities,
//! such as `object-cover`, `object-contain`, `object-fill`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct ObjectFitParser;

impl ObjectFitParser {
    pub fn new() -> Self { Self }

    /// Parse object fit classes
    fn parse_object_fit_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "object-contain" => Some(vec![CssProperty { name: "object-fit".to_string(), value: "contain".to_string(), important: false }]),
            "object-cover" => Some(vec![CssProperty { name: "object-fit".to_string(), value: "cover".to_string(), important: false }]),
            "object-fill" => Some(vec![CssProperty { name: "object-fit".to_string(), value: "fill".to_string(), important: false }]),
            "object-none" => Some(vec![CssProperty { name: "object-fit".to_string(), value: "none".to_string(), important: false }]),
            "object-scale-down" => Some(vec![CssProperty { name: "object-fit".to_string(), value: "scale-down".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse object position classes
    fn parse_object_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "object-bottom" => Some(vec![CssProperty { name: "object-position".to_string(), value: "bottom".to_string(), important: false }]),
            "object-center" => Some(vec![CssProperty { name: "object-position".to_string(), value: "center".to_string(), important: false }]),
            "object-left" => Some(vec![CssProperty { name: "object-position".to_string(), value: "left".to_string(), important: false }]),
            "object-left-bottom" => Some(vec![CssProperty { name: "object-position".to_string(), value: "left bottom".to_string(), important: false }]),
            "object-left-top" => Some(vec![CssProperty { name: "object-position".to_string(), value: "left top".to_string(), important: false }]),
            "object-right" => Some(vec![CssProperty { name: "object-position".to_string(), value: "right".to_string(), important: false }]),
            "object-right-bottom" => Some(vec![CssProperty { name: "object-position".to_string(), value: "right bottom".to_string(), important: false }]),
            "object-right-top" => Some(vec![CssProperty { name: "object-position".to_string(), value: "right top".to_string(), important: false }]),
            "object-top" => Some(vec![CssProperty { name: "object-position".to_string(), value: "top".to_string(), important: false }]),
            _ => None,
        }
    }
}

impl UtilityParser for ObjectFitParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_object_fit_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_object_position_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "object-contain", "object-cover", "object-fill", "object-none", "object-scale-down",
            "object-bottom", "object-center", "object-left", "object-left-bottom", "object-left-top",
            "object-right", "object-right-bottom", "object-right-top", "object-top"
        ]
    }

    fn get_priority(&self) -> u32 { 85 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for ObjectFitParser {
    fn default() -> Self { Self::new() }
}
