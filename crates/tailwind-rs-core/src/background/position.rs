//! Background Position Parser Module

use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct BackgroundPositionParser;

impl BackgroundPositionParser {
    pub fn new() -> Self { Self }
    pub fn parse_position_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn supported_patterns(&self) -> Vec<&'static str> { vec![] }
}

impl Default for BackgroundPositionParser {
    fn default() -> Self { Self::new() }
}
