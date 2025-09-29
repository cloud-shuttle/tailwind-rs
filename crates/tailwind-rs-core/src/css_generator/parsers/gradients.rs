//! Gradient Utilities Parser
//!
//! This module provides parsing logic for gradient-related Tailwind CSS utilities,
//! such as `bg-linear-to-r`, `from-teal-500/0`, `via-teal-500/40`, `to-teal-500/0`.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct GradientParser;

impl GradientParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse gradient direction classes - generate actual CSS gradients
    fn parse_gradient_direction_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let direction = match class {
            "bg-gradient-to-r" | "bg-linear-to-r" => "to right",
            "bg-linear-to-l" => "to left",
            "bg-linear-to-t" => "to top",
            "bg-linear-to-b" => "to bottom",
            "bg-linear-to-tr" => "to top right",
            "bg-linear-to-tl" => "to top left",
            "bg-linear-to-br" => "to bottom right",
            "bg-linear-to-bl" => "to bottom left",
            _ => return None,
        };

        // Generate a basic linear gradient with placeholder colors
        // Note: Real gradients would need to collect from-, via-, to- stops
        let background_image = format!("linear-gradient({}, #3b82f6, #ef4444)", direction);

        Some(vec![CssProperty {
            name: "background-image".to_string(),
            value: background_image,
            important: false,
        }])
    }

    /// Parse gradient stop classes - add to gradient context for later combination
    pub fn parse_gradient_stop_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Note: This method is now handled by the CssGenerator's context system
        // The actual parsing happens in the CssGeneratorParsers trait implementation
        None
    }

    /// Parse percentage-based gradient stops
    fn parse_percentage_gradient_stop_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Handle from-<percentage> syntax (e.g., from-0%, from-25%, from-50%)
        if let Some(percent_str) = class.strip_prefix("from-") {
            if let Some(percent) = percent_str.strip_suffix("%") {
                if let Ok(percentage) = percent.parse::<f32>() {
                    if percentage >= 0.0 && percentage <= 100.0 {
                        return Some(vec![CssProperty {
                            name: "--tw-gradient-from-position".to_string(),
                            value: format!("{}%", percentage),
                            important: false,
                        }]);
                    }
                }
            }
        }

        // Handle via-<percentage> syntax
        if let Some(percent_str) = class.strip_prefix("via-") {
            if let Some(percent) = percent_str.strip_suffix("%") {
                if let Ok(percentage) = percent.parse::<f32>() {
                    if percentage >= 0.0 && percentage <= 100.0 {
                        return Some(vec![CssProperty {
                            name: "--tw-gradient-via-position".to_string(),
                            value: format!("{}%", percentage),
                            important: false,
                        }]);
                    }
                }
            }
        }

        // Handle to-<percentage> syntax
        if let Some(percent_str) = class.strip_prefix("to-") {
            if let Some(percent) = percent_str.strip_suffix("%") {
                if let Ok(percentage) = percent.parse::<f32>() {
                    if percentage >= 0.0 && percentage <= 100.0 {
                        return Some(vec![CssProperty {
                            name: "--tw-gradient-to-position".to_string(),
                            value: format!("{}%", percentage),
                            important: false,
                        }]);
                    }
                }
            }
        }

        None
    }

    /// Get gradient color value
    fn get_gradient_color_value(&self, color: &str) -> Option<String> {
        // Handle opacity with slash notation (e.g., teal-500/0, teal-500/40)
        if let Some((color_name, opacity)) = color.split_once('/') {
            let base_color = self.get_base_color_value(color_name)?;
            let opacity_value = self.parse_opacity_value(opacity)?;
            return Some(self.apply_opacity_to_color(&base_color, &opacity_value));
        }

        self.get_base_color_value(color)
    }

    /// Get comprehensive Tailwind color value for gradients
    fn get_tailwind_color_value(&self, color_name: &str, intensity: u16) -> Option<String> {
        let hex_value = match color_name {
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
                400 => "#a3e635", 500 => "#65a30d", 600 => "#4d7c0f", 700 => "#3f6212",
                800 => "#365314", 900 => "#1a2e05", 950 => "#0f172a", _ => "#65a30d",
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
            "gray" | "grey" => match intensity {
                50 => "#f9fafb", 100 => "#f3f4f6", 200 => "#e5e7eb", 300 => "#d1d5db",
                400 => "#9ca3af", 500 => "#6b7280", 600 => "#4b5563", 700 => "#374151",
                800 => "#1f2937", 900 => "#111827", 950 => "#030712", _ => "#6b7280",
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
            "sky" => match intensity {
                50 => "#f0f9ff", 100 => "#e0f2fe", 200 => "#bae6fd", 300 => "#7dd3fc",
                400 => "#38bdf8", 500 => "#0ea5e9", 600 => "#0284c7", 700 => "#0369a1",
                800 => "#075985", 900 => "#0c4a6e", 950 => "#082f49", _ => "#0ea5e9",
            },
            "rose" => match intensity {
                50 => "#fff1f2", 100 => "#ffe4e6", 200 => "#fecdd3", 300 => "#fda4af",
                400 => "#fb7185", 500 => "#f43f5e", 600 => "#e11d48", 700 => "#be123c",
                800 => "#9f1239", 900 => "#881337", 950 => "#4c0519", _ => "#f43f5e",
            },
            _ => return None,
        };

        Some(hex_value.to_string())
    }

    /// Get base color value using comprehensive Tailwind color system
    fn get_base_color_value(&self, color: &str) -> Option<String> {
        // Handle transparent
        if color == "transparent" {
            return Some("transparent".to_string());
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

        // Parse Tailwind color scale (e.g., "blue-500", "slate-900", "pink-400")
        if let Some((color_name, intensity_str)) = color.split_once('-') {
            if let Ok(intensity) = intensity_str.parse::<u16>() {
                if (50..=950).contains(&intensity) && intensity % 50 == 0 {
                    return self.get_tailwind_color_value(color_name, intensity);
                }
            }
        }

        // Fallback to basic named colors
        match color {
            // Teal colors
            "teal-50" => Some("#f0fdfa".to_string()),
            "teal-100" => Some("#ccfbf1".to_string()),
            "teal-200" => Some("#99f6e4".to_string()),
            "teal-300" => Some("#5eead4".to_string()),
            "teal-400" => Some("#2dd4bf".to_string()),
            "teal-500" => Some("#14b8a6".to_string()),
            "teal-600" => Some("#0d9488".to_string()),
            "teal-700" => Some("#0f766e".to_string()),
            "teal-800" => Some("#115e59".to_string()),
            "teal-900" => Some("#134e4a".to_string()),
            // Red colors
            "red-50" => Some("#fef2f2".to_string()),
            "red-100" => Some("#fee2e2".to_string()),
            "red-200" => Some("#fecaca".to_string()),
            "red-300" => Some("#fca5a5".to_string()),
            "red-400" => Some("#f87171".to_string()),
            "red-500" => Some("#ef4444".to_string()),
            "red-600" => Some("#dc2626".to_string()),
            "red-700" => Some("#b91c1c".to_string()),
            "red-800" => Some("#991b1b".to_string()),
            "red-900" => Some("#7f1d1d".to_string()),
            // Yellow colors
            "yellow-50" => Some("#fffbeb".to_string()),
            "yellow-100" => Some("#fef3c7".to_string()),
            "yellow-200" => Some("#fde68a".to_string()),
            "yellow-300" => Some("#fcd34d".to_string()),
            "yellow-400" => Some("#fbbf24".to_string()),
            "yellow-500" => Some("#f59e0b".to_string()),
            "yellow-600" => Some("#d97706".to_string()),
            "yellow-700" => Some("#b45309".to_string()),
            "yellow-800" => Some("#92400e".to_string()),
            "yellow-900" => Some("#78350f".to_string()),
            // Blue colors
            "blue-50" => Some("#eff6ff".to_string()),
            "blue-100" => Some("#dbeafe".to_string()),
            "blue-200" => Some("#bfdbfe".to_string()),
            "blue-300" => Some("#93c5fd".to_string()),
            "blue-400" => Some("#60a5fa".to_string()),
            "blue-500" => Some("#3b82f6".to_string()),
            "blue-600" => Some("#2563eb".to_string()),
            "blue-700" => Some("#1d4ed8".to_string()),
            "blue-800" => Some("#1e40af".to_string()),
            "blue-900" => Some("#1e3a8a".to_string()),
            // Indigo colors
            "indigo-50" => Some("#eef2ff".to_string()),
            "indigo-100" => Some("#e0e7ff".to_string()),
            "indigo-200" => Some("#c7d2fe".to_string()),
            "indigo-300" => Some("#a5b4fc".to_string()),
            "indigo-400" => Some("#818cf8".to_string()),
            "indigo-500" => Some("#6366f1".to_string()),
            "indigo-600" => Some("#4f46e5".to_string()),
            "indigo-700" => Some("#4338ca".to_string()),
            "indigo-800" => Some("#3730a3".to_string()),
            "indigo-900" => Some("#312e81".to_string()),
            // Purple colors
            "purple-50" => Some("#faf5ff".to_string()),
            "purple-100" => Some("#f3e8ff".to_string()),
            "purple-200" => Some("#e9d5ff".to_string()),
            "purple-300" => Some("#d8b4fe".to_string()),
            "purple-400" => Some("#c084fc".to_string()),
            "purple-500" => Some("#a855f7".to_string()),
            "purple-600" => Some("#9333ea".to_string()),
            "purple-700" => Some("#7c3aed".to_string()),
            "purple-800" => Some("#6b21a8".to_string()),
            "purple-900" => Some("#581c87".to_string()),
            // Basic colors
            "white" => Some("#ffffff".to_string()),
            "black" => Some("#000000".to_string()),
            "transparent" => Some("transparent".to_string()),
            "current" => Some("currentColor".to_string()),
            _ => None,
        }
    }

    /// Parse opacity value
    fn parse_opacity_value(&self, opacity: &str) -> Option<f32> {
        opacity.parse::<f32>().ok().map(|o| o / 100.0)
    }

    /// Apply opacity to a color
    fn apply_opacity_to_color(&self, color: &str, opacity: &f32) -> String {
        if color.starts_with('#') && color.len() == 7 {
            let r = u8::from_str_radix(&color[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&color[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&color[5..7], 16).unwrap_or(0);
            format!("rgba({}, {}, {}, {})", r, g, b, opacity)
        } else {
            format!("{} / {}", color, opacity)
        }
    }
}

impl UtilityParser for GradientParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try gradient direction classes first
        if let Some(properties) = self.parse_gradient_direction_class(class) {
            return Some(properties);
        }

        // For gradient stops, we can't generate meaningful CSS without the direction
        // In a real implementation, this would need stateful parsing to collect stops
        // For now, return None to let fallback CSS handle it
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "bg-linear-to-*",
            "bg-gradient-to-*",
            "bg-radial",
            "bg-gradient-radial",
            "bg-conic",
            "bg-gradient-conic",
            "from-*",
            "via-*",
            "to-*",
        ]
    }

    fn get_priority(&self) -> u32 {
        95
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Color
    }
}

impl Default for GradientParser {
    fn default() -> Self {
        Self::new()
    }
}
