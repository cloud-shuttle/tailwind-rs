//! Text Color Utilities
//!
//! This module provides parsing logic for text-color related Tailwind CSS utilities.

use crate::css_generator::types::CssProperty;

/// Parse text-color classes with comprehensive color and opacity support
pub fn parse_text_color_class(class: &str) -> Option<Vec<CssProperty>> {
    if let Some(color_part) = class.strip_prefix("text-") {
        // Handle opacity modifiers (e.g., text-blue-500/20)
        if let Some((color_name, opacity)) = color_part.split_once('/') {
            let color_value = get_text_color_value(color_name)?;
            let final_color = convert_hex_to_rgba(&color_value, opacity)?;
            return Some(vec![CssProperty {
                name: "color".to_string(),
                value: final_color,
                important: false,
            }]);
        }

        let color_value = get_text_color_value(color_part)?;
        return Some(vec![CssProperty {
            name: "color".to_string(),
            value: color_value,
            important: false,
        }]);
    }
    None
}

/// Get comprehensive text color values with Tailwind support
fn get_text_color_value(color: &str) -> Option<String> {
    // Handle special colors
    match color {
        "transparent" => Some("transparent".to_string()),
        "current" => Some("currentColor".to_string()),
        "black" => Some("#000000".to_string()),
        "white" => Some("#ffffff".to_string()),
        // Custom neon colors
        "neon-blue" => Some("#00FFFF".to_string()),
        "neon-purple" => Some("#FF00FF".to_string()),
        "neon-green" => Some("#00FF00".to_string()),
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

/// Get Tailwind color value for text colors
fn get_tailwind_color_value(color_name: &str, intensity: u16) -> Option<String> {
    match color_name {
        "red" => match intensity {
            500 => Some("#ef4444".to_string()), 600 => Some("#dc2626".to_string()),
            700 => Some("#b91c1c".to_string()), _ => Some("#ef4444".to_string()),
        },
        "blue" => match intensity {
            300 => Some("#93c5fd".to_string()), 500 => Some("#3b82f6".to_string()),
            600 => Some("#2563eb".to_string()), 700 => Some("#1d4ed8".to_string()),
            200 => Some("#bfdbfe".to_string()), _ => Some("#3b82f6".to_string()),
        },
        "cyan" => match intensity {
            400 => Some("#22d3ee".to_string()), _ => Some("#06b6d4".to_string()),
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
            300 => Some("#d1d5db".to_string()), 400 => Some("#9ca3af".to_string()),
            500 => Some("#6b7280".to_string()), 600 => Some("#4b5563".to_string()),
            700 => Some("#374151".to_string()), _ => Some("#6b7280".to_string()),
        },
        _ => None,
    }
}

/// Convert hex color to rgba with opacity for text colors
fn convert_hex_to_rgba(hex_color: &str, opacity: &str) -> Option<String> {
    if !hex_color.starts_with('#') {
        return None;
    }

    // Parse opacity (e.g., "25" -> 0.25)
    let opacity_value: f32 = opacity.parse::<f32>().ok()? / 100.0;

    // Parse hex color to RGB components
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

    Some(format!("rgba({}, {}, {}, {:.2})", r, g, b, opacity_value))
}
