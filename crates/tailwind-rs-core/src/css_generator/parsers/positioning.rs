//! Positioning Utilities Parser
//!
//! This module provides parsing logic for positioning-related Tailwind CSS utilities,
//! including position, top, right, bottom, left, and z-index.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct PositioningParser;

impl PositioningParser {
    pub fn new() -> Self {
        Self
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

    /// Parse top/right/bottom/left classes
    fn parse_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "inset-0" => Some(vec![
                CssProperty {
                    name: "top".to_string(),
                    value: "0".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "right".to_string(),
                    value: "0".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "bottom".to_string(),
                    value: "0".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "left".to_string(),
                    value: "0".to_string(),
                    important: false,
                },
            ]),
            "inset-auto" => Some(vec![
                CssProperty {
                    name: "top".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "right".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "bottom".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "left".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
            ]),
            _ => None,
        }
    }

    /// Parse inset-x and inset-y classes
    fn parse_inset_xy_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("inset-x-") {
            let spacing_value = self.parse_spacing_value(value)?;
            return Some(vec![
                CssProperty {
                    name: "left".to_string(),
                    value: spacing_value.clone(),
                    important: false,
                },
                CssProperty {
                    name: "right".to_string(),
                    value: spacing_value,
                    important: false,
                },
            ]);
        }
        if let Some(value) = class.strip_prefix("inset-y-") {
            let spacing_value = self.parse_spacing_value(value)?;
            return Some(vec![
                CssProperty {
                    name: "top".to_string(),
                    value: spacing_value.clone(),
                    important: false,
                },
                CssProperty {
                    name: "bottom".to_string(),
                    value: spacing_value,
                    important: false,
                },
            ]);
        }
        if let Some(value) = class.strip_prefix("-inset-x-") {
            let spacing_value = self.parse_spacing_value(value)?;
            return Some(vec![
                CssProperty {
                    name: "left".to_string(),
                    value: format!("-{}", spacing_value.clone()),
                    important: false,
                },
                CssProperty {
                    name: "right".to_string(),
                    value: format!("-{}", spacing_value),
                    important: false,
                },
            ]);
        }
        if let Some(value) = class.strip_prefix("-inset-y-") {
            let spacing_value = self.parse_spacing_value(value)?;
            return Some(vec![
                CssProperty {
                    name: "top".to_string(),
                    value: format!("-{}", spacing_value.clone()),
                    important: false,
                },
                CssProperty {
                    name: "bottom".to_string(),
                    value: format!("-{}", spacing_value),
                    important: false,
                },
            ]);
        }
        None
    }

    /// Parse top classes
    fn parse_top_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("top-") {
            let css_value = self.parse_spacing_value(value)?;
            return Some(vec![CssProperty {
                name: "top".to_string(),
                value: css_value,
                important: false,
            }]);
        }
        if let Some(value) = class.strip_prefix("-top-") {
            let css_value = self.parse_spacing_value(value)?;
            return Some(vec![CssProperty {
                name: "top".to_string(),
                value: format!("-{}", css_value),
                important: false,
            }]);
        }
        None
    }

    /// Parse right classes
    fn parse_right_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("right-") {
            let css_value = self.parse_spacing_value(value)?;
            return Some(vec![CssProperty {
                name: "right".to_string(),
                value: css_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse bottom classes
    fn parse_bottom_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("bottom-") {
            let css_value = self.parse_spacing_value(value)?;
            return Some(vec![CssProperty {
                name: "bottom".to_string(),
                value: css_value,
                important: false,
            }]);
        }
        if let Some(value) = class.strip_prefix("-bottom-") {
            let css_value = self.parse_spacing_value(value)?;
            return Some(vec![CssProperty {
                name: "bottom".to_string(),
                value: format!("-{}", css_value),
                important: false,
            }]);
        }
        None
    }

    /// Parse left classes
    fn parse_left_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("left-") {
            let css_value = self.parse_spacing_value(value)?;
            return Some(vec![CssProperty {
                name: "left".to_string(),
                value: css_value,
                important: false,
            }]);
        }
        if let Some(value) = class.strip_prefix("-left-") {
            let css_value = self.parse_spacing_value(value)?;
            return Some(vec![CssProperty {
                name: "left".to_string(),
                value: format!("-{}", css_value),
                important: false,
            }]);
        }
        None
    }

    /// Parse z-index classes
    fn parse_z_index_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("z-") {
            let css_value = match value {
                "auto" => "auto".to_string(),
                "0" => "0".to_string(),
                "10" => "10".to_string(),
                "20" => "20".to_string(),
                "30" => "30".to_string(),
                "40" => "40".to_string(),
                "50" => "50".to_string(),
                _ => return None,
            };
            return Some(vec![CssProperty {
                name: "z-index".to_string(),
                value: css_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse spacing values (including negative values)
    fn parse_spacing_value(&self, value: &str) -> Option<String> {
        match value {
            "auto" => Some("auto".to_string()),
            "0" => Some("0".to_string()),
            "px" => Some("1px".to_string()),
            "0.5" => Some("0.125rem".to_string()),
            "1" => Some("0.25rem".to_string()),
            "1.5" => Some("0.375rem".to_string()),
            "2" => Some("0.5rem".to_string()),
            "2.5" => Some("0.625rem".to_string()),
            "3" => Some("0.75rem".to_string()),
            "3.5" => Some("0.875rem".to_string()),
            "4" => Some("1rem".to_string()),
            "5" => Some("1.25rem".to_string()),
            "6" => Some("1.5rem".to_string()),
            "7" => Some("1.75rem".to_string()),
            "8" => Some("2rem".to_string()),
            "9" => Some("2.25rem".to_string()),
            "10" => Some("2.5rem".to_string()),
            "11" => Some("2.75rem".to_string()),
            "12" => Some("3rem".to_string()),
            "14" => Some("3.5rem".to_string()),
            "16" => Some("4rem".to_string()),
            "20" => Some("5rem".to_string()),
            "24" => Some("6rem".to_string()),
            "28" => Some("7rem".to_string()),
            "32" => Some("8rem".to_string()),
            "36" => Some("9rem".to_string()),
            "40" => Some("10rem".to_string()),
            "44" => Some("11rem".to_string()),
            "48" => Some("12rem".to_string()),
            "52" => Some("13rem".to_string()),
            "56" => Some("14rem".to_string()),
            "60" => Some("15rem".to_string()),
            "64" => Some("16rem".to_string()),
            "72" => Some("18rem".to_string()),
            "80" => Some("20rem".to_string()),
            "96" => Some("24rem".to_string()),
            // Negative values
            s if s.starts_with('-') => {
                let positive_value = &s[1..];
                if let Some(css_value) = self.parse_spacing_value(positive_value) {
                    Some(format!("-{}", css_value))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl UtilityParser for PositioningParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        if let Some(properties) = self.parse_position_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_inset_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_inset_xy_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_top_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_right_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_bottom_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_left_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_z_index_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "static", "fixed", "absolute", "relative", "sticky", "inset-*", "top-*", "right-*",
            "bottom-*", "left-*", "z-*",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

impl Default for PositioningParser {
    fn default() -> Self {
        Self::new()
    }
}
