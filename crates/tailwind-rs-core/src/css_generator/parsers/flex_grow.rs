//! Flex Grow Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS flex-grow utilities,
//! such as `grow`, `grow-0`, `grow-3`, `grow-[25vw]`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FlexGrowParser {
    grow_map: HashMap<String, String>,
}

impl FlexGrowParser {
    pub fn new() -> Self {
        let mut grow_map = HashMap::new();
        grow_map.insert("grow".to_string(), "1".to_string());
        grow_map.insert("grow-0".to_string(), "0".to_string());
        
        // Add grow-1 through grow-12
        for i in 1..=12 {
            grow_map.insert(format!("grow-{}", i), i.to_string());
        }
        
        Self { grow_map }
    }

    fn parse_grow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(grow_value) = self.grow_map.get(class) {
            return Some(vec![CssProperty { name: "flex-grow".to_string(), value: grow_value.clone(), important: false }]);
        }
        None
    }

    fn parse_arbitrary_grow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("grow-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "flex-grow".to_string(), value: value.to_string(), important: false }]);
            }
        }
        None
    }

    fn parse_custom_property_grow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("grow-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "flex-grow".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        None
    }
}

impl UtilityParser for FlexGrowParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_grow_class(class)
            .or_else(|| self.parse_arbitrary_grow_class(class))
            .or_else(|| self.parse_custom_property_grow_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "grow", "grow-0", "grow-1", "grow-2", "grow-3", "grow-4", "grow-5", "grow-6", "grow-7", "grow-8", "grow-9", "grow-10", "grow-11", "grow-12",
            "grow-[*]", "grow-(*)"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Flexbox }
}

impl Default for FlexGrowParser {
    fn default() -> Self { Self::new() }
}
