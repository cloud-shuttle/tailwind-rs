use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

/// Parser for effects utilities
#[derive(Debug, Clone)]
pub struct EffectsUtilitiesParser;

impl Default for EffectsUtilitiesParser {
    fn default() -> Self {
        Self::new()
    }
}

impl EffectsUtilitiesParser {
    /// Create a new EffectsUtilitiesParser
    pub fn new() -> Self {
        Self
    }

    /// Parse box-shadow classes
    fn parse_box_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "shadow-2xs" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-2xs)".to_string(),
                important: false,
            }]),
            "shadow-xs" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-xs)".to_string(),
                important: false,
            }]),
            "shadow-sm" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-sm)".to_string(),
                important: false,
            }]),
            "shadow" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-md)".to_string(),
                important: false,
            }]),
            "shadow-md" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-md)".to_string(),
                important: false,
            }]),
            "shadow-lg" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-lg)".to_string(),
                important: false,
            }]),
            "shadow-xl" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-xl)".to_string(),
                important: false,
            }]),
            "shadow-2xl" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "var(--shadow-2xl)".to_string(),
                important: false,
            }]),
            "shadow-none" => Some(vec![CssProperty {
                name: "box-shadow".to_string(),
                value: "0 0 #0000".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for box shadow
                if let Some(value) = class.strip_prefix("shadow-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "box-shadow".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for box shadow
                if let Some(value) = class.strip_prefix("shadow-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "box-shadow".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                // Shadow with opacity modifier (e.g., shadow-xl/20)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_shadow = parts[0];
                        let opacity = parts[1];
                        if let Some(shadow_value) = self.get_shadow_value(base_shadow) {
                            return Some(vec![CssProperty {
                                name: "box-shadow".to_string(),
                                value: format!("{}/{}", shadow_value, opacity),
                                important: false,
                            }]);
                        }
                    }
                }

                // Shadow with color (e.g., shadow-indigo-500)
                if let Some(color_value) = self.get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "box-shadow".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }

                // Inset shadows
                if class.starts_with("inset-shadow-") {
                    if let Some(shadow_type) = class.strip_prefix("inset-shadow-") {
                        if let Some(shadow_value) = self.get_inset_shadow_value(shadow_type) {
                            return Some(vec![CssProperty {
                                name: "box-shadow".to_string(),
                                value: shadow_value,
                                important: false,
                            }]);
                        }
                    }
                }

                None
            }
        }
    }

    /// Parse text-shadow classes
    fn parse_text_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "text-shadow-2xs" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "var(--text-shadow-2xs)".to_string(),
                important: false,
            }]),
            "text-shadow-xs" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "var(--text-shadow-xs)".to_string(),
                important: false,
            }]),
            "text-shadow-sm" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "var(--text-shadow-sm)".to_string(),
                important: false,
            }]),
            "text-shadow-md" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "var(--text-shadow-md)".to_string(),
                important: false,
            }]),
            "text-shadow-lg" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "var(--text-shadow-lg)".to_string(),
                important: false,
            }]),
            "text-shadow-none" => Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for text shadow
                if let Some(value) = class.strip_prefix("text-shadow-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "text-shadow".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for text shadow
                if let Some(value) = class.strip_prefix("text-shadow-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "text-shadow".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                // Text shadow with opacity modifier (e.g., text-shadow-lg/20)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_shadow = parts[0];
                        let opacity = parts[1];
                        if let Some(shadow_value) = self.get_text_shadow_value(base_shadow) {
                            return Some(vec![CssProperty {
                                name: "text-shadow".to_string(),
                                value: format!("{}/{}", shadow_value, opacity),
                                important: false,
                            }]);
                        }
                    }
                }

                // Text shadow with color (e.g., text-shadow-indigo-500)
                if let Some(color_value) = self.get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "text-shadow".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }

                None
            }
        }
    }

    /// Parse opacity classes
    fn parse_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(number) = class.strip_prefix("opacity-") {
            if number.parse::<u32>().is_ok() {
                return Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: format!("{}%", number),
                    important: false,
                }]);
            }
        }

        // Custom properties for opacity
        if let Some(value) = class.strip_prefix("opacity-(") {
            if let Some(value) = value.strip_suffix(")") {
                return Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: format!("var({})", value),
                    important: false,
                }]);
            }
        }

        // Arbitrary values for opacity
        if let Some(value) = class.strip_prefix("opacity-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty {
                    name: "opacity".to_string(),
                    value: value.to_string(),
                    important: false,
                }]);
            }
        }

        None
    }

    /// Parse mix-blend-mode classes
    fn parse_mix_blend_mode_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mix-blend-normal" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "normal".to_string(),
                important: false,
            }]),
            "mix-blend-multiply" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "multiply".to_string(),
                important: false,
            }]),
            "mix-blend-screen" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "screen".to_string(),
                important: false,
            }]),
            "mix-blend-overlay" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "overlay".to_string(),
                important: false,
            }]),
            "mix-blend-darken" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "darken".to_string(),
                important: false,
            }]),
            "mix-blend-lighten" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "lighten".to_string(),
                important: false,
            }]),
            "mix-blend-color-dodge" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "color-dodge".to_string(),
                important: false,
            }]),
            "mix-blend-color-burn" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "color-burn".to_string(),
                important: false,
            }]),
            "mix-blend-hard-light" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "hard-light".to_string(),
                important: false,
            }]),
            "mix-blend-soft-light" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "soft-light".to_string(),
                important: false,
            }]),
            "mix-blend-difference" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "difference".to_string(),
                important: false,
            }]),
            "mix-blend-exclusion" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "exclusion".to_string(),
                important: false,
            }]),
            "mix-blend-hue" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "hue".to_string(),
                important: false,
            }]),
            "mix-blend-saturation" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "saturation".to_string(),
                important: false,
            }]),
            "mix-blend-color" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "color".to_string(),
                important: false,
            }]),
            "mix-blend-luminosity" => Some(vec![CssProperty {
                name: "mix-blend-mode".to_string(),
                value: "luminosity".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse background-blend-mode classes
    fn parse_background_blend_mode_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-blend-normal" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "normal".to_string(),
                important: false,
            }]),
            "bg-blend-multiply" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "multiply".to_string(),
                important: false,
            }]),
            "bg-blend-screen" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "screen".to_string(),
                important: false,
            }]),
            "bg-blend-overlay" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "overlay".to_string(),
                important: false,
            }]),
            "bg-blend-darken" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "darken".to_string(),
                important: false,
            }]),
            "bg-blend-lighten" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "lighten".to_string(),
                important: false,
            }]),
            "bg-blend-color-dodge" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "color-dodge".to_string(),
                important: false,
            }]),
            "bg-blend-color-burn" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "color-burn".to_string(),
                important: false,
            }]),
            "bg-blend-hard-light" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "hard-light".to_string(),
                important: false,
            }]),
            "bg-blend-soft-light" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "soft-light".to_string(),
                important: false,
            }]),
            "bg-blend-difference" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "difference".to_string(),
                important: false,
            }]),
            "bg-blend-exclusion" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "exclusion".to_string(),
                important: false,
            }]),
            "bg-blend-hue" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "hue".to_string(),
                important: false,
            }]),
            "bg-blend-saturation" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "saturation".to_string(),
                important: false,
            }]),
            "bg-blend-color" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "color".to_string(),
                important: false,
            }]),
            "bg-blend-luminosity" => Some(vec![CssProperty {
                name: "background-blend-mode".to_string(),
                value: "luminosity".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse mask-clip classes
    fn parse_mask_clip_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mask-clip-border" => Some(vec![CssProperty {
                name: "mask-clip".to_string(),
                value: "border-box".to_string(),
                important: false,
            }]),
            "mask-clip-padding" => Some(vec![CssProperty {
                name: "mask-clip".to_string(),
                value: "padding-box".to_string(),
                important: false,
            }]),
            "mask-clip-content" => Some(vec![CssProperty {
                name: "mask-clip".to_string(),
                value: "content-box".to_string(),
                important: false,
            }]),
            "mask-clip-fill" => Some(vec![CssProperty {
                name: "mask-clip".to_string(),
                value: "fill-box".to_string(),
                important: false,
            }]),
            "mask-clip-stroke" => Some(vec![CssProperty {
                name: "mask-clip".to_string(),
                value: "stroke-box".to_string(),
                important: false,
            }]),
            "mask-clip-view" => Some(vec![CssProperty {
                name: "mask-clip".to_string(),
                value: "view-box".to_string(),
                important: false,
            }]),
            "mask-no-clip" => Some(vec![CssProperty {
                name: "mask-clip".to_string(),
                value: "no-clip".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse mask-composite classes
    fn parse_mask_composite_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "mask-add" => Some(vec![CssProperty {
                name: "mask-composite".to_string(),
                value: "add".to_string(),
                important: false,
            }]),
            "mask-subtract" => Some(vec![CssProperty {
                name: "mask-composite".to_string(),
                value: "subtract".to_string(),
                important: false,
            }]),
            "mask-intersect" => Some(vec![CssProperty {
                name: "mask-composite".to_string(),
                value: "intersect".to_string(),
                important: false,
            }]),
            "mask-exclude" => Some(vec![CssProperty {
                name: "mask-composite".to_string(),
                value: "exclude".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Get shadow value for box shadows
    fn get_shadow_value(&self, shadow_type: &str) -> Option<String> {
        match shadow_type {
            "2xs" => Some("var(--shadow-2xs)".to_string()),
            "xs" => Some("var(--shadow-xs)".to_string()),
            "sm" => Some("var(--shadow-sm)".to_string()),
            "md" => Some("var(--shadow-md)".to_string()),
            "lg" => Some("var(--shadow-lg)".to_string()),
            "xl" => Some("var(--shadow-xl)".to_string()),
            "2xl" => Some("var(--shadow-2xl)".to_string()),
            _ => None,
        }
    }

    /// Get inset shadow value
    fn get_inset_shadow_value(&self, shadow_type: &str) -> Option<String> {
        match shadow_type {
            "2xs" => Some("var(--inset-shadow-2xs)".to_string()),
            "xs" => Some("var(--inset-shadow-xs)".to_string()),
            "sm" => Some("var(--inset-shadow-sm)".to_string()),
            "md" => Some("var(--inset-shadow-md)".to_string()),
            "lg" => Some("var(--inset-shadow-lg)".to_string()),
            "xl" => Some("var(--inset-shadow-xl)".to_string()),
            "2xl" => Some("var(--inset-shadow-2xl)".to_string()),
            "none" => Some("none".to_string()),
            _ => None,
        }
    }

    /// Get text shadow value
    fn get_text_shadow_value(&self, shadow_type: &str) -> Option<String> {
        match shadow_type {
            "2xs" => Some("var(--text-shadow-2xs)".to_string()),
            "xs" => Some("var(--text-shadow-xs)".to_string()),
            "sm" => Some("var(--text-shadow-sm)".to_string()),
            "md" => Some("var(--text-shadow-md)".to_string()),
            "lg" => Some("var(--text-shadow-lg)".to_string()),
            _ => None,
        }
    }

    /// Get color value for shadows
    fn get_color_value(&self, class: &str) -> Option<String> {
        // This is a simplified color mapping - in a real implementation,
        // you'd want a comprehensive color system
        match class {
            "shadow-red-50" => Some("var(--color-red-50)".to_string()),
            "shadow-red-100" => Some("var(--color-red-100)".to_string()),
            "shadow-red-200" => Some("var(--color-red-200)".to_string()),
            "shadow-red-300" => Some("var(--color-red-300)".to_string()),
            "shadow-red-400" => Some("var(--color-red-400)".to_string()),
            "shadow-red-500" => Some("var(--color-red-500)".to_string()),
            "shadow-red-600" => Some("var(--color-red-600)".to_string()),
            "shadow-red-700" => Some("var(--color-red-700)".to_string()),
            "shadow-red-800" => Some("var(--color-red-800)".to_string()),
            "shadow-red-900" => Some("var(--color-red-950)".to_string()),
            "shadow-blue-50" => Some("var(--color-blue-50)".to_string()),
            "shadow-blue-100" => Some("var(--color-blue-100)".to_string()),
            "shadow-blue-200" => Some("var(--color-blue-200)".to_string()),
            "shadow-blue-300" => Some("var(--color-blue-300)".to_string()),
            "shadow-blue-400" => Some("var(--color-blue-400)".to_string()),
            "shadow-blue-500" => Some("var(--color-blue-500)".to_string()),
            "shadow-blue-600" => Some("var(--color-blue-600)".to_string()),
            "shadow-blue-700" => Some("var(--color-blue-700)".to_string()),
            "shadow-blue-800" => Some("var(--color-blue-800)".to_string()),
            "shadow-blue-900" => Some("var(--color-blue-900)".to_string()),
            "shadow-blue-950" => Some("var(--color-blue-950)".to_string()),
            "shadow-green-50" => Some("var(--color-green-50)".to_string()),
            "shadow-green-100" => Some("var(--color-green-100)".to_string()),
            "shadow-green-200" => Some("var(--color-green-200)".to_string()),
            "shadow-green-300" => Some("var(--color-green-300)".to_string()),
            "shadow-green-400" => Some("var(--color-green-400)".to_string()),
            "shadow-green-500" => Some("var(--color-green-500)".to_string()),
            "shadow-green-600" => Some("var(--color-green-600)".to_string()),
            "shadow-green-700" => Some("var(--color-green-700)".to_string()),
            "shadow-green-800" => Some("var(--color-green-800)".to_string()),
            "shadow-green-900" => Some("var(--color-green-900)".to_string()),
            "shadow-green-950" => Some("var(--color-green-950)".to_string()),
            "text-shadow-red-50" => Some("var(--color-red-50)".to_string()),
            "text-shadow-red-100" => Some("var(--color-red-100)".to_string()),
            "text-shadow-red-200" => Some("var(--color-red-200)".to_string()),
            "text-shadow-red-300" => Some("var(--color-red-300)".to_string()),
            "text-shadow-red-400" => Some("var(--color-red-400)".to_string()),
            "text-shadow-red-500" => Some("var(--color-red-500)".to_string()),
            "text-shadow-red-600" => Some("var(--color-red-600)".to_string()),
            "text-shadow-red-700" => Some("var(--color-red-700)".to_string()),
            "text-shadow-red-800" => Some("var(--color-red-800)".to_string()),
            "text-shadow-red-900" => Some("var(--color-red-900)".to_string()),
            "text-shadow-red-950" => Some("var(--color-red-950)".to_string()),
            "text-shadow-blue-50" => Some("var(--color-blue-50)".to_string()),
            "text-shadow-blue-100" => Some("var(--color-blue-100)".to_string()),
            "text-shadow-blue-200" => Some("var(--color-blue-200)".to_string()),
            "text-shadow-blue-300" => Some("var(--color-blue-300)".to_string()),
            "text-shadow-blue-400" => Some("var(--color-blue-400)".to_string()),
            "text-shadow-blue-500" => Some("var(--color-blue-500)".to_string()),
            "text-shadow-blue-600" => Some("var(--color-blue-600)".to_string()),
            "text-shadow-blue-700" => Some("var(--color-blue-700)".to_string()),
            "text-shadow-blue-800" => Some("var(--color-blue-800)".to_string()),
            "text-shadow-blue-900" => Some("var(--color-blue-900)".to_string()),
            "text-shadow-blue-950" => Some("var(--color-blue-950)".to_string()),
            "text-shadow-green-50" => Some("var(--color-green-50)".to_string()),
            "text-shadow-green-100" => Some("var(--color-green-100)".to_string()),
            "text-shadow-green-200" => Some("var(--color-green-200)".to_string()),
            "text-shadow-green-300" => Some("var(--color-green-300)".to_string()),
            "text-shadow-green-400" => Some("var(--color-green-400)".to_string()),
            "text-shadow-green-500" => Some("var(--color-green-500)".to_string()),
            "text-shadow-green-600" => Some("var(--color-green-600)".to_string()),
            "text-shadow-green-700" => Some("var(--color-green-700)".to_string()),
            "text-shadow-green-800" => Some("var(--color-green-800)".to_string()),
            "text-shadow-green-900" => Some("var(--color-green-900)".to_string()),
            "text-shadow-green-950" => Some("var(--color-green-950)".to_string()),
            _ => None,
        }
    }
}

impl UtilityParser for EffectsUtilitiesParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity

        // Mask composite (most specific)
        if let Some(properties) = self.parse_mask_composite_class(class) {
            return Some(properties);
        }

        // Mask clip
        if let Some(properties) = self.parse_mask_clip_class(class) {
            return Some(properties);
        }

        // Background blend mode
        if let Some(properties) = self.parse_background_blend_mode_class(class) {
            return Some(properties);
        }

        // Mix blend mode
        if let Some(properties) = self.parse_mix_blend_mode_class(class) {
            return Some(properties);
        }

        // Opacity
        if let Some(properties) = self.parse_opacity_class(class) {
            return Some(properties);
        }

        // Text shadow
        if let Some(properties) = self.parse_text_shadow_class(class) {
            return Some(properties);
        }

        // Box shadow (least specific)
        if let Some(properties) = self.parse_box_shadow_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "shadow-*",
            "text-shadow-*",
            "opacity-*",
            "mix-blend-*",
            "bg-blend-*",
            "mask-clip-*",
            "mask-*",
            "inset-shadow-*",
            "ring-*",
            "inset-ring-*",
        ]
    }

    fn get_priority(&self) -> u32 {
        90
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Effects
    }
}
