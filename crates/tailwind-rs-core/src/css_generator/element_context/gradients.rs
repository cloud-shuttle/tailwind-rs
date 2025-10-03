//! Gradient Context Module
//!
//! Handles gradient-related classes like bg-gradient-to-r, from-pink-400, via-purple-500, to-blue-600

use crate::css_generator::types::CssProperty;

/// Represents the different types of gradient stops
#[derive(Debug, Clone, PartialEq)]
pub enum GradientStopType {
    From,
    Via,
    To,
}

/// Context for managing gradient state within an element
#[derive(Debug, Clone)]
pub struct GradientContext {
    /// Gradient direction (e.g., "to right", "45deg")
    pub direction: Option<String>,
    /// Start color (from-*)
    pub from_color: Option<String>,
    /// Middle color (via-*)
    pub via_color: Option<String>,
    /// End color (to-*)
    pub to_color: Option<String>,
    /// Start position (default: "0%")
    pub from_position: String,
    /// Middle position (default: "50%")
    pub via_position: String,
    /// End position (default: "100%")
    pub to_position: String,
}

impl Default for GradientContext {
    fn default() -> Self {
        Self {
            direction: None,
            from_color: None,
            via_color: None,
            to_color: None,
            from_position: "0%".to_string(),
            via_position: "50%".to_string(),
            to_position: "100%".to_string(),
        }
    }
}

impl GradientContext {
    /// Update context from a gradient class
    pub fn update_from_class(&mut self, class: &str) {
        if let Some(direction) = Self::parse_gradient_direction(class) {
            self.direction = Some(direction);
        } else if let Some((stop_type, color)) = Self::parse_gradient_stop(class) {
            match stop_type {
                GradientStopType::From => self.from_color = color,
                GradientStopType::Via => self.via_color = color,
                GradientStopType::To => self.to_color = color,
            }
        }
    }

    /// Parse gradient direction classes (bg-gradient-to-*, bg-linear-to-*, etc.)
    fn parse_gradient_direction(class: &str) -> Option<String> {
        match class {
            "bg-gradient-to-r" | "bg-linear-to-r" => Some("to right".to_string()),
            "bg-gradient-to-l" | "bg-linear-to-l" => Some("to left".to_string()),
            "bg-gradient-to-t" | "bg-linear-to-t" => Some("to top".to_string()),
            "bg-gradient-to-b" | "bg-linear-to-b" => Some("to bottom".to_string()),
            "bg-gradient-to-tr" | "bg-linear-to-tr" => Some("to top right".to_string()),
            "bg-gradient-to-tl" | "bg-linear-to-tl" => Some("to top left".to_string()),
            "bg-gradient-to-br" | "bg-linear-to-br" => Some("to bottom right".to_string()),
            "bg-gradient-to-bl" | "bg-linear-to-bl" => Some("to bottom left".to_string()),
            _ => {
                // Handle conic gradients
                if class.starts_with("bg-conic") {
                    match class {
                        "bg-conic" => Some("from 0deg at center".to_string()),
                        "bg-conic-at-t" => Some("from 0deg at top".to_string()),
                        "bg-conic-at-b" => Some("from 0deg at bottom".to_string()),
                        "bg-conic-at-l" => Some("from 0deg at left".to_string()),
                        "bg-conic-at-r" => Some("from 0deg at right".to_string()),
                        _ => None,
                    }
                } else if class.starts_with("bg-radial") {
                    match class {
                        "bg-radial" => Some("circle at center".to_string()),
                        "bg-radial-at-t" => Some("ellipse at top".to_string()),
                        "bg-radial-at-b" => Some("ellipse at bottom".to_string()),
                        "bg-radial-at-l" => Some("ellipse at left".to_string()),
                        "bg-radial-at-r" => Some("ellipse at right".to_string()),
                        "bg-radial-at-tl" => Some("ellipse at top left".to_string()),
                        "bg-radial-at-tr" => Some("ellipse at top right".to_string()),
                        "bg-radial-at-bl" => Some("ellipse at bottom left".to_string()),
                        "bg-radial-at-br" => Some("ellipse at bottom right".to_string()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Parse gradient stop classes (from-*, via-*, to-*)
    fn parse_gradient_stop(class: &str) -> Option<(GradientStopType, Option<String>)> {
        if let Some(color) = class.strip_prefix("from-") {
            Some((GradientStopType::From, Self::resolve_color(color)))
        } else if let Some(color) = class.strip_prefix("via-") {
            Some((GradientStopType::Via, Self::resolve_color(color)))
        } else if let Some(color) = class.strip_prefix("to-") {
            Some((GradientStopType::To, Self::resolve_color(color)))
        } else {
            None
        }
    }

    /// Resolve Tailwind color name to hex value
    fn resolve_color(color_name: &str) -> Option<String> {
        // Parse color name and intensity (e.g., "pink-400" -> "pink", 400)
        if let Some(dash_pos) = color_name.rfind('-') {
            let (color, intensity_str) = color_name.split_at(dash_pos);
            let intensity_str = &intensity_str[1..]; // Remove the dash

            if let Ok(intensity) = intensity_str.parse::<u16>() {
                // Basic color resolution for now
                return match color {
                    "pink" => match intensity {
                        400 => Some("#f472b6".to_string()),
                        500 => Some("#ec4899".to_string()),
                        600 => Some("#db2777".to_string()),
                        _ => Some("#ec4899".to_string()),
                    },
                    "purple" => match intensity {
                        400 => Some("#c084fc".to_string()),
                        500 => Some("#a855f7".to_string()),
                        600 => Some("#9333ea".to_string()),
                        900 => Some("#581c87".to_string()),
                        _ => Some("#a855f7".to_string()),
                    },
                    "cyan" => match intensity {
                        400 => Some("#22d3ee".to_string()),
                        500 => Some("#06b6d4".to_string()),
                        _ => Some("#06b6d4".to_string()),
                    },
                    "blue" => match intensity {
                        400 => Some("#60a5fa".to_string()),
                        500 => Some("#3b82f6".to_string()),
                        _ => Some("#3b82f6".to_string()),
                    },
                    "emerald" => match intensity {
                        400 => Some("#34d399".to_string()),
                        500 => Some("#10b981".to_string()),
                        _ => Some("#10b981".to_string()),
                    },
                    "teal" => match intensity {
                        500 => Some("#14b8a6".to_string()),
                        _ => Some("#14b8a6".to_string()),
                    },
                    "green" => match intensity {
                        400 => Some("#4ade80".to_string()),
                        500 => Some("#22c55e".to_string()),
                        _ => Some("#22c55e".to_string()),
                    },
                    "yellow" => match intensity {
                        500 => Some("#f59e0b".to_string()),
                        _ => Some("#f59e0b".to_string()),
                    },
                    "orange" => match intensity {
                        500 => Some("#f97316".to_string()),
                        _ => Some("#f97316".to_string()),
                    },
                    "red" => match intensity {
                        400 => Some("#f87171".to_string()),
                        500 => Some("#ef4444".to_string()),
                        600 => Some("#dc2626".to_string()),
                        _ => Some("#ef4444".to_string()),
                    },
                    "gray" => match intensity {
                        500 => Some("#6b7280".to_string()),
                        600 => Some("#4b5563".to_string()),
                        700 => Some("#374151".to_string()),
                        800 => Some("#1f2937".to_string()),
                        900 => Some("#111827".to_string()),
                        _ => Some("#6b7280".to_string()),
                    },
                    "slate" => match intensity {
                        900 => Some("#0f172a".to_string()),
                        _ => Some("#64748b".to_string()),
                    },
                    _ => None,
                };
            }
        }

        // Handle special color names
        match color_name {
            "white" => Some("#ffffff".to_string()),
            "black" => Some("#000000".to_string()),
            "current" => Some("currentColor".to_string()),
            "transparent" => Some("#00000000".to_string()),
            "inherit" => Some("inherit".to_string()),
            _ => None,
        }
    }

    /// Generate CSS properties for the current gradient context
    pub fn to_css_properties(&self) -> Vec<CssProperty> {
        let mut properties = Vec::new();

        // Always define gradient properties (following Tailwind's approach)
        properties.push(CssProperty::new(
            "--tw-gradient-position".to_string(),
            self.direction.clone().unwrap_or_else(|| "to right".to_string()),
        ));

        properties.push(CssProperty::new(
            "--tw-gradient-from".to_string(),
            self.from_color.clone().unwrap_or_else(|| "#0000".to_string()), // transparent
        ));

        properties.push(CssProperty::new(
            "--tw-gradient-via".to_string(),
            self.via_color.clone().unwrap_or_else(|| "#0000".to_string()), // transparent
        ));

        properties.push(CssProperty::new(
            "--tw-gradient-to".to_string(),
            self.to_color.clone().unwrap_or_else(|| "#0000".to_string()), // transparent
        ));

        // Positions
        properties.push(CssProperty::new("--tw-gradient-from-position".to_string(), self.from_position.clone()));
        properties.push(CssProperty::new("--tw-gradient-via-position".to_string(), self.via_position.clone()));
        properties.push(CssProperty::new("--tw-gradient-to-position".to_string(), self.to_position.clone()));

        // Complex gradient-stops logic (matching real Tailwind)
        let gradient_stops = if self.via_color.is_some() {
            // With via: use --tw-gradient-via-stops
            properties.push(CssProperty::new(
                "--tw-gradient-via-stops".to_string(),
                format!(
                    "var(--tw-gradient-position), var(--tw-gradient-from) var(--tw-gradient-from-position), var(--tw-gradient-via) var(--tw-gradient-via-position), var(--tw-gradient-to) var(--tw-gradient-to-position)"
                ),
            ));
            "var(--tw-gradient-via-stops)".to_string()
        } else {
            // Without via: direct fallback (matching real Tailwind)
            "var(--tw-gradient-via-stops, var(--tw-gradient-position), var(--tw-gradient-from) var(--tw-gradient-from-position), var(--tw-gradient-to) var(--tw-gradient-to-position))".to_string()
        };

        properties.push(CssProperty::new("--tw-gradient-stops".to_string(), gradient_stops));

        // Background image using the gradient stops
        properties.push(CssProperty::new(
            "background-image".to_string(),
            "linear-gradient(var(--tw-gradient-stops))".to_string(),
        ));

        properties
    }

    /// Check if this context has any gradient information
    pub fn has_gradient(&self) -> bool {
        self.direction.is_some() || self.from_color.is_some() || self.via_color.is_some() || self.to_color.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_direction_parsing() {
        let mut context = GradientContext::default();

        // Test direction classes
        context.update_from_class("bg-gradient-to-r");
        assert_eq!(context.direction, Some("to right".to_string()));

        context.update_from_class("bg-gradient-to-tl");
        assert_eq!(context.direction, Some("to top left".to_string()));
    }

    #[test]
    fn gradient_stops_parsing() {
        let mut context = GradientContext::default();

        // Test gradient stops
        context.update_from_class("from-pink-400");
        context.update_from_class("via-purple-500");
        context.update_from_class("to-blue-600");

        assert_eq!(context.from_color, Some("#f472b6".to_string()));
        assert_eq!(context.via_color, Some("#a855f7".to_string()));
        assert_eq!(context.to_color, Some("#9333ea".to_string()));
    }

    #[test]
    fn gradient_css_generation() {
        let mut context = GradientContext::default();
        context.update_from_class("bg-gradient-to-r");
        context.update_from_class("from-pink-400");
        context.update_from_class("to-purple-500");

        let properties = context.to_css_properties();

        // Should generate multiple CSS custom properties
        assert!(properties.len() > 5);

        // Check for specific properties
        let has_background_image = properties.iter().any(|p| p.name == "background-image");
        assert!(has_background_image, "Should generate background-image property");
    }

    #[test]
    fn color_resolution() {
        // Test basic color resolution
        assert_eq!(GradientContext::resolve_color("pink-400"), Some("#f472b6".to_string()));
        assert_eq!(GradientContext::resolve_color("purple-500"), Some("#a855f7".to_string()));
        assert_eq!(GradientContext::resolve_color("white"), Some("#ffffff".to_string()));
        assert_eq!(GradientContext::resolve_color("transparent"), Some("#00000000".to_string()));
    }
}
