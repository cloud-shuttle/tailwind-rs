//! Justify Content Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS justify-content utilities,
//! such as `justify-start`, `justify-end`, `justify-center`, `justify-between`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct JustifyContentParser {
    justify_map: HashMap<String, String>,
}

impl JustifyContentParser {
    pub fn new() -> Self {
        let mut justify_map = HashMap::new();
        justify_map.insert("justify-start".to_string(), "flex-start".to_string());
        justify_map.insert("justify-end".to_string(), "flex-end".to_string());
        justify_map.insert("justify-end-safe".to_string(), "safe flex-end".to_string());
        justify_map.insert("justify-center".to_string(), "center".to_string());
        justify_map.insert("justify-center-safe".to_string(), "safe center".to_string());
        justify_map.insert("justify-between".to_string(), "space-between".to_string());
        justify_map.insert("justify-around".to_string(), "space-around".to_string());
        justify_map.insert("justify-evenly".to_string(), "space-evenly".to_string());
        justify_map.insert("justify-stretch".to_string(), "stretch".to_string());
        justify_map.insert("justify-baseline".to_string(), "baseline".to_string());
        justify_map.insert("justify-normal".to_string(), "normal".to_string());
        
        Self { justify_map }
    }

    fn parse_justify_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(justify_value) = self.justify_map.get(class) {
            return Some(vec![CssProperty { name: "justify-content".to_string(), value: justify_value.clone(), important: false }]);
        }
        None
    }
}

impl UtilityParser for JustifyContentParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_justify_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "justify-start", "justify-end", "justify-end-safe", "justify-center", "justify-center-safe",
            "justify-between", "justify-around", "justify-evenly", "justify-stretch", "justify-baseline", "justify-normal"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Flexbox }
}

impl Default for JustifyContentParser {
    fn default() -> Self { Self::new() }
}
