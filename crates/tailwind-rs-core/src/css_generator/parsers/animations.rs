//! Animation Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::core::CssProperty;

#[derive(Debug, Clone)]
pub struct AnimationParser;

impl AnimationParser {
    pub fn new() -> Self { Self }
}

impl UtilityParser for AnimationParser {
    fn parse_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn get_supported_patterns(&self) -> Vec<&'static str> { vec!["animate-*", "transition-*"] }
    fn get_priority(&self) -> u32 { 10 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Animations }
}

impl Default for AnimationParser {
    fn default() -> Self { Self::new() }
}
