//! Grid Auto Rows Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS grid-auto-rows utilities,
//! such as `auto-rows-auto`, `auto-rows-min`, `auto-rows-max`, `auto-rows-fr`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GridAutoRowsParser {
    auto_rows_map: HashMap<String, String>,
}

impl GridAutoRowsParser {
    pub fn new() -> Self {
        let mut auto_rows_map = HashMap::new();
        auto_rows_map.insert("auto-rows-auto".to_string(), "auto".to_string());
        auto_rows_map.insert("auto-rows-min".to_string(), "min-content".to_string());
        auto_rows_map.insert("auto-rows-max".to_string(), "max-content".to_string());
        auto_rows_map.insert("auto-rows-fr".to_string(), "minmax(0, 1fr)".to_string());

        Self { auto_rows_map }
    }

    fn parse_auto_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(auto_rows_value) = self.auto_rows_map.get(class) {
            return Some(vec![CssProperty {
                name: "grid-auto-rows".to_string(),
                value: auto_rows_value.clone(),
                important: false,
            }]);
        }
        None
    }

    fn parse_arbitrary_auto_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("auto-rows-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "grid-auto-rows".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    fn parse_custom_property_auto_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("auto-rows-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "grid-auto-rows".to_string(),
                    value: format!("var({})", prop),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for GridAutoRowsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_auto_rows_class(class)
            .or_else(|| self.parse_arbitrary_auto_rows_class(class))
            .or_else(|| self.parse_custom_property_auto_rows_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "auto-rows-auto",
            "auto-rows-min",
            "auto-rows-max",
            "auto-rows-fr",
            "auto-rows-[*]",
            "auto-rows-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Grid
    }
}

impl Default for GridAutoRowsParser {
    fn default() -> Self {
        Self::new()
    }
}
