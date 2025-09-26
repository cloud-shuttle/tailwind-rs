//! Z-Index Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS z-index utilities,
//! such as `z-0`, `z-10`, `z-50`, `z-auto`, `z-[100]`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ZIndexParser {
    z_index_map: HashMap<String, String>,
}

impl ZIndexParser {
    pub fn new() -> Self {
        let mut z_index_map = HashMap::new();
        z_index_map.insert("0".to_string(), "0".to_string());
        z_index_map.insert("10".to_string(), "10".to_string());
        z_index_map.insert("20".to_string(), "20".to_string());
        z_index_map.insert("30".to_string(), "30".to_string());
        z_index_map.insert("40".to_string(), "40".to_string());
        z_index_map.insert("50".to_string(), "50".to_string());
        z_index_map.insert("auto".to_string(), "auto".to_string());
        
        Self { z_index_map }
    }

    fn parse_z_index_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("z-") {
            if let Some(z_index_value) = self.z_index_map.get(value) {
                return Some(vec![CssProperty { name: "z-index".to_string(), value: z_index_value.clone(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("-z-") {
            if let Some(z_index_value) = self.z_index_map.get(value) {
                return Some(vec![CssProperty { name: "z-index".to_string(), value: format!("-{}", z_index_value), important: false }]);
            }
        }
        None
    }

    fn parse_arbitrary_z_index_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("z-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "z-index".to_string(), value: value.to_string(), important: false }]);
            }
        }
        None
    }

    fn parse_custom_property_z_index_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("z-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "z-index".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        None
    }
}

impl UtilityParser for ZIndexParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_z_index_class(class)
            .or_else(|| self.parse_arbitrary_z_index_class(class))
            .or_else(|| self.parse_custom_property_z_index_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "z-0", "z-10", "z-20", "z-30", "z-40", "z-50", "z-auto",
            "-z-0", "-z-10", "-z-20", "-z-30", "-z-40", "-z-50",
            "z-[*]", "z-(*)"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for ZIndexParser {
    fn default() -> Self { Self::new() }
}
