//! Text color utilities for tailwind-rs

use crate::classes::ClassBuilder;
use crate::utilities::colors::{Color, ColorPalette, ColorShade};

/// Trait for adding text color utilities to a class builder
pub trait TextColorUtilities {
    fn text_color(self, color: Color) -> Self;
    fn text_color_palette_shade(self, palette: ColorPalette, shade: ColorShade) -> Self;
}

impl TextColorUtilities for ClassBuilder {
    fn text_color(self, color: Color) -> Self {
        self.class(format!("text-{}", color.to_class_name()))
    }
    
    fn text_color_palette_shade(self, palette: ColorPalette, shade: ColorShade) -> Self {
        let color = Color::new(palette, shade);
        self.text_color(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_color_utilities() {
        let color = Color::new(ColorPalette::Blue, ColorShade::Shade500);
        let classes = ClassBuilder::new()
            .text_color(color)
            .build();
        
        assert!(classes.to_css_classes().contains("text-blue-500"));
    }

    #[test]
    fn test_text_color_palette_shade() {
        let classes = ClassBuilder::new()
            .text_color_palette_shade(ColorPalette::Red, ColorShade::Shade600)
            .build();
        
        assert!(classes.to_css_classes().contains("text-red-600"));
    }
}
