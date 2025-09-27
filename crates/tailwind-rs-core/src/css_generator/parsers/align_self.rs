//! Align Self Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS align-self utilities,
//! such as `self-auto`, `self-start`, `self-end`, `self-center`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AlignSelfParser {
    self_map: HashMap<String, String>,
}

impl AlignSelfParser {
    pub fn new() -> Self {
        let mut self_map = HashMap::new();
        self_map.insert("self-auto".to_string(), "auto".to_string());
        self_map.insert("self-start".to_string(), "flex-start".to_string());
        self_map.insert("self-end".to_string(), "flex-end".to_string());
        self_map.insert("self-end-safe".to_string(), "safe flex-end".to_string());
        self_map.insert("self-center".to_string(), "center".to_string());
        self_map.insert("self-center-safe".to_string(), "safe center".to_string());
        self_map.insert("self-stretch".to_string(), "stretch".to_string());
        self_map.insert("self-baseline".to_string(), "baseline".to_string());
        self_map.insert(
            "self-baseline-last".to_string(),
            "last baseline".to_string(),
        );

        Self { self_map }
    }

    fn parse_self_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(self_value) = self.self_map.get(class) {
            return Some(vec![CssProperty {
                name: "align-self".to_string(),
                value: self_value.clone(),
                important: false,
            }]);
        }
        None
    }
}

impl UtilityParser for AlignSelfParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_self_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "self-auto",
            "self-start",
            "self-end",
            "self-end-safe",
            "self-center",
            "self-center-safe",
            "self-stretch",
            "self-baseline",
            "self-baseline-last",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Flexbox
    }
}

impl Default for AlignSelfParser {
    fn default() -> Self {
        Self::new()
    }
}
