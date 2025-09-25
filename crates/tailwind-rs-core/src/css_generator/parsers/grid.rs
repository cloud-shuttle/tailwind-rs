//! Grid Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::core::CssProperty;

#[derive(Debug, Clone)]
pub struct GridParser;

impl GridParser {
    pub fn new() -> Self { Self }
}

impl UtilityParser for GridParser {
    fn parse_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn get_supported_patterns(&self) -> Vec<&'static str> { vec!["grid-*", "col-*", "row-*"] }
    fn get_priority(&self) -> u32 { 50 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Grid }
}

impl Default for GridParser {
    fn default() -> Self { Self::new() }
}
