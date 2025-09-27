//! Accessibility Utilities Parser
//!
//! This module provides parsing logic for accessibility-related Tailwind CSS utilities,
//! including forced-color-adjust and other accessibility features.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct AccessibilityParser;

impl AccessibilityParser {
    pub fn new() -> Self { Self }

    /// Parse forced-color-adjust classes
    fn parse_forced_color_adjust_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "forced-color-adjust-auto" => Some(vec![CssProperty {
                name: "forced-color-adjust".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "forced-color-adjust-none" => Some(vec![CssProperty {
                name: "forced-color-adjust".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }
}

impl UtilityParser for AccessibilityParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try forced-color-adjust classes
        if let Some(properties) = self.parse_forced_color_adjust_class(class) {
            return Some(properties);
        }
        
        None
    }
    
    fn get_supported_patterns(&self) -> Vec<&'static str> { 
        vec![
            "forced-color-adjust-*"
        ] 
    }
    
    fn get_priority(&self) -> u32 { 50 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Accessibility }
}

impl Default for AccessibilityParser {
    fn default() -> Self {
        Self::new()
    }
}
