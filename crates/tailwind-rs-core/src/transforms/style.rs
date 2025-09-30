//! Transform Style Parser Module

use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct TransformStyleParser;

impl TransformStyleParser {
    pub fn new() -> Self { Self }
    pub fn parse_transform_style_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn supported_patterns(&self) -> Vec<&'static str> { vec![] }
}

impl Default for TransformStyleParser {
    fn default() -> Self { Self::new() }
}
