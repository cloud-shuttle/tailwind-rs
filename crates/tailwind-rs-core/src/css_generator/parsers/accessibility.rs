//! Accessibility Utilities Parser
//!
//! This module provides parsing logic for accessibility-related Tailwind CSS utilities,
//! including forced-color-adjust and other accessibility features.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct AccessibilityParser;

impl AccessibilityParser {
    pub fn new() -> Self {
        Self
    }

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

    /// Parse screen reader classes
    fn parse_screen_reader_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "sr-only" => Some(vec![
                CssProperty {
                    name: "position".to_string(),
                    value: "absolute".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "width".to_string(),
                    value: "1px".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "height".to_string(),
                    value: "1px".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "padding".to_string(),
                    value: "0".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "margin".to_string(),
                    value: "-1px".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "overflow".to_string(),
                    value: "hidden".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "clip".to_string(),
                    value: "rect(0, 0, 0, 0)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "white-space".to_string(),
                    value: "nowrap".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "border".to_string(),
                    value: "0".to_string(),
                    important: false,
                },
            ]),
            "not-sr-only" => Some(vec![
                CssProperty {
                    name: "position".to_string(),
                    value: "static".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "width".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "height".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "padding".to_string(),
                    value: "0".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "margin".to_string(),
                    value: "0".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "overflow".to_string(),
                    value: "visible".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "clip".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "white-space".to_string(),
                    value: "normal".to_string(),
                    important: false,
                },
            ]),
            _ => None,
        }
    }

    /// Parse motion preference classes (prefers-reduced-motion)
    fn parse_motion_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "motion-reduce" => Some(vec![CssProperty {
                name: "transition".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            "motion-safe" => None, // motion-safe is handled by CSS media queries, not direct properties
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

        // Try screen reader classes
        if let Some(properties) = self.parse_screen_reader_class(class) {
            return Some(properties);
        }

        // Try motion preference classes
        if let Some(properties) = self.parse_motion_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "forced-color-adjust-auto",
            "forced-color-adjust-none",
            "sr-only",
            "not-sr-only",
            "motion-reduce",
        ]
    }

    fn get_priority(&self) -> u32 {
        50
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Accessibility
    }
}

impl Default for AccessibilityParser {
    fn default() -> Self {
        Self::new()
    }
}
