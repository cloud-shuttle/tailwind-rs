//! Transition Utilities Parser
//!
//! This module provides parsing logic for transition-related Tailwind CSS utilities,
//! including transition properties, duration, timing, and delay.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct TransitionParser;

impl TransitionParser {
    pub fn new() -> Self { Self }

    /// Parse transition property classes
    fn parse_transition_property_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "transition-none" => Some(vec![CssProperty { name: "transition-property".to_string(), value: "none".to_string(), important: false }]),
            "transition-all" => Some(vec![CssProperty { name: "transition-property".to_string(), value: "all".to_string(), important: false }]),
            "transition" => Some(vec![CssProperty { name: "transition-property".to_string(), value: "background-color, border-color, color, fill, stroke, opacity, box-shadow, transform".to_string(), important: false }]),
            "transition-colors" => Some(vec![CssProperty { name: "transition-property".to_string(), value: "color, background-color, border-color, text-decoration-color, fill, stroke".to_string(), important: false }]),
            "transition-opacity" => Some(vec![CssProperty { name: "transition-property".to_string(), value: "opacity".to_string(), important: false }]),
            "transition-shadow" => Some(vec![CssProperty { name: "transition-property".to_string(), value: "box-shadow".to_string(), important: false }]),
            "transition-transform" => Some(vec![CssProperty { name: "transition-property".to_string(), value: "transform".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse transition duration classes
    fn parse_transition_duration_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(duration) = class.strip_prefix("duration-") {
            let duration_value = self.parse_duration_value(duration)?;
            return Some(vec![CssProperty { name: "transition-duration".to_string(), value: duration_value, important: false }]);
        }
        None
    }

    /// Parse transition timing classes
    fn parse_transition_timing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(timing) = class.strip_prefix("ease-") {
            let timing_value = self.parse_timing_value(timing)?;
            return Some(vec![CssProperty { name: "transition-timing-function".to_string(), value: timing_value, important: false }]);
        }
        None
    }

    /// Parse transition delay classes
    fn parse_transition_delay_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(delay) = class.strip_prefix("delay-") {
            let delay_value = self.parse_delay_value(delay)?;
            return Some(vec![CssProperty { name: "transition-delay".to_string(), value: delay_value, important: false }]);
        }
        None
    }

    /// Parse duration values
    fn parse_duration_value(&self, duration: &str) -> Option<String> {
        match duration {
            "75" => Some("75ms".to_string()),
            "100" => Some("100ms".to_string()),
            "150" => Some("150ms".to_string()),
            "200" => Some("200ms".to_string()),
            "300" => Some("300ms".to_string()),
            "500" => Some("500ms".to_string()),
            "700" => Some("700ms".to_string()),
            "1000" => Some("1000ms".to_string()),
            _ => None,
        }
    }

    /// Parse timing values
    fn parse_timing_value(&self, timing: &str) -> Option<String> {
        match timing {
            "linear" => Some("linear".to_string()),
            "in" => Some("cubic-bezier(0.4, 0, 1, 1)".to_string()),
            "out" => Some("cubic-bezier(0, 0, 0.2, 1)".to_string()),
            "in-out" => Some("cubic-bezier(0.4, 0, 0.2, 1)".to_string()),
            _ => None,
        }
    }

    /// Parse delay values
    fn parse_delay_value(&self, delay: &str) -> Option<String> {
        match delay {
            "75" => Some("75ms".to_string()),
            "100" => Some("100ms".to_string()),
            "150" => Some("150ms".to_string()),
            "200" => Some("200ms".to_string()),
            "300" => Some("300ms".to_string()),
            "500" => Some("500ms".to_string()),
            "700" => Some("700ms".to_string()),
            "1000" => Some("1000ms".to_string()),
            _ => None,
        }
    }
}

impl UtilityParser for TransitionParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_transition_property_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_transition_duration_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_transition_timing_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_transition_delay_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["transition-none", "transition-all", "transition", "transition-colors", 
             "transition-opacity", "transition-shadow", "transition-transform",
             "duration-*", "ease-*", "delay-*"]
    }

    fn get_priority(&self) -> u32 { 75 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Animations }
}

impl Default for TransitionParser {
    fn default() -> Self { Self::new() }
}
