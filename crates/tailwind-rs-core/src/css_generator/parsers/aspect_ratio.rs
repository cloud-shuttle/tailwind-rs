//! Aspect Ratio Utilities Parser
//!
//! This module provides parsing logic for aspect ratio utilities,
//! such as `aspect-square`, `aspect-video`, `aspect-3/2`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct AspectRatioParser;

impl AspectRatioParser {
    pub fn new() -> Self { Self }

    /// Parse aspect ratio classes
    fn parse_aspect_ratio_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "aspect-auto" => Some(vec![CssProperty { name: "aspect-ratio".to_string(), value: "auto".to_string(), important: false }]),
            "aspect-square" => Some(vec![CssProperty { name: "aspect-ratio".to_string(), value: "1 / 1".to_string(), important: false }]),
            "aspect-video" => Some(vec![CssProperty { name: "aspect-ratio".to_string(), value: "16 / 9".to_string(), important: false }]),
            _ => {
                // Handle aspect-<ratio> classes like aspect-3/2, aspect-4/3, etc.
                if let Some(ratio) = class.strip_prefix("aspect-") {
                    if ratio.contains('/') {
                        return Some(vec![CssProperty { name: "aspect-ratio".to_string(), value: ratio.to_string(), important: false }]);
                    }
                }
                None
            }
        }
    }

    /// Parse arbitrary aspect ratio classes
    fn parse_arbitrary_aspect_ratio_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("aspect-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "aspect-ratio".to_string(), value: value.to_string(), important: false }]);
            }
        }
        None
    }

    /// Parse custom property aspect ratio classes
    fn parse_custom_property_aspect_ratio_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("aspect-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "aspect-ratio".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        None
    }
}

impl UtilityParser for AspectRatioParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_aspect_ratio_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_arbitrary_aspect_ratio_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_custom_property_aspect_ratio_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "aspect-auto", "aspect-square", "aspect-video",
            "aspect-*", "aspect-[*]", "aspect-(*)"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for AspectRatioParser {
    fn default() -> Self { Self::new() }
}
