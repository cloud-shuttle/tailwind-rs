//! Justify Self Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS justify-self utilities,
//! such as `justify-self-auto`, `justify-self-start`, `justify-self-center`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct JustifySelfParser {
    justify_self_map: HashMap<String, String>,
}

impl JustifySelfParser {
    pub fn new() -> Self {
        let mut justify_self_map = HashMap::new();
        justify_self_map.insert("justify-self-auto".to_string(), "auto".to_string());
        justify_self_map.insert("justify-self-start".to_string(), "start".to_string());
        justify_self_map.insert("justify-self-center".to_string(), "center".to_string());
        justify_self_map.insert("justify-self-center-safe".to_string(), "safe center".to_string());
        justify_self_map.insert("justify-self-end".to_string(), "end".to_string());
        justify_self_map.insert("justify-self-end-safe".to_string(), "safe end".to_string());
        justify_self_map.insert("justify-self-stretch".to_string(), "stretch".to_string());
        
        Self { justify_self_map }
    }

    fn parse_justify_self_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(justify_self_value) = self.justify_self_map.get(class) {
            return Some(vec![CssProperty { name: "justify-self".to_string(), value: justify_self_value.clone(), important: false }]);
        }
        None
    }
}

impl UtilityParser for JustifySelfParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_justify_self_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "justify-self-auto", "justify-self-start", "justify-self-center", "justify-self-center-safe",
            "justify-self-end", "justify-self-end-safe", "justify-self-stretch"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Grid }
}

impl Default for JustifySelfParser {
    fn default() -> Self { Self::new() }
}
