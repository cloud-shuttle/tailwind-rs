//! Visibility Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS visibility utilities,
//! such as `visible`, `invisible`, `collapse`.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VisibilityParser {
    visibility_map: HashMap<String, String>,
}

impl VisibilityParser {
    pub fn new() -> Self {
        let mut visibility_map = HashMap::new();
        visibility_map.insert("visible".to_string(), "visible".to_string());
        visibility_map.insert("invisible".to_string(), "hidden".to_string());
        visibility_map.insert("collapse".to_string(), "collapse".to_string());
        
        Self { visibility_map }
    }

    fn parse_visibility_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(visibility_value) = self.visibility_map.get(class) {
            return Some(vec![CssProperty { name: "visibility".to_string(), value: visibility_value.clone(), important: false }]);
        }
        None
    }
}

impl UtilityParser for VisibilityParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_visibility_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "visible", "invisible", "collapse"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for VisibilityParser {
    fn default() -> Self { Self::new() }
}
