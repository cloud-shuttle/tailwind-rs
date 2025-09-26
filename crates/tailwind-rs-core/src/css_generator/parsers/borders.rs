//! Border Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct BorderParser;

impl BorderParser {
    pub fn new() -> Self { Self }
}

impl UtilityParser for BorderParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Parse basic border class
        match class {
            "border" => Some(vec![
                CssProperty { name: "border-width".to_string(), value: "1px".to_string(), important: false },
                CssProperty { name: "border-style".to_string(), value: "solid".to_string(), important: false },
            ]),
            "border-0" => Some(vec![CssProperty { name: "border-width".to_string(), value: "0px".to_string(), important: false }]),
            "border-2" => Some(vec![CssProperty { name: "border-width".to_string(), value: "2px".to_string(), important: false }]),
            "border-4" => Some(vec![CssProperty { name: "border-width".to_string(), value: "4px".to_string(), important: false }]),
            "border-8" => Some(vec![CssProperty { name: "border-width".to_string(), value: "8px".to_string(), important: false }]),
            _ => None,
        }
    }
    fn get_supported_patterns(&self) -> Vec<&'static str> { vec!["border", "border-0", "border-2", "border-4", "border-8"] }
    fn get_priority(&self) -> u32 { 40 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Borders }
}

impl Default for BorderParser {
    fn default() -> Self { Self::new() }
}
