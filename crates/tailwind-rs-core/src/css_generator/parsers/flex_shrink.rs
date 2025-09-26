//! Flex Shrink Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS flex-shrink utilities,
//! such as `shrink`, `shrink-0`, `shrink-3`, `shrink-[calc(100vw-var(--sidebar))]`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FlexShrinkParser {
    shrink_map: HashMap<String, String>,
}

impl FlexShrinkParser {
    pub fn new() -> Self {
        let mut shrink_map = HashMap::new();
        shrink_map.insert("shrink".to_string(), "1".to_string());
        shrink_map.insert("shrink-0".to_string(), "0".to_string());

        // Add shrink-1 through shrink-12
        for i in 1..=12 {
            shrink_map.insert(format!("shrink-{}", i), i.to_string());
        }

        Self { shrink_map }
    }

    fn parse_shrink_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(shrink_value) = self.shrink_map.get(class) {
            return Some(vec![CssProperty {
                name: "flex-shrink".to_string(),
                value: shrink_value.clone(),
                important: false,
            }]);
        }
        None
    }

    fn parse_arbitrary_shrink_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("shrink-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "flex-shrink".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    fn parse_custom_property_shrink_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("shrink-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "flex-shrink".to_string(),
                    value: format!("var({})", prop),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for FlexShrinkParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_shrink_class(class)
            .or_else(|| self.parse_arbitrary_shrink_class(class))
            .or_else(|| self.parse_custom_property_shrink_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "shrink",
            "shrink-0",
            "shrink-1",
            "shrink-2",
            "shrink-3",
            "shrink-4",
            "shrink-5",
            "shrink-6",
            "shrink-7",
            "shrink-8",
            "shrink-9",
            "shrink-10",
            "shrink-11",
            "shrink-12",
            "shrink-[*]",
            "shrink-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Flexbox
    }
}

impl Default for FlexShrinkParser {
    fn default() -> Self {
        Self::new()
    }
}
