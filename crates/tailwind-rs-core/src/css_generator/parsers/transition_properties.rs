//! Transition Properties Parser
//!
//! This module provides parsing logic for transition property classes in Tailwind CSS,
//! such as `data-enter:ease-out`, `data-leave:ease-in`, `data-closed:scale-95`.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct TransitionPropertiesParser;

impl TransitionPropertiesParser {
    pub fn new() -> Self { Self }

    /// Parse transition timing function classes
    fn parse_transition_timing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "ease-linear" => Some(vec![CssProperty { name: "transition-timing-function".to_string(), value: "linear".to_string(), important: false }]),
            "ease-in" => Some(vec![CssProperty { name: "transition-timing-function".to_string(), value: "cubic-bezier(0.4, 0, 1, 1)".to_string(), important: false }]),
            "ease-out" => Some(vec![CssProperty { name: "transition-timing-function".to_string(), value: "cubic-bezier(0, 0, 0.2, 1)".to_string(), important: false }]),
            "ease-in-out" => Some(vec![CssProperty { name: "transition-timing-function".to_string(), value: "cubic-bezier(0.4, 0, 0.2, 1)".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse transition duration classes
    fn parse_transition_duration_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "duration-75" => Some(vec![CssProperty { name: "transition-duration".to_string(), value: "75ms".to_string(), important: false }]),
            "duration-100" => Some(vec![CssProperty { name: "transition-duration".to_string(), value: "100ms".to_string(), important: false }]),
            "duration-150" => Some(vec![CssProperty { name: "transition-duration".to_string(), value: "150ms".to_string(), important: false }]),
            "duration-200" => Some(vec![CssProperty { name: "transition-duration".to_string(), value: "200ms".to_string(), important: false }]),
            "duration-300" => Some(vec![CssProperty { name: "transition-duration".to_string(), value: "300ms".to_string(), important: false }]),
            "duration-500" => Some(vec![CssProperty { name: "transition-duration".to_string(), value: "500ms".to_string(), important: false }]),
            "duration-700" => Some(vec![CssProperty { name: "transition-duration".to_string(), value: "700ms".to_string(), important: false }]),
            "duration-1000" => Some(vec![CssProperty { name: "transition-duration".to_string(), value: "1000ms".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse transition delay classes
    fn parse_transition_delay_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "delay-75" => Some(vec![CssProperty { name: "transition-delay".to_string(), value: "75ms".to_string(), important: false }]),
            "delay-100" => Some(vec![CssProperty { name: "transition-delay".to_string(), value: "100ms".to_string(), important: false }]),
            "delay-150" => Some(vec![CssProperty { name: "transition-delay".to_string(), value: "150ms".to_string(), important: false }]),
            "delay-200" => Some(vec![CssProperty { name: "transition-delay".to_string(), value: "200ms".to_string(), important: false }]),
            "delay-300" => Some(vec![CssProperty { name: "transition-delay".to_string(), value: "300ms".to_string(), important: false }]),
            "delay-500" => Some(vec![CssProperty { name: "transition-delay".to_string(), value: "500ms".to_string(), important: false }]),
            "delay-700" => Some(vec![CssProperty { name: "transition-delay".to_string(), value: "700ms".to_string(), important: false }]),
            "delay-1000" => Some(vec![CssProperty { name: "transition-delay".to_string(), value: "1000ms".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse scale classes for data attributes
    fn parse_scale_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "scale-0" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0)".to_string(), important: false }]),
            "scale-50" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.5)".to_string(), important: false }]),
            "scale-75" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.75)".to_string(), important: false }]),
            "scale-90" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.9)".to_string(), important: false }]),
            "scale-95" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.95)".to_string(), important: false }]),
            "scale-100" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1)".to_string(), important: false }]),
            "scale-105" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.05)".to_string(), important: false }]),
            "scale-110" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.1)".to_string(), important: false }]),
            "scale-125" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.25)".to_string(), important: false }]),
            "scale-150" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.5)".to_string(), important: false }]),
            _ => None,
        }
    }
}

impl UtilityParser for TransitionPropertiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_transition_timing_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_transition_duration_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_transition_delay_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_scale_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "ease-linear", "ease-in", "ease-out", "ease-in-out",
            "duration-75", "duration-100", "duration-150", "duration-200", "duration-300", "duration-500", "duration-700", "duration-1000",
            "delay-75", "delay-100", "delay-150", "delay-200", "delay-300", "delay-500", "delay-700", "delay-1000",
            "scale-0", "scale-50", "scale-75", "scale-90", "scale-95", "scale-100", "scale-105", "scale-110", "scale-125", "scale-150"
        ]
    }

    fn get_priority(&self) -> u32 { 85 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Transitions }
}

impl Default for TransitionPropertiesParser {
    fn default() -> Self { Self::new() }
}
