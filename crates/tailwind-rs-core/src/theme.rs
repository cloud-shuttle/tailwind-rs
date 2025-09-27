//! Theme system for tailwind-rs

use crate::error::{Result, TailwindError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

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

impl FromStr for Color {
    type Err = TailwindError;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();

        if s.starts_with('#') {
            Ok(Color::hex(s))
        } else if s.starts_with("rgb(") {
            // Parse rgb(r, g, b) format
            let content = s
                .strip_prefix("rgb(")
                .and_then(|s| s.strip_suffix(')'))
                .ok_or_else(|| TailwindError::theme("Invalid RGB format"))?;

            let values: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
            if values.len() != 3 {
                return Err(TailwindError::theme("RGB must have 3 values"));
            }

            let red = values[0]
                .parse::<u8>()
                .map_err(|_| TailwindError::theme("Invalid RGB red value"))?;
            let green = values[1]
                .parse::<u8>()
                .map_err(|_| TailwindError::theme("Invalid RGB green value"))?;
            let blue = values[2]
                .parse::<u8>()
                .map_err(|_| TailwindError::theme("Invalid RGB blue value"))?;

            Ok(Color::rgb(red, green, blue))
        } else if s.starts_with("rgba(") {
            // Parse rgba(r, g, b, a) format
            let content = s
                .strip_prefix("rgba(")
                .and_then(|s| s.strip_suffix(')'))
                .ok_or_else(|| TailwindError::theme("Invalid RGBA format"))?;

            let values: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
            if values.len() != 4 {
                return Err(TailwindError::theme("RGBA must have 4 values"));
            }

            let red = values[0]
                .parse::<u8>()
                .map_err(|_| TailwindError::theme("Invalid RGBA red value"))?;
            let green = values[1]
                .parse::<u8>()
                .map_err(|_| TailwindError::theme("Invalid RGBA green value"))?;
            let blue = values[2]
                .parse::<u8>()
                .map_err(|_| TailwindError::theme("Invalid RGBA blue value"))?;
            let alpha = values[3]
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid RGBA alpha value"))?;

            Ok(Color::rgba(red, green, blue, alpha))
        } else {
            // Named color
            Ok(Color::named(s))
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

impl FromStr for Spacing {
    type Err = TailwindError;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();

        if s.ends_with("px") {
            let value = s
                .strip_suffix("px")
                .ok_or_else(|| TailwindError::theme("Invalid pixel value"))?
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid pixel value"))?;
            Ok(Spacing::px(value))
        } else if s.ends_with("rem") {
            let value = s
                .strip_suffix("rem")
                .ok_or_else(|| TailwindError::theme("Invalid rem value"))?
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid rem value"))?;
            Ok(Spacing::rem(value))
        } else if s.ends_with("em") {
            let value = s
                .strip_suffix("em")
                .ok_or_else(|| TailwindError::theme("Invalid em value"))?
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid em value"))?;
            Ok(Spacing::em(value))
        } else if s.ends_with('%') {
            let value = s
                .strip_suffix('%')
                .ok_or_else(|| TailwindError::theme("Invalid percentage value"))?
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid percentage value"))?;
            Ok(Spacing::percent(value))
        } else if s.ends_with("vw") {
            let value = s
                .strip_suffix("vw")
                .ok_or_else(|| TailwindError::theme("Invalid vw value"))?
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid vw value"))?;
            Ok(Spacing::vw(value))
        } else if s.ends_with("vh") {
            let value = s
                .strip_suffix("vh")
                .ok_or_else(|| TailwindError::theme("Invalid vh value"))?
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid vh value"))?;
            Ok(Spacing::vh(value))
        } else {
            // Try parsing as number (defaults to rem)
            let value = s
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid spacing value"))?;
            Ok(Spacing::rem(value))
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

impl FromStr for BorderRadius {
    type Err = TailwindError;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();

        if s.ends_with("px") {
            let value = s
                .strip_suffix("px")
                .ok_or_else(|| TailwindError::theme("Invalid pixel value"))?
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid pixel value"))?;
            Ok(BorderRadius::px(value))
        } else if s.ends_with("rem") {
            let value = s
                .strip_suffix("rem")
                .ok_or_else(|| TailwindError::theme("Invalid rem value"))?
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid rem value"))?;
            Ok(BorderRadius::rem(value))
        } else if s.ends_with('%') {
            let value = s
                .strip_suffix('%')
                .ok_or_else(|| TailwindError::theme("Invalid percentage value"))?
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid percentage value"))?;
            Ok(BorderRadius::percent(value))
        } else {
            // Try parsing as number (defaults to rem)
            let value = s
                .parse::<f32>()
                .map_err(|_| TailwindError::theme("Invalid border radius value"))?;
            Ok(BorderRadius::rem(value))
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

impl FromStr for BoxShadow {
    type Err = TailwindError;

    fn from_str(s: &str) -> Result<Self> {
        // Parse box shadow string like "0 1px 3px rgba(0, 0, 0, 0.1)"
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() < 3 {
            return Err(TailwindError::theme("Invalid box shadow format"));
        }

        let offset_x = parts[0]
            .parse::<f32>()
            .map_err(|_| TailwindError::theme("Invalid box shadow offset x"))?;
        let offset_y = parts[1]
            .parse::<f32>()
            .map_err(|_| TailwindError::theme("Invalid box shadow offset y"))?;
        let blur_radius = parts[2]
            .parse::<f32>()
            .map_err(|_| TailwindError::theme("Invalid box shadow blur radius"))?;

        let spread_radius =
            if parts.len() > 3 && !parts[3].starts_with("rgba") && !parts[3].starts_with("rgb") {
                parts[3]
                    .parse::<f32>()
                    .map_err(|_| TailwindError::theme("Invalid box shadow spread radius"))?
            } else {
                0.0
            };

        let color_part =
            if parts.len() > 3 && (parts[3].starts_with("rgba") || parts[3].starts_with("rgb")) {
                parts[3..].join(" ")
            } else if parts.len() > 4 {
                parts[4..].join(" ")
            } else {
                "rgba(0, 0, 0, 0.1)".to_string()
            };

        let color = Color::from_str(&color_part)?;

        Ok(BoxShadow::new(
            offset_x,
            offset_y,
            blur_radius,
            spread_radius,
            color,
            false,
        ))
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

    /// Validate the theme
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(TailwindError::theme(
                "Theme name cannot be empty".to_string(),
            ));
        }

        // Validate colors
        for (name, color) in &self.colors {
            match color {
                Color::Hex(hex) => {
                    if !hex.starts_with('#') || hex.len() != 7 {
                        return Err(TailwindError::theme(format!(
                            "Invalid hex color '{}' for '{}'",
                            hex, name
                        )));
                    }
                }
                Color::Rgb { r: _, g: _, b: _ } => {
                    // RGB values are already validated as u8, so they're always valid
                }
                Color::Rgba {
                    r: _,
                    g: _,
                    b: _,
                    a,
                } => {
                    if *a < 0.0 || *a > 1.0 {
                        return Err(TailwindError::theme(format!(
                            "Invalid RGBA alpha value for '{}'",
                            name
                        )));
                    }
                }
                Color::Hsl { h, s, l } => {
                    if *h < 0.0 || *h > 360.0 || *s < 0.0 || *s > 100.0 || *l < 0.0 || *l > 100.0 {
                        return Err(TailwindError::theme(format!(
                            "Invalid HSL values for '{}'",
                            name
                        )));
                    }
                }
                Color::Hsla { h, s, l, a } => {
                    if *h < 0.0
                        || *h > 360.0
                        || *s < 0.0
                        || *s > 100.0
                        || *l < 0.0
                        || *l > 100.0
                        || *a < 0.0
                        || *a > 1.0
                    {
                        return Err(TailwindError::theme(format!(
                            "Invalid HSLA values for '{}'",
                            name
                        )));
                    }
                }
                Color::Named(_) => {} // Named colors are always valid
            }
        }

        Ok(())
    }
}

/// TOML representation of theme configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeToml {
    pub name: String,
    pub colors: Option<HashMap<String, String>>,
    pub spacing: Option<HashMap<String, String>>,
    pub border_radius: Option<HashMap<String, String>>,
    pub box_shadows: Option<HashMap<String, String>>,
    pub custom: Option<HashMap<String, toml::Value>>,
}

impl From<Theme> for ThemeToml {
    fn from(theme: Theme) -> Self {
        Self {
            name: theme.name,
            colors: Some(
                theme
                    .colors
                    .into_iter()
                    .map(|(k, v)| (k, v.to_css()))
                    .collect(),
            ),
            spacing: Some(
                theme
                    .spacing
                    .into_iter()
                    .map(|(k, v)| (k, v.to_css()))
                    .collect(),
            ),
            border_radius: Some(
                theme
                    .border_radius
                    .into_iter()
                    .map(|(k, v)| (k, v.to_css()))
                    .collect(),
            ),
            box_shadows: Some(
                theme
                    .box_shadows
                    .into_iter()
                    .map(|(k, v)| (k, v.to_css()))
                    .collect(),
            ),
            custom: Some(
                theme
                    .custom
                    .into_iter()
                    .map(|(k, v)| {
                        let toml_value = match v {
                            ThemeValue::String(s) => toml::Value::String(s),
                            ThemeValue::Number(n) => toml::Value::Float(n as f64),
                            ThemeValue::Boolean(b) => toml::Value::Boolean(b),
                            ThemeValue::Color(c) => toml::Value::String(c.to_css()),
                            ThemeValue::Spacing(s) => toml::Value::String(s.to_css()),
                            ThemeValue::BorderRadius(br) => toml::Value::String(br.to_css()),
                            ThemeValue::BoxShadow(bs) => toml::Value::String(bs.to_css()),
                        };
                        (k, toml_value)
                    })
                    .collect(),
            ),
        }
    }
}

impl From<ThemeToml> for Theme {
    fn from(toml_theme: ThemeToml) -> Self {
        let mut theme = Theme::new(toml_theme.name);

        if let Some(colors) = toml_theme.colors {
            for (name, color_str) in colors {
                if let Ok(color) = Color::from_str(&color_str) {
                    theme.add_color(name, color);
                }
            }
        }

        if let Some(spacing) = toml_theme.spacing {
            for (name, spacing_str) in spacing {
                if let Ok(spacing_value) = Spacing::from_str(&spacing_str) {
                    theme.add_spacing(name, spacing_value);
                }
            }
        }

        if let Some(border_radius) = toml_theme.border_radius {
            for (name, radius_str) in border_radius {
                if let Ok(radius_value) = BorderRadius::from_str(&radius_str) {
                    theme.add_border_radius(name, radius_value);
                }
            }
        }

        if let Some(box_shadows) = toml_theme.box_shadows {
            for (name, shadow_str) in box_shadows {
                if let Ok(shadow_value) = BoxShadow::from_str(&shadow_str) {
                    theme.add_box_shadow(name, shadow_value);
                }
            }
        }

        theme
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
