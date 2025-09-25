//! Layout Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::core::CssProperty;

#[derive(Debug, Clone)]
pub struct LayoutParser;

impl LayoutParser {
    pub fn new() -> Self { Self }
}

impl UtilityParser for LayoutParser {
    fn parse_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn get_supported_patterns(&self) -> Vec<&'static str> { vec!["block", "inline", "flex", "grid"] }
    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for LayoutParser {
    fn default() -> Self { Self::new() }
}
