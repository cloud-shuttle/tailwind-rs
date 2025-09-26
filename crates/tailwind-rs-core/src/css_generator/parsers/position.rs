//! Position Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS position utilities,
//! such as `static`, `fixed`, `absolute`, `relative`, `sticky`.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PositionParser {
    position_map: HashMap<String, String>,
}

impl PositionParser {
    pub fn new() -> Self {
        let mut position_map = HashMap::new();
        position_map.insert("static".to_string(), "static".to_string());
        position_map.insert("fixed".to_string(), "fixed".to_string());
        position_map.insert("absolute".to_string(), "absolute".to_string());
        position_map.insert("relative".to_string(), "relative".to_string());
        position_map.insert("sticky".to_string(), "sticky".to_string());
        
        Self { position_map }
    }

    fn parse_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(position_value) = self.position_map.get(class) {
            return Some(vec![CssProperty { name: "position".to_string(), value: position_value.clone(), important: false }]);
        }
        None
    }
}

impl UtilityParser for PositionParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_position_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "static", "fixed", "absolute", "relative", "sticky"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for PositionParser {
    fn default() -> Self { Self::new() }
}
