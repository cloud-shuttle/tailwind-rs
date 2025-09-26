//! Grid Auto Columns Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS grid-auto-columns utilities,
//! such as `auto-cols-auto`, `auto-cols-min`, `auto-cols-max`, `auto-cols-fr`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GridAutoColumnsParser {
    auto_cols_map: HashMap<String, String>,
}

impl GridAutoColumnsParser {
    pub fn new() -> Self {
        let mut auto_cols_map = HashMap::new();
        auto_cols_map.insert("auto-cols-auto".to_string(), "auto".to_string());
        auto_cols_map.insert("auto-cols-min".to_string(), "min-content".to_string());
        auto_cols_map.insert("auto-cols-max".to_string(), "max-content".to_string());
        auto_cols_map.insert("auto-cols-fr".to_string(), "minmax(0, 1fr)".to_string());
        
        Self { auto_cols_map }
    }

    fn parse_auto_cols_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(auto_cols_value) = self.auto_cols_map.get(class) {
            return Some(vec![CssProperty { name: "grid-auto-columns".to_string(), value: auto_cols_value.clone(), important: false }]);
        }
        None
    }

    fn parse_arbitrary_auto_cols_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("auto-cols-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "grid-auto-columns".to_string(), value: value.to_string(), important: false }]);
            }
        }
        None
    }

    fn parse_custom_property_auto_cols_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("auto-cols-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "grid-auto-columns".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        None
    }
}

impl UtilityParser for GridAutoColumnsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_auto_cols_class(class)
            .or_else(|| self.parse_arbitrary_auto_cols_class(class))
            .or_else(|| self.parse_custom_property_auto_cols_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "auto-cols-auto", "auto-cols-min", "auto-cols-max", "auto-cols-fr",
            "auto-cols-[*]", "auto-cols-(*)"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Grid }
}

impl Default for GridAutoColumnsParser {
    fn default() -> Self { Self::new() }
}
