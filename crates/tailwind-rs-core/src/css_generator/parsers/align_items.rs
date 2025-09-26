//! Align Items Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS align-items utilities,
//! such as `items-start`, `items-end`, `items-center`, `items-baseline`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AlignItemsParser {
    items_map: HashMap<String, String>,
}

impl AlignItemsParser {
    pub fn new() -> Self {
        let mut items_map = HashMap::new();
        items_map.insert("items-start".to_string(), "flex-start".to_string());
        items_map.insert("items-end".to_string(), "flex-end".to_string());
        items_map.insert("items-end-safe".to_string(), "safe flex-end".to_string());
        items_map.insert("items-center".to_string(), "center".to_string());
        items_map.insert("items-center-safe".to_string(), "safe center".to_string());
        items_map.insert("items-baseline".to_string(), "baseline".to_string());
        items_map.insert(
            "items-baseline-last".to_string(),
            "last baseline".to_string(),
        );
        items_map.insert("items-stretch".to_string(), "stretch".to_string());

        Self { items_map }
    }

    fn parse_items_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(items_value) = self.items_map.get(class) {
            return Some(vec![CssProperty {
                name: "align-items".to_string(),
                value: items_value.clone(),
                important: false,
            }]);
        }
        None
    }
}

impl UtilityParser for AlignItemsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_items_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "items-start",
            "items-end",
            "items-end-safe",
            "items-center",
            "items-center-safe",
            "items-baseline",
            "items-baseline-last",
            "items-stretch",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Flexbox
    }
}

impl Default for AlignItemsParser {
    fn default() -> Self {
        Self::new()
    }
}
