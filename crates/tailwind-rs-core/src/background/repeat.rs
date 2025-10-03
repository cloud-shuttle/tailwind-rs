//! Background Repeat Parser Module

use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct BackgroundRepeatParser;

impl BackgroundRepeatParser {
    pub fn new() -> Self { Self }
    pub fn parse_repeat_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn supported_patterns(&self) -> Vec<&'static str> { vec![] }
}

impl Default for BackgroundRepeatParser {
    fn default() -> Self { Self::new() }
}
