//! Color struct utilities for tailwind-rs

use crate::utilities::colors::{ColorPalette, ColorShade};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Color value combining palette and shade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    pub palette: ColorPalette,
    pub shade: ColorShade,
}

impl Color {
    pub fn new(palette: ColorPalette, shade: ColorShade) -> Self {
        Self { palette, shade }
    }

    pub fn to_class_name(&self) -> String {
        format!("{}-{}", self.palette, self.shade)
    }

    pub fn to_css_value(&self) -> String {
        // This would contain the actual CSS color values
        // For now, returning a placeholder
        format!("var(--color-{}-{})", self.palette, self.shade)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::new(ColorPalette::Blue, ColorShade::Shade500);
        assert_eq!(color.palette, ColorPalette::Blue);
        assert_eq!(color.shade, ColorShade::Shade500);
    }

    #[test]
    fn test_color_class_name() {
        let color = Color::new(ColorPalette::Red, ColorShade::Shade600);
        assert_eq!(color.to_class_name(), "red-600");
    }

    #[test]
    fn test_color_display() {
        let color = Color::new(ColorPalette::Green, ColorShade::Shade500);
        assert_eq!(color.to_string(), "green-500");
    }

    #[test]
    fn test_color_serialization() {
        let color = Color::new(ColorPalette::Blue, ColorShade::Shade500);
        let serialized = serde_json::to_string(&color).unwrap();
        let deserialized: Color = serde_json::from_str(&serialized).unwrap();
        assert_eq!(color, deserialized);
    }
}
