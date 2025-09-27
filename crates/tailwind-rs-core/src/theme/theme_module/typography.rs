//! Typography system for tailwind-rs
//!
//! This module provides typography utilities and definitions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Font family definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontFamily {
    /// Font family name
    pub name: String,
    /// Font family stack
    pub stack: Vec<String>,
    /// Font weights available
    pub weights: Vec<FontWeight>,
    /// Font styles available
    pub styles: Vec<FontStyle>,
}

impl FontFamily {
    /// Create a new font family
    pub fn new(name: impl Into<String>, stack: Vec<String>) -> Self {
        Self {
            name: name.into(),
            stack,
            weights: vec![
                FontWeight::Normal,
                FontWeight::Bold,
                FontWeight::Light,
                FontWeight::Medium,
                FontWeight::Semibold,
                FontWeight::Extrabold,
            ],
            styles: vec![FontStyle::Normal, FontStyle::Italic],
        }
    }

    /// Get font family CSS value
    pub fn css_value(&self) -> String {
        self.stack.join(", ")
    }

    /// Get font family name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Font weight
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FontWeight {
    /// Thin (100)
    Thin,
    /// Extra Light (200)
    ExtraLight,
    /// Light (300)
    Light,
    /// Normal (400)
    Normal,
    /// Medium (500)
    Medium,
    /// Semibold (600)
    Semibold,
    /// Bold (700)
    Bold,
    /// Extrabold (800)
    Extrabold,
    /// Black (900)
    Black,
}

impl FontWeight {
    /// Get CSS value for font weight
    pub fn css_value(&self) -> String {
        match self {
            FontWeight::Thin => "100".to_string(),
            FontWeight::ExtraLight => "200".to_string(),
            FontWeight::Light => "300".to_string(),
            FontWeight::Normal => "400".to_string(),
            FontWeight::Medium => "500".to_string(),
            FontWeight::Semibold => "600".to_string(),
            FontWeight::Bold => "700".to_string(),
            FontWeight::Extrabold => "800".to_string(),
            FontWeight::Black => "900".to_string(),
        }
    }

    /// Get Tailwind class name
    pub fn class_name(&self) -> &'static str {
        match self {
            FontWeight::Thin => "font-thin",
            FontWeight::ExtraLight => "font-extralight",
            FontWeight::Light => "font-light",
            FontWeight::Normal => "font-normal",
            FontWeight::Medium => "font-medium",
            FontWeight::Semibold => "font-semibold",
            FontWeight::Bold => "font-bold",
            FontWeight::Extrabold => "font-extrabold",
            FontWeight::Black => "font-black",
        }
    }
}

/// Font style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FontStyle {
    /// Normal style
    Normal,
    /// Italic style
    Italic,
}

impl FontStyle {
    /// Get CSS value for font style
    pub fn css_value(&self) -> &'static str {
        match self {
            FontStyle::Normal => "normal",
            FontStyle::Italic => "italic",
        }
    }

    /// Get Tailwind class name
    pub fn class_name(&self) -> &'static str {
        match self {
            FontStyle::Normal => "not-italic",
            FontStyle::Italic => "italic",
        }
    }
}

/// Typography scale definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypographyScale {
    /// Font size scale
    pub font_sizes: HashMap<String, FontSize>,
    /// Line height scale
    pub line_heights: HashMap<String, f32>,
    /// Letter spacing scale
    pub letter_spacings: HashMap<String, f32>,
}

impl Default for TypographyScale {
    fn default() -> Self {
        let mut font_sizes = HashMap::new();
        font_sizes.insert("xs".to_string(), FontSize::rem(0.75));
        font_sizes.insert("sm".to_string(), FontSize::rem(0.875));
        font_sizes.insert("base".to_string(), FontSize::rem(1.0));
        font_sizes.insert("lg".to_string(), FontSize::rem(1.125));
        font_sizes.insert("xl".to_string(), FontSize::rem(1.25));
        font_sizes.insert("2xl".to_string(), FontSize::rem(1.5));
        font_sizes.insert("3xl".to_string(), FontSize::rem(1.875));
        font_sizes.insert("4xl".to_string(), FontSize::rem(2.25));
        font_sizes.insert("5xl".to_string(), FontSize::rem(3.0));
        font_sizes.insert("6xl".to_string(), FontSize::rem(3.75));

        let mut line_heights = HashMap::new();
        line_heights.insert("none".to_string(), 1.0);
        line_heights.insert("tight".to_string(), 1.25);
        line_heights.insert("snug".to_string(), 1.375);
        line_heights.insert("normal".to_string(), 1.5);
        line_heights.insert("relaxed".to_string(), 1.625);
        line_heights.insert("loose".to_string(), 2.0);

        let mut letter_spacings = HashMap::new();
        letter_spacings.insert("tighter".to_string(), -0.05);
        letter_spacings.insert("tight".to_string(), -0.025);
        letter_spacings.insert("normal".to_string(), 0.0);
        letter_spacings.insert("wide".to_string(), 0.025);
        letter_spacings.insert("wider".to_string(), 0.05);
        letter_spacings.insert("widest".to_string(), 0.1);

        Self {
            font_sizes,
            line_heights,
            letter_spacings,
        }
    }
}

impl TypographyScale {
    /// Create a new typography scale
    pub fn new() -> Self {
        Self::default()
    }

    /// Add font size
    pub fn add_font_size(&mut self, name: impl Into<String>, size: FontSize) {
        self.font_sizes.insert(name.into(), size);
    }

    /// Add line height
    pub fn add_line_height(&mut self, name: impl Into<String>, height: f32) {
        self.line_heights.insert(name.into(), height);
    }

    /// Add letter spacing
    pub fn add_letter_spacing(&mut self, name: impl Into<String>, spacing: f32) {
        self.letter_spacings.insert(name.into(), spacing);
    }
}

/// Font size definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontSize {
    /// Size value
    pub size: String,
    /// Line height
    pub line_height: Option<String>,
}

impl FontSize {
    /// Create a rem font size
    pub fn rem(size: f32) -> Self {
        Self {
            size: format!("{}rem", size),
            line_height: None,
        }
    }

    /// Create a rem font size with line height
    pub fn rem_with_line_height(size: f32, line_height: f32) -> Self {
        Self {
            size: format!("{}rem", size),
            line_height: Some(format!("{}rem", line_height)),
        }
    }

    /// Create a pixel font size
    pub fn px(size: u32) -> Self {
        Self {
            size: format!("{}px", size),
            line_height: None,
        }
    }

    /// Get CSS value
    pub fn css_value(&self) -> String {
        if let Some(line_height) = &self.line_height {
            format!("{} / {}", self.size, line_height)
        } else {
            self.size.clone()
        }
    }
}
