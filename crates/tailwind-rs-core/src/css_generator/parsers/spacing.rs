//! Spacing Utilities Parser
//! 
//! This module handles parsing of spacing-related utilities:
//! - Padding (p-*, px-*, py-*, etc.)
//! - Margin (m-*, mx-*, my-*, etc.)
//! - Gap (gap-*, gap-x-*, gap-y-*, etc.)

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

/// Parser for spacing utilities
#[derive(Debug, Clone)]
pub struct SpacingParser;

impl SpacingParser {
    /// Create a new spacing parser
    pub fn new() -> Self {
        Self
    }

    /// Parse padding classes
    fn parse_padding_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("p-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "padding".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("px-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "padding-left".to_string(),
                    value: spacing.clone(),
                    important: false,
                }, CssProperty {
                    name: "padding-right".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("py-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "padding-top".to_string(),
                    value: spacing.clone(),
                    important: false,
                }, CssProperty {
                    name: "padding-bottom".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pt-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "padding-top".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pr-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "padding-right".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pb-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "padding-bottom".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pl-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "padding-left".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        // Logical properties for padding
        if let Some(value) = class.strip_prefix("ps-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "padding-inline-start".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pe-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "padding-inline-end".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        // Arbitrary values for padding
        if let Some(value) = class.strip_prefix("p-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "padding".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("px-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "padding-left".to_string(),
                    value: value.to_string(),
                    important: false,
                }, CssProperty {
                    name: "padding-right".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("py-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "padding-top".to_string(),
                    value: value.to_string(),
                    important: false,
                }, CssProperty {
                    name: "padding-bottom".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pt-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "padding-top".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pr-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "padding-right".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pb-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "padding-bottom".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pl-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "padding-left".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ps-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "padding-inline-start".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pe-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "padding-inline-end".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        // Custom properties for padding
        if let Some(value) = class.strip_prefix("p-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "padding".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("px-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "padding-left".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }, CssProperty {
                    name: "padding-right".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("py-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "padding-top".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }, CssProperty {
                    name: "padding-bottom".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pt-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "padding-top".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pr-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "padding-right".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pb-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "padding-bottom".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pl-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "padding-left".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ps-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "padding-inline-start".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("pe-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "padding-inline-end".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        None
    }

    /// Parse margin classes
    fn parse_margin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("m-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mx-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: spacing.clone(),
                    important: false,
                }, CssProperty {
                    name: "margin-right".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("my-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: spacing.clone(),
                    important: false,
                }, CssProperty {
                    name: "margin-bottom".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mt-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mr-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-right".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mb-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-bottom".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ml-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        // Logical properties for margin
        if let Some(value) = class.strip_prefix("ms-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-inline-start".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("me-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "margin-inline-end".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        // Arbitrary values for margin
        if let Some(value) = class.strip_prefix("m-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mx-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: value.to_string(),
                    important: false,
                }, CssProperty {
                    name: "margin-right".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("my-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: value.to_string(),
                    important: false,
                }, CssProperty {
                    name: "margin-bottom".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mt-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mr-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-right".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mb-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-bottom".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ml-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ms-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-inline-start".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("me-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "margin-inline-end".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }
        
        // Custom properties for margin
        if let Some(value) = class.strip_prefix("m-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mx-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }, CssProperty {
                    name: "margin-right".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("my-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }, CssProperty {
                    name: "margin-bottom".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mt-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-top".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mr-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-right".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("mb-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-bottom".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ml-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-left".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("ms-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-inline-start".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("me-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "margin-inline-end".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }
        
        None
    }

    /// Parse gap classes
    fn parse_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("gap-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "gap".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("gap-x-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "column-gap".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        if let Some(value) = class.strip_prefix("gap-y-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "row-gap".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        
        // Space utilities
        if let Some(direction_and_value) = class.strip_prefix("space-") {
            if let Some(value) = direction_and_value.strip_prefix("x-") {
                if let Some(spacing) = self.get_spacing_value(value) {
                    return Some(vec![
                        CssProperty { 
                            name: "margin-left".to_string(), 
                            value: format!("calc({} * 0.5)", spacing), 
                            important: false 
                        },
                        CssProperty { 
                            name: "margin-right".to_string(), 
                            value: format!("calc({} * 0.5)", spacing), 
                            important: false 
                        },
                    ]);
                }
            }
            if let Some(value) = direction_and_value.strip_prefix("y-") {
                if let Some(spacing) = self.get_spacing_value(value) {
                    return Some(vec![
                        CssProperty { 
                            name: "margin-top".to_string(), 
                            value: format!("calc({} * 0.5)", spacing), 
                            important: false 
                        },
                        CssProperty { 
                            name: "margin-bottom".to_string(), 
                            value: format!("calc({} * 0.5)", spacing), 
                            important: false 
                        },
                    ]);
                }
            }
        }
        
        // Space reverse utilities
        if class == "space-x-reverse" {
            return Some(vec![CssProperty {
                name: "--tw-space-x-reverse".to_string(),
                value: "1".to_string(),
                important: false,
            }]);
        }
        
        if class == "space-y-reverse" {
            return Some(vec![CssProperty {
                name: "--tw-space-y-reverse".to_string(),
                value: "1".to_string(),
                important: false,
            }]);
        }
        
        // Arbitrary space utilities
        if let Some(value) = class.strip_prefix("space-x-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![
                    CssProperty { 
                        name: "margin-left".to_string(), 
                        value: format!("calc({} * 0.5)", value), 
                        important: false 
                    },
                    CssProperty { 
                        name: "margin-right".to_string(), 
                        value: format!("calc({} * 0.5)", value), 
                        important: false 
                    },
                ]);
            }
        }
        
        if let Some(value) = class.strip_prefix("space-y-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![
                    CssProperty { 
                        name: "margin-top".to_string(), 
                        value: format!("calc({} * 0.5)", value), 
                        important: false 
                    },
                    CssProperty { 
                        name: "margin-bottom".to_string(), 
                        value: format!("calc({} * 0.5)", value), 
                        important: false 
                    },
                ]);
            }
        }
        
        None
    }

    /// Get spacing value from class suffix
    fn get_spacing_value(&self, value: &str) -> Option<String> {
        match value {
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
            _ => None,
        }
    }
}

impl UtilityParser for SpacingParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try padding first
        if let Some(properties) = self.parse_padding_class(class) {
            return Some(properties);
        }
        
        // Try margin
        if let Some(properties) = self.parse_margin_class(class) {
            return Some(properties);
        }
        
        // Try gap
        if let Some(properties) = self.parse_gap_class(class) {
            return Some(properties);
        }
        
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "p-*", "px-*", "py-*", "pt-*", "pr-*", "pb-*", "pl-*",
            "m-*", "mx-*", "my-*", "mt-*", "mr-*", "mb-*", "ml-*",
            "gap-*", "gap-x-*", "gap-y-*",
        ]
    }

    fn get_priority(&self) -> u32 {
        100
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Spacing
    }
}

impl Default for SpacingParser {
    fn default() -> Self {
        Self::new()
    }
}
