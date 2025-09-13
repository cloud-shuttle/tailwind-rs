//! Color utilities for tailwind-rs
//!
//! This module provides utilities for text, background, border, ring, accent, and caret colors.
//! It includes all Tailwind CSS color palettes and shades.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Color palette values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorPalette {
    // Grays
    Gray, Slate, Zinc, Neutral, Stone,
    // Reds
    Red, Rose, Pink,
    // Oranges
    Orange, Amber, Yellow,
    // Greens
    Lime, Green, Emerald, Teal, Cyan,
    // Blues
    Sky, Blue, Indigo, Violet,
    // Purples
    Purple, Fuchsia,
}

/// Color shade values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorShade {
    Shade50, Shade100, Shade200, Shade300, Shade400,
    Shade500, Shade600, Shade700, Shade800, Shade900, Shade950,
}

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
        match (self.palette, self.shade) {
            // Gray palette
            (ColorPalette::Gray, ColorShade::Shade50) => "#f9fafb".to_string(),
            (ColorPalette::Gray, ColorShade::Shade100) => "#f3f4f6".to_string(),
            (ColorPalette::Gray, ColorShade::Shade200) => "#e5e7eb".to_string(),
            (ColorPalette::Gray, ColorShade::Shade300) => "#d1d5db".to_string(),
            (ColorPalette::Gray, ColorShade::Shade400) => "#9ca3af".to_string(),
            (ColorPalette::Gray, ColorShade::Shade500) => "#6b7280".to_string(),
            (ColorPalette::Gray, ColorShade::Shade600) => "#4b5563".to_string(),
            (ColorPalette::Gray, ColorShade::Shade700) => "#374151".to_string(),
            (ColorPalette::Gray, ColorShade::Shade800) => "#1f2937".to_string(),
            (ColorPalette::Gray, ColorShade::Shade900) => "#111827".to_string(),
            (ColorPalette::Gray, ColorShade::Shade950) => "#030712".to_string(),
            
            // Blue palette
            (ColorPalette::Blue, ColorShade::Shade50) => "#eff6ff".to_string(),
            (ColorPalette::Blue, ColorShade::Shade100) => "#dbeafe".to_string(),
            (ColorPalette::Blue, ColorShade::Shade200) => "#bfdbfe".to_string(),
            (ColorPalette::Blue, ColorShade::Shade300) => "#93c5fd".to_string(),
            (ColorPalette::Blue, ColorShade::Shade400) => "#60a5fa".to_string(),
            (ColorPalette::Blue, ColorShade::Shade500) => "#3b82f6".to_string(),
            (ColorPalette::Blue, ColorShade::Shade600) => "#2563eb".to_string(),
            (ColorPalette::Blue, ColorShade::Shade700) => "#1d4ed8".to_string(),
            (ColorPalette::Blue, ColorShade::Shade800) => "#1e40af".to_string(),
            (ColorPalette::Blue, ColorShade::Shade900) => "#1e3a8a".to_string(),
            (ColorPalette::Blue, ColorShade::Shade950) => "#172554".to_string(),
            
            // Red palette
            (ColorPalette::Red, ColorShade::Shade50) => "#fef2f2".to_string(),
            (ColorPalette::Red, ColorShade::Shade100) => "#fee2e2".to_string(),
            (ColorPalette::Red, ColorShade::Shade200) => "#fecaca".to_string(),
            (ColorPalette::Red, ColorShade::Shade300) => "#fca5a5".to_string(),
            (ColorPalette::Red, ColorShade::Shade400) => "#f87171".to_string(),
            (ColorPalette::Red, ColorShade::Shade500) => "#ef4444".to_string(),
            (ColorPalette::Red, ColorShade::Shade600) => "#dc2626".to_string(),
            (ColorPalette::Red, ColorShade::Shade700) => "#b91c1c".to_string(),
            (ColorPalette::Red, ColorShade::Shade800) => "#991b1b".to_string(),
            (ColorPalette::Red, ColorShade::Shade900) => "#7f1d1d".to_string(),
            (ColorPalette::Red, ColorShade::Shade950) => "#450a0a".to_string(),
            
            // Green palette
            (ColorPalette::Green, ColorShade::Shade50) => "#f0fdf4".to_string(),
            (ColorPalette::Green, ColorShade::Shade100) => "#dcfce7".to_string(),
            (ColorPalette::Green, ColorShade::Shade200) => "#bbf7d0".to_string(),
            (ColorPalette::Green, ColorShade::Shade300) => "#86efac".to_string(),
            (ColorPalette::Green, ColorShade::Shade400) => "#4ade80".to_string(),
            (ColorPalette::Green, ColorShade::Shade500) => "#22c55e".to_string(),
            (ColorPalette::Green, ColorShade::Shade600) => "#16a34a".to_string(),
            (ColorPalette::Green, ColorShade::Shade700) => "#15803d".to_string(),
            (ColorPalette::Green, ColorShade::Shade800) => "#166534".to_string(),
            (ColorPalette::Green, ColorShade::Shade900) => "#14532d".to_string(),
            (ColorPalette::Green, ColorShade::Shade950) => "#052e16".to_string(),
            
            _ => "#000000".to_string(), // Default fallback
        }
    }
    
    pub fn all_colors() -> Vec<Color> {
        let mut colors = Vec::new();
        for palette in ColorPalette::all_palettes() {
            for shade in ColorShade::all_shades() {
                colors.push(Color::new(palette, shade));
            }
        }
        colors
    }
}

impl ColorPalette {
    pub fn all_palettes() -> Vec<ColorPalette> {
        vec![
            ColorPalette::Gray, ColorPalette::Slate, ColorPalette::Zinc, ColorPalette::Neutral, ColorPalette::Stone,
            ColorPalette::Red, ColorPalette::Rose, ColorPalette::Pink,
            ColorPalette::Orange, ColorPalette::Amber, ColorPalette::Yellow,
            ColorPalette::Lime, ColorPalette::Green, ColorPalette::Emerald, ColorPalette::Teal, ColorPalette::Cyan,
            ColorPalette::Sky, ColorPalette::Blue, ColorPalette::Indigo, ColorPalette::Violet,
            ColorPalette::Purple, ColorPalette::Fuchsia,
        ]
    }
}

impl ColorShade {
    pub fn all_shades() -> Vec<ColorShade> {
        vec![
            ColorShade::Shade50, ColorShade::Shade100, ColorShade::Shade200, ColorShade::Shade300, ColorShade::Shade400,
            ColorShade::Shade500, ColorShade::Shade600, ColorShade::Shade700, ColorShade::Shade800, ColorShade::Shade900, ColorShade::Shade950,
        ]
    }
}

impl std::fmt::Display for ColorPalette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ColorPalette::Gray => "gray",
            ColorPalette::Slate => "slate",
            ColorPalette::Zinc => "zinc",
            ColorPalette::Neutral => "neutral",
            ColorPalette::Stone => "stone",
            ColorPalette::Red => "red",
            ColorPalette::Rose => "rose",
            ColorPalette::Pink => "pink",
            ColorPalette::Orange => "orange",
            ColorPalette::Amber => "amber",
            ColorPalette::Yellow => "yellow",
            ColorPalette::Lime => "lime",
            ColorPalette::Green => "green",
            ColorPalette::Emerald => "emerald",
            ColorPalette::Teal => "teal",
            ColorPalette::Cyan => "cyan",
            ColorPalette::Sky => "sky",
            ColorPalette::Blue => "blue",
            ColorPalette::Indigo => "indigo",
            ColorPalette::Violet => "violet",
            ColorPalette::Purple => "purple",
            ColorPalette::Fuchsia => "fuchsia",
        };
        write!(f, "{}", name)
    }
}

impl std::fmt::Display for ColorShade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shade = match self {
            ColorShade::Shade50 => "50",
            ColorShade::Shade100 => "100",
            ColorShade::Shade200 => "200",
            ColorShade::Shade300 => "300",
            ColorShade::Shade400 => "400",
            ColorShade::Shade500 => "500",
            ColorShade::Shade600 => "600",
            ColorShade::Shade700 => "700",
            ColorShade::Shade800 => "800",
            ColorShade::Shade900 => "900",
            ColorShade::Shade950 => "950",
        };
        write!(f, "{}", shade)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding text color utilities to a class builder
pub trait TextColorUtilities {
    fn text_color(self, color: Color) -> Self;
    fn text_transparent(self) -> Self;
    fn text_current(self) -> Self;
    fn text_black(self) -> Self;
    fn text_white(self) -> Self;
}

impl TextColorUtilities for ClassBuilder {
    fn text_color(self, color: Color) -> Self {
        self.class(format!("text-{}", color.to_class_name()))
    }
    
    fn text_transparent(self) -> Self {
        self.class("text-transparent")
    }
    
    fn text_current(self) -> Self {
        self.class("text-current")
    }
    
    fn text_black(self) -> Self {
        self.class("text-black")
    }
    
    fn text_white(self) -> Self {
        self.class("text-white")
    }
}

/// Trait for adding background color utilities to a class builder
pub trait BackgroundColorUtilities {
    fn background_color(self, color: Color) -> Self;
    fn background_transparent(self) -> Self;
    fn background_current(self) -> Self;
    fn background_black(self) -> Self;
    fn background_white(self) -> Self;
}

impl BackgroundColorUtilities for ClassBuilder {
    fn background_color(self, color: Color) -> Self {
        self.class(format!("bg-{}", color.to_class_name()))
    }
    
    fn background_transparent(self) -> Self {
        self.class("bg-transparent")
    }
    
    fn background_current(self) -> Self {
        self.class("bg-current")
    }
    
    fn background_black(self) -> Self {
        self.class("bg-black")
    }
    
    fn background_white(self) -> Self {
        self.class("bg-white")
    }
}

/// Trait for adding border color utilities to a class builder
pub trait BorderColorUtilities {
    fn border_color(self, color: Color) -> Self;
    fn border_transparent(self) -> Self;
    fn border_current(self) -> Self;
    fn border_black(self) -> Self;
    fn border_white(self) -> Self;
}

impl BorderColorUtilities for ClassBuilder {
    fn border_color(self, color: Color) -> Self {
        self.class(format!("border-{}", color.to_class_name()))
    }
    
    fn border_transparent(self) -> Self {
        self.class("border-transparent")
    }
    
    fn border_current(self) -> Self {
        self.class("border-current")
    }
    
    fn border_black(self) -> Self {
        self.class("border-black")
    }
    
    fn border_white(self) -> Self {
        self.class("border-white")
    }
}

/// Trait for adding ring color utilities to a class builder
pub trait RingColorUtilities {
    fn ring_color(self, color: Color) -> Self;
    fn ring_transparent(self) -> Self;
    fn ring_current(self) -> Self;
    fn ring_black(self) -> Self;
    fn ring_white(self) -> Self;
}

impl RingColorUtilities for ClassBuilder {
    fn ring_color(self, color: Color) -> Self {
        self.class(format!("ring-{}", color.to_class_name()))
    }
    
    fn ring_transparent(self) -> Self {
        self.class("ring-transparent")
    }
    
    fn ring_current(self) -> Self {
        self.class("ring-current")
    }
    
    fn ring_black(self) -> Self {
        self.class("ring-black")
    }
    
    fn ring_white(self) -> Self {
        self.class("ring-white")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_color_utilities() {
        let classes = ClassBuilder::new()
            .text_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .text_color(Color::new(ColorPalette::Red, ColorShade::Shade600))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("text-blue-500"));
        assert!(css_classes.contains("text-red-600"));
    }
    
    #[test]
    fn test_background_color_utilities() {
        let classes = ClassBuilder::new()
            .background_color(Color::new(ColorPalette::Gray, ColorShade::Shade100))
            .background_color(Color::new(ColorPalette::Green, ColorShade::Shade500))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-gray-100"));
        assert!(css_classes.contains("bg-green-500"));
    }
    
    #[test]
    fn test_border_color_utilities() {
        let classes = ClassBuilder::new()
            .border_color(Color::new(ColorPalette::Gray, ColorShade::Shade300))
            .border_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("border-gray-300"));
        assert!(css_classes.contains("border-blue-500"));
    }
    
    #[test]
    fn test_ring_color_utilities() {
        let classes = ClassBuilder::new()
            .ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .ring_color(Color::new(ColorPalette::Red, ColorShade::Shade600))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("ring-blue-500"));
        assert!(css_classes.contains("ring-red-600"));
    }
    
    #[test]
    fn test_special_color_values() {
        let classes = ClassBuilder::new()
            .text_transparent()
            .background_transparent()
            .border_transparent()
            .text_current()
            .background_current()
            .border_current()
            .text_black()
            .background_white()
            .border_black()
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("text-transparent"));
        assert!(css_classes.contains("bg-transparent"));
        assert!(css_classes.contains("border-transparent"));
        assert!(css_classes.contains("text-current"));
        assert!(css_classes.contains("bg-current"));
        assert!(css_classes.contains("border-current"));
        assert!(css_classes.contains("text-black"));
        assert!(css_classes.contains("bg-white"));
        assert!(css_classes.contains("border-black"));
    }
}
