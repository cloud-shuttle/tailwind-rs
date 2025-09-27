//! Basic Transforms Parser
//!
//! This module provides parsing logic for basic transform utilities,
//! including translate-x and translate-y classes that were previously missing.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BasicTransformsParser {
    translate_x_map: HashMap<String, String>,
    translate_y_map: HashMap<String, String>,
}

impl BasicTransformsParser {
    pub fn new() -> Self {
        let mut translate_x_map = HashMap::new();
        let mut translate_y_map = HashMap::new();

        // Initialize translate-x values
        translate_x_map.insert("0".to_string(), "translateX(0)".to_string());
        translate_x_map.insert("1".to_string(), "translateX(0.25rem)".to_string());
        translate_x_map.insert("2".to_string(), "translateX(0.5rem)".to_string());
        translate_x_map.insert("3".to_string(), "translateX(0.75rem)".to_string());
        translate_x_map.insert("4".to_string(), "translateX(1rem)".to_string());
        translate_x_map.insert("5".to_string(), "translateX(1.25rem)".to_string());
        translate_x_map.insert("6".to_string(), "translateX(1.5rem)".to_string());
        translate_x_map.insert("7".to_string(), "translateX(1.75rem)".to_string());
        translate_x_map.insert("8".to_string(), "translateX(2rem)".to_string());
        translate_x_map.insert("9".to_string(), "translateX(2.25rem)".to_string());
        translate_x_map.insert("10".to_string(), "translateX(2.5rem)".to_string());
        translate_x_map.insert("11".to_string(), "translateX(2.75rem)".to_string());
        translate_x_map.insert("12".to_string(), "translateX(3rem)".to_string());
        translate_x_map.insert("14".to_string(), "translateX(3.5rem)".to_string());
        translate_x_map.insert("16".to_string(), "translateX(4rem)".to_string());
        translate_x_map.insert("20".to_string(), "translateX(5rem)".to_string());
        translate_x_map.insert("24".to_string(), "translateX(6rem)".to_string());
        translate_x_map.insert("28".to_string(), "translateX(7rem)".to_string());
        translate_x_map.insert("32".to_string(), "translateX(8rem)".to_string());
        translate_x_map.insert("36".to_string(), "translateX(9rem)".to_string());
        translate_x_map.insert("40".to_string(), "translateX(10rem)".to_string());
        translate_x_map.insert("44".to_string(), "translateX(11rem)".to_string());
        translate_x_map.insert("48".to_string(), "translateX(12rem)".to_string());
        translate_x_map.insert("52".to_string(), "translateX(13rem)".to_string());
        translate_x_map.insert("56".to_string(), "translateX(14rem)".to_string());
        translate_x_map.insert("60".to_string(), "translateX(15rem)".to_string());
        translate_x_map.insert("64".to_string(), "translateX(16rem)".to_string());
        translate_x_map.insert("72".to_string(), "translateX(18rem)".to_string());
        translate_x_map.insert("80".to_string(), "translateX(20rem)".to_string());
        translate_x_map.insert("96".to_string(), "translateX(24rem)".to_string());
        translate_x_map.insert("px".to_string(), "translateX(1px)".to_string());
        translate_x_map.insert("0.5".to_string(), "translateX(0.125rem)".to_string());
        translate_x_map.insert("1.5".to_string(), "translateX(0.375rem)".to_string());
        translate_x_map.insert("2.5".to_string(), "translateX(0.625rem)".to_string());
        translate_x_map.insert("3.5".to_string(), "translateX(0.875rem)".to_string());

        // Initialize translate-y values (same as translate-x)
        translate_y_map.insert("0".to_string(), "translateY(0)".to_string());
        translate_y_map.insert("1".to_string(), "translateY(0.25rem)".to_string());
        translate_y_map.insert("2".to_string(), "translateY(0.5rem)".to_string());
        translate_y_map.insert("3".to_string(), "translateY(0.75rem)".to_string());
        translate_y_map.insert("4".to_string(), "translateY(1rem)".to_string());
        translate_y_map.insert("5".to_string(), "translateY(1.25rem)".to_string());
        translate_y_map.insert("6".to_string(), "translateY(1.5rem)".to_string());
        translate_y_map.insert("7".to_string(), "translateY(1.75rem)".to_string());
        translate_y_map.insert("8".to_string(), "translateY(2rem)".to_string());
        translate_y_map.insert("9".to_string(), "translateY(2.25rem)".to_string());
        translate_y_map.insert("10".to_string(), "translateY(2.5rem)".to_string());
        translate_y_map.insert("11".to_string(), "translateY(2.75rem)".to_string());
        translate_y_map.insert("12".to_string(), "translateY(3rem)".to_string());
        translate_y_map.insert("14".to_string(), "translateY(3.5rem)".to_string());
        translate_y_map.insert("16".to_string(), "translateY(4rem)".to_string());
        translate_y_map.insert("20".to_string(), "translateY(5rem)".to_string());
        translate_y_map.insert("24".to_string(), "translateY(6rem)".to_string());
        translate_y_map.insert("28".to_string(), "translateY(7rem)".to_string());
        translate_y_map.insert("32".to_string(), "translateY(8rem)".to_string());
        translate_y_map.insert("36".to_string(), "translateY(9rem)".to_string());
        translate_y_map.insert("40".to_string(), "translateY(10rem)".to_string());
        translate_y_map.insert("44".to_string(), "translateY(11rem)".to_string());
        translate_y_map.insert("48".to_string(), "translateY(12rem)".to_string());
        translate_y_map.insert("52".to_string(), "translateY(13rem)".to_string());
        translate_y_map.insert("56".to_string(), "translateY(14rem)".to_string());
        translate_y_map.insert("60".to_string(), "translateY(15rem)".to_string());
        translate_y_map.insert("64".to_string(), "translateY(16rem)".to_string());
        translate_y_map.insert("72".to_string(), "translateY(18rem)".to_string());
        translate_y_map.insert("80".to_string(), "translateY(20rem)".to_string());
        translate_y_map.insert("96".to_string(), "translateY(24rem)".to_string());
        translate_y_map.insert("px".to_string(), "translateY(1px)".to_string());
        translate_y_map.insert("0.5".to_string(), "translateY(0.125rem)".to_string());
        translate_y_map.insert("1.5".to_string(), "translateY(0.375rem)".to_string());
        translate_y_map.insert("2.5".to_string(), "translateY(0.625rem)".to_string());
        translate_y_map.insert("3.5".to_string(), "translateY(0.875rem)".to_string());

        Self {
            translate_x_map,
            translate_y_map,
        }
    }

    /// Parse translate-x classes
    fn parse_translate_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("translate-x-") {
            if let Some(transform_value) = self.translate_x_map.get(value) {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: transform_value.clone(),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse translate-y classes
    fn parse_translate_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("translate-y-") {
            if let Some(transform_value) = self.translate_y_map.get(value) {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: transform_value.clone(),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for BasicTransformsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_translate_x_class(class)
            .or_else(|| self.parse_translate_y_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "translate-x-0", "translate-x-1", "translate-x-2", "translate-x-3", "translate-x-4",
            "translate-x-5", "translate-x-6", "translate-x-7", "translate-x-8", "translate-x-9",
            "translate-x-10", "translate-x-11", "translate-x-12", "translate-x-14", "translate-x-16",
            "translate-x-20", "translate-x-24", "translate-x-28", "translate-x-32", "translate-x-36",
            "translate-x-40", "translate-x-44", "translate-x-48", "translate-x-52", "translate-x-56",
            "translate-x-60", "translate-x-64", "translate-x-72", "translate-x-80", "translate-x-96",
            "translate-x-px", "translate-x-0.5", "translate-x-1.5", "translate-x-2.5", "translate-x-3.5",
            "translate-y-0", "translate-y-1", "translate-y-2", "translate-y-3", "translate-y-4",
            "translate-y-5", "translate-y-6", "translate-y-7", "translate-y-8", "translate-y-9",
            "translate-y-10", "translate-y-11", "translate-y-12", "translate-y-14", "translate-y-16",
            "translate-y-20", "translate-y-24", "translate-y-28", "translate-y-32", "translate-y-36",
            "translate-y-40", "translate-y-44", "translate-y-48", "translate-y-52", "translate-y-56",
            "translate-y-60", "translate-y-64", "translate-y-72", "translate-y-80", "translate-y-96",
            "translate-y-px", "translate-y-0.5", "translate-y-1.5", "translate-y-2.5", "translate-y-3.5",
        ]
    }

    fn get_priority(&self) -> u32 { 80 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Transforms }
}

impl Default for BasicTransformsParser {
    fn default() -> Self { Self::new() }
}
