//! Flex Wrap Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS flex-wrap utilities,
//! such as `flex-nowrap`, `flex-wrap`, `flex-wrap-reverse`.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FlexWrapParser {
    wrap_map: HashMap<String, String>,
}

impl FlexWrapParser {
    pub fn new() -> Self {
        let mut wrap_map = HashMap::new();
        wrap_map.insert("flex-nowrap".to_string(), "nowrap".to_string());
        wrap_map.insert("flex-wrap".to_string(), "wrap".to_string());
        wrap_map.insert("flex-wrap-reverse".to_string(), "wrap-reverse".to_string());

        Self { wrap_map }
    }

    fn parse_flex_wrap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(wrap_value) = self.wrap_map.get(class) {
            return Some(vec![CssProperty {
                name: "flex-wrap".to_string(),
                value: wrap_value.clone(),
                important: false,
            }]);
        }
        None
    }
}

impl UtilityParser for FlexWrapParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_flex_wrap_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["flex-nowrap", "flex-wrap", "flex-wrap-reverse"]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Flexbox
    }
}

impl Default for FlexWrapParser {
    fn default() -> Self {
        Self::new()
    }
}
