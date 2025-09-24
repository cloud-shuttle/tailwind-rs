# 🎨 Color System

## 📋 Overview

The color system is a comprehensive implementation of Tailwind CSS's color palette, providing utilities for text, background, border, and other color-related properties. This document outlines the complete color system implementation in Rust.

## 🌈 Color Palette

### Core Color Types

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorPalette {
    // Grays
    Gray,
    Slate,
    Zinc,
    Neutral,
    Stone,
    
    // Reds
    Red,
    Rose,
    Pink,
    
    // Oranges
    Orange,
    Amber,
    Yellow,
    
    // Greens
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    
    // Blues
    Sky,
    Blue,
    Indigo,
    Violet,
    
    // Purples
    Purple,
    Fuchsia,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorShade {
    Shade50,
    Shade100,
    Shade200,
    Shade300,
    Shade400,
    Shade500,
    Shade600,
    Shade700,
    Shade800,
    Shade900,
    Shade950,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    pub palette: ColorPalette,
    pub shade: ColorShade,
}
```

### Color Implementation

```rust
impl Color {
    /// Create a new color
    pub fn new(palette: ColorPalette, shade: ColorShade) -> Self {
        Self { palette, shade }
    }
    
    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        format!("{}-{}", self.palette.to_string(), self.shade.to_string())
    }
    
    /// Convert to CSS value
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
            
            // ... (all other color combinations)
            
            _ => "#000000".to_string(), // Default fallback
        }
    }
    
    /// Get all available colors
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

impl std::fmt::Display for ColorPalette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
```

## 🎨 Color Utilities

### Text Color Utilities

```rust
/// Trait for adding text color utilities to a class builder
pub trait TextColorUtilities {
    /// Set text color
    fn text_color(self, color: Color) -> Self;
    
    /// Set text color to transparent
    fn text_transparent(self) -> Self;
    
    /// Set text color to current
    fn text_current(self) -> Self;
    
    /// Set text color to black
    fn text_black(self) -> Self;
    
    /// Set text color to white
    fn text_white(self) -> Self;
}

impl TextColorUtilities for ClassBuilder {
    fn text_color(self, color: Color) -> Self {
        self.class(&format!("text-{}", color.to_class_name()))
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
```

### Background Color Utilities

```rust
/// Trait for adding background color utilities to a class builder
pub trait BackgroundColorUtilities {
    /// Set background color
    fn background_color(self, color: Color) -> Self;
    
    /// Set background color to transparent
    fn background_transparent(self) -> Self;
    
    /// Set background color to current
    fn background_current(self) -> Self;
    
    /// Set background color to black
    fn background_black(self) -> Self;
    
    /// Set background color to white
    fn background_white(self) -> Self;
}

impl BackgroundColorUtilities for ClassBuilder {
    fn background_color(self, color: Color) -> Self {
        self.class(&format!("bg-{}", color.to_class_name()))
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
```

### Border Color Utilities

```rust
/// Trait for adding border color utilities to a class builder
pub trait BorderColorUtilities {
    /// Set border color
    fn border_color(self, color: Color) -> Self;
    
    /// Set border color to transparent
    fn border_transparent(self) -> Self;
    
    /// Set border color to current
    fn border_current(self) -> Self;
    
    /// Set border color to black
    fn border_black(self) -> Self;
    
    /// Set border color to white
    fn border_white(self) -> Self;
}

impl BorderColorUtilities for ClassBuilder {
    fn border_color(self, color: Color) -> Self {
        self.class(&format!("border-{}", color.to_class_name()))
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
```

### Ring Color Utilities

```rust
/// Trait for adding ring color utilities to a class builder
pub trait RingColorUtilities {
    /// Set ring color
    fn ring_color(self, color: Color) -> Self;
    
    /// Set ring color to transparent
    fn ring_transparent(self) -> Self;
    
    /// Set ring color to current
    fn ring_current(self) -> Self;
    
    /// Set ring color to black
    fn ring_black(self) -> Self;
    
    /// Set ring color to white
    fn ring_white(self) -> Self;
}

impl RingColorUtilities for ClassBuilder {
    fn ring_color(self, color: Color) -> Self {
        self.class(&format!("ring-{}", color.to_class_name()))
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
```

### Accent Color Utilities

```rust
/// Trait for adding accent color utilities to a class builder
pub trait AccentColorUtilities {
    /// Set accent color
    fn accent_color(self, color: Color) -> Self;
    
    /// Set accent color to transparent
    fn accent_transparent(self) -> Self;
    
    /// Set accent color to current
    fn accent_current(self) -> Self;
    
    /// Set accent color to black
    fn accent_black(self) -> Self;
    
    /// Set accent color to white
    fn accent_white(self) -> Self;
}

impl AccentColorUtilities for ClassBuilder {
    fn accent_color(self, color: Color) -> Self {
        self.class(&format!("accent-{}", color.to_class_name()))
    }
    
    fn accent_transparent(self) -> Self {
        self.class("accent-transparent")
    }
    
    fn accent_current(self) -> Self {
        self.class("accent-current")
    }
    
    fn accent_black(self) -> Self {
        self.class("accent-black")
    }
    
    fn accent_white(self) -> Self {
        self.class("accent-white")
    }
}
```

### Caret Color Utilities

```rust
/// Trait for adding caret color utilities to a class builder
pub trait CaretColorUtilities {
    /// Set caret color
    fn caret_color(self, color: Color) -> Self;
    
    /// Set caret color to transparent
    fn caret_transparent(self) -> Self;
    
    /// Set caret color to current
    fn caret_current(self) -> Self;
    
    /// Set caret color to black
    fn caret_black(self) -> Self;
    
    /// Set caret color to white
    fn caret_white(self) -> Self;
}

impl CaretColorUtilities for ClassBuilder {
    fn caret_color(self, color: Color) -> Self {
        self.class(&format!("caret-{}", color.to_class_name()))
    }
    
    fn caret_transparent(self) -> Self {
        self.class("caret-transparent")
    }
    
    fn caret_current(self) -> Self {
        self.class("caret-current")
    }
    
    fn caret_black(self) -> Self {
        self.class("caret-black")
    }
    
    fn caret_white(self) -> Self {
        self.class("caret-white")
    }
}
```

## 🎯 Usage Examples

### Basic Color Usage

```rust
// Text colors
let classes = ClassBuilder::new()
    .text_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))    // text-blue-500
    .text_color(Color::new(ColorPalette::Red, ColorShade::Shade600))     // text-red-600
    .text_color(Color::new(ColorPalette::Green, ColorShade::Shade700))   // text-green-700
    .build();

// Background colors
let classes = ClassBuilder::new()
    .background_color(Color::new(ColorPalette::Gray, ColorShade::Shade100))  // bg-gray-100
    .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))  // bg-blue-500
    .background_color(Color::new(ColorPalette::Red, ColorShade::Shade600))   // bg-red-600
    .build();

// Border colors
let classes = ClassBuilder::new()
    .border_color(Color::new(ColorPalette::Gray, ColorShade::Shade300))      // border-gray-300
    .border_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))      // border-blue-500
    .border_color(Color::new(ColorPalette::Red, ColorShade::Shade600))       // border-red-600
    .build();
```

### Special Color Values

```rust
// Transparent colors
let classes = ClassBuilder::new()
    .text_transparent()      // text-transparent
    .background_transparent() // bg-transparent
    .border_transparent()    // border-transparent
    .build();

// Current colors
let classes = ClassBuilder::new()
    .text_current()          // text-current
    .background_current()    // bg-current
    .border_current()        // border-current
    .build();

// Black and white
let classes = ClassBuilder::new()
    .text_black()            // text-black
    .background_white()      // bg-white
    .border_black()          // border-black
    .build();
```

### Responsive Colors

```rust
// Responsive text colors
let classes = ClassBuilder::new()
    .text_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))        // text-blue-500
    .sm(|b| b.text_color(Color::new(ColorPalette::Blue, ColorShade::Shade600))) // sm:text-blue-600
    .md(|b| b.text_color(Color::new(ColorPalette::Blue, ColorShade::Shade700))) // md:text-blue-700
    .lg(|b| b.text_color(Color::new(ColorPalette::Blue, ColorShade::Shade800))) // lg:text-blue-800
    .build();

// Responsive background colors
let classes = ClassBuilder::new()
    .background_color(Color::new(ColorPalette::Gray, ColorShade::Shade100))      // bg-gray-100
    .sm(|b| b.background_color(Color::new(ColorPalette::Gray, ColorShade::Shade200))) // sm:bg-gray-200
    .md(|b| b.background_color(Color::new(ColorPalette::Gray, ColorShade::Shade300))) // md:bg-gray-300
    .lg(|b| b.background_color(Color::new(ColorPalette::Gray, ColorShade::Shade400))) // lg:bg-gray-400
    .build();
```

### State Variant Colors

```rust
// Hover colors
let classes = ClassBuilder::new()
    .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))     // bg-blue-500
    .hover(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade600))) // hover:bg-blue-600
    .build();

// Focus colors
let classes = ClassBuilder::new()
    .ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))           // ring-blue-500
    .focus(|b| b.ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade600))) // focus:ring-blue-600
    .build();

// Active colors
let classes = ClassBuilder::new()
    .text_color(Color::new(ColorPalette::White, ColorShade::Shade500))          // text-white
    .active(|b| b.text_color(Color::new(ColorPalette::Gray, ColorShade::Shade300))) // active:text-gray-300
    .build();
```

## 🧪 Testing

### Unit Tests

```rust
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
    fn test_accent_color_utilities() {
        let classes = ClassBuilder::new()
            .accent_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .accent_color(Color::new(ColorPalette::Green, ColorShade::Shade600))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("accent-blue-500"));
        assert!(css_classes.contains("accent-green-600"));
    }
    
    #[test]
    fn test_caret_color_utilities() {
        let classes = ClassBuilder::new()
            .caret_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .caret_color(Color::new(ColorPalette::Red, ColorShade::Shade600))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("caret-blue-500"));
        assert!(css_classes.contains("caret-red-600"));
    }
    
    #[test]
    fn test_special_color_values() {
        let classes = ClassBuilder::new()
            .text_transparent()      // text-transparent
            .background_transparent() // bg-transparent
            .border_transparent()    // border-transparent
            .text_current()          // text-current
            .background_current()    // bg-current
            .border_current()        // border-current
            .text_black()            // text-black
            .background_white()      // bg-white
            .border_black()          // border-black
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
```

### Property-based Tests

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_color_properties(
            palette in prop::sample::select(&ColorPalette::all_palettes()),
            shade in prop::sample::select(&ColorShade::all_shades())
        ) {
            let color = Color::new(palette, shade);
            
            // Test that CSS value is always valid
            let css_value = color.to_css_value();
            assert!(!css_value.is_empty());
            assert!(css_value.starts_with('#'));
            assert_eq!(css_value.len(), 7); // #RRGGBB format
            
            // Test that class name is always valid
            let class_name = color.to_class_name();
            assert!(!class_name.is_empty());
            assert!(!class_name.contains(' '));
            assert!(!class_name.contains('\n'));
            assert!(!class_name.contains('\r'));
            assert!(!class_name.contains('\t'));
        }
    }
    
    proptest! {
        #[test]
        fn test_color_utility_combinations(
            text_color in prop::sample::select(&Color::all_colors()),
            background_color in prop::sample::select(&Color::all_colors()),
            border_color in prop::sample::select(&Color::all_colors())
        ) {
            let classes = ClassBuilder::new()
                .text_color(text_color)
                .background_color(background_color)
                .border_color(border_color)
                .build();
            
            let css_classes = classes.to_css_classes();
            
            // Test that all classes are present
            assert!(css_classes.contains(&format!("text-{}", text_color.to_class_name())));
            assert!(css_classes.contains(&format!("bg-{}", background_color.to_class_name())));
            assert!(css_classes.contains(&format!("border-{}", border_color.to_class_name())));
            
            // Test that CSS classes are valid
            assert!(!css_classes.is_empty());
            assert!(!css_classes.contains("  ")); // No double spaces
        }
    }
}
```

## 📊 Complete Color Reference

### Available Palettes
- **Grays**: `gray`, `slate`, `zinc`, `neutral`, `stone`
- **Reds**: `red`, `rose`, `pink`
- **Oranges**: `orange`, `amber`, `yellow`
- **Greens**: `lime`, `green`, `emerald`, `teal`, `cyan`
- **Blues**: `sky`, `blue`, `indigo`, `violet`
- **Purples**: `purple`, `fuchsia`

### Available Shades
- `50`, `100`, `200`, `300`, `400`, `500`, `600`, `700`, `800`, `900`, `950`

### Color Utilities
- **Text**: `text-{color}-{shade}`
- **Background**: `bg-{color}-{shade}`
- **Border**: `border-{color}-{shade}`
- **Ring**: `ring-{color}-{shade}`
- **Accent**: `accent-{color}-{shade}`
- **Caret**: `caret-{color}-{shade}`

### Special Values
- `transparent`, `current`, `black`, `white`

---

**Next**: [09-layout-system.md](./09-layout-system.md) - Display, position, and layout utilities
