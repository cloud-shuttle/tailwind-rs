//! Justify Items Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS justify-items utilities,
//! such as `justify-items-start`, `justify-items-end`, `justify-items-center`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct JustifyItemsParser {
    justify_items_map: HashMap<String, String>,
}

impl JustifyItemsParser {
    pub fn new() -> Self {
        let mut justify_items_map = HashMap::new();
        justify_items_map.insert("justify-items-start".to_string(), "start".to_string());
        justify_items_map.insert("justify-items-end".to_string(), "end".to_string());
        justify_items_map.insert("justify-items-end-safe".to_string(), "safe end".to_string());
        justify_items_map.insert("justify-items-center".to_string(), "center".to_string());
        justify_items_map.insert(
            "justify-items-center-safe".to_string(),
            "safe center".to_string(),
        );
        justify_items_map.insert("justify-items-stretch".to_string(), "stretch".to_string());
        justify_items_map.insert("justify-items-normal".to_string(), "normal".to_string());

        Self { justify_items_map }
    }

    fn parse_justify_items_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(justify_items_value) = self.justify_items_map.get(class) {
            return Some(vec![CssProperty {
                name: "justify-items".to_string(),
                value: justify_items_value.clone(),
                important: false,
            }]);
        }
        None
    }
}

impl UtilityParser for JustifyItemsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_justify_items_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "justify-items-start",
            "justify-items-end",
            "justify-items-end-safe",
            "justify-items-center",
            "justify-items-center-safe",
            "justify-items-stretch",
            "justify-items-normal",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Grid
    }
}

impl Default for JustifyItemsParser {
    fn default() -> Self {
        Self::new()
    }
}
