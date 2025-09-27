//! Color system for tailwind-rs
//!
//! This module provides color utilities and definitions.

use serde::{Deserialize, Serialize};

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
