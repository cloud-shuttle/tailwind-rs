//! Flex Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS flex utilities,
//! such as `flex-1`, `flex-auto`, `flex-initial`, `flex-none`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FlexParser {
    flex_map: HashMap<String, String>,
    fraction_map: HashMap<String, String>,
}

impl FlexParser {
    pub fn new() -> Self {
        let mut flex_map = HashMap::new();
        flex_map.insert("flex-auto".to_string(), "auto".to_string());
        flex_map.insert("flex-initial".to_string(), "0 auto".to_string());
        flex_map.insert("flex-none".to_string(), "none".to_string());

        // Add flex-1 through flex-12
        for i in 1..=12 {
            flex_map.insert(format!("flex-{}", i), i.to_string());
        }

        let mut fraction_map = HashMap::new();
        fraction_map.insert("1/2".to_string(), "50%".to_string());
        fraction_map.insert("1/3".to_string(), "33.333333%".to_string());
        fraction_map.insert("2/3".to_string(), "66.666667%".to_string());
        fraction_map.insert("1/4".to_string(), "25%".to_string());
        fraction_map.insert("2/4".to_string(), "50%".to_string());
        fraction_map.insert("3/4".to_string(), "75%".to_string());
        fraction_map.insert("1/5".to_string(), "20%".to_string());
        fraction_map.insert("2/5".to_string(), "40%".to_string());
        fraction_map.insert("3/5".to_string(), "60%".to_string());
        fraction_map.insert("4/5".to_string(), "80%".to_string());
        fraction_map.insert("1/6".to_string(), "16.666667%".to_string());
        fraction_map.insert("2/6".to_string(), "33.333333%".to_string());
        fraction_map.insert("3/6".to_string(), "50%".to_string());
        fraction_map.insert("4/6".to_string(), "66.666667%".to_string());
        fraction_map.insert("5/6".to_string(), "83.333333%".to_string());
        fraction_map.insert("1/12".to_string(), "8.333333%".to_string());
        fraction_map.insert("2/12".to_string(), "16.666667%".to_string());
        fraction_map.insert("3/12".to_string(), "25%".to_string());
        fraction_map.insert("4/12".to_string(), "33.333333%".to_string());
        fraction_map.insert("5/12".to_string(), "41.666667%".to_string());
        fraction_map.insert("6/12".to_string(), "50%".to_string());
        fraction_map.insert("7/12".to_string(), "58.333333%".to_string());
        fraction_map.insert("8/12".to_string(), "66.666667%".to_string());
        fraction_map.insert("9/12".to_string(), "75%".to_string());
        fraction_map.insert("10/12".to_string(), "83.333333%".to_string());
        fraction_map.insert("11/12".to_string(), "91.666667%".to_string());

        Self {
            flex_map,
            fraction_map,
        }
    }

    fn parse_flex_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(flex_value) = self.flex_map.get(class) {
            return Some(vec![CssProperty {
                name: "flex".to_string(),
                value: flex_value.clone(),
                important: false,
            }]);
        }
        None
    }

    fn parse_flex_fraction_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("flex-") {
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty {
                    name: "flex".to_string(),
                    value: fraction_value.clone(),
                    important: false,
                }]);
            }
        }
        None
    }

    fn parse_arbitrary_flex_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("flex-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "flex".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        None
    }

    fn parse_custom_property_flex_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("flex-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "flex".to_string(),
                    value: format!("var({})", prop),
                    important: false,
                }]);
            }
        }
        None
    }
}

impl UtilityParser for FlexParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_flex_class(class)
            .or_else(|| self.parse_flex_fraction_class(class))
            .or_else(|| self.parse_arbitrary_flex_class(class))
            .or_else(|| self.parse_custom_property_flex_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "flex-1",
            "flex-2",
            "flex-3",
            "flex-4",
            "flex-5",
            "flex-6",
            "flex-7",
            "flex-8",
            "flex-9",
            "flex-10",
            "flex-11",
            "flex-12",
            "flex-auto",
            "flex-initial",
            "flex-none",
            "flex-1/2",
            "flex-1/3",
            "flex-2/3",
            "flex-1/4",
            "flex-2/4",
            "flex-3/4",
            "flex-1/5",
            "flex-2/5",
            "flex-3/5",
            "flex-4/5",
            "flex-1/6",
            "flex-2/6",
            "flex-3/6",
            "flex-4/6",
            "flex-5/6",
            "flex-1/12",
            "flex-2/12",
            "flex-3/12",
            "flex-4/12",
            "flex-5/12",
            "flex-6/12",
            "flex-7/12",
            "flex-8/12",
            "flex-9/12",
            "flex-10/12",
            "flex-11/12",
            "flex-[*]",
            "flex-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Flexbox
    }
}

impl Default for FlexParser {
    fn default() -> Self {
        Self::new()
    }
}
