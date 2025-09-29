//! Font Size Utilities
//!
//! This module provides parsing logic for font-size related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse font-size classes
pub fn parse_font_size_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "text-xs" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "0.75rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "1rem".to_string(),
                important: false,
            },
        ]),
        "text-sm" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "0.875rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "1.25rem".to_string(),
                important: false,
            },
        ]),
        "text-base" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "1rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "1.5rem".to_string(),
                important: false,
            },
        ]),
        "text-lg" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "1.125rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "1.75rem".to_string(),
                important: false,
            },
        ]),
        "text-xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "1.25rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "1.75rem".to_string(),
                important: false,
            },
        ]),
        "text-2xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "1.5rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "2rem".to_string(),
                important: false,
            },
        ]),
        "text-3xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "1.875rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "2.25rem".to_string(),
                important: false,
            },
        ]),
        "text-4xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "2.25rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "2.5rem".to_string(),
                important: false,
            },
        ]),
        "text-5xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "3rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "1".to_string(),
                important: false,
            },
        ]),
        "text-6xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "3.75rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "1".to_string(),
                important: false,
            },
        ]),
        "text-7xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "4.5rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "1".to_string(),
                important: false,
            },
        ]),
        "text-8xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "6rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "1".to_string(),
                important: false,
            },
        ]),
        "text-9xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "8rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "1".to_string(),
                important: false,
            },
        ]),
        _ => {
            // Custom properties for font-size
            if let Some(value) = class.strip_prefix("text-(") {
                if let Some(value) = value.strip_suffix(")") {
                    return Some(vec![CssProperty {
                        name: "font-size".to_string(),
                        value: format!("var({})", value),
                        important: false,
                    }]);
                }
            }

            // Arbitrary values for font-size
            if let Some(value) = class.strip_prefix("text-[") {
                if let Some(value) = value.strip_suffix("]") {
                    return Some(vec![CssProperty {
                        name: "font-size".to_string(),
                        value: value.to_string(),
                        important: false,
                    }]);
                }
            }

            // Font-size with line-height (e.g., text-sm/6, text-lg/7)
            if let Some(parts) = class.strip_prefix("text-") {
                if let Some((size, line_height)) = parse_font_size_with_line_height(parts) {
                    return Some(vec![
                        CssProperty {
                            name: "font-size".to_string(),
                            value: size,
                            important: false,
                        },
                        CssProperty {
                            name: "line-height".to_string(),
                            value: line_height,
                            important: false,
                        },
                    ]);
                }
            }

            None
        }
    }
}

/// Parse font-size with line-height (e.g., text-sm/6, text-lg/7)
fn parse_font_size_with_line_height(parts: &str) -> Option<(String, String)> {
    if let Some((size, line_height)) = parts.split_once('/') {
        let size_value = match size {
            "xs" => "0.75rem".to_string(),
            "sm" => "0.875rem".to_string(),
            "base" => "1rem".to_string(),
            "lg" => "1.125rem".to_string(),
            "xl" => "1.25rem".to_string(),
            "2xl" => "1.5rem".to_string(),
            "3xl" => "1.875rem".to_string(),
            "4xl" => "2.25rem".to_string(),
            "5xl" => "3rem".to_string(),
            "6xl" => "3.75rem".to_string(),
            "7xl" => "4.5rem".to_string(),
            "8xl" => "6rem".to_string(),
            "9xl" => "8rem".to_string(),
            _ => return None,
        };

        let line_height_value = match line_height {
            "none" => "1".to_string(),
            "tight" => "1.25".to_string(),
            "snug" => "1.375".to_string(),
            "normal" => "1.5".to_string(),
            "relaxed" => "1.625".to_string(),
            "loose" => "2".to_string(),
            "3" => "0.75rem".to_string(),
            "4" => "1rem".to_string(),
            "5" => "1.25rem".to_string(),
            "6" => "1.5rem".to_string(),
            "7" => "1.75rem".to_string(),
            "8" => "2rem".to_string(),
            "9" => "2.25rem".to_string(),
            "10" => "2.5rem".to_string(),
            _ => {
                // Try to parse as number
                if let Ok(num) = line_height.parse::<f64>() {
                    num.to_string()
                } else {
                    return None;
                }
            }
        };

        Some((size_value, line_height_value))
    } else {
        None
    }
}
