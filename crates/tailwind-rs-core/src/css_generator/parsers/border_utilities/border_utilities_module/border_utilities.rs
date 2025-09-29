//! Border Utilities Parser
//!
//! This module provides parsing logic for general border utilities.

use crate::css_generator::types::CssProperty;

/// Parse border utilities
pub fn parse_border_utilities_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "border" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "1px".to_string(),
            important: false,
        }, CssProperty {
            name: "border-style".to_string(),
            value: "solid".to_string(),
            important: false,
        }]),
        "border-0" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "0px".to_string(),
            important: false,
        }]),
        // Handle border width classes
        "border-2" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "2px".to_string(),
            important: false,
        }]),
        "border-4" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "4px".to_string(),
            important: false,
        }]),
        "border-8" => Some(vec![CssProperty {
            name: "border-width".to_string(),
            value: "8px".to_string(),
            important: false,
        }]),
        _ => {
            // Check if this is a border color class (delegate to color parser logic)
            if class.starts_with("border-") && class.len() > 7 {
                // Try to parse as a border color
                let color_part = &class[7..]; // Remove "border-"

                // Handle basic named colors and Tailwind colors with opacity
                if color_part == "transparent" {
                    return Some(vec![CssProperty {
                        name: "border-color".to_string(),
                        value: "transparent".to_string(),
                        important: false,
                    }]);
                }

                // Handle opacity suffix (e.g., "purple-500/50")
                let (color_base, opacity_part) = if let Some((color_base, opacity_str)) = color_part.split_once('/') {
                    (color_base, Some(opacity_str))
                } else {
                    (color_part, None)
                };

                // Parse the base color
                if let Some(base_color) = parse_border_color(color_base) {
                    // If there's opacity, convert to rgba
                    if let Some(opacity_str) = opacity_part {
                        if let Ok(opacity_value) = opacity_str.parse::<f32>() {
                            let alpha = opacity_value / 100.0;
                            if let Some(rgba_color) = convert_hex_to_rgba(&base_color, alpha) {
                                return Some(vec![CssProperty {
                                    name: "border-color".to_string(),
                                    value: rgba_color,
                                    important: false,
                                }]);
                            }
                        }
                    }

                    // No opacity, return the base color
                    return Some(vec![CssProperty {
                        name: "border-color".to_string(),
                        value: base_color,
                        important: false,
                    }]);
                }
            }

            None
        },
    }
}

/// Parse border color values
fn parse_border_color(color: &str) -> Option<String> {
    // Handle named colors
    match color {
        "black" => Some("#000000".to_string()),
        "white" => Some("#ffffff".to_string()),
        "gray" | "grey" => Some("#6b7280".to_string()),
        "red" => Some("#ef4444".to_string()),
        "blue" => Some("#3b82f6".to_string()),
        "green" => Some("#22c55e".to_string()),
        "yellow" => Some("#eab308".to_string()),
        "purple" => Some("#a855f7".to_string()),
        "pink" => Some("#ec4899".to_string()),
        _ => {
            // Handle Tailwind color scale (e.g., "blue-500", "red-300")
            if let Some((color_name, intensity)) = color.split_once('-') {
                if let Ok(intensity_num) = intensity.parse::<u16>() {
                    if (100..=950).contains(&intensity_num) && intensity_num % 50 == 0 {
                        return get_tailwind_color_value(color_name, intensity_num);
                    }
                }
            }
            None
        },
    }
}

/// Get Tailwind color value for border colors
fn get_tailwind_color_value(color_name: &str, intensity: u16) -> Option<String> {
    match color_name {
        "red" => match intensity {
            500 => Some("#ef4444".to_string()), 600 => Some("#dc2626".to_string()),
            700 => Some("#b91c1c".to_string()), _ => Some("#ef4444".to_string()),
        },
        "blue" => match intensity {
            500 => Some("#3b82f6".to_string()), 600 => Some("#2563eb".to_string()),
            700 => Some("#1d4ed8".to_string()), _ => Some("#3b82f6".to_string()),
        },
        "green" => match intensity {
            500 => Some("#22c55e".to_string()), 600 => Some("#16a34a".to_string()),
            700 => Some("#15803d".to_string()), _ => Some("#22c55e".to_string()),
        },
        "purple" => match intensity {
            500 => Some("#a855f7".to_string()), 600 => Some("#9333ea".to_string()),
            700 => Some("#7c3aed".to_string()), _ => Some("#a855f7".to_string()),
        },
        "gray" | "grey" => match intensity {
            500 => Some("#6b7280".to_string()), 600 => Some("#4b5563".to_string()),
            700 => Some("#374151".to_string()), _ => Some("#6b7280".to_string()),
        },
        _ => None,
    }
}

/// Convert hex color to rgba with opacity
fn convert_hex_to_rgba(hex_color: &str, alpha: f32) -> Option<String> {
    if !hex_color.starts_with('#') {
        return None;
    }

    let hex = &hex_color[1..];
    let r: u8;
    let g: u8;
    let b: u8;

    match hex.len() {
        3 => {
            r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
        },
        6 => {
            r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        },
        _ => return None,
    }

    Some(format!("rgba({}, {}, {}, {:.2})", r, g, b, alpha))
}

/// Border utilities parser
#[derive(Debug, Clone)]
pub struct BorderUtilitiesParser;

impl BorderUtilitiesParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, class: &str) -> Option<Vec<CssProperty>> {
        parse_border_utilities_class(class)
    }

    pub fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse(class)
    }
}
