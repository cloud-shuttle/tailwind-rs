//! Rotation Parser Module

use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct RotationParser;

impl RotationParser {
    pub fn new() -> Self { Self }
    pub fn parse_rotate_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn parse_rotate_3d_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    pub fn supported_patterns(&self) -> Vec<&'static str> { vec![] }
}

impl Default for RotationParser {
    fn default() -> Self { Self::new() }
}
