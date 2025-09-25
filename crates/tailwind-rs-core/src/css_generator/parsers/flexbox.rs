//! Flexbox Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::core::CssProperty;

#[derive(Debug, Clone)]
pub struct FlexboxParser;

impl FlexboxParser {
    pub fn new() -> Self { Self }
}

impl UtilityParser for FlexboxParser {
    fn parse_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn get_supported_patterns(&self) -> Vec<&'static str> { vec!["flex-*", "justify-*", "items-*"] }
    fn get_priority(&self) -> u32 { 60 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Flexbox }
}

impl Default for FlexboxParser {
    fn default() -> Self { Self::new() }
}
