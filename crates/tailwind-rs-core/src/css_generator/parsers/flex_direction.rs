//! Flex Direction Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS flex-direction utilities,
//! such as `flex-row`, `flex-row-reverse`, `flex-col`, `flex-col-reverse`.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FlexDirectionParser {
    direction_map: HashMap<String, String>,
}

impl FlexDirectionParser {
    pub fn new() -> Self {
        let mut direction_map = HashMap::new();
        direction_map.insert("flex-row".to_string(), "row".to_string());
        direction_map.insert("flex-row-reverse".to_string(), "row-reverse".to_string());
        direction_map.insert("flex-col".to_string(), "column".to_string());
        direction_map.insert("flex-col-reverse".to_string(), "column-reverse".to_string());

        Self { direction_map }
    }

    fn parse_flex_direction_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(direction_value) = self.direction_map.get(class) {
            return Some(vec![CssProperty {
                name: "flex-direction".to_string(),
                value: direction_value.clone(),
                important: false,
            }]);
        }
        None
    }
}

impl UtilityParser for FlexDirectionParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_flex_direction_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "flex-row",
            "flex-row-reverse",
            "flex-col",
            "flex-col-reverse",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Flexbox
    }
}

impl Default for FlexDirectionParser {
    fn default() -> Self {
        Self::new()
    }
}
