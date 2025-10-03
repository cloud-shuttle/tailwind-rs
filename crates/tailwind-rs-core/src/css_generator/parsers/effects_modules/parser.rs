//! Effects Parser Module
//!
//! Main parsing logic for effects utilities:
//! - EffectsParser: Core parser implementation
//! - Individual effect type parsers

use super::types::*;
use super::utilities::*;
use crate::css_generator::types::CssProperty;

/// Core effects parser
#[derive(Debug, Clone)]
pub struct EffectsParser;

impl EffectsParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse shadow classes
    pub fn parse_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try predefined shadow types first
        if let Some(shadow_type) = EffectClassParser::parse_shadow_type(class) {
            return Some(vec![EffectCssGenerator::generate_shadow_property(shadow_type.css_value())]);
        }

        // Try arbitrary values
        if let Some(value) = EffectValueParser::parse_arbitrary_value(class, "shadow") {
            return Some(vec![EffectCssGenerator::generate_shadow_property(&value)]);
        }

        // Try custom properties
        if let Some(value) = EffectValueParser::parse_custom_property_value(class, "shadow") {
            return Some(vec![EffectCssGenerator::generate_shadow_property(&value)]);
        }

        None
    }

    /// Parse drop shadow classes
    pub fn parse_drop_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "drop-shadow-sm" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))")]),
            "drop-shadow" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))")]),
            "drop-shadow-md" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))")]),
            "drop-shadow-lg" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))")]),
            "drop-shadow-xl" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))")]),
            "drop-shadow-2xl" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))")]),
            "drop-shadow-none" => Some(vec![EffectCssGenerator::generate_filter_property("drop-shadow(0 0 #0000)")]),
            _ => None,
        }
    }

    /// Parse shadow color classes
    pub fn parse_shadow_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(color_part) = class.strip_prefix("shadow-") {
            // Use the existing gradient color parsing logic which handles opacity
            let color_value = self.extract_shadow_color(color_part)?;
            return Some(vec![EffectCssGenerator::generate_shadow_color_property(&color_value)]);
        }
        None
    }

    /// Parse backdrop blur classes
    pub fn parse_backdrop_blur_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(blur_value) = class.strip_prefix("backdrop-blur") {
            let value = match blur_value {
                "" | "-none" => "none",
                "-sm" => "blur(4px)",
                "" => "blur(8px)",
                "-md" => "blur(12px)",
                "-lg" => "blur(16px)",
                "-xl" => "blur(24px)",
                "-2xl" => "blur(40px)",
                "-3xl" => "blur(64px)",
                _ => return None,
            };
            return Some(vec![EffectCssGenerator::generate_backdrop_filter_property(value)]);
        }
        None
    }

    /// Parse backdrop opacity classes
    pub fn parse_backdrop_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(opacity_part) = class.strip_prefix("backdrop-opacity-") {
            let opacity_value = EffectValueParser::parse_opacity_value(opacity_part)?;
            return Some(vec![EffectCssGenerator::generate_backdrop_opacity_property(&opacity_value)]);
        }
        None
    }

    /// Extract shadow color from a shadow color class (same logic as gradient colors)
    pub fn extract_shadow_color(&self, color_part: &str) -> Option<String> {
        // Split color and opacity (e.g., "purple-500/50" -> "purple-500", "50")
        let (color_name, opacity) = if let Some((color, opacity_str)) = color_part.split_once('/') {
            (color, Some(opacity_str))
        } else {
            (color_part, None)
        };

        // Get the base color value (using the same color mapping as gradients)
        let base_color = match color_name {
            // Extended color palette - same as gradient colors
            "slate-50" => "#f8fafc",
            "slate-100" => "#f1f5f9",
            "slate-200" => "#e2e8f0",
            "slate-300" => "#cbd5e1",
            "slate-400" => "#94a3b8",
            "slate-500" => "#64748b",
            "slate-600" => "#475569",
            "slate-700" => "#334155",
            "slate-800" => "#1e293b",
            "slate-900" => "#0f172a",
            "gray-50" => "#f9fafb",
            "gray-100" => "#f3f4f6",
            "gray-200" => "#e5e7eb",
            "gray-300" => "#d1d5db",
            "gray-400" => "#9ca3af",
            "gray-500" => "#6b7280",
            "gray-600" => "#4b5563",
            "gray-700" => "#374151",
            "gray-800" => "#1f2937",
            "gray-900" => "#111827",
            "zinc-50" => "#fafafa",
            "zinc-100" => "#f4f4f5",
            "zinc-200" => "#e4e4e7",
            "zinc-300" => "#d4d4d8",
            "zinc-400" => "#a1a1aa",
            "zinc-500" => "#71717a",
            "zinc-600" => "#52525b",
            "zinc-700" => "#3f3f46",
            "zinc-800" => "#27272a",
            "zinc-900" => "#18181b",
            "neutral-50" => "#fafafa",
            "neutral-100" => "#f5f5f5",
            "neutral-200" => "#e5e5e5",
            "neutral-300" => "#d4d4d4",
            "neutral-400" => "#a3a3a3",
            "neutral-500" => "#737373",
            "neutral-600" => "#525252",
            "neutral-700" => "#404040",
            "neutral-800" => "#262626",
            "neutral-900" => "#171717",
            "stone-50" => "#fafaf9",
            "stone-100" => "#f5f5f4",
            "stone-200" => "#e7e5e4",
            "stone-300" => "#d6d3d1",
            "stone-400" => "#a8a29e",
            "stone-500" => "#78716c",
            "stone-600" => "#57534e",
            "stone-700" => "#44403c",
            "stone-800" => "#292524",
            "stone-900" => "#1c1917",
            "red-50" => "#fef2f2",
            "red-100" => "#fee2e2",
            "red-200" => "#fecaca",
            "red-300" => "#fca5a5",
            "red-400" => "#f87171",
            "red-500" => "#ef4444",
            "red-600" => "#dc2626",
            "red-700" => "#b91c1c",
            "red-800" => "#991b1b",
            "red-900" => "#7f1d1d",
            "orange-50" => "#fff7ed",
            "orange-100" => "#ffedd5",
            "orange-200" => "#fed7aa",
            "orange-300" => "#fdba74",
            "orange-400" => "#fb923c",
            "orange-500" => "#f97316",
            "orange-600" => "#ea580c",
            "orange-700" => "#c2410c",
            "orange-800" => "#9a3412",
            "orange-900" => "#7c2d12",
            "amber-50" => "#fffbeb",
            "amber-100" => "#fef3c7",
            "amber-200" => "#fde68a",
            "amber-300" => "#fcd34d",
            "amber-400" => "#fbbf24",
            "amber-500" => "#f59e0b",
            "amber-600" => "#d97706",
            "amber-700" => "#b45309",
            "amber-800" => "#92400e",
            "amber-900" => "#78350f",
            "yellow-50" => "#fefce8",
            "yellow-100" => "#fef9c3",
            "yellow-200" => "#fef08a",
            "yellow-300" => "#fde047",
            "yellow-400" => "#facc15",
            "yellow-500" => "#eab308",
            "yellow-600" => "#ca8a04",
            "yellow-700" => "#a16207",
            "yellow-800" => "#854d0e",
            "yellow-900" => "#713f12",
            "lime-50" => "#f7fee7",
            "lime-100" => "#ecfccb",
            "lime-200" => "#d9f99d",
            "lime-300" => "#bef264",
            "lime-400" => "#a3e635",
            "lime-500" => "#84cc16",
            "lime-600" => "#65a30d",
            "lime-700" => "#4d7c0f",
            "lime-800" => "#3f6212",
            "lime-900" => "#365314",
            "green-50" => "#f0fdf4",
            "green-100" => "#dcfce7",
            "green-200" => "#bbf7d0",
            "green-300" => "#86efac",
            "green-400" => "#4ade80",
            "green-500" => "#22c55e",
            "green-600" => "#16a34a",
            "green-700" => "#15803d",
            "green-800" => "#166534",
            "green-900" => "#14532d",
            "emerald-50" => "#ecfdf5",
            "emerald-100" => "#d1fae5",
            "emerald-200" => "#a7f3d0",
            "emerald-300" => "#6ee7b7",
            "emerald-400" => "#34d399",
            "emerald-500" => "#10b981",
            "emerald-600" => "#059669",
            "emerald-700" => "#047857",
            "emerald-800" => "#065f46",
            "emerald-900" => "#064e3b",
            "teal-50" => "#f0fdfa",
            "teal-100" => "#ccfbf1",
            "teal-200" => "#99f6e4",
            "teal-300" => "#5eead4",
            "teal-400" => "#2dd4bf",
            "teal-500" => "#14b8a6",
            "teal-600" => "#0d9488",
            "teal-700" => "#0f766e",
            "teal-800" => "#115e59",
            "teal-900" => "#134e4a",
            "cyan-50" => "#ecfeff",
            "cyan-100" => "#cffafe",
            "cyan-200" => "#a5f3fc",
            "cyan-300" => "#67e8f9",
            "cyan-400" => "#22d3ee",
            "cyan-500" => "#06b6d4",
            "cyan-600" => "#0891b2",
            "cyan-700" => "#0e7490",
            "cyan-800" => "#155e75",
            "cyan-900" => "#164e63",
            "sky-50" => "#f0f9ff",
            "sky-100" => "#e0f2fe",
            "sky-200" => "#bae6fd",
            "sky-300" => "#7dd3fc",
            "sky-400" => "#38bdf8",
            "sky-500" => "#0ea5e9",
            "sky-600" => "#0284c7",
            "sky-700" => "#0369a1",
            "sky-800" => "#075985",
            "sky-900" => "#0c4a6e",
            "blue-50" => "#eff6ff",
            "blue-100" => "#dbeafe",
            "blue-200" => "#bfdbfe",
            "blue-300" => "#93c5fd",
            "blue-400" => "#60a5fa",
            "blue-500" => "#3b82f6",
            "blue-600" => "#2563eb",
            "blue-700" => "#1d4ed8",
            "blue-800" => "#1e40af",
            "blue-900" => "#1e3a8a",
            "indigo-50" => "#eef2ff",
            "indigo-100" => "#e0e7ff",
            "indigo-200" => "#c7d2fe",
            "indigo-300" => "#a5b4fc",
            "indigo-400" => "#818cf8",
            "indigo-500" => "#6366f1",
            "indigo-600" => "#4f46e5",
            "indigo-700" => "#4338ca",
            "indigo-800" => "#3730a3",
            "indigo-900" => "#312e81",
            "violet-50" => "#f5f3ff",
            "violet-100" => "#ede9fe",
            "violet-200" => "#ddd6fe",
            "violet-300" => "#c4b5fd",
            "violet-400" => "#a78bfa",
            "violet-500" => "#8b5cf6",
            "violet-600" => "#7c3aed",
            "violet-700" => "#6d28d9",
            "violet-800" => "#5b21b6",
            "violet-900" => "#4c1d95",
            "purple-50" => "#faf5ff",
            "purple-100" => "#f3e8ff",
            "purple-200" => "#e9d5ff",
            "purple-300" => "#d8b4fe",
            "purple-400" => "#c084fc",
            "purple-500" => "#a855f7",
            "purple-600" => "#9333ea",
            "purple-700" => "#7c3aed",
            "purple-800" => "#6b21a8",
            "purple-900" => "#581c87",
            "fuchsia-50" => "#fdf4ff",
            "fuchsia-100" => "#fae8ff",
            "fuchsia-200" => "#f5d0fe",
            "fuchsia-300" => "#f0abfc",
            "fuchsia-400" => "#e879f9",
            "fuchsia-500" => "#d946ef",
            "fuchsia-600" => "#c026d3",
            "fuchsia-700" => "#a21caf",
            "fuchsia-800" => "#86198f",
            "fuchsia-900" => "#701a75",
            "pink-50" => "#fdf2f8",
            "pink-100" => "#fce7f3",
            "pink-200" => "#fbcfe8",
            "pink-300" => "#f9a8d4",
            "pink-400" => "#f472b6",
            "pink-500" => "#ec4899",
            "pink-600" => "#db2777",
            "pink-700" => "#be185d",
            "pink-800" => "#9d174d",
            "pink-900" => "#831843",
            "rose-50" => "#fff1f2",
            "rose-100" => "#ffe4e6",
            "rose-200" => "#fecdd3",
            "rose-300" => "#fda4af",
            "rose-400" => "#fb7185",
            "rose-500" => "#f43f5e",
            "rose-600" => "#e11d48",
            "rose-700" => "#be123c",
            "rose-800" => "#9f1239",
            "rose-900" => "#881337",
            _ => return None,
        };

        // Apply opacity if specified (same logic as gradients)
        if let Some(opacity_str) = opacity {
            if let Ok(opacity_val) = opacity_str.parse::<f32>() {
                let alpha = opacity_val / 100.0;
                // Convert hex to rgba
                if base_color.starts_with('#') && base_color.len() == 7 {
                    let r = u8::from_str_radix(&base_color[1..3], 16).ok()?;
                    let g = u8::from_str_radix(&base_color[3..5], 16).ok()?;
                    let b = u8::from_str_radix(&base_color[5..7], 16).ok()?;
                    let rgba = format!("rgba({}, {}, {}, {})", r, g, b, alpha);
                    Some(rgba)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            Some(base_color.to_string())
        }
    }

    /// Parse opacity classes
    pub fn parse_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try predefined opacity levels first
        if let Some(opacity_level) = EffectClassParser::parse_opacity_level(class) {
            return Some(vec![EffectCssGenerator::generate_opacity_property(&opacity_level.to_css_value())]);
        }

        // Try arbitrary values
        if let Some(value) = EffectValueParser::parse_arbitrary_value(class, "opacity") {
            // Validate that it's a valid opacity value
            if let Ok(parsed) = value.parse::<f32>() {
                if EffectValidator::is_valid_opacity(parsed) {
                    return Some(vec![EffectCssGenerator::generate_opacity_property(&value)]);
                }
            }
            // If it's not a number, use it as-is (for CSS variables, etc.)
            return Some(vec![EffectCssGenerator::generate_opacity_property(&value)]);
        }

        // Try custom properties
        if let Some(value) = EffectValueParser::parse_custom_property_value(class, "opacity") {
            return Some(vec![EffectCssGenerator::generate_opacity_property(&value)]);
        }

        None
    }

    /// Parse mix-blend-mode classes
    pub fn parse_mix_blend_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try predefined blend modes first
        if let Some(blend_mode) = EffectClassParser::parse_blend_mode(class) {
            return Some(vec![EffectCssGenerator::generate_blend_mode_property(blend_mode.css_value())]);
        }

        // Try arbitrary values
        if let Some(value) = EffectValueParser::parse_arbitrary_value(class, "mix-blend") {
            return Some(vec![EffectCssGenerator::generate_blend_mode_property(&value)]);
        }

        // Try custom properties
        if let Some(value) = EffectValueParser::parse_custom_property_value(class, "mix-blend") {
            return Some(vec![EffectCssGenerator::generate_blend_mode_property(&value)]);
        }

        None
    }

    /// Parse mask classes
    pub fn parse_mask_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class == "mask-none" {
            return Some(vec![EffectCssGenerator::generate_mask_property("none")]);
        }

        // Try arbitrary values
        if let Some(value) = EffectValueParser::parse_arbitrary_value(class, "mask") {
            return Some(vec![EffectCssGenerator::generate_mask_property(&value)]);
        }

        // Try custom properties
        if let Some(value) = EffectValueParser::parse_custom_property_value(class, "mask") {
            return Some(vec![EffectCssGenerator::generate_mask_property(&value)]);
        }

        None
    }

    /// Main parsing method that dispatches to specific effect parsers
    pub fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Dispatch to specific effect parsers based on prefix
        if class.starts_with("shadow-") {
            self.parse_shadow_class(class)
        } else if class.starts_with("opacity-") {
            self.parse_opacity_class(class)
        } else if class.starts_with("mix-blend-") {
            self.parse_mix_blend_class(class)
        } else if class.starts_with("mask-") {
            self.parse_mask_class(class)
        } else {
            None
        }
    }
}

impl Default for EffectsParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Effect builder for complex effect combinations
#[derive(Debug, Clone)]
pub struct EffectBuilder {
    effects: Vec<EffectConfig>,
}

impl EffectBuilder {
    /// Create a new effect builder
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    /// Add a shadow effect
    pub fn shadow(mut self, value: String) -> Self {
        self.effects.push(EffectConfig::new(EffectType::Shadow, value));
        self
    }

    /// Add an opacity effect
    pub fn opacity(mut self, value: String) -> Self {
        self.effects.push(EffectConfig::new(EffectType::Opacity, value));
        self
    }

    /// Add a blend mode effect
    pub fn blend_mode(mut self, value: String) -> Self {
        self.effects.push(EffectConfig::new(EffectType::BlendMode, value));
        self
    }

    /// Add a mask effect
    pub fn mask(mut self, value: String) -> Self {
        self.effects.push(EffectConfig::new(EffectType::Mask, value));
        self
    }

    /// Build all CSS properties
    pub fn build(&self) -> Vec<CssProperty> {
        self.effects.iter().map(|config| config.to_css_property()).collect()
    }

    /// Check if the effect combination might have performance issues
    pub fn has_performance_concerns(&self) -> bool {
        self.effects.iter().any(|config| {
            performance::may_cause_mobile_performance_issues(&config.effect_type, &config.value)
        })
    }

    /// Get performance cost estimate
    pub fn performance_cost(&self) -> u32 {
        self.effects.iter()
            .map(|config| performance::estimate_performance_cost(&config.effect_type) as u32)
            .sum()
    }
}

impl Default for EffectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Preset effect combinations
pub mod presets {
    use super::*;

    /// Create a subtle shadow effect
    pub fn subtle_shadow() -> EffectBuilder {
        EffectBuilder::new()
            .shadow("0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string())
            .opacity("1".to_string())
    }

    /// Create a card-like effect
    pub fn card_effect() -> EffectBuilder {
        EffectBuilder::new()
            .shadow("0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string())
            .opacity("1".to_string())
    }

    /// Create a modal backdrop effect
    pub fn modal_backdrop() -> EffectBuilder {
        EffectBuilder::new()
            .opacity("0.5".to_string())
            .blend_mode("multiply".to_string())
    }

    /// Create a glass morphism effect
    pub fn glass_morphism() -> EffectBuilder {
        EffectBuilder::new()
            .opacity("0.25".to_string())
            .blend_mode("overlay".to_string())
            .shadow("0 8px 32px 0 rgba(31, 38, 135, 0.37)".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effects_parser_creation() {
        let parser = EffectsParser::new();
        assert!(parser.parse_class("shadow-sm").is_some());
    }

    #[test]
    fn parse_shadow_classes() {
        let parser = EffectsParser::new();

        let result = parser.parse_shadow_class("shadow-md");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].name, "box-shadow");

        let result = parser.parse_shadow_class("shadow-[custom]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "custom");
    }

    #[test]
    fn parse_opacity_classes() {
        let parser = EffectsParser::new();

        let result = parser.parse_opacity_class("opacity-50");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "0.50");

        let result = parser.parse_opacity_class("opacity-[0.75]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "0.75");
    }

    #[test]
    fn parse_mix_blend_classes() {
        let parser = EffectsParser::new();

        let result = parser.parse_mix_blend_class("mix-blend-multiply");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "multiply");

        let result = parser.parse_mix_blend_class("mix-blend-[screen]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "screen");
    }

    #[test]
    fn parse_mask_classes() {
        let parser = EffectsParser::new();

        let result = parser.parse_mask_class("mask-none");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "none");

        let result = parser.parse_mask_class("mask-[circle]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "circle");
    }

    #[test]
    fn effect_builder_operations() {
        let builder = EffectBuilder::new()
            .shadow("0 1px 2px black".to_string())
            .opacity("0.8".to_string())
            .blend_mode("multiply".to_string());

        let properties = builder.build();
        assert_eq!(properties.len(), 3);

        let shadow_prop = &properties[0];
        assert_eq!(shadow_prop.name, "box-shadow");
        assert_eq!(shadow_prop.value, "0 1px 2px black");

        let opacity_prop = &properties[1];
        assert_eq!(opacity_prop.name, "opacity");
        assert_eq!(opacity_prop.value, "0.8");

        let blend_prop = &properties[2];
        assert_eq!(blend_prop.name, "mix-blend-mode");
        assert_eq!(blend_prop.value, "multiply");
    }

    #[test]
    fn effect_builder_performance() {
        let simple_builder = EffectBuilder::new().opacity("0.5".to_string());
        assert!(!simple_builder.has_performance_concerns());
        assert_eq!(simple_builder.performance_cost(), 1);

        let complex_builder = EffectBuilder::new()
            .blend_mode("multiply".to_string())
            .mask("circle".to_string());
        assert!(complex_builder.has_performance_concerns());
        assert_eq!(complex_builder.performance_cost(), 9); // 4 + 5
    }

    #[test]
    fn effect_presets() {
        let card = presets::card_effect();
        let properties = card.build();
        assert!(properties.len() >= 2); // shadow + opacity

        let glass = presets::glass_morphism();
        let properties = glass.build();
        assert_eq!(properties.len(), 3); // opacity + blend_mode + shadow

        assert!(glass.has_performance_concerns());
    }
}
