use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

/// Parser for background utilities
#[derive(Debug, Clone)]
pub struct BackgroundParser;

impl Default for BackgroundParser {
    fn default() -> Self {
        Self::new()
    }
}

impl BackgroundParser {
    /// Create a new BackgroundParser
    pub fn new() -> Self {
        Self
    }
}

impl BackgroundParser {
    /// Parse background-attachment classes
    fn parse_background_attachment_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-fixed" => Some(vec![CssProperty {
                name: "background-attachment".to_string(),
                value: "fixed".to_string(),
                important: false,
            }]),
            "bg-local" => Some(vec![CssProperty {
                name: "background-attachment".to_string(),
                value: "local".to_string(),
                important: false,
            }]),
            "bg-scroll" => Some(vec![CssProperty {
                name: "background-attachment".to_string(),
                value: "scroll".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse background-clip classes
    fn parse_background_clip_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-clip-border" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "border-box".to_string(),
                important: false,
            }]),
            "bg-clip-padding" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "padding-box".to_string(),
                important: false,
            }]),
            "bg-clip-content" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "content-box".to_string(),
                important: false,
            }]),
            "bg-clip-text" => Some(vec![CssProperty {
                name: "background-clip".to_string(),
                value: "text".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse background-image gradient direction classes - generate actual CSS
    fn parse_background_gradient_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let direction = match class {
            "bg-gradient-to-t" => "to top",
            "bg-gradient-to-tr" => "to top right",
            "bg-gradient-to-r" => "to right",
            "bg-gradient-to-br" => "to bottom right",
            "bg-gradient-to-b" => "to bottom",
            "bg-gradient-to-bl" => "to bottom left",
            "bg-gradient-to-l" => "to left",
            "bg-gradient-to-tl" => "to top left",
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

    /// Parse background-color classes
    fn parse_background_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-inherit" => Some(vec![CssProperty {
                name: "background-color".to_string(),
                value: "inherit".to_string(),
                important: false,
            }]),
            "bg-current" => Some(vec![CssProperty {
                name: "background-color".to_string(),
                value: "currentColor".to_string(),
                important: false,
            }]),
            "bg-transparent" => Some(vec![CssProperty {
                name: "background-color".to_string(),
                value: "transparent".to_string(),
                important: false,
            }]),
            "bg-black" => Some(vec![CssProperty {
                name: "background-color".to_string(),
                value: "#000000".to_string(),
                important: false,
            }]),
            "bg-white" => Some(vec![CssProperty {
                name: "background-color".to_string(),
                value: "#ffffff".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for background color
                if let Some(value) = class.strip_prefix("bg-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "background-color".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for background color
                if let Some(value) = class.strip_prefix("bg-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "background-color".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                // Color with opacity modifier (e.g., bg-blue-600/50)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_color = parts[0];
                        let opacity = parts[1];
                        // Handle basic colors like bg-white, bg-black, bg-transparent
                        let color_value = match base_color {
                            "bg-white" => Some("#ffffff".to_string()),
                            "bg-black" => Some("#000000".to_string()),
                            "bg-transparent" => Some("transparent".to_string()),
                            "bg-current" => Some("currentColor".to_string()),
                            "bg-inherit" => Some("inherit".to_string()),
                            _ => self.get_color_value(base_color),
                        };
                        if let Some(color_value) = color_value {
                            // Convert to rgba with opacity
                            if let Some(rgba_value) = convert_hex_to_rgba(&color_value, opacity) {
                                return Some(vec![CssProperty {
                                    name: "background-color".to_string(),
                                    value: rgba_value,
                                    important: false,
                                }]);
                            }
                        }
                    }
                }

                // Standard color classes (bg-red-500, bg-blue-600, etc.)
                if let Some(color_value) = self.get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "background-color".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }

                None
            }
        }
    }

    /// Parse background-image classes
    fn parse_background_image_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-none" => Some(vec![CssProperty {
                name: "background-image".to_string(),
                value: "none".to_string(),
                important: false,
            }]),
            "bg-linear-to-t" => Some(vec![CssProperty {
                name: "background-image".to_string(),
                value: "linear-gradient(to top, var(--tw-gradient-stops))".to_string(),
                important: false,
            }]),
            "bg-linear-to-tr" => Some(vec![CssProperty {
                name: "background-image".to_string(),
                value: "linear-gradient(to top right, var(--tw-gradient-stops))".to_string(),
                important: false,
            }]),
            "bg-linear-to-r" => Some(vec![CssProperty {
                name: "background-image".to_string(),
                value: "linear-gradient(to right, var(--tw-gradient-stops))".to_string(),
                important: false,
            }]),
            "bg-linear-to-br" => Some(vec![CssProperty {
                name: "background-image".to_string(),
                value: "linear-gradient(to bottom right, var(--tw-gradient-stops))".to_string(),
                important: false,
            }]),
            "bg-linear-to-b" => Some(vec![CssProperty {
                name: "background-image".to_string(),
                value: "linear-gradient(to bottom, var(--tw-gradient-stops))".to_string(),
                important: false,
            }]),
            "bg-linear-to-bl" => Some(vec![CssProperty {
                name: "background-image".to_string(),
                value: "linear-gradient(to bottom left, var(--tw-gradient-stops))".to_string(),
                important: false,
            }]),
            "bg-linear-to-l" => Some(vec![CssProperty {
                name: "background-image".to_string(),
                value: "linear-gradient(to left, var(--tw-gradient-stops))".to_string(),
                important: false,
            }]),
            "bg-linear-to-tl" => Some(vec![CssProperty {
                name: "background-image".to_string(),
                value: "linear-gradient(to top left, var(--tw-gradient-stops))".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for background image
                if let Some(value) = class.strip_prefix("bg-(image:") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "background-image".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for background image
                if let Some(value) = class.strip_prefix("bg-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "background-image".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                // Linear gradient with angle
                if let Some(angle) = class.strip_prefix("bg-linear-") {
                    if let Some(angle) = angle.strip_suffix("deg") {
                        if angle.parse::<f32>().is_ok() {
                            return Some(vec![CssProperty {
                                name: "background-image".to_string(),
                                value: format!(
                                    "linear-gradient({}deg, var(--tw-gradient-stops))",
                                    angle
                                ),
                                important: false,
                            }]);
                        }
                    }
                }

                // Radial gradient
                if class.starts_with("bg-radial") {
                    if class == "bg-radial" {
                        return Some(vec![CssProperty {
                            name: "background-image".to_string(),
                            value: "radial-gradient(var(--tw-gradient-stops))".to_string(),
                            important: false,
                        }]);
                    }

                    // Radial gradient with position
                    if let Some(position) = class.strip_prefix("bg-radial-[at_") {
                        if let Some(position) = position.strip_suffix("]") {
                            return Some(vec![CssProperty {
                                name: "background-image".to_string(),
                                value: format!(
                                    "radial-gradient(at {}, var(--tw-gradient-stops))",
                                    position
                                ),
                                important: false,
                            }]);
                        }
                    }
                }

                // Conic gradient
                if class.starts_with("bg-conic") {
                    if class == "bg-conic" {
                        return Some(vec![CssProperty {
                            name: "background-image".to_string(),
                            value: "conic-gradient(var(--tw-gradient-stops))".to_string(),
                            important: false,
                        }]);
                    }

                    // Conic gradient with angle
                    if let Some(angle) = class.strip_prefix("bg-conic-") {
                        if let Some(angle) = angle.strip_suffix("deg") {
                            if angle.parse::<f32>().is_ok() {
                                return Some(vec![CssProperty {
                                    name: "background-image".to_string(),
                                    value: format!(
                                        "conic-gradient({}deg, var(--tw-gradient-stops))",
                                        angle
                                    ),
                                    important: false,
                                }]);
                            }
                        }
                    }
                }

                None
            }
        }
    }

    /// Parse background-origin classes
    fn parse_background_origin_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-origin-border" => Some(vec![CssProperty {
                name: "background-origin".to_string(),
                value: "border-box".to_string(),
                important: false,
            }]),
            "bg-origin-padding" => Some(vec![CssProperty {
                name: "background-origin".to_string(),
                value: "padding-box".to_string(),
                important: false,
            }]),
            "bg-origin-content" => Some(vec![CssProperty {
                name: "background-origin".to_string(),
                value: "content-box".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse background-position classes
    fn parse_background_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-top-left" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "top left".to_string(),
                important: false,
            }]),
            "bg-top" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "top".to_string(),
                important: false,
            }]),
            "bg-top-right" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "top right".to_string(),
                important: false,
            }]),
            "bg-left" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "left".to_string(),
                important: false,
            }]),
            "bg-center" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "center".to_string(),
                important: false,
            }]),
            "bg-right" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "right".to_string(),
                important: false,
            }]),
            "bg-bottom-left" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "bottom left".to_string(),
                important: false,
            }]),
            "bg-bottom" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "bottom".to_string(),
                important: false,
            }]),
            "bg-bottom-right" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "bottom right".to_string(),
                important: false,
            }]),
            "bg-position-top" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "top".to_string(),
                important: false,
            }]),
            "bg-position-bottom" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "bottom".to_string(),
                important: false,
            }]),
            "bg-position-left" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "left".to_string(),
                important: false,
            }]),
            "bg-position-right" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "right".to_string(),
                important: false,
            }]),
            "bg-position-center" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "center".to_string(),
                important: false,
            }]),
            "bg-position-top-left" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "top left".to_string(),
                important: false,
            }]),
            "bg-position-top-right" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "top right".to_string(),
                important: false,
            }]),
            "bg-position-bottom-left" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "bottom left".to_string(),
                important: false,
            }]),
            "bg-position-bottom-right" => Some(vec![CssProperty {
                name: "background-position".to_string(),
                value: "bottom right".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for background position
                if let Some(value) = class.strip_prefix("bg-position-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "background-position".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for background position
                if let Some(value) = class.strip_prefix("bg-position-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "background-position".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                None
            }
        }
    }

    /// Parse background-repeat classes
    fn parse_background_repeat_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-repeat" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "repeat".to_string(),
                important: false,
            }]),
            "bg-repeat-x" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "repeat-x".to_string(),
                important: false,
            }]),
            "bg-repeat-y" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "repeat-y".to_string(),
                important: false,
            }]),
            "bg-repeat-space" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "space".to_string(),
                important: false,
            }]),
            "bg-repeat-round" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "round".to_string(),
                important: false,
            }]),
            "bg-no-repeat" => Some(vec![CssProperty {
                name: "background-repeat".to_string(),
                value: "no-repeat".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse background-size classes
    fn parse_background_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "bg-auto" => Some(vec![CssProperty {
                name: "background-size".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "bg-cover" => Some(vec![CssProperty {
                name: "background-size".to_string(),
                value: "cover".to_string(),
                important: false,
            }]),
            "bg-contain" => Some(vec![CssProperty {
                name: "background-size".to_string(),
                value: "contain".to_string(),
                important: false,
            }]),
            "bg-size-auto" => Some(vec![CssProperty {
                name: "background-size".to_string(),
                value: "auto".to_string(),
                important: false,
            }]),
            "bg-size-cover" => Some(vec![CssProperty {
                name: "background-size".to_string(),
                value: "cover".to_string(),
                important: false,
            }]),
            "bg-size-contain" => Some(vec![CssProperty {
                name: "background-size".to_string(),
                value: "contain".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for background size
                if let Some(value) = class.strip_prefix("bg-size-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "background-size".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }

                // Arbitrary values for background size
                if let Some(value) = class.strip_prefix("bg-size-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "background-size".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }

                None
            }
        }
    }

    /// Get color value for background colors
    fn get_color_value(&self, class: &str) -> Option<String> {
        // This is a simplified color mapping - in a real implementation,
        // you'd want a comprehensive color system
        match class {
            // Basic colors
            "bg-white" => Some("#ffffff".to_string()),
            "bg-black" => Some("#000000".to_string()),
            "bg-transparent" => Some("transparent".to_string()),
            "bg-current" => Some("currentColor".to_string()),
            "bg-inherit" => Some("inherit".to_string()),
            "bg-red-50" => Some("#fef2f2".to_string()),
            "bg-red-100" => Some("#fee2e2".to_string()),
            "bg-red-200" => Some("#fecaca".to_string()),
            "bg-red-300" => Some("#fca5a5".to_string()),
            "bg-red-400" => Some("#f87171".to_string()),
            "bg-red-500" => Some("#ef4444".to_string()),
            "bg-red-600" => Some("#dc2626".to_string()),
            "bg-red-700" => Some("#b91c1c".to_string()),
            "bg-red-800" => Some("#991b1b".to_string()),
            "bg-red-900" => Some("#7f1d1d".to_string()),
            "bg-red-950" => Some("#450a0a".to_string()),
            "bg-blue-50" => Some("#eff6ff".to_string()),
            "bg-blue-100" => Some("#dbeafe".to_string()),
            "bg-blue-200" => Some("#bfdbfe".to_string()),
            "bg-blue-300" => Some("#93c5fd".to_string()),
            "bg-blue-400" => Some("#60a5fa".to_string()),
            "bg-blue-500" => Some("#3b82f6".to_string()),
            "bg-blue-600" => Some("#2563eb".to_string()),
            "bg-blue-700" => Some("#1d4ed8".to_string()),
            "bg-blue-800" => Some("#1e40af".to_string()),
            "bg-blue-900" => Some("#1e3a8a".to_string()),
            "bg-blue-950" => Some("#172554".to_string()),
            "bg-green-50" => Some("#f0fdf4".to_string()),
            "bg-green-100" => Some("#dcfce7".to_string()),
            "bg-green-200" => Some("#bbf7d0".to_string()),
            "bg-green-300" => Some("#86efac".to_string()),
            "bg-green-400" => Some("#4ade80".to_string()),
            "bg-green-500" => Some("#22c55e".to_string()),
            "bg-green-600" => Some("#16a34a".to_string()),
            "bg-green-700" => Some("#15803d".to_string()),
            "bg-green-800" => Some("#166534".to_string()),
            "bg-green-900" => Some("#14532d".to_string()),
            "bg-green-950" => Some("#052e16".to_string()),
            "bg-gray-50" => Some("#f9fafb".to_string()),
            "bg-gray-100" => Some("#f3f4f6".to_string()),
            "bg-gray-200" => Some("#e5e7eb".to_string()),
            "bg-gray-300" => Some("#d1d5db".to_string()),
            "bg-gray-400" => Some("#9ca3af".to_string()),
            "bg-gray-500" => Some("#6b7280".to_string()),
            "bg-gray-600" => Some("#4b5563".to_string()),
            "bg-gray-700" => Some("#374151".to_string()),
            "bg-gray-800" => Some("#1f2937".to_string()),
            "bg-gray-900" => Some("#111827".to_string()),
            "bg-gray-950" => Some("#030712".to_string()),
            _ => None,
        }
    }
}

impl UtilityParser for BackgroundParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity

        // Background attachment (most specific)
        if let Some(properties) = self.parse_background_attachment_class(class) {
            return Some(properties);
        }

        // Background gradient
        if let Some(properties) = self.parse_background_gradient_class(class) {
            return Some(properties);
        }

        // Background clip
        if let Some(properties) = self.parse_background_clip_class(class) {
            return Some(properties);
        }

        // Background origin
        if let Some(properties) = self.parse_background_origin_class(class) {
            return Some(properties);
        }

        // Background position
        if let Some(properties) = self.parse_background_position_class(class) {
            return Some(properties);
        }

        // Background repeat
        if let Some(properties) = self.parse_background_repeat_class(class) {
            return Some(properties);
        }

        // Background size
        if let Some(properties) = self.parse_background_size_class(class) {
            return Some(properties);
        }

        // Background image
        if let Some(properties) = self.parse_background_image_class(class) {
            return Some(properties);
        }

        // Background color (least specific)
        if let Some(properties) = self.parse_background_color_class(class) {
            return Some(properties);
        }

        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "bg-*",
            "bg-fixed",
            "bg-local",
            "bg-scroll",
            "bg-clip-*",
            "bg-origin-*",
            "bg-top-*",
            "bg-left",
            "bg-center",
            "bg-right",
            "bg-bottom-*",
            "bg-repeat-*",
            "bg-no-repeat",
            "bg-auto",
            "bg-cover",
            "bg-contain",
            "bg-linear-*",
            "bg-radial*",
            "bg-conic*",
            "bg-none",
        ]
    }

    fn get_priority(&self) -> u32 {
        80
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Background
    }

}

/// Convert hex color to rgba with opacity
fn convert_hex_to_rgba(hex_color: &str, opacity: &str) -> Option<String> {
    if hex_color.starts_with("#") && hex_color.len() == 7 {
        // Parse hex color
        let r = u8::from_str_radix(&hex_color[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex_color[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex_color[5..7], 16).ok()?;

        // Parse opacity (assuming it's a percentage like "10", "20", etc.)
        let opacity_value = opacity.parse::<f32>().ok()? / 100.0;

        Some(format!("rgba({}, {}, {}, {:.2})", r, g, b, opacity_value))
    } else if hex_color == "transparent" {
        Some("transparent".to_string())
    } else {
        None
    }
}
