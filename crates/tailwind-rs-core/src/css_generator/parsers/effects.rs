//! Effects Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct EffectsParser;

impl EffectsParser {
    pub fn new() -> Self { Self }
}

impl UtilityParser for EffectsParser {
    fn parse_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn get_supported_patterns(&self) -> Vec<&'static str> { vec!["shadow-*", "opacity-*"] }
    fn get_priority(&self) -> u32 { 30 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Effects }
}

impl Default for EffectsParser {
    fn default() -> Self { Self::new() }
}
