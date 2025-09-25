//! Animation Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct AnimationParser;

impl AnimationParser {
    pub fn new() -> Self { Self }

    /// Parse animation classes
    pub fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "animate-none" => Some(vec![CssProperty { name: "animation".to_string(), value: "none".to_string(), important: false }]),
            "animate-spin" => Some(vec![CssProperty { name: "animation".to_string(), value: "spin 1s linear infinite".to_string(), important: false }]),
            "animate-ping" => Some(vec![CssProperty { name: "animation".to_string(), value: "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(), important: false }]),
            "animate-pulse" => Some(vec![CssProperty { name: "animation".to_string(), value: "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(), important: false }]),
            "animate-bounce" => Some(vec![CssProperty { name: "animation".to_string(), value: "bounce 1s infinite".to_string(), important: false }]),
            
            _ => None,
        }
    }
}

impl UtilityParser for AnimationParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_animation_class(class)
    }
    
    fn get_supported_patterns(&self) -> Vec<&'static str> { 
        vec!["animate-"] 
    }
    
    fn get_priority(&self) -> u32 { 50 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Animations }
}

impl Default for AnimationParser {
    fn default() -> Self { Self::new() }
}
