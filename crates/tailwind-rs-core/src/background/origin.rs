//! Background Origin Parser Module

use crate::css_generator::types::CssProperty;

pub struct BackgroundOriginParser;

impl BackgroundOriginParser {
    pub fn new() -> Self { Self }
    pub fn parse_origin_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn supported_patterns(&self) -> Vec<&'static str> { vec![] }
}

impl Default for BackgroundOriginParser {
    fn default() -> Self { Self::new() }
}
