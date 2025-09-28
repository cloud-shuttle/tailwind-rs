//! Color Utilities Parser
//!
//! This module handles parsing of color-related utilities.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

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
            value,
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
                50 => "#fef2f2", 100 => "#fee2e2", 200 => "#fecaca", 300 => "#fca5a5",
                400 => "#f87171", 500 => "#ef4444", 600 => "#dc2626", 700 => "#b91c1c",
                800 => "#991b1b", 900 => "#7f1d1d", 950 => "#450a0a", _ => "#ef4444",
            },
            "blue" => match intensity {
                50 => "#eff6ff", 100 => "#dbeafe", 200 => "#bfdbfe", 300 => "#93c5fd",
                400 => "#60a5fa", 500 => "#3b82f6", 600 => "#2563eb", 700 => "#1d4ed8",
                800 => "#1e40af", 900 => "#1e3a8a", 950 => "#172554", _ => "#3b82f6",
            },
            "green" => match intensity {
                50 => "#f0fdf4", 100 => "#dcfce7", 200 => "#bbf7d0", 300 => "#86efac",
                400 => "#4ade80", 500 => "#22c55e", 600 => "#16a34a", 700 => "#15803d",
                800 => "#166534", 900 => "#14532d", 950 => "#052e16", _ => "#22c55e",
            },
            "purple" => match intensity {
                50 => "#faf5ff", 100 => "#f3e8ff", 200 => "#e9d5ff", 300 => "#d8b4fe",
                400 => "#c084fc", 500 => "#a855f7", 600 => "#9333ea", 700 => "#7c3aed",
                800 => "#6b21a8", 900 => "#581c87", 950 => "#3b0764", _ => "#a855f7",
            },
            "cyan" => match intensity {
                50 => "#ecfeff", 100 => "#cffafe", 200 => "#a5f3fc", 300 => "#67e8f9",
                400 => "#22d3ee", 500 => "#06b6d4", 600 => "#0891b2", 700 => "#0e7490",
                800 => "#155e75", 900 => "#164e63", 950 => "#083344", _ => "#06b6d4",
            },
            "pink" => match intensity {
                50 => "#fdf2f8", 100 => "#fce7f3", 200 => "#fbcfe8", 300 => "#f9a8d4",
                400 => "#f472b6", 500 => "#ec4899", 600 => "#db2777", 700 => "#be185d",
                800 => "#9d174d", 900 => "#831843", 950 => "#500724", _ => "#ec4899",
            },
            "yellow" => match intensity {
                50 => "#fffbeb", 100 => "#fef3c7", 200 => "#fde68a", 300 => "#fcd34d",
                400 => "#fbbf24", 500 => "#f59e0b", 600 => "#d97706", 700 => "#b45309",
                800 => "#92400e", 900 => "#78350f", 950 => "#451a03", _ => "#f59e0b",
            },
            "orange" => match intensity {
                50 => "#fff7ed", 100 => "#ffedd5", 200 => "#fed7aa", 300 => "#fdba74",
                400 => "#fb923c", 500 => "#f97316", 600 => "#ea580c", 700 => "#c2410c",
                800 => "#9a3412", 900 => "#7c2d12", 950 => "#431407", _ => "#f97316",
            },
            "teal" => match intensity {
                50 => "#f0fdfa", 100 => "#ccfbf1", 200 => "#99f6e4", 300 => "#5eead4",
                400 => "#2dd4bf", 500 => "#14b8a6", 600 => "#0f766e", 700 => "#0d9488",
                800 => "#0f766e", 900 => "#134e4a", 950 => "#042f2e", _ => "#14b8a6",
            },
            "indigo" => match intensity {
                50 => "#eef2ff", 100 => "#e0e7ff", 200 => "#c7d2fe", 300 => "#a5b4fc",
                400 => "#818cf8", 500 => "#6366f1", 600 => "#4f46e5", 700 => "#4338ca",
                800 => "#3730a3", 900 => "#312e81", 950 => "#1e1b4b", _ => "#6366f1",
            },
            "violet" => match intensity {
                50 => "#f5f3ff", 100 => "#ede9fe", 200 => "#ddd6fe", 300 => "#c4b5fd",
                400 => "#a78bfa", 500 => "#8b5cf6", 600 => "#7c3aed", 700 => "#6d28d9",
                800 => "#5b21b6", 900 => "#4c1d95", 950 => "#2e1065", _ => "#8b5cf6",
            },
            "lime" => match intensity {
                50 => "#f7fee7", 100 => "#ecfccb", 200 => "#d9f99d", 300 => "#bef264",
                400 => "#a3e635", 500 => "#84cc16", 600 => "#65a30d", 700 => "#4d7c0f",
                800 => "#3f6212", 900 => "#365314", 950 => "#1a2e05", _ => "#84cc16",
            },
            "emerald" => match intensity {
                50 => "#ecfdf5", 100 => "#d1fae5", 200 => "#a7f3d0", 300 => "#6ee7b7",
                400 => "#34d399", 500 => "#10b981", 600 => "#059669", 700 => "#047857",
                800 => "#065f46", 900 => "#064e3b", 950 => "#022c22", _ => "#10b981",
            },
            "slate" => match intensity {
                50 => "#f8fafc", 100 => "#f1f5f9", 200 => "#e2e8f0", 300 => "#cbd5e1",
                400 => "#94a3b8", 500 => "#64748b", 600 => "#475569", 700 => "#334155",
                800 => "#1e293b", 900 => "#0f172a", 950 => "#020617", _ => "#64748b",
            },
            "zinc" => match intensity {
                50 => "#fafafa", 100 => "#f4f4f5", 200 => "#e4e4e7", 300 => "#d4d4d8",
                400 => "#a1a1aa", 500 => "#71717a", 600 => "#52525b", 700 => "#3f3f46",
                800 => "#27272a", 900 => "#18181b", 950 => "#09090b", _ => "#71717a",
            },
            "neutral" => match intensity {
                50 => "#fafafa", 100 => "#f5f5f5", 200 => "#e5e5e5", 300 => "#d4d4d4",
                400 => "#a3a3a3", 500 => "#737373", 600 => "#525252", 700 => "#404040",
                800 => "#262626", 900 => "#171717", 950 => "#0a0a0a", _ => "#737373",
            },
            "stone" => match intensity {
                50 => "#fafaf9", 100 => "#f5f5f4", 200 => "#e7e5e4", 300 => "#d6d3d1",
                400 => "#a8a29e", 500 => "#78716c", 600 => "#57534e", 700 => "#44403c",
                800 => "#292524", 900 => "#1c1917", 950 => "#0c0a09", _ => "#78716c",
            },
            "amber" => match intensity {
                50 => "#fffbeb", 100 => "#fef3c7", 200 => "#fde68a", 300 => "#fcd34d",
                400 => "#fbbf24", 500 => "#f59e0b", 600 => "#d97706", 700 => "#b45309",
                800 => "#92400e", 900 => "#78350f", 950 => "#451a03", _ => "#f59e0b",
            },
            "rose" => match intensity {
                50 => "#fff1f2", 100 => "#ffe4e6", 200 => "#fecdd3", 300 => "#fda4af",
                400 => "#fb7185", 500 => "#f43f5e", 600 => "#e11d48", 700 => "#be185d",
                800 => "#9f1239", 900 => "#881337", 950 => "#4c0519", _ => "#f43f5e",
            },
            "sky" => match intensity {
                50 => "#f0f9ff", 100 => "#e0f2fe", 200 => "#bae6fd", 300 => "#7dd3fc",
                400 => "#38bdf8", 500 => "#0ea5e9", 600 => "#0284c7", 700 => "#0369a1",
                800 => "#075985", 900 => "#0c4a6e", 950 => "#082f49", _ => "#0ea5e9",
            },
            "gray" | "grey" => match intensity {
                50 => "#f9fafb", 100 => "#f3f4f6", 200 => "#e5e7eb", 300 => "#d1d5db",
                400 => "#9ca3af", 500 => "#6b7280", 600 => "#4b5563", 700 => "#374151",
                800 => "#1f2937", 900 => "#111827", 950 => "#030712", _ => "#6b7280",
            },
            _ => "#000000", // Default to black for unknown colors
        }
        .to_string()
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
