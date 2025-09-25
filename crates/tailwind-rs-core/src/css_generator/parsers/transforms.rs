//! Transform Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::core::CssProperty;

#[derive(Debug, Clone)]
pub struct TransformParser;

impl TransformParser {
    pub fn new() -> Self { Self }
}

impl UtilityParser for TransformParser {
    fn parse_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn get_supported_patterns(&self) -> Vec<&'static str> { vec!["transform", "scale-*", "rotate-*"] }
    fn get_priority(&self) -> u32 { 20 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Transforms }
}

impl Default for TransformParser {
    fn default() -> Self { Self::new() }
}
