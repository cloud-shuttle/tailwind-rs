//! Color system for tailwind-rs
//!
//! This module provides color utilities and definitions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a color value in the theme system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Color {
    /// Hex color value (e.g., "#3b82f6")
    Hex(String),
    /// RGB color value
    Rgb { r: u8, g: u8, b: u8 },
    /// RGBA color value
    Rgba { r: u8, g: u8, b: u8, a: f32 },
    /// HSL color value
    Hsl { h: f32, s: f32, l: f32 },
    /// HSLA color value
    Hsla { h: f32, s: f32, l: f32, a: f32 },
    /// Named color reference
    Named(String),
}

impl Color {
    /// Create a new hex color
    pub fn hex(value: impl Into<String>) -> Self {
        Self::Hex(value.into())
    }

    /// Create a new RGB color
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { r, g, b }
    }

    /// Create a new RGBA color
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self::Rgba { r, g, b, a }
    }

    /// Create a new HSL color
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        Self::Hsl { h, s, l }
    }

    /// Create a new HSLA color
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self::Hsla { h, s, l, a }
    }

    /// Create a named color
    pub fn named(value: impl Into<String>) -> Self {
        Self::Named(value.into())
    }

    /// Get CSS value for the color
    pub fn css_value(&self) -> String {
        match self {
            Color::Hex(hex) => hex.clone(),
            Color::Rgb { r, g, b } => format!("rgb({}, {}, {})", r, g, b),
            Color::Rgba { r, g, b, a } => format!("rgba({}, {}, {}, {})", r, g, b, a),
            Color::Hsl { h, s, l } => format!("hsl({}, {}%, {}%)", h, s * 100.0, l * 100.0),
            Color::Hsla { h, s, l, a } => {
                format!("hsla({}, {}%, {}%, {})", h, s * 100.0, l * 100.0, a)
            }
            Color::Named(name) => name.clone(),
        }
    }
}

/// Color palette definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Palette name
    pub name: String,
    /// Color values in the palette
    pub colors: HashMap<String, Color>,
}

impl ColorPalette {
    /// Create a new color palette
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            colors: HashMap::new(),
        }
    }

    /// Add a color to the palette
    pub fn add_color(&mut self, name: impl Into<String>, color: Color) {
        self.colors.insert(name.into(), color);
    }

    /// Get a color from the palette
    pub fn get_color(&self, name: &str) -> Option<&Color> {
        self.colors.get(name)
    }

    /// Get all colors in the palette
    pub fn colors(&self) -> &HashMap<String, Color> {
        &self.colors
    }

    /// Get palette name
    pub fn name(&self) -> &str {
        &self.name
    }
}

// Border radius utilities for backward compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderRadius {
    /// No radius
    None,
    /// Small radius
    Sm,
    /// Default radius
    Default,
    /// Medium radius
    Md,
    /// Large radius
    Lg,
    /// Extra large radius
    Xl,
    /// 2X large radius
    Xl2,
    /// 3X large radius
    Xl3,
    /// Full radius (circular)
    Full,
}

impl BorderRadius {
    pub fn css_value(&self) -> &'static str {
        match self {
            BorderRadius::None => "0",
            BorderRadius::Sm => "0.125rem",
            BorderRadius::Default => "0.25rem",
            BorderRadius::Md => "0.375rem",
            BorderRadius::Lg => "0.5rem",
            BorderRadius::Xl => "0.75rem",
            BorderRadius::Xl2 => "1rem",
            BorderRadius::Xl3 => "1.5rem",
            BorderRadius::Full => "9999px",
        }
    }
}

// Box shadow utilities for backward compatibility
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BoxShadow {
    /// Small shadow
    Sm,
    /// Default shadow
    Default,
    /// Medium shadow
    Md,
    /// Large shadow
    Lg,
    /// Extra large shadow
    Xl,
    /// 2X large shadow
    Xl2,
    /// Inner shadow
    Inner,
    /// No shadow
    None,
}

impl BoxShadow {
    pub fn css_value(&self) -> &'static str {
        match self {
            BoxShadow::Sm => "0 1px 2px 0 rgb(0 0 0 / 0.05)",
            BoxShadow::Default => "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
            BoxShadow::Md => "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
            BoxShadow::Lg => "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
            BoxShadow::Xl => "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
            BoxShadow::Xl2 => "0 25px 50px -12px rgb(0 0 0 / 0.25)",
            BoxShadow::Inner => "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)",
            BoxShadow::None => "none",
        }
    }
}
