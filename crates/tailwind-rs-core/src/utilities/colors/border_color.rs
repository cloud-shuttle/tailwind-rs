//! Border color utilities for tailwind-rs

use crate::classes::ClassBuilder;
use crate::utilities::colors::{Color, ColorPalette, ColorShade};

/// Trait for adding border color utilities to a class builder
pub trait BorderColorUtilities {
    fn border_color(self, color: Color) -> Self;
    fn border_color_palette_shade(self, palette: ColorPalette, shade: ColorShade) -> Self;
}

impl BorderColorUtilities for ClassBuilder {
    fn border_color(self, color: Color) -> Self {
        self.class(format!("border-{}", color.to_class_name()))
    }
    
    fn border_color_palette_shade(self, palette: ColorPalette, shade: ColorShade) -> Self {
        let color = Color::new(palette, shade);
        self.border_color(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_color_utilities() {
        let color = Color::new(ColorPalette::Purple, ColorShade::Shade500);
        let classes = ClassBuilder::new()
            .border_color(color)
            .build();
        
        assert!(classes.to_css_classes().contains("border-purple-500"));
    }

    #[test]
    fn test_border_color_palette_shade() {
        let classes = ClassBuilder::new()
            .border_color_palette_shade(ColorPalette::Pink, ColorShade::Shade300)
            .build();
        
        assert!(classes.to_css_classes().contains("border-pink-300"));
    }
}
