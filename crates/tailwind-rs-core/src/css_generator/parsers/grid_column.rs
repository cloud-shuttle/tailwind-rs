//! Grid Column Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS grid-column utilities,
//! such as `col-span-1`, `col-span-2`, `col-start-1`, `col-end-2`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GridColumnParser {
    col_span_map: HashMap<String, String>,
    col_start_map: HashMap<String, String>,
    col_end_map: HashMap<String, String>,
}

impl GridColumnParser {
    pub fn new() -> Self {
        let mut col_span_map = HashMap::new();
        col_span_map.insert("col-span-full".to_string(), "1 / -1".to_string());
        
        // Add col-span-1 through col-span-12
        for i in 1..=12 {
            col_span_map.insert(format!("col-span-{}", i), format!("span {} / span {}", i, i));
        }

        let mut col_start_map = HashMap::new();
        col_start_map.insert("col-start-auto".to_string(), "auto".to_string());
        
        // Add col-start-1 through col-start-12
        for i in 1..=12 {
            col_start_map.insert(format!("col-start-{}", i), i.to_string());
        }
        
        // Add negative col-start values
        for i in 1..=12 {
            col_start_map.insert(format!("-col-start-{}", i), format!("calc({} * -1)", i));
        }

        let mut col_end_map = HashMap::new();
        col_end_map.insert("col-end-auto".to_string(), "auto".to_string());
        
        // Add col-end-1 through col-end-12
        for i in 1..=12 {
            col_end_map.insert(format!("col-end-{}", i), i.to_string());
        }
        
        // Add negative col-end values
        for i in 1..=12 {
            col_end_map.insert(format!("-col-end-{}", i), format!("calc({} * -1)", i));
        }
        
        Self { col_span_map, col_start_map, col_end_map }
    }

    fn parse_col_span_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(col_span_value) = self.col_span_map.get(class) {
            return Some(vec![CssProperty { name: "grid-column".to_string(), value: col_span_value.clone(), important: false }]);
        }
        None
    }

    fn parse_col_start_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(col_start_value) = self.col_start_map.get(class) {
            return Some(vec![CssProperty { name: "grid-column-start".to_string(), value: col_start_value.clone(), important: false }]);
        }
        None
    }

    fn parse_col_end_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(col_end_value) = self.col_end_map.get(class) {
            return Some(vec![CssProperty { name: "grid-column-end".to_string(), value: col_end_value.clone(), important: false }]);
        }
        None
    }

    fn parse_arbitrary_col_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("col-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "grid-column".to_string(), value: value.to_string(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("col-span-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "grid-column".to_string(), value: format!("span {} / span {}", value, value), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("col-start-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "grid-column-start".to_string(), value: value.to_string(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("col-end-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "grid-column-end".to_string(), value: value.to_string(), important: false }]);
            }
        }
        None
    }

    fn parse_custom_property_col_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("col-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "grid-column".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("col-span-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "grid-column".to_string(), value: format!("span var({}) / span var({})", prop, prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("col-start-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "grid-column-start".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("col-end-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "grid-column-end".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        None
    }
}

impl UtilityParser for GridColumnParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_col_span_class(class)
            .or_else(|| self.parse_col_start_class(class))
            .or_else(|| self.parse_col_end_class(class))
            .or_else(|| self.parse_arbitrary_col_class(class))
            .or_else(|| self.parse_custom_property_col_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "col-span-1", "col-span-2", "col-span-3", "col-span-4", "col-span-5", "col-span-6", "col-span-7", "col-span-8", "col-span-9", "col-span-10", "col-span-11", "col-span-12", "col-span-full",
            "col-start-1", "col-start-2", "col-start-3", "col-start-4", "col-start-5", "col-start-6", "col-start-7", "col-start-8", "col-start-9", "col-start-10", "col-start-11", "col-start-12", "col-start-auto",
            "-col-start-1", "-col-start-2", "-col-start-3", "-col-start-4", "-col-start-5", "-col-start-6", "-col-start-7", "-col-start-8", "-col-start-9", "-col-start-10", "-col-start-11", "-col-start-12",
            "col-end-1", "col-end-2", "col-end-3", "col-end-4", "col-end-5", "col-end-6", "col-end-7", "col-end-8", "col-end-9", "col-end-10", "col-end-11", "col-end-12", "col-end-auto",
            "-col-end-1", "-col-end-2", "-col-end-3", "-col-end-4", "-col-end-5", "-col-end-6", "-col-end-7", "-col-end-8", "-col-end-9", "-col-end-10", "-col-end-11", "-col-end-12",
            "col-[*]", "col-span-[*]", "col-start-[*]", "col-end-[*]", "col-(*)", "col-span-(*)", "col-start-(*)", "col-end-(*)"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Grid }
}

impl Default for GridColumnParser {
    fn default() -> Self { Self::new() }
}
