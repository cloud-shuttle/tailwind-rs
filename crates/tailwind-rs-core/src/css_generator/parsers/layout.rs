//! Layout Utilities Parser

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct LayoutParser;

impl LayoutParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse display classes
    fn parse_display_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "block" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "block".to_string(),
                important: false,
            }]),
            "inline-block" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "inline-block".to_string(),
                important: false,
            }]),
            "inline" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "inline".to_string(),
                important: false,
            }]),
            "flex" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "flex".to_string(),
                important: false,
            }]),
            "inline-flex" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "inline-flex".to_string(),
                important: false,
            }]),
            "grid" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "grid".to_string(),
                important: false,
            }]),
            "inline-grid" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "inline-grid".to_string(),
                important: false,
            }]),
            "hidden" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            "table" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "table".to_string(),
                important: false,
            }]),
            "table-cell" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "table-cell".to_string(),
                important: false,
            }]),
            "table-row" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "table-row".to_string(),
                important: false,
            }]),
            "list-item" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "list-item".to_string(),
                important: false,
            }]),
            "contents" => Some(vec![CssProperty {
                name: "display".to_string(),
                value: "contents".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse position classes
    fn parse_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "static" => Some(vec![CssProperty {
                name: "position".to_string(),
                value: "static".to_string(),
                important: false,
            }]),
            "fixed" => Some(vec![CssProperty {
                name: "position".to_string(),
                value: "fixed".to_string(),
                important: false,
            }]),
            "absolute" => Some(vec![CssProperty {
                name: "position".to_string(),
                value: "absolute".to_string(),
                important: false,
            }]),
            "relative" => Some(vec![CssProperty {
                name: "position".to_string(),
                value: "relative".to_string(),
                important: false,
            }]),
            "sticky" => Some(vec![CssProperty {
                name: "position".to_string(),
                value: "sticky".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse visibility classes
    fn parse_visibility_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "visible" => Some(vec![CssProperty {
                name: "visibility".to_string(),
                value: "visible".to_string(),
                important: false,
            }]),
            "invisible" => Some(vec![CssProperty {
                name: "visibility".to_string(),
                value: "hidden".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse overflow classes
    fn parse_overflow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "overflow-auto" => Some(vec![CssProperty {
                name: "overflow".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "overflow-hidden" => Some(vec![CssProperty {
                name: "overflow".to_string(),
                value: "hidden".to_string(),
                important: false,
            }]),
            "overflow-clip" => Some(vec![CssProperty {
                name: "overflow".to_string(),
                value: "clip".to_string(),
                important: false,
            }]),
            "overflow-visible" => Some(vec![CssProperty {
                name: "overflow".to_string(),
                value: "visible".to_string(),
                important: false,
            }]),
            "overflow-scroll" => Some(vec![CssProperty {
                name: "overflow".to_string(),
                value: "scroll".to_string(),
                important: false,
            }]),
            "overflow-x-auto" => Some(vec![CssProperty {
                name: "overflow-x".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "overflow-x-hidden" => Some(vec![CssProperty {
                name: "overflow-x".to_string(),
                value: "hidden".to_string(),
                important: false,
            }]),
            "overflow-x-clip" => Some(vec![CssProperty {
                name: "overflow-x".to_string(),
                value: "clip".to_string(),
                important: false,
            }]),
            "overflow-x-visible" => Some(vec![CssProperty {
                name: "overflow-x".to_string(),
                value: "visible".to_string(),
                important: false,
            }]),
            "overflow-x-scroll" => Some(vec![CssProperty {
                name: "overflow-x".to_string(),
                value: "scroll".to_string(),
                important: false,
            }]),
            "overflow-y-auto" => Some(vec![CssProperty {
                name: "overflow-y".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "overflow-y-hidden" => Some(vec![CssProperty {
                name: "overflow-y".to_string(),
                value: "hidden".to_string(),
                important: false,
            }]),
            "overflow-y-clip" => Some(vec![CssProperty {
                name: "overflow-y".to_string(),
                value: "clip".to_string(),
                important: false,
            }]),
            "overflow-y-visible" => Some(vec![CssProperty {
                name: "overflow-y".to_string(),
                value: "visible".to_string(),
                important: false,
            }]),
            "overflow-y-scroll" => Some(vec![CssProperty {
                name: "overflow-y".to_string(),
                value: "scroll".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }
}

impl UtilityParser for LayoutParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try display classes first
        if let Some(properties) = self.parse_display_class(class) {
            return Some(properties);
        }

        // Try position classes
        if let Some(properties) = self.parse_position_class(class) {
            return Some(properties);
        }

        // Try visibility classes
        if let Some(properties) = self.parse_visibility_class(class) {
            return Some(properties);
        }

        // Try overflow classes
        if let Some(properties) = self.parse_overflow_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "block",
            "inline-block",
            "inline",
            "flex",
            "inline-flex",
            "grid",
            "inline-grid",
            "hidden",
            "static",
            "fixed",
            "absolute",
            "relative",
            "sticky",
            "visible",
            "invisible",
            "overflow-*",
            "overflow-x-*",
            "overflow-y-*",
        ]
    }
    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

impl Default for LayoutParser {
    fn default() -> Self {
        Self::new()
    }
}
