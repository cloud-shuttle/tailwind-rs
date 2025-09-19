//! Ring color utilities for tailwind-rs

use crate::classes::ClassBuilder;
use crate::utilities::colors::{Color, ColorPalette, ColorShade};

/// Trait for adding ring color utilities to a class builder
pub trait RingColorUtilities {
    fn ring_color(self, color: Color) -> Self;
    fn ring_color_palette_shade(self, palette: ColorPalette, shade: ColorShade) -> Self;
}

impl RingColorUtilities for ClassBuilder {
    fn ring_color(self, color: Color) -> Self {
        self.class(format!("ring-{}", color.to_class_name()))
    }
    
    fn ring_color_palette_shade(self, palette: ColorPalette, shade: ColorShade) -> Self {
        let color = Color::new(palette, shade);
        self.ring_color(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_color_utilities() {
        let color = Color::new(ColorPalette::Indigo, ColorShade::Shade500);
        let classes = ClassBuilder::new()
            .ring_color(color)
            .build();
        
        assert!(classes.to_css_classes().contains("ring-indigo-500"));
    }

    #[test]
    fn test_ring_color_palette_shade() {
        let classes = ClassBuilder::new()
            .ring_color_palette_shade(ColorPalette::Cyan, ColorShade::Shade400)
            .build();
        
        assert!(classes.to_css_classes().contains("ring-cyan-400"));
    }
}
