//! Theme system for tailwind-rs

use crate::error::{Result, TailwindError};
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

    /// Create a new named color
    pub fn named(name: impl Into<String>) -> Self {
        Self::Named(name.into())
    }

    /// Convert color to CSS string
    pub fn to_css(&self) -> String {
        match self {
            Color::Hex(value) => value.clone(),
            Color::Rgb { r, g, b } => format!("rgb({}, {}, {})", r, g, b),
            Color::Rgba { r, g, b, a } => format!("rgba({}, {}, {}, {})", r, g, b, a),
            Color::Hsl { h, s, l } => format!("hsl({}, {}%, {}%)", h, s * 100.0, l * 100.0),
            Color::Hsla { h, s, l, a } => {
                format!("hsla({}, {}%, {}%, {})", h, s * 100.0, l * 100.0, a)
            }
            Color::Named(name) => format!("var(--color-{})", name),
        }
    }
}

/// Represents a spacing value in the theme system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Spacing {
    /// Pixel value
    Px(f32),
    /// Rem value
    Rem(f32),
    /// Em value
    Em(f32),
    /// Percentage value
    Percent(f32),
    /// Viewport width percentage
    Vw(f32),
    /// Viewport height percentage
    Vh(f32),
    /// Named spacing reference
    Named(String),
}

impl Spacing {
    /// Create a new pixel spacing
    pub fn px(value: f32) -> Self {
        Self::Px(value)
    }

    /// Create a new rem spacing
    pub fn rem(value: f32) -> Self {
        Self::Rem(value)
    }

    /// Create a new em spacing
    pub fn em(value: f32) -> Self {
        Self::Em(value)
    }

    /// Create a new percentage spacing
    pub fn percent(value: f32) -> Self {
        Self::Percent(value)
    }

    /// Create a new viewport width spacing
    pub fn vw(value: f32) -> Self {
        Self::Vw(value)
    }

    /// Create a new viewport height spacing
    pub fn vh(value: f32) -> Self {
        Self::Vh(value)
    }

    /// Create a new named spacing
    pub fn named(name: impl Into<String>) -> Self {
        Self::Named(name.into())
    }

    /// Convert spacing to CSS string
    pub fn to_css(&self) -> String {
        match self {
            Spacing::Px(value) => format!("{}px", value),
            Spacing::Rem(value) => format!("{}rem", value),
            Spacing::Em(value) => format!("{}em", value),
            Spacing::Percent(value) => format!("{}%", value),
            Spacing::Vw(value) => format!("{}vw", value),
            Spacing::Vh(value) => format!("{}vh", value),
            Spacing::Named(name) => format!("var(--spacing-{})", name),
        }
    }
}

/// Represents a border radius value in the theme system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BorderRadius {
    /// Pixel value
    Px(f32),
    /// Rem value
    Rem(f32),
    /// Percentage value
    Percent(f32),
    /// Named border radius reference
    Named(String),
}

impl BorderRadius {
    /// Create a new pixel border radius
    pub fn px(value: f32) -> Self {
        Self::Px(value)
    }

    /// Create a new rem border radius
    pub fn rem(value: f32) -> Self {
        Self::Rem(value)
    }

    /// Create a new percentage border radius
    pub fn percent(value: f32) -> Self {
        Self::Percent(value)
    }

    /// Create a new named border radius
    pub fn named(name: impl Into<String>) -> Self {
        Self::Named(name.into())
    }

    /// Convert border radius to CSS string
    pub fn to_css(&self) -> String {
        match self {
            BorderRadius::Px(value) => format!("{}px", value),
            BorderRadius::Rem(value) => format!("{}rem", value),
            BorderRadius::Percent(value) => format!("{}%", value),
            BorderRadius::Named(name) => format!("var(--border-radius-{})", name),
        }
    }
}

/// Represents a box shadow value in the theme system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoxShadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
    pub spread_radius: f32,
    pub color: Color,
    pub inset: bool,
}

impl BoxShadow {
    /// Create a new box shadow
    pub fn new(
        offset_x: f32,
        offset_y: f32,
        blur_radius: f32,
        spread_radius: f32,
        color: Color,
        inset: bool,
    ) -> Self {
        Self {
            offset_x,
            offset_y,
            blur_radius,
            spread_radius,
            color,
            inset,
        }
    }

    /// Convert box shadow to CSS string
    pub fn to_css(&self) -> String {
        let inset = if self.inset { "inset " } else { "" };
        format!(
            "{}box-shadow: {}px {}px {}px {}px {}",
            inset,
            self.offset_x,
            self.offset_y,
            self.blur_radius,
            self.spread_radius,
            self.color.to_css()
        )
    }
}

/// Represents a theme value that can be any of the theme types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThemeValue {
    Color(Color),
    Spacing(Spacing),
    BorderRadius(BorderRadius),
    BoxShadow(BoxShadow),
    String(String),
    Number(f32),
    Boolean(bool),
}

/// Main theme structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: HashMap<String, Color>,
    pub spacing: HashMap<String, Spacing>,
    pub border_radius: HashMap<String, BorderRadius>,
    pub box_shadows: HashMap<String, BoxShadow>,
    pub custom: HashMap<String, ThemeValue>,
}

impl Theme {
    /// Create a new theme
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            colors: HashMap::new(),
            spacing: HashMap::new(),
            border_radius: HashMap::new(),
            box_shadows: HashMap::new(),
            custom: HashMap::new(),
        }
    }

    /// Add a color to the theme
    pub fn add_color(&mut self, name: impl Into<String>, color: Color) {
        self.colors.insert(name.into(), color);
    }

    /// Add spacing to the theme
    pub fn add_spacing(&mut self, name: impl Into<String>, spacing: Spacing) {
        self.spacing.insert(name.into(), spacing);
    }

    /// Add border radius to the theme
    pub fn add_border_radius(&mut self, name: impl Into<String>, radius: BorderRadius) {
        self.border_radius.insert(name.into(), radius);
    }

    /// Add box shadow to the theme
    pub fn add_box_shadow(&mut self, name: impl Into<String>, shadow: BoxShadow) {
        self.box_shadows.insert(name.into(), shadow);
    }

    /// Add custom value to the theme
    pub fn add_custom(&mut self, name: impl Into<String>, value: ThemeValue) {
        self.custom.insert(name.into(), value);
    }

    /// Get a color from the theme
    pub fn get_color(&self, name: &str) -> Result<&Color> {
        self.colors.get(name).ok_or_else(|| {
            TailwindError::theme(format!(
                "Color '{}' not found in theme '{}'",
                name, self.name
            ))
        })
    }

    /// Get spacing from the theme
    pub fn get_spacing(&self, name: &str) -> Result<&Spacing> {
        self.spacing.get(name).ok_or_else(|| {
            TailwindError::theme(format!(
                "Spacing '{}' not found in theme '{}'",
                name, self.name
            ))
        })
    }

    /// Get border radius from the theme
    pub fn get_border_radius(&self, name: &str) -> Result<&BorderRadius> {
        self.border_radius.get(name).ok_or_else(|| {
            TailwindError::theme(format!(
                "Border radius '{}' not found in theme '{}'",
                name, self.name
            ))
        })
    }

    /// Get box shadow from the theme
    pub fn get_box_shadow(&self, name: &str) -> Result<&BoxShadow> {
        self.box_shadows.get(name).ok_or_else(|| {
            TailwindError::theme(format!(
                "Box shadow '{}' not found in theme '{}'",
                name, self.name
            ))
        })
    }

    /// Get custom value from the theme
    pub fn get_custom(&self, name: &str) -> Result<&ThemeValue> {
        self.custom.get(name).ok_or_else(|| {
            TailwindError::theme(format!(
                "Custom value '{}' not found in theme '{}'",
                name, self.name
            ))
        })
    }
}

/// Create a default theme with common values
pub fn create_default_theme() -> Theme {
    let mut theme = Theme::new("default");

    // Add default colors
    theme.add_color("primary", Color::hex("#3b82f6"));
    theme.add_color("secondary", Color::hex("#64748b"));
    theme.add_color("success", Color::hex("#10b981"));
    theme.add_color("warning", Color::hex("#f59e0b"));
    theme.add_color("error", Color::hex("#ef4444"));
    theme.add_color("white", Color::hex("#ffffff"));
    theme.add_color("black", Color::hex("#000000"));
    theme.add_color("gray-100", Color::hex("#f3f4f6"));
    theme.add_color("gray-500", Color::hex("#6b7280"));
    theme.add_color("gray-900", Color::hex("#111827"));

    // Add default spacing
    theme.add_spacing("xs", Spacing::rem(0.25));
    theme.add_spacing("sm", Spacing::rem(0.5));
    theme.add_spacing("md", Spacing::rem(1.0));
    theme.add_spacing("lg", Spacing::rem(1.5));
    theme.add_spacing("xl", Spacing::rem(2.0));
    theme.add_spacing("2xl", Spacing::rem(3.0));

    // Add default border radius
    theme.add_border_radius("sm", BorderRadius::rem(0.125));
    theme.add_border_radius("md", BorderRadius::rem(0.375));
    theme.add_border_radius("lg", BorderRadius::rem(0.5));
    theme.add_border_radius("xl", BorderRadius::rem(0.75));
    theme.add_border_radius("full", BorderRadius::percent(50.0));

    // Add default box shadows
    theme.add_box_shadow(
        "sm",
        BoxShadow::new(0.0, 1.0, 2.0, 0.0, Color::hex("#000000"), false),
    );
    theme.add_box_shadow(
        "md",
        BoxShadow::new(0.0, 4.0, 6.0, -1.0, Color::hex("#000000"), false),
    );
    theme.add_box_shadow(
        "lg",
        BoxShadow::new(0.0, 10.0, 15.0, -3.0, Color::hex("#000000"), false),
    );

    theme
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let hex_color = Color::hex("#3b82f6");
        assert_eq!(hex_color, Color::Hex("#3b82f6".to_string()));

        let rgb_color = Color::rgb(59, 130, 246);
        assert_eq!(
            rgb_color,
            Color::Rgb {
                r: 59,
                g: 130,
                b: 246
            }
        );

        let named_color = Color::named("primary");
        assert_eq!(named_color, Color::Named("primary".to_string()));
    }

    #[test]
    fn test_color_to_css() {
        let hex_color = Color::hex("#3b82f6");
        assert_eq!(hex_color.to_css(), "#3b82f6");

        let rgb_color = Color::rgb(59, 130, 246);
        assert_eq!(rgb_color.to_css(), "rgb(59, 130, 246)");

        let named_color = Color::named("primary");
        assert_eq!(named_color.to_css(), "var(--color-primary)");
    }

    #[test]
    fn test_spacing_creation() {
        let px_spacing = Spacing::px(16.0);
        assert_eq!(px_spacing, Spacing::Px(16.0));

        let rem_spacing = Spacing::rem(1.0);
        assert_eq!(rem_spacing, Spacing::Rem(1.0));

        let named_spacing = Spacing::named("md");
        assert_eq!(named_spacing, Spacing::Named("md".to_string()));
    }

    #[test]
    fn test_spacing_to_css() {
        let px_spacing = Spacing::px(16.0);
        assert_eq!(px_spacing.to_css(), "16px");

        let rem_spacing = Spacing::rem(1.0);
        assert_eq!(rem_spacing.to_css(), "1rem");

        let named_spacing = Spacing::named("md");
        assert_eq!(named_spacing.to_css(), "var(--spacing-md)");
    }

    #[test]
    fn test_theme_creation() {
        let mut theme = Theme::new("test");
        assert_eq!(theme.name, "test");

        theme.add_color("primary", Color::hex("#3b82f6"));
        assert!(theme.colors.contains_key("primary"));

        let color = theme.get_color("primary").unwrap();
        assert_eq!(color, &Color::hex("#3b82f6"));
    }

    #[test]
    fn test_theme_error_handling() {
        let theme = Theme::new("test");
        let result = theme.get_color("nonexistent");
        assert!(result.is_err());

        if let Err(TailwindError::Theme { message }) = result {
            assert!(message.contains("Color 'nonexistent' not found"));
        }
    }

    #[test]
    fn test_default_theme() {
        let theme = create_default_theme();
        assert_eq!(theme.name, "default");

        // Test that default colors exist
        assert!(theme.get_color("primary").is_ok());
        assert!(theme.get_color("secondary").is_ok());
        assert!(theme.get_color("success").is_ok());

        // Test that default spacing exists
        assert!(theme.get_spacing("sm").is_ok());
        assert!(theme.get_spacing("md").is_ok());
        assert!(theme.get_spacing("lg").is_ok());

        // Test that default border radius exists
        assert!(theme.get_border_radius("sm").is_ok());
        assert!(theme.get_border_radius("md").is_ok());
        assert!(theme.get_border_radius("lg").is_ok());
    }
}
