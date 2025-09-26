//! Overflow Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS overflow utilities,
//! such as `overflow-auto`, `overflow-hidden`, `overflow-x-auto`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OverflowParser {
    overflow_map: HashMap<String, String>,
}

impl OverflowParser {
    pub fn new() -> Self {
        let mut overflow_map = HashMap::new();
        overflow_map.insert("auto".to_string(), "auto".to_string());
        overflow_map.insert("hidden".to_string(), "hidden".to_string());
        overflow_map.insert("clip".to_string(), "clip".to_string());
        overflow_map.insert("visible".to_string(), "visible".to_string());
        overflow_map.insert("scroll".to_string(), "scroll".to_string());
        
        Self { overflow_map }
    }

    fn parse_overflow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("overflow-") {
            if let Some(overflow_value) = self.overflow_map.get(value) {
                return Some(vec![CssProperty { name: "overflow".to_string(), value: overflow_value.clone(), important: false }]);
            }
        }
        None
    }

    fn parse_overflow_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("overflow-x-") {
            if let Some(overflow_value) = self.overflow_map.get(value) {
                return Some(vec![CssProperty { name: "overflow-x".to_string(), value: overflow_value.clone(), important: false }]);
            }
        }
        None
    }

    fn parse_overflow_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("overflow-y-") {
            if let Some(overflow_value) = self.overflow_map.get(value) {
                return Some(vec![CssProperty { name: "overflow-y".to_string(), value: overflow_value.clone(), important: false }]);
            }
        }
        None
    }
}

impl UtilityParser for OverflowParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_overflow_class(class)
            .or_else(|| self.parse_overflow_x_class(class))
            .or_else(|| self.parse_overflow_y_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "overflow-auto", "overflow-hidden", "overflow-clip", "overflow-visible", "overflow-scroll",
            "overflow-x-auto", "overflow-x-hidden", "overflow-x-clip", "overflow-x-visible", "overflow-x-scroll",
            "overflow-y-auto", "overflow-y-hidden", "overflow-y-clip", "overflow-y-visible", "overflow-y-scroll"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for OverflowParser {
    fn default() -> Self { Self::new() }
}
