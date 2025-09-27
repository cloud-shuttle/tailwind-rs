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
                value: "var(--text-xs)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-xs--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-sm" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-sm)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-sm--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-base" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-base)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-base--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-lg" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-lg)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-lg--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-xl)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-xl--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-2xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-2xl)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-2xl--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-3xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-3xl)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-3xl--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-4xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-4xl)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-4xl--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-5xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-5xl)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-5xl--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-6xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-6xl)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-6xl--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-7xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-7xl)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-7xl--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-8xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-8xl)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-8xl--line-height)".to_string(),
                important: false,
            },
        ]),
        "text-9xl" => Some(vec![
            CssProperty {
                name: "font-size".to_string(),
                value: "var(--text-9xl)".to_string(),
                important: false,
            },
            CssProperty {
                name: "line-height".to_string(),
                value: "var(--text-9xl--line-height)".to_string(),
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
            "xs" => "var(--text-xs)".to_string(),
            "sm" => "var(--text-sm)".to_string(),
            "base" => "var(--text-base)".to_string(),
            "lg" => "var(--text-lg)".to_string(),
            "xl" => "var(--text-xl)".to_string(),
            "2xl" => "var(--text-2xl)".to_string(),
            "3xl" => "var(--text-3xl)".to_string(),
            "4xl" => "var(--text-4xl)".to_string(),
            "5xl" => "var(--text-5xl)".to_string(),
            "6xl" => "var(--text-6xl)".to_string(),
            "7xl" => "var(--text-7xl)".to_string(),
            "8xl" => "var(--text-8xl)".to_string(),
            "9xl" => "var(--text-9xl)".to_string(),
            _ => return None,
        };

        let line_height_value = match line_height {
            "none" => "1".to_string(),
            "tight" => "1.25".to_string(),
            "snug" => "1.375".to_string(),
            "normal" => "1.5".to_string(),
            "relaxed" => "1.625".to_string(),
            "loose" => "2".to_string(),
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
