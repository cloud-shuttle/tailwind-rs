//! Box Utilities Parser
//!
//! This module provides parsing logic for box-related utilities,
//! such as `box-decoration-break`, `box-sizing`, and additional display utilities.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct BoxUtilitiesParser;

impl BoxUtilitiesParser {
    pub fn new() -> Self { Self }

    /// Parse box-decoration-break classes
    fn parse_box_decoration_break_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "box-decoration-clone" => Some(vec![CssProperty { name: "box-decoration-break".to_string(), value: "clone".to_string(), important: false }]),
            "box-decoration-slice" => Some(vec![CssProperty { name: "box-decoration-break".to_string(), value: "slice".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse box-sizing classes
    fn parse_box_sizing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "box-border" => Some(vec![CssProperty { name: "box-sizing".to_string(), value: "border-box".to_string(), important: false }]),
            "box-content" => Some(vec![CssProperty { name: "box-sizing".to_string(), value: "content-box".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse additional display classes
    fn parse_display_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "flow-root" => Some(vec![CssProperty { name: "display".to_string(), value: "flow-root".to_string(), important: false }]),
            "table-caption" => Some(vec![CssProperty { name: "display".to_string(), value: "table-caption".to_string(), important: false }]),
            "table-column" => Some(vec![CssProperty { name: "display".to_string(), value: "table-column".to_string(), important: false }]),
            "table-column-group" => Some(vec![CssProperty { name: "display".to_string(), value: "table-column-group".to_string(), important: false }]),
            "table-header-group" => Some(vec![CssProperty { name: "display".to_string(), value: "table-header-group".to_string(), important: false }]),
            "table-row-group" => Some(vec![CssProperty { name: "display".to_string(), value: "table-row-group".to_string(), important: false }]),
            "table-footer-group" => Some(vec![CssProperty { name: "display".to_string(), value: "table-footer-group".to_string(), important: false }]),
            "sr-only" => Some(vec![
                CssProperty { name: "position".to_string(), value: "absolute".to_string(), important: false },
                CssProperty { name: "width".to_string(), value: "1px".to_string(), important: false },
                CssProperty { name: "height".to_string(), value: "1px".to_string(), important: false },
                CssProperty { name: "padding".to_string(), value: "0".to_string(), important: false },
                CssProperty { name: "margin".to_string(), value: "-1px".to_string(), important: false },
                CssProperty { name: "overflow".to_string(), value: "hidden".to_string(), important: false },
                CssProperty { name: "clip".to_string(), value: "rect(0, 0, 0, 0)".to_string(), important: false },
                CssProperty { name: "white-space".to_string(), value: "nowrap".to_string(), important: false },
                CssProperty { name: "border-width".to_string(), value: "0".to_string(), important: false },
            ]),
            "not-sr-only" => Some(vec![
                CssProperty { name: "position".to_string(), value: "static".to_string(), important: false },
                CssProperty { name: "width".to_string(), value: "auto".to_string(), important: false },
                CssProperty { name: "height".to_string(), value: "auto".to_string(), important: false },
                CssProperty { name: "padding".to_string(), value: "0".to_string(), important: false },
                CssProperty { name: "margin".to_string(), value: "0".to_string(), important: false },
                CssProperty { name: "overflow".to_string(), value: "visible".to_string(), important: false },
                CssProperty { name: "clip".to_string(), value: "auto".to_string(), important: false },
                CssProperty { name: "white-space".to_string(), value: "normal".to_string(), important: false },
            ]),
            _ => None,
        }
    }
}

impl UtilityParser for BoxUtilitiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_box_decoration_break_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_box_sizing_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_display_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "box-decoration-clone", "box-decoration-slice",
            "box-border", "box-content",
            "flow-root", "table-caption", "table-column", "table-column-group",
            "table-header-group", "table-row-group", "table-footer-group",
            "sr-only", "not-sr-only"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for BoxUtilitiesParser {
    fn default() -> Self { Self::new() }
}
