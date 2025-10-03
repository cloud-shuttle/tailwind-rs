//! CSS Generator Operations
//!
//! This module contains the add/remove/update operations for CssGenerator.

use super::types::{CssProperty, CssRule};
use super::generator_parsers::CssGeneratorParsers;
use crate::error::Result;
use crate::responsive::Breakpoint;

/// Operations trait for CssGenerator
pub trait CssGeneratorOperations {
    /// Add a class to the generator
    fn add_class(&mut self, class: &str) -> Result<()>;

    /// Add multiple classes for an element (useful for gradient combinations)
    fn add_classes_for_element(&mut self, classes: &[&str]) -> Result<()>;

    /// Add a CSS selector directly (for non-Tailwind CSS selectors)
    fn add_css_selector(&mut self, selector: &str, properties: &str) -> Result<()>;

    /// Add a responsive class
    fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: &str) -> Result<()>;

    /// Add a custom CSS property
    fn add_custom_property(&mut self, name: &str, value: &str);

    /// Remove a rule by selector
    fn remove_rule(&mut self, selector: &str);

    /// Update a rule
    fn update_rule(&mut self, selector: &str, rule: CssRule);
}

impl super::CssGenerator {
    /// Extract gradient stop type from a class name (from, via, to)
    pub fn extract_gradient_stop_type(class: &str) -> Option<&'static str> {
        if class.starts_with("from-") {
            Some("from")
        } else if class.starts_with("via-") {
            Some("via")
        } else if class.starts_with("to-") {
            Some("to")
        } else {
            None
        }
    }

          /// Extract color from a gradient stop class
          pub fn extract_gradient_color(class: &str, stop_type: &str) -> Option<String> {
              let color_part = class.strip_prefix(&format!("{}-", stop_type))?;

              // Split color and opacity (e.g., "blue-900/30" -> "blue-900", "30")
              let (color_name, opacity) = if let Some((color, opacity_str)) = color_part.split_once('/') {
                  (color, Some(opacity_str))
              } else {
                  (color_part, None)
              };

        // Get the base color value
        let base_color = match color_name {
            // Extended color palette
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

        // Apply opacity if specified
        if let Some(opacity_str) = opacity {
            eprintln!("DEBUG: Applying opacity '{}' to color '{}'", opacity_str, base_color);
            if let Ok(opacity_val) = opacity_str.parse::<f32>() {
                let alpha = opacity_val / 100.0;
                eprintln!("DEBUG: Parsed opacity_val={}, alpha={}", opacity_val, alpha);
                // Convert hex to rgba
                if base_color.starts_with('#') && base_color.len() == 7 {
                    let r = u8::from_str_radix(&base_color[1..3], 16).ok()?;
                    let g = u8::from_str_radix(&base_color[3..5], 16).ok()?;
                    let b = u8::from_str_radix(&base_color[5..7], 16).ok()?;
                    let rgba = format!("rgba({}, {}, {}, {})", r, g, b, alpha);
                    eprintln!("DEBUG: Generated rgba: {}", rgba);
                    Some(rgba)
                } else {
                    eprintln!("DEBUG: Invalid hex color format: {}", base_color);
                    None
                }
            } else {
                eprintln!("DEBUG: Failed to parse opacity: {}", opacity_str);
                None
            }
        } else {
            eprintln!("DEBUG: No opacity, returning base color: {}", base_color);
            Some(base_color.to_string())
        }
    }

    /// Extract gradient direction from a class name
    pub fn extract_gradient_direction(class: &str) -> Option<&'static str> {
        match class {
            "bg-gradient-to-t" => Some("to top"),
            "bg-gradient-to-tr" => Some("to top right"),
            "bg-gradient-to-r" => Some("to right"),
            "bg-gradient-to-br" => Some("to bottom right"),
            "bg-gradient-to-b" => Some("to bottom"),
            "bg-gradient-to-bl" => Some("to bottom left"),
            "bg-gradient-to-l" => Some("to left"),
            "bg-gradient-to-tl" => Some("to top left"),
            _ => None,
        }
    }
}

impl CssGeneratorOperations for super::CssGenerator {
    fn add_class(&mut self, class: &str) -> Result<()> {
        let (variants, base_class) = self.parse_variants(class);

        // Handle gradient stops - they may need special handling based on variants
        if let Some(stop_type) = Self::extract_gradient_stop_type(&base_class) {
            if let Some(color) = Self::extract_gradient_color(&base_class, stop_type) {
                // Always generate CSS variable rule for gradient stops (with or without variants)
                // For variant classes like hover:from-red-600, we need .hover\:from-red-600:hover
                let escaped_class = class.replace(':', "\\:");
                let variant_selector = self.variant_parser.combine_variant_selectors(&variants);
                let selector = format!(".{}{}", escaped_class, variant_selector);

                let rule = super::types::CssRule {
                    selector,
                    properties: vec![super::types::CssProperty {
                        name: format!("--tw-gradient-{}", stop_type),
                        value: color.clone(),
                        important: false,
                    }],
                    media_query: self.variant_parser.get_variant_media_query(&variants),
                    specificity: variants.len() as u32 * 10 + 10,
                };
                self.rules.insert(class.to_string(), rule);

                // Also add to context for element-based parsing
                self.add_gradient_stop(stop_type, color);
                return Ok(());
            }
        }

        // Handle gradient directions - they generate CSS variables
        if let Some(direction) = Self::extract_gradient_direction(&base_class) {
            // Build selector with variants using the new complex variant system
            let escaped_class = class.replace(':', "\\:");
            let variant_selector = self.variant_parser.combine_variant_selectors(&variants);
            let selector = if variant_selector.is_empty() {
                format!(".{}", escaped_class)
            } else {
                format!("{}{}", variant_selector, &format!(".{}", escaped_class))
            };

            let properties = vec![
                super::types::CssProperty {
                    name: "--tw-gradient-stops".to_string(),
                    value: "var(--tw-gradient-from), var(--tw-gradient-via), var(--tw-gradient-to, transparent)".to_string(),
                    important: false,
                },
                super::types::CssProperty {
                    name: "background-image".to_string(),
                    value: format!("linear-gradient({}, var(--tw-gradient-stops))", direction),
                    important: false,
                },
            ];

            let rule = super::types::CssRule {
                selector,
                properties,
                media_query: self.variant_parser.get_variant_media_query(&variants).map(|s| s.to_string()),
                specificity: variants.len() as u32 * 10 + 10,
            };
            self.rules.insert(class.to_string(), rule);
            return Ok(());
        }

        // Handle all other classes normally
        let rule = self.class_to_css_rule(class)?;
        self.rules.insert(class.to_string(), rule);
        Ok(())
    }

    fn add_classes_for_element(&mut self, classes: &[&str]) -> Result<()> {
        // Clear gradient context for this element
        self.clear_element_context();

        // First pass: collect gradient stops
        for &class in classes {
            if let Some(stop_type) = Self::extract_gradient_stop_type(class) {
                if let Some(color) = Self::extract_gradient_color(class, stop_type) {
                    self.add_gradient_stop(stop_type, color);
                }
            }
        }

        // Second pass: process all classes, handling gradients specially
        for &class in classes {
            // Handle gradient directions - they may generate CSS using collected context
            if let Some(direction) = Self::extract_gradient_direction(class) {
                if let Some(gradient_css) = self.generate_gradient_css(direction) {
                    // Create a rule with the gradient CSS
                    let rule = super::types::CssRule {
                        selector: format!(".{}", class),
                        properties: vec![super::types::CssProperty {
                            name: "background-image".to_string(),
                            value: gradient_css,
                            important: false,
                        }],
                        media_query: None,
                        specificity: 10,
                    };
                    self.rules.insert(class.to_string(), rule);
                    continue; // Skip normal processing for this class
                }
            }

            // Handle gradient stops - skip them as they were collected above
            if Self::extract_gradient_stop_type(class).is_some() {
                continue; // Skip gradient stops, they don't generate individual rules
            }

            // Handle all other classes normally
            let rule = self.class_to_css_rule(class)?;
            self.rules.insert(class.to_string(), rule);
        }

        Ok(())
    }

    fn add_css_selector(&mut self, selector: &str, properties: &str) -> Result<()> {
        let rule = CssRule {
            selector: selector.to_string(),
            properties: vec![CssProperty {
                name: "content".to_string(),
                value: properties.to_string(),
                important: false,
            }],
            media_query: None,
            specificity: 0, // CSS selectors have low specificity
        };
        self.rules.insert(selector.to_string(), rule);
        Ok(())
    }

    fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: &str) -> Result<()> {
        let mut rule = self.class_to_css_rule(class)?;
        rule.selector = format!("{}{}", breakpoint.prefix(), class);
        rule.media_query = self.breakpoints.get(&breakpoint).cloned();
        rule.specificity = 20; // Higher specificity for responsive rules

        let responsive_class = format!("{}:{}", breakpoint.prefix().trim_end_matches(':'), class);
        self.rules.insert(responsive_class, rule);
        Ok(())
    }

    fn add_custom_property(&mut self, name: &str, value: &str) {
        self.custom_properties
            .insert(name.to_string(), value.to_string());
    }

    fn remove_rule(&mut self, selector: &str) {
        self.rules.remove(selector);
    }

    fn update_rule(&mut self, selector: &str, rule: CssRule) {
        self.rules.insert(selector.to_string(), rule);
    }
}
