//! Padding Parser for Tailwind CSS
//!
//! This module handles parsing of padding-related utilities:
//! - All-direction padding (p-*)
//! - Axis padding (px-*, py-*)
//! - Directional padding (pt-*, pr-*, pb-*, pl-*)
//! - Logical padding (ps-*, pe-*)
//! - Arbitrary padding (pt-[...], pr-[...], etc.)
//! - Custom padding (pt-(...), pr-(...), etc.)

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

/// Parser for padding utilities
#[derive(Debug, Clone)]
pub struct PaddingParser;

impl Default for PaddingParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PaddingParser {
    /// Create a new padding parser
    pub fn new() -> Self {
        Self
    }

    /// Parse padding classes
    pub fn parse_padding_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try different padding patterns in order of specificity
        self.parse_all_padding(class)
            .or_else(|| self.parse_axis_padding(class))
            .or_else(|| self.parse_directional_padding(class))
    }

    /// Parse all-direction padding (p-*)
    fn parse_all_padding(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("p-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "padding".to_string(),
                    value: spacing,
                    important: false,
                }]);
            }
        }
        None
    }

    /// Parse axis padding (px-*, py-*)
    fn parse_axis_padding(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("px-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "padding-left".to_string(),
                        value: spacing.clone(),
                        important: false,
                    },
                    CssProperty {
                        name: "padding-right".to_string(),
                        value: spacing,
                        important: false,
                    },
                ]);
            }
        }

        if let Some(value) = class.strip_prefix("py-") {
            if let Some(spacing) = self.get_spacing_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "padding-top".to_string(),
                        value: spacing.clone(),
                        important: false,
                    },
                    CssProperty {
                        name: "padding-bottom".to_string(),
                        value: spacing,
                        important: false,
                    },
                ]);
            }
        }

        None
    }

    /// Parse directional padding (pt-*, pr-*, pb-*, pl-*)
    fn parse_directional_padding(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try different directional patterns
        self.parse_standard_directional_padding(class)
            .or_else(|| self.parse_logical_directional_padding(class))
            .or_else(|| self.parse_arbitrary_directional_padding(class))
            .or_else(|| self.parse_custom_directional_padding(class))
    }

    /// Parse standard directional padding (pt-*, pr-*, pb-*, pl-*)
    fn parse_standard_directional_padding(&self, class: &str) -> Option<Vec<CssProperty>> {
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

        None
    }

    /// Parse logical directional padding (ps-*, pe-*)
    fn parse_logical_directional_padding(&self, class: &str) -> Option<Vec<CssProperty>> {
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

        None
    }

    /// Parse arbitrary directional padding (pt-[...], pr-[...], etc.)
    fn parse_arbitrary_directional_padding(&self, class: &str) -> Option<Vec<CssProperty>> {
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

        None
    }

    /// Parse custom directional padding (pt-(...), pr-(...), etc.)
    fn parse_custom_directional_padding(&self, class: &str) -> Option<Vec<CssProperty>> {
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

    /// Get spacing value for a given size
    fn get_spacing_value(&self, size: &str) -> Option<String> {
        match size {
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
            _ => {
                // Check for arbitrary values
                if let Some(value) = size.strip_prefix("[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(value.to_string());
                    }
                }
                // Check for custom properties
                if let Some(value) = size.strip_prefix("(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(format!("var({})", value));
                    }
                }
                None
            }
        }
    }
}

impl UtilityParser for PaddingParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_padding_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "p-*", "px-*", "py-*", "pt-*", "pr-*", "pb-*", "pl-*", "ps-*", "pe-*", "pt-[*]",
            "pr-[*]", "pb-[*]", "pl-[*]", "ps-[*]", "pe-[*]", "pt-(*)", "pr-(*)", "pb-(*)",
            "pl-(*)", "ps-(*)", "pe-(*)",
        ]
    }

    fn get_priority(&self) -> u32 {
        50 // Medium priority for padding
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Spacing
    }
}
