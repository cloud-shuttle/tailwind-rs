//! Border Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct BorderParser;

impl BorderParser {
    pub fn new() -> Self { Self }
}

impl UtilityParser for BorderParser {
    fn parse_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn get_supported_patterns(&self) -> Vec<&'static str> { vec!["border-*", "rounded-*"] }
    fn get_priority(&self) -> u32 { 40 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Borders }
}

impl Default for BorderParser {
    fn default() -> Self { Self::new() }
}
