//! Effects Utilities Parser

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct EffectsParser;

impl EffectsParser {
    pub fn new() -> Self { Self }
    
    /// Parse shadow classes
    fn parse_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "shadow-sm" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(), important: false }]),
            "shadow" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(), important: false }]),
            "shadow-md" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(), important: false }]),
            "shadow-lg" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(), important: false }]),
            "shadow-xl" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(), important: false }]),
            "shadow-2xl" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(), important: false }]),
            "shadow-inner" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string(), important: false }]),
            "shadow-none" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "none".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse opacity classes
    fn parse_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "opacity-0" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0".to_string(), important: false }]),
            "opacity-5" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.05".to_string(), important: false }]),
            "opacity-10" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.1".to_string(), important: false }]),
            "opacity-20" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.2".to_string(), important: false }]),
            "opacity-25" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.25".to_string(), important: false }]),
            "opacity-30" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.3".to_string(), important: false }]),
            "opacity-40" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.4".to_string(), important: false }]),
            "opacity-50" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.5".to_string(), important: false }]),
            "opacity-60" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.6".to_string(), important: false }]),
            "opacity-70" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.7".to_string(), important: false }]),
            "opacity-75" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.75".to_string(), important: false }]),
            "opacity-80" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.8".to_string(), important: false }]),
            "opacity-90" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.9".to_string(), important: false }]),
            "opacity-95" => Some(vec![CssProperty { name: "opacity".to_string(), value: "0.95".to_string(), important: false }]),
            "opacity-100" => Some(vec![CssProperty { name: "opacity".to_string(), value: "1".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse backdrop blur classes
    fn parse_backdrop_blur_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "backdrop-blur-none" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "blur(0)".to_string(), important: false }]),
            "backdrop-blur-xs" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "blur(2px)".to_string(), important: false }]),
            "backdrop-blur-sm" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "blur(4px)".to_string(), important: false }]),
            "backdrop-blur" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "blur(8px)".to_string(), important: false }]),
            "backdrop-blur-md" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "blur(12px)".to_string(), important: false }]),
            "backdrop-blur-lg" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "blur(16px)".to_string(), important: false }]),
            "backdrop-blur-xl" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "blur(24px)".to_string(), important: false }]),
            "backdrop-blur-2xl" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "blur(40px)".to_string(), important: false }]),
            "backdrop-blur-3xl" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "blur(64px)".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse backdrop opacity classes
    fn parse_backdrop_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "backdrop-opacity-0" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0)".to_string(), important: false }]),
            "backdrop-opacity-5" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.05)".to_string(), important: false }]),
            "backdrop-opacity-10" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.1)".to_string(), important: false }]),
            "backdrop-opacity-20" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.2)".to_string(), important: false }]),
            "backdrop-opacity-25" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.25)".to_string(), important: false }]),
            "backdrop-opacity-30" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.3)".to_string(), important: false }]),
            "backdrop-opacity-40" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.4)".to_string(), important: false }]),
            "backdrop-opacity-50" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.5)".to_string(), important: false }]),
            "backdrop-opacity-60" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.6)".to_string(), important: false }]),
            "backdrop-opacity-70" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.7)".to_string(), important: false }]),
            "backdrop-opacity-75" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.75)".to_string(), important: false }]),
            "backdrop-opacity-80" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.8)".to_string(), important: false }]),
            "backdrop-opacity-90" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.9)".to_string(), important: false }]),
            "backdrop-opacity-95" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(0.95)".to_string(), important: false }]),
            "backdrop-opacity-100" => Some(vec![CssProperty { name: "backdrop-filter".to_string(), value: "opacity(1)".to_string(), important: false }]),
            _ => None,
        }
    }
    
    /// Parse mix blend mode classes
    fn parse_mix_blend_mode_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mix-blend-normal" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "normal".to_string(), important: false }]),
            "mix-blend-multiply" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "multiply".to_string(), important: false }]),
            "mix-blend-screen" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "screen".to_string(), important: false }]),
            "mix-blend-overlay" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "overlay".to_string(), important: false }]),
            "mix-blend-darken" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "darken".to_string(), important: false }]),
            "mix-blend-lighten" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "lighten".to_string(), important: false }]),
            "mix-blend-color-dodge" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "color-dodge".to_string(), important: false }]),
            "mix-blend-color-burn" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "color-burn".to_string(), important: false }]),
            "mix-blend-hard-light" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "hard-light".to_string(), important: false }]),
            "mix-blend-soft-light" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "soft-light".to_string(), important: false }]),
            "mix-blend-difference" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "difference".to_string(), important: false }]),
            "mix-blend-exclusion" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "exclusion".to_string(), important: false }]),
            "mix-blend-hue" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "hue".to_string(), important: false }]),
            "mix-blend-saturation" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "saturation".to_string(), important: false }]),
            "mix-blend-color" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "color".to_string(), important: false }]),
            "mix-blend-luminosity" => Some(vec![CssProperty { name: "mix-blend-mode".to_string(), value: "luminosity".to_string(), important: false }]),
            _ => None,
        }
    }
}

impl UtilityParser for EffectsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try shadow classes first
        if let Some(properties) = self.parse_shadow_class(class) {
            return Some(properties);
        }
        
        // Try opacity classes
        if let Some(properties) = self.parse_opacity_class(class) {
            return Some(properties);
        }
        
        // Try backdrop blur classes
        if let Some(properties) = self.parse_backdrop_blur_class(class) {
            return Some(properties);
        }
        
        // Try backdrop opacity classes
        if let Some(properties) = self.parse_backdrop_opacity_class(class) {
            return Some(properties);
        }
        
        // Try mix blend mode classes
        if let Some(properties) = self.parse_mix_blend_mode_class(class) {
            return Some(properties);
        }
        
        None
    }
    
    fn get_supported_patterns(&self) -> Vec<&'static str> { 
        vec![
            "shadow-*", "opacity-*", "backdrop-blur-*", "backdrop-opacity-*", "mix-blend-*"
        ] 
    }
    fn get_priority(&self) -> u32 { 30 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Effects }
}

impl Default for EffectsParser {
    fn default() -> Self { Self::new() }
}
