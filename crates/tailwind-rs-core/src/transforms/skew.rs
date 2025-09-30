//! Skew Parser Module

use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct SkewParser;

impl SkewParser {
    pub fn new() -> Self { Self }
    pub fn parse_skew_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn parse_skew_x_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn parse_skew_y_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn supported_patterns(&self) -> Vec<&'static str> { vec![] }
}

impl Default for SkewParser {
    fn default() -> Self { Self::new() }
}
