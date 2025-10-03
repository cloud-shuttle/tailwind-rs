//! Background Image Parser Module
//!
//! Handles parsing of background-image utilities (simplified version)

use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct BackgroundImageParser;

impl BackgroundImageParser {
    pub fn new() -> Self { Self }
    pub fn parse_image_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn supported_patterns(&self) -> Vec<&'static str> { vec![] }
}

impl Default for BackgroundImageParser {
    fn default() -> Self { Self::new() }
}
