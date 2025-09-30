//! Transform Origin Parser Module

use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct TransformOriginParser;

impl TransformOriginParser {
    pub fn new() -> Self { Self }
    pub fn parse_origin_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn parse_perspective_origin_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn supported_patterns(&self) -> Vec<&'static str> { vec![] }
}

impl Default for TransformOriginParser {
    fn default() -> Self { Self::new() }
}
