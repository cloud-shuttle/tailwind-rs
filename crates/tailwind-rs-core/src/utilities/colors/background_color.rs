//! Background color utilities for tailwind-rs

use crate::classes::ClassBuilder;
use crate::utilities::colors::{Color, ColorPalette, ColorShade};

/// Trait for adding background color utilities to a class builder
pub trait BackgroundColorUtilities {
    fn background_color(self, color: Color) -> Self;
    fn background_color_palette_shade(self, palette: ColorPalette, shade: ColorShade) -> Self;
}

impl BackgroundColorUtilities for ClassBuilder {
    fn background_color(self, color: Color) -> Self {
        self.class(format!("bg-{}", color.to_class_name()))
    }

    fn background_color_palette_shade(self, palette: ColorPalette, shade: ColorShade) -> Self {
        let color = Color::new(palette, shade);
        self.background_color(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_color_utilities() {
        let color = Color::new(ColorPalette::Green, ColorShade::Shade500);
        let classes = ClassBuilder::new().background_color(color).build();

        assert!(classes.to_css_classes().contains("bg-green-500"));
    }

    #[test]
    fn test_background_color_palette_shade() {
        let classes = ClassBuilder::new()
            .background_color_palette_shade(ColorPalette::Yellow, ColorShade::Shade400)
            .build();

        assert!(classes.to_css_classes().contains("bg-yellow-400"));
    }
}
