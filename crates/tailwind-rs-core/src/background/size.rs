//! Background Size Parser Module

use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct BackgroundSizeParser;

impl BackgroundSizeParser {
    pub fn new() -> Self { Self }
    pub fn parse_size_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn supported_patterns(&self) -> Vec<&'static str> { vec![] }
}

impl Default for BackgroundSizeParser {
    fn default() -> Self { Self::new() }
}
