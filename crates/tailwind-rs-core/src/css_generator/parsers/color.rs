//! Color Utilities Parser
//! 
//! This module handles parsing of color-related utilities.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::core::CssProperty;

/// Parser for color utilities
#[derive(Debug, Clone)]
pub struct ColorParser;

impl ColorParser {
    pub fn new() -> Self {
        Self
    }
    
    /// Parse a color value and return CSS properties
    fn parse_color_property(&self, property: &str, color: &str) -> Option<Vec<CssProperty>> {
        let value = self.parse_color_value(color)?;
        Some(vec![CssProperty {
            name: property.to_string(),
            value: value,
            important: false,
        }])
    }
    
    /// Parse a color value (hex, rgb, named colors, etc.)
    fn parse_color_value(&self, color: &str) -> Option<String> {
        // Handle transparent
        if color == "transparent" {
            return Some("transparent".to_string());
        }
        
        // Handle named colors
        if let Some(named_color) = self.parse_named_color(color) {
            return Some(named_color);
        }
        
        // Handle hex colors
        if color.starts_with('#') {
            return Some(color.to_string());
        }
        
        // Handle rgb/rgba
        if color.starts_with("rgb") {
            return Some(color.to_string());
        }
        
        // Handle hsl/hsla
        if color.starts_with("hsl") {
            return Some(color.to_string());
        }
        
        // Handle Tailwind color scale (e.g., "blue-500", "red-300")
        if let Some(tailwind_color) = self.parse_tailwind_color(color) {
            return Some(tailwind_color);
        }
        
        None
    }
    
    /// Parse named colors
    fn parse_named_color(&self, color: &str) -> Option<String> {
        match color {
            "black" => Some("#000000".to_string()),
            "white" => Some("#ffffff".to_string()),
            "red" => Some("#ef4444".to_string()),
            "green" => Some("#22c55e".to_string()),
            "blue" => Some("#3b82f6".to_string()),
            "yellow" => Some("#eab308".to_string()),
            "purple" => Some("#a855f7".to_string()),
            "pink" => Some("#ec4899".to_string()),
            "gray" | "grey" => Some("#6b7280".to_string()),
            _ => None,
        }
    }
    
    /// Parse Tailwind color scale
    fn parse_tailwind_color(&self, color: &str) -> Option<String> {
        // Parse format like "blue-500", "red-300", etc.
        if let Some((color_name, intensity)) = color.split_once('-') {
            if let Ok(intensity_num) = intensity.parse::<u16>() {
                if (100..=950).contains(&intensity_num) && intensity_num % 50 == 0 {
                    return Some(self.get_tailwind_color_value(color_name, intensity_num));
                }
            }
        }
        None
    }
    
    /// Get Tailwind color value for a given color and intensity
    fn get_tailwind_color_value(&self, color_name: &str, intensity: u16) -> String {
        match color_name {
            "red" => match intensity {
                50 => "#fef2f2",
                100 => "#fee2e2",
                200 => "#fecaca",
                300 => "#fca5a5",
                400 => "#f87171",
                500 => "#ef4444",
                600 => "#dc2626",
                700 => "#b91c1c",
                800 => "#991b1b",
                900 => "#7f1d1d",
                950 => "#450a0a",
                _ => "#ef4444",
            },
            "blue" => match intensity {
                50 => "#eff6ff",
                100 => "#dbeafe",
                200 => "#bfdbfe",
                300 => "#93c5fd",
                400 => "#60a5fa",
                500 => "#3b82f6",
                600 => "#2563eb",
                700 => "#1d4ed8",
                800 => "#1e40af",
                900 => "#1e3a8a",
                950 => "#172554",
                _ => "#3b82f6",
            },
            "green" => match intensity {
                50 => "#f0fdf4",
                100 => "#dcfce7",
                200 => "#bbf7d0",
                300 => "#86efac",
                400 => "#4ade80",
                500 => "#22c55e",
                600 => "#16a34a",
                700 => "#15803d",
                800 => "#166534",
                900 => "#14532d",
                950 => "#052e16",
                _ => "#22c55e",
            },
            "gray" | "grey" => match intensity {
                50 => "#f9fafb",
                100 => "#f3f4f6",
                200 => "#e5e7eb",
                300 => "#d1d5db",
                400 => "#9ca3af",
                500 => "#6b7280",
                600 => "#4b5563",
                700 => "#374151",
                800 => "#1f2937",
                900 => "#111827",
                950 => "#030712",
                _ => "#6b7280",
            },
            _ => "#000000", // Default to black for unknown colors
        }.to_string()
    }
}

impl UtilityParser for ColorParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Parse background colors
        if let Some(color) = class.strip_prefix("bg-") {
            return self.parse_color_property("background-color", color);
        }
        
        // Parse text colors
        if let Some(color) = class.strip_prefix("text-") {
            return self.parse_color_property("color", color);
        }
        
        // Parse border colors
        if let Some(color) = class.strip_prefix("border-") {
            return self.parse_color_property("border-color", color);
        }
        
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["bg-*", "text-*", "border-*"]
    }

    fn get_priority(&self) -> u32 {
        90
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Color
    }
}

impl Default for ColorParser {
    fn default() -> Self {
        Self::new()
    }
}
