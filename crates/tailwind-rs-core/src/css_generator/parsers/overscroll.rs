//! Overscroll Behavior Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS overscroll behavior utilities,
//! such as `overscroll-auto`, `overscroll-contain`, `overscroll-x-auto`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OverscrollParser {
    overscroll_map: HashMap<String, String>,
}

impl OverscrollParser {
    pub fn new() -> Self {
        let mut overscroll_map = HashMap::new();
        overscroll_map.insert("auto".to_string(), "auto".to_string());
        overscroll_map.insert("contain".to_string(), "contain".to_string());
        overscroll_map.insert("none".to_string(), "none".to_string());

        Self { overscroll_map }
    }

    fn parse_overscroll_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("overscroll-") {
            if let Some(overscroll_value) = self.overscroll_map.get(value) {
                return Some(vec![CssProperty {
                    name: "overscroll-behavior".to_string(),
                    value: overscroll_value.clone(),
                    important: false,
                }]);
            }
        }
        None
    }

    fn parse_overscroll_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("overscroll-x-") {
            if let Some(overscroll_value) = self.overscroll_map.get(value) {
                return Some(vec![CssProperty {
                    name: "overscroll-behavior-x".to_string(),
                    value: overscroll_value.clone(),
                    important: false,
                }]);
            }
        }
        None
    }

    fn parse_overscroll_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("overscroll-y-") {
            if let Some(overscroll_value) = self.overscroll_map.get(value) {
                return Some(vec![CssProperty {
                    name: "overscroll-behavior-y".to_string(),
                    value: overscroll_value.clone(),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for OverscrollParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_overscroll_class(class)
            .or_else(|| self.parse_overscroll_x_class(class))
            .or_else(|| self.parse_overscroll_y_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "overscroll-auto",
            "overscroll-contain",
            "overscroll-none",
            "overscroll-x-auto",
            "overscroll-x-contain",
            "overscroll-x-none",
            "overscroll-y-auto",
            "overscroll-y-contain",
            "overscroll-y-none",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

impl Default for OverscrollParser {
    fn default() -> Self {
        Self::new()
    }
}
