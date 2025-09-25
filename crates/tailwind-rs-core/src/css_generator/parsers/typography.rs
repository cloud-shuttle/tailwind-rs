//! Typography Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct TypographyParser;

impl TypographyParser {
    pub fn new() -> Self { Self }
}

impl UtilityParser for TypographyParser {
    fn parse_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn get_supported_patterns(&self) -> Vec<&'static str> { vec!["text-*", "font-*"] }
    fn get_priority(&self) -> u32 { 80 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Typography }
}

impl Default for TypographyParser {
    fn default() -> Self { Self::new() }
}
