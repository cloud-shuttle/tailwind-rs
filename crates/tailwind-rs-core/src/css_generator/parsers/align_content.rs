//! Align Content Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS align-content utilities,
//! such as `content-normal`, `content-center`, `content-start`, `content-end`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AlignContentParser {
    content_map: HashMap<String, String>,
}

impl AlignContentParser {
    pub fn new() -> Self {
        let mut content_map = HashMap::new();
        content_map.insert("content-normal".to_string(), "normal".to_string());
        content_map.insert("content-center".to_string(), "center".to_string());
        content_map.insert("content-start".to_string(), "flex-start".to_string());
        content_map.insert("content-end".to_string(), "flex-end".to_string());
        content_map.insert("content-between".to_string(), "space-between".to_string());
        content_map.insert("content-around".to_string(), "space-around".to_string());
        content_map.insert("content-evenly".to_string(), "space-evenly".to_string());
        content_map.insert("content-baseline".to_string(), "baseline".to_string());
        content_map.insert("content-stretch".to_string(), "stretch".to_string());
        
        Self { content_map }
    }

    fn parse_content_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(content_value) = self.content_map.get(class) {
            return Some(vec![CssProperty { name: "align-content".to_string(), value: content_value.clone(), important: false }]);
        }
        None
    }
}

impl UtilityParser for AlignContentParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_content_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "content-normal", "content-center", "content-start", "content-end",
            "content-between", "content-around", "content-evenly", "content-baseline", "content-stretch"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Flexbox }
}

impl Default for AlignContentParser {
    fn default() -> Self { Self::new() }
}
