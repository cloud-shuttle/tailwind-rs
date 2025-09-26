//! Table Utilities Parser
//!
//! This module provides parsing logic for table-related Tailwind CSS utilities,
//! including table-layout, border-collapse, border-spacing, and caption-side.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct TableParser;

impl TableParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse table-layout classes
    fn parse_table_layout_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "table-auto" => Some(vec![CssProperty {
                name: "table-layout".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "table-fixed" => Some(vec![CssProperty {
                name: "table-layout".to_string(),
                value: "fixed".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse border-collapse classes
    fn parse_border_collapse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "border-collapse" => Some(vec![CssProperty {
                name: "border-collapse".to_string(),
                value: "collapse".to_string(),
                important: false,
            }]),
            "border-separate" => Some(vec![CssProperty {
                name: "border-collapse".to_string(),
                value: "separate".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse border-spacing classes
    fn parse_border_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("border-spacing-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "border-spacing".to_string(),
                    value: format!("calc(var(--spacing) * {})", spacing),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse border-spacing-x classes
    fn parse_border_spacing_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("border-spacing-x-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "border-spacing".to_string(),
                    value: format!(
                        "calc(var(--spacing) * {}) var(--tw-border-spacing-y)",
                        spacing
                    ),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse border-spacing-y classes
    fn parse_border_spacing_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("border-spacing-y-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "border-spacing".to_string(),
                    value: format!(
                        "var(--tw-border-spacing-x) calc(var(--spacing) * {})",
                        spacing
                    ),
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse caption-side classes
    fn parse_caption_side_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "caption-top" => Some(vec![CssProperty {
                name: "caption-side".to_string(),
                value: "top".to_string(),
                important: false,
            }]),
            "caption-bottom" => Some(vec![CssProperty {
                name: "caption-side".to_string(),
                value: "bottom".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Get spacing value for border-spacing utilities
    fn get_spacing_value(&self, value: &str) -> Option<f32> {
        match value {
            "0" => Some(0.0),
            "px" => Some(1.0),
            "0.5" => Some(0.125),
            "1" => Some(0.25),
            "1.5" => Some(0.375),
            "2" => Some(0.5),
            "2.5" => Some(0.625),
            "3" => Some(0.75),
            "3.5" => Some(0.875),
            "4" => Some(1.0),
            "5" => Some(1.25),
            "6" => Some(1.5),
            "7" => Some(1.75),
            "8" => Some(2.0),
            "9" => Some(2.25),
            "10" => Some(2.5),
            "11" => Some(2.75),
            "12" => Some(3.0),
            "14" => Some(3.5),
            "16" => Some(4.0),
            "20" => Some(5.0),
            "24" => Some(6.0),
            "28" => Some(7.0),
            "32" => Some(8.0),
            "36" => Some(9.0),
            "40" => Some(10.0),
            "44" => Some(11.0),
            "48" => Some(12.0),
            "52" => Some(13.0),
            "56" => Some(14.0),
            "60" => Some(15.0),
            "64" => Some(16.0),
            "72" => Some(18.0),
            "80" => Some(20.0),
            "96" => Some(24.0),
            _ => None,
        }
    }
}

impl UtilityParser for TableParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_table_layout_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_border_collapse_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_border_spacing_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_border_spacing_x_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_border_spacing_y_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_caption_side_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "table-*",
            "border-collapse",
            "border-separate",
            "border-spacing-*",
            "border-spacing-x-*",
            "border-spacing-y-*",
            "caption-*",
        ]
    }

    fn get_priority(&self) -> u32 {
        50
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

impl Default for TableParser {
    fn default() -> Self {
        Self::new()
    }
}
