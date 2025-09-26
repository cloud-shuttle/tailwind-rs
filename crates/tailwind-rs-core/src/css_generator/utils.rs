//! Utility Functions for CSS Generation
//! 
//! This module provides utility functions for CSS generation,
//! including helper functions and common operations.

use crate::css_generator::types::CssProperty;

/// Convert hex color to RGBA with opacity
pub fn hex_to_rgba(hex: &str, opacity: f32) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    format!("rgba({}, {}, {}, {})", r, g, b, opacity)
}

/// Parse opacity from class string (e.g., "bg-blue-500/50")
pub fn parse_opacity_from_class(class: &str) -> Option<f32> {
    if let Some((_, opacity_str)) = class.split_once('/') {
        if let Ok(opacity) = opacity_str.parse::<f32>() {
            return Some(opacity / 100.0);
        }
    }
    None
}

/// Remove opacity suffix from class string
pub fn remove_opacity_suffix(class: &str) -> String {
    if let Some((base, _)) = class.split_once('/') {
        base.to_string()
    } else {
        class.to_string()
    }
}

/// Create a CSS property with optional important flag
pub fn create_css_property(name: &str, value: &str, important: bool) -> CssProperty {
    CssProperty {
        name: name.to_string(),
        value: value.to_string(),
        important,
    }
}

/// Create multiple CSS properties for a single value
pub fn create_multiple_properties(properties: &[(&str, &str)], important: bool) -> Vec<CssProperty> {
    properties
        .iter()
        .map(|(name, value)| create_css_property(name, value, important))
        .collect()
}

/// Check if a class matches a pattern
pub fn matches_pattern(class: &str, pattern: &str) -> bool {
    if pattern.ends_with('*') {
        let prefix = pattern.trim_end_matches('*');
        class.starts_with(prefix)
    } else {
        class == pattern
    }
}

/// Extract numeric value from class string
pub fn extract_numeric_value(class: &str, prefix: &str) -> Option<f32> {
    if let Some(value_str) = class.strip_prefix(prefix) {
        value_str.parse().ok()
    } else {
        None
    }
}

/// Convert numeric value to CSS unit
pub fn numeric_to_css_unit(value: f32, unit: &str) -> String {
    format!("{}{}", value, unit)
}

/// Convert spacing value to CSS
pub fn spacing_to_css(value: f32) -> String {
    if value == 0.0 {
        "0".to_string()
    } else {
        format!("{}rem", value * 0.25)
    }
}

/// Convert percentage to CSS
pub fn percentage_to_css(value: f32) -> String {
    format!("{}%", value)
}

/// Convert pixel value to CSS
pub fn pixel_to_css(value: f32) -> String {
    format!("{}px", value)
}

/// Convert viewport unit to CSS
pub fn viewport_to_css(value: f32, unit: &str) -> String {
    format!("{}vw", value)
}

/// Check if a value is a valid CSS color
pub fn is_valid_color(value: &str) -> bool {
    value.starts_with('#') || 
    value.starts_with("rgb") || 
    value.starts_with("hsl") ||
    value.starts_with("var(") ||
    matches!(value, "transparent" | "currentColor" | "inherit" | "initial" | "unset")
}

/// Check if a value is a valid CSS length
pub fn is_valid_length(value: &str) -> bool {
    value.ends_with("px") || 
    value.ends_with("rem") || 
    value.ends_with("em") || 
    value.ends_with("%") || 
    value.ends_with("vw") || 
    value.ends_with("vh") ||
    value == "0" ||
    value == "auto"
}

/// Normalize CSS property name
pub fn normalize_property_name(name: &str) -> String {
    name.replace('_', "-").to_lowercase()
}

/// Normalize CSS property value
pub fn normalize_property_value(value: &str) -> String {
    value.trim().to_string()
}

/// Create a CSS rule selector
pub fn create_selector(class: &str) -> String {
    format!(".{}", class)
}

/// Create a responsive selector
pub fn create_responsive_selector(breakpoint: &str, class: &str) -> String {
    format!("{}.{}", breakpoint, class)
}

/// Create a media query
pub fn create_media_query(condition: &str) -> String {
    format!("@media {}", condition)
}

/// Validate CSS property
pub fn validate_css_property(property: &CssProperty) -> bool {
    !property.name.is_empty() && !property.value.is_empty()
}

/// Merge CSS properties
pub fn merge_properties(properties: &[CssProperty]) -> Vec<CssProperty> {
    let mut merged = Vec::new();
    let mut seen = std::collections::HashSet::new();
    
    for property in properties {
        let key = format!("{}:{}", property.name, property.value);
        if seen.insert(key) {
            merged.push(property.clone());
        }
    }
    
    merged
}

/// Sort CSS properties by name
pub fn sort_properties(properties: &mut [CssProperty]) {
    properties.sort_by(|a, b| a.name.cmp(&b.name));
}

/// Create CSS property with important flag
pub fn create_important_property(name: &str, value: &str) -> CssProperty {
    create_css_property(name, value, true)
}

/// Create CSS property without important flag
pub fn create_normal_property(name: &str, value: &str) -> CssProperty {
    create_css_property(name, value, false)
}
