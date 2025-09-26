//! Break Control Utilities Parser
//!
//! This module provides parsing logic for break control utilities,
//! such as `break-after-auto`, `break-before-column`, `break-inside-avoid`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct BreakControlParser;

impl BreakControlParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse break-after classes
    fn parse_break_after_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "break-after-auto" => Some(vec![CssProperty {
                name: "break-after".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "break-after-avoid" => Some(vec![CssProperty {
                name: "break-after".to_string(),
                value: "avoid".to_string(),
                important: false,
            }]),
            "break-after-all" => Some(vec![CssProperty {
                name: "break-after".to_string(),
                value: "all".to_string(),
                important: false,
            }]),
            "break-after-avoid-page" => Some(vec![CssProperty {
                name: "break-after".to_string(),
                value: "avoid-page".to_string(),
                important: false,
            }]),
            "break-after-page" => Some(vec![CssProperty {
                name: "break-after".to_string(),
                value: "page".to_string(),
                important: false,
            }]),
            "break-after-left" => Some(vec![CssProperty {
                name: "break-after".to_string(),
                value: "left".to_string(),
                important: false,
            }]),
            "break-after-right" => Some(vec![CssProperty {
                name: "break-after".to_string(),
                value: "right".to_string(),
                important: false,
            }]),
            "break-after-column" => Some(vec![CssProperty {
                name: "break-after".to_string(),
                value: "column".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse break-before classes
    fn parse_break_before_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "break-before-auto" => Some(vec![CssProperty {
                name: "break-before".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "break-before-avoid" => Some(vec![CssProperty {
                name: "break-before".to_string(),
                value: "avoid".to_string(),
                important: false,
            }]),
            "break-before-all" => Some(vec![CssProperty {
                name: "break-before".to_string(),
                value: "all".to_string(),
                important: false,
            }]),
            "break-before-avoid-page" => Some(vec![CssProperty {
                name: "break-before".to_string(),
                value: "avoid-page".to_string(),
                important: false,
            }]),
            "break-before-page" => Some(vec![CssProperty {
                name: "break-before".to_string(),
                value: "page".to_string(),
                important: false,
            }]),
            "break-before-left" => Some(vec![CssProperty {
                name: "break-before".to_string(),
                value: "left".to_string(),
                important: false,
            }]),
            "break-before-right" => Some(vec![CssProperty {
                name: "break-before".to_string(),
                value: "right".to_string(),
                important: false,
            }]),
            "break-before-column" => Some(vec![CssProperty {
                name: "break-before".to_string(),
                value: "column".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse break-inside classes
    fn parse_break_inside_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "break-inside-auto" => Some(vec![CssProperty {
                name: "break-inside".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "break-inside-avoid" => Some(vec![CssProperty {
                name: "break-inside".to_string(),
                value: "avoid".to_string(),
                important: false,
            }]),
            "break-inside-avoid-page" => Some(vec![CssProperty {
                name: "break-inside".to_string(),
                value: "avoid-page".to_string(),
                important: false,
            }]),
            "break-inside-avoid-column" => Some(vec![CssProperty {
                name: "break-inside".to_string(),
                value: "avoid-column".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }
}

impl UtilityParser for BreakControlParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_break_after_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_break_before_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_break_inside_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "break-after-auto",
            "break-after-avoid",
            "break-after-all",
            "break-after-avoid-page",
            "break-after-page",
            "break-after-left",
            "break-after-right",
            "break-after-column",
            "break-before-auto",
            "break-before-avoid",
            "break-before-all",
            "break-before-avoid-page",
            "break-before-page",
            "break-before-left",
            "break-before-right",
            "break-before-column",
            "break-inside-auto",
            "break-inside-avoid",
            "break-inside-avoid-page",
            "break-inside-avoid-column",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

impl Default for BreakControlParser {
    fn default() -> Self {
        Self::new()
    }
}
