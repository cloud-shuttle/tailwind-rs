//! Gap Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS gap utilities,
//! such as `gap-0`, `gap-1`, `gap-x-2`, `gap-y-4`, `gap-[10vw]`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GapParser {
    gap_map: HashMap<String, String>,
}

impl GapParser {
    pub fn new() -> Self {
        let mut gap_map = HashMap::new();
        gap_map.insert("gap-0".to_string(), "0".to_string());
        gap_map.insert("gap-px".to_string(), "1px".to_string());
        gap_map.insert("gap-0.5".to_string(), "0.125rem".to_string());
        gap_map.insert("gap-1".to_string(), "0.25rem".to_string());
        gap_map.insert("gap-1.5".to_string(), "0.375rem".to_string());
        gap_map.insert("gap-2".to_string(), "0.5rem".to_string());
        gap_map.insert("gap-2.5".to_string(), "0.625rem".to_string());
        gap_map.insert("gap-3".to_string(), "0.75rem".to_string());
        gap_map.insert("gap-3.5".to_string(), "0.875rem".to_string());
        gap_map.insert("gap-4".to_string(), "1rem".to_string());
        gap_map.insert("gap-5".to_string(), "1.25rem".to_string());
        gap_map.insert("gap-6".to_string(), "1.5rem".to_string());
        gap_map.insert("gap-7".to_string(), "1.75rem".to_string());
        gap_map.insert("gap-8".to_string(), "2rem".to_string());
        gap_map.insert("gap-9".to_string(), "2.25rem".to_string());
        gap_map.insert("gap-10".to_string(), "2.5rem".to_string());
        gap_map.insert("gap-11".to_string(), "2.75rem".to_string());
        gap_map.insert("gap-12".to_string(), "3rem".to_string());
        gap_map.insert("gap-14".to_string(), "3.5rem".to_string());
        gap_map.insert("gap-16".to_string(), "4rem".to_string());
        gap_map.insert("gap-20".to_string(), "5rem".to_string());
        gap_map.insert("gap-24".to_string(), "6rem".to_string());
        gap_map.insert("gap-28".to_string(), "7rem".to_string());
        gap_map.insert("gap-32".to_string(), "8rem".to_string());
        gap_map.insert("gap-36".to_string(), "9rem".to_string());
        gap_map.insert("gap-40".to_string(), "10rem".to_string());
        gap_map.insert("gap-44".to_string(), "11rem".to_string());
        gap_map.insert("gap-48".to_string(), "12rem".to_string());
        gap_map.insert("gap-52".to_string(), "13rem".to_string());
        gap_map.insert("gap-56".to_string(), "14rem".to_string());
        gap_map.insert("gap-60".to_string(), "15rem".to_string());
        gap_map.insert("gap-64".to_string(), "16rem".to_string());
        gap_map.insert("gap-72".to_string(), "18rem".to_string());
        gap_map.insert("gap-80".to_string(), "20rem".to_string());
        gap_map.insert("gap-96".to_string(), "24rem".to_string());
        
        // Add gap-x-* variants
        for (key, value) in gap_map.clone() {
            if key.starts_with("gap-") && !key.starts_with("gap-x-") && !key.starts_with("gap-y-") {
                let x_key = key.replace("gap-", "gap-x-");
                gap_map.insert(x_key, value.clone());
            }
        }
        
        // Add gap-y-* variants
        for (key, value) in gap_map.clone() {
            if key.starts_with("gap-") && !key.starts_with("gap-x-") && !key.starts_with("gap-y-") {
                let y_key = key.replace("gap-", "gap-y-");
                gap_map.insert(y_key, value.clone());
            }
        }
        
        Self { gap_map }
    }

    fn parse_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(gap_value) = self.gap_map.get(class) {
            if class.starts_with("gap-x-") {
                return Some(vec![CssProperty { name: "column-gap".to_string(), value: gap_value.clone(), important: false }]);
            } else if class.starts_with("gap-y-") {
                return Some(vec![CssProperty { name: "row-gap".to_string(), value: gap_value.clone(), important: false }]);
            } else if class.starts_with("gap-") {
                return Some(vec![CssProperty { name: "gap".to_string(), value: gap_value.clone(), important: false }]);
            }
        }
        None
    }

    fn parse_arbitrary_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("gap-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "gap".to_string(), value: value.to_string(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("gap-x-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "column-gap".to_string(), value: value.to_string(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("gap-y-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "row-gap".to_string(), value: value.to_string(), important: false }]);
            }
        }
        None
    }

    fn parse_custom_property_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("gap-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "gap".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("gap-x-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "column-gap".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("gap-y-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "row-gap".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        None
    }
}

impl UtilityParser for GapParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_gap_class(class)
            .or_else(|| self.parse_arbitrary_gap_class(class))
            .or_else(|| self.parse_custom_property_gap_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "gap-0", "gap-px", "gap-0.5", "gap-1", "gap-1.5", "gap-2", "gap-2.5", "gap-3", "gap-3.5", "gap-4",
            "gap-5", "gap-6", "gap-7", "gap-8", "gap-9", "gap-10", "gap-11", "gap-12", "gap-14", "gap-16",
            "gap-20", "gap-24", "gap-28", "gap-32", "gap-36", "gap-40", "gap-44", "gap-48", "gap-52", "gap-56",
            "gap-60", "gap-64", "gap-72", "gap-80", "gap-96",
            "gap-x-0", "gap-x-px", "gap-x-0.5", "gap-x-1", "gap-x-1.5", "gap-x-2", "gap-x-2.5", "gap-x-3", "gap-x-3.5", "gap-x-4",
            "gap-x-5", "gap-x-6", "gap-x-7", "gap-x-8", "gap-x-9", "gap-x-10", "gap-x-11", "gap-x-12", "gap-x-14", "gap-x-16",
            "gap-x-20", "gap-x-24", "gap-x-28", "gap-x-32", "gap-x-36", "gap-x-40", "gap-x-44", "gap-x-48", "gap-x-52", "gap-x-56",
            "gap-x-60", "gap-x-64", "gap-x-72", "gap-x-80", "gap-x-96",
            "gap-y-0", "gap-y-px", "gap-y-0.5", "gap-y-1", "gap-y-1.5", "gap-y-2", "gap-y-2.5", "gap-y-3", "gap-y-3.5", "gap-y-4",
            "gap-y-5", "gap-y-6", "gap-y-7", "gap-y-8", "gap-y-9", "gap-y-10", "gap-y-11", "gap-y-12", "gap-y-14", "gap-y-16",
            "gap-y-20", "gap-y-24", "gap-y-28", "gap-y-32", "gap-y-36", "gap-y-40", "gap-y-44", "gap-y-48", "gap-y-52", "gap-y-56",
            "gap-y-60", "gap-y-64", "gap-y-72", "gap-y-80", "gap-y-96",
            "gap-[*]", "gap-x-[*]", "gap-y-[*]", "gap-(*)", "gap-x-(*)", "gap-y-(*)"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Flexbox }
}

impl Default for GapParser {
    fn default() -> Self { Self::new() }
}
