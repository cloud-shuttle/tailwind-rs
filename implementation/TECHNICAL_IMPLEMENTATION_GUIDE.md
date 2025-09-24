# 🛠️ Technical Implementation Guide

## 📋 Overview

This document provides detailed technical guidance for implementing the complete Tailwind CSS v4.1 utility classes in Rust. It covers the architecture, code structure, and implementation patterns.

## 🏗️ Architecture Overview

### Core Components

```
tailwind-rs-core/
├── src/
│   ├── lib.rs                 # Main library entry point
│   ├── classes.rs             # Class management system
│   ├── utilities/             # Utility class implementations
│   │   ├── spacing.rs         # Padding, margin utilities
│   │   ├── sizing.rs          # Width, height utilities
│   │   ├── typography.rs      # Font, text utilities
│   │   ├── colors.rs          # Color system
│   │   ├── layout.rs          # Display, position utilities
│   │   ├── flexbox.rs         # Flexbox utilities
│   │   ├── grid.rs            # Grid utilities
│   │   ├── backgrounds.rs     # Background utilities
│   │   ├── borders.rs         # Border utilities
│   │   ├── effects.rs         # Shadow, opacity utilities
│   │   ├── filters.rs         # Filter utilities
│   │   ├── transitions.rs     # Transition utilities
│   │   ├── transforms.rs      # Transform utilities
│   │   └── interactivity.rs   # Interactive utilities
│   ├── macros/                # Procedural macros
│   │   ├── lib.rs             # Macro entry point
│   │   ├── utility_macro.rs   # Utility generation macro
│   │   └── builder_macro.rs   # Builder pattern macro
│   ├── validation.rs          # Class validation system
│   ├── theme.rs               # Theme system
│   └── responsive.rs          # Responsive utilities
```

## 🔧 Implementation Patterns

### 1. Utility Class Structure

Each utility category follows a consistent pattern:

```rust
// Example: spacing.rs
use crate::classes::{ClassBuilder, ClassSet};
use crate::responsive::Breakpoint;
use crate::validation::ValidationError;

/// Spacing utility values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpacingValue {
    Zero,
    Px,
    Fractional(f32), // 0.5, 1.5, 2.5, 3.5
    Integer(u32),    // 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 56, 64, 72, 80, 96
    Auto,
    Full,
    Screen,
    Min,
    Max,
    Fit,
}

impl SpacingValue {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            SpacingValue::Zero => "0".to_string(),
            SpacingValue::Px => "1px".to_string(),
            SpacingValue::Fractional(f) => format!("{}rem", f * 0.25), // 0.25rem per unit
            SpacingValue::Integer(i) => format!("{}rem", i * 0.25),
            SpacingValue::Auto => "auto".to_string(),
            SpacingValue::Full => "100%".to_string(),
            SpacingValue::Screen => "100vh".to_string(),
            SpacingValue::Min => "min-content".to_string(),
            SpacingValue::Max => "max-content".to_string(),
            SpacingValue::Fit => "fit-content".to_string(),
        }
    }
    
    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        match self {
            SpacingValue::Zero => "0".to_string(),
            SpacingValue::Px => "px".to_string(),
            SpacingValue::Fractional(f) => format!("{}", f),
            SpacingValue::Integer(i) => i.to_string(),
            SpacingValue::Auto => "auto".to_string(),
            SpacingValue::Full => "full".to_string(),
            SpacingValue::Screen => "screen".to_string(),
            SpacingValue::Min => "min".to_string(),
            SpacingValue::Max => "max".to_string(),
            SpacingValue::Fit => "fit".to_string(),
        }
    }
}

/// Padding utilities
pub trait PaddingUtilities {
    /// Add padding to all sides
    fn padding(self, value: SpacingValue) -> Self;
    
    /// Add horizontal padding
    fn padding_x(self, value: SpacingValue) -> Self;
    
    /// Add vertical padding
    fn padding_y(self, value: SpacingValue) -> Self;
    
    /// Add top padding
    fn padding_top(self, value: SpacingValue) -> Self;
    
    /// Add right padding
    fn padding_right(self, value: SpacingValue) -> Self;
    
    /// Add bottom padding
    fn padding_bottom(self, value: SpacingValue) -> Self;
    
    /// Add left padding
    fn padding_left(self, value: SpacingValue) -> Self;
}

impl PaddingUtilities for ClassBuilder {
    fn padding(self, value: SpacingValue) -> Self {
        self.class(&format!("p-{}", value.to_class_name()))
    }
    
    fn padding_x(self, value: SpacingValue) -> Self {
        self.class(&format!("px-{}", value.to_class_name()))
    }
    
    fn padding_y(self, value: SpacingValue) -> Self {
        self.class(&format!("py-{}", value.to_class_name()))
    }
    
    fn padding_top(self, value: SpacingValue) -> Self {
        self.class(&format!("pt-{}", value.to_class_name()))
    }
    
    fn padding_right(self, value: SpacingValue) -> Self {
        self.class(&format!("pr-{}", value.to_class_name()))
    }
    
    fn padding_bottom(self, value: SpacingValue) -> Self {
        self.class(&format!("pb-{}", value.to_class_name()))
    }
    
    fn padding_left(self, value: SpacingValue) -> Self {
        self.class(&format!("pl-{}", value.to_class_name()))
    }
}

/// Margin utilities
pub trait MarginUtilities {
    /// Add margin to all sides
    fn margin(self, value: SpacingValue) -> Self;
    
    /// Add horizontal margin
    fn margin_x(self, value: SpacingValue) -> Self;
    
    /// Add vertical margin
    fn margin_y(self, value: SpacingValue) -> Self;
    
    /// Add top margin
    fn margin_top(self, value: SpacingValue) -> Self;
    
    /// Add right margin
    fn margin_right(self, value: SpacingValue) -> Self;
    
    /// Add bottom margin
    fn margin_bottom(self, value: SpacingValue) -> Self;
    
    /// Add left margin
    fn margin_left(self, value: SpacingValue) -> Self;
    
    /// Add negative margin to all sides
    fn margin_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative horizontal margin
    fn margin_x_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative vertical margin
    fn margin_y_negative(self, value: SpacingValue) -> Self;
}

impl MarginUtilities for ClassBuilder {
    fn margin(self, value: SpacingValue) -> Self {
        self.class(&format!("m-{}", value.to_class_name()))
    }
    
    fn margin_x(self, value: SpacingValue) -> Self {
        self.class(&format!("mx-{}", value.to_class_name()))
    }
    
    fn margin_y(self, value: SpacingValue) -> Self {
        self.class(&format!("my-{}", value.to_class_name()))
    }
    
    fn margin_top(self, value: SpacingValue) -> Self {
        self.class(&format!("mt-{}", value.to_class_name()))
    }
    
    fn margin_right(self, value: SpacingValue) -> Self {
        self.class(&format!("mr-{}", value.to_class_name()))
    }
    
    fn margin_bottom(self, value: SpacingValue) -> Self {
        self.class(&format!("mb-{}", value.to_class_name()))
    }
    
    fn margin_left(self, value: SpacingValue) -> Self {
        self.class(&format!("ml-{}", value.to_class_name()))
    }
    
    fn margin_negative(self, value: SpacingValue) -> Self {
        self.class(&format!("-m-{}", value.to_class_name()))
    }
    
    fn margin_x_negative(self, value: SpacingValue) -> Self {
        self.class(&format!("-mx-{}", value.to_class_name()))
    }
    
    fn margin_y_negative(self, value: SpacingValue) -> Self {
        self.class(&format!("-my-{}", value.to_class_name()))
    }
}
```

### 2. Color System Implementation

```rust
// colors.rs
use serde::{Deserialize, Serialize};

/// Color palette definitions
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

/// Color shade values
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
}

/// Color definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    pub palette: ColorPalette,
    pub shade: ColorShade,
}

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
        // This would contain the actual CSS color values
        // For now, we'll use a placeholder
        match (self.palette, self.shade) {
            (ColorPalette::Gray, ColorShade::Shade500) => "#6b7280".to_string(),
            (ColorPalette::Blue, ColorShade::Shade500) => "#3b82f6".to_string(),
            // ... all color combinations
            _ => "#000000".to_string(),
        }
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
        };
        write!(f, "{}", shade)
    }
}

/// Color utilities
pub trait ColorUtilities {
    /// Set text color
    fn text_color(self, color: Color) -> Self;
    
    /// Set background color
    fn background_color(self, color: Color) -> Self;
    
    /// Set border color
    fn border_color(self, color: Color) -> Self;
    
    /// Set ring color
    fn ring_color(self, color: Color) -> Self;
    
    /// Set accent color
    fn accent_color(self, color: Color) -> Self;
    
    /// Set caret color
    fn caret_color(self, color: Color) -> Self;
}

impl ColorUtilities for ClassBuilder {
    fn text_color(self, color: Color) -> Self {
        self.class(&format!("text-{}", color.to_class_name()))
    }
    
    fn background_color(self, color: Color) -> Self {
        self.class(&format!("bg-{}", color.to_class_name()))
    }
    
    fn border_color(self, color: Color) -> Self {
        self.class(&format!("border-{}", color.to_class_name()))
    }
    
    fn ring_color(self, color: Color) -> Self {
        self.class(&format!("ring-{}", color.to_class_name()))
    }
    
    fn accent_color(self, color: Color) -> Self {
        self.class(&format!("accent-{}", color.to_class_name()))
    }
    
    fn caret_color(self, color: Color) -> Self {
        self.class(&format!("caret-{}", color.to_class_name()))
    }
}
```

### 3. Responsive Utilities

```rust
// responsive.rs
use crate::classes::ClassBuilder;
use crate::responsive::Breakpoint;

/// Responsive utility builder
pub struct ResponsiveBuilder<'a> {
    builder: &'a mut ClassBuilder,
    breakpoint: Breakpoint,
}

impl<'a> ResponsiveBuilder<'a> {
    /// Create a new responsive builder
    pub fn new(builder: &'a mut ClassBuilder, breakpoint: Breakpoint) -> Self {
        Self { builder, breakpoint }
    }
    
    /// Add a class for this breakpoint
    pub fn class(mut self, class: &str) -> Self {
        let responsive_class = format!("{}:{}", self.breakpoint.prefix(), class);
        self.builder.class(&responsive_class);
        self
    }
    
    /// Add padding for this breakpoint
    pub fn padding(self, value: SpacingValue) -> Self {
        self.class(&format!("p-{}", value.to_class_name()))
    }
    
    /// Add margin for this breakpoint
    pub fn margin(self, value: SpacingValue) -> Self {
        self.class(&format!("m-{}", value.to_class_name()))
    }
    
    /// Add width for this breakpoint
    pub fn width(self, value: SizingValue) -> Self {
        self.class(&format!("w-{}", value.to_class_name()))
    }
    
    /// Add height for this breakpoint
    pub fn height(self, value: SizingValue) -> Self {
        self.class(&format!("h-{}", value.to_class_name()))
    }
}

/// Responsive utilities
pub trait ResponsiveUtilities {
    /// Add classes for small screens (640px+)
    fn sm<F>(self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder;
    
    /// Add classes for medium screens (768px+)
    fn md<F>(self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder;
    
    /// Add classes for large screens (1024px+)
    fn lg<F>(self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder;
    
    /// Add classes for extra large screens (1280px+)
    fn xl<F>(self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder;
    
    /// Add classes for 2x large screens (1536px+)
    fn xl2<F>(self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder;
}

impl ResponsiveUtilities for ClassBuilder {
    fn sm<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder,
    {
        let mut responsive_builder = ResponsiveBuilder::new(&mut self, Breakpoint::Small);
        f(responsive_builder);
        self
    }
    
    fn md<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder,
    {
        let mut responsive_builder = ResponsiveBuilder::new(&mut self, Breakpoint::Medium);
        f(responsive_builder);
        self
    }
    
    fn lg<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder,
    {
        let mut responsive_builder = ResponsiveBuilder::new(&mut self, Breakpoint::Large);
        f(responsive_builder);
        self
    }
    
    fn xl<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder,
    {
        let mut responsive_builder = ResponsiveBuilder::new(&mut self, Breakpoint::ExtraLarge);
        f(responsive_builder);
        self
    }
    
    fn xl2<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder,
    {
        let mut responsive_builder = ResponsiveBuilder::new(&mut self, Breakpoint::ExtraLarge2);
        f(responsive_builder);
        self
    }
}
```

### 4. State Variants

```rust
// state_variants.rs
use crate::classes::ClassBuilder;

/// State variant builder
pub struct StateBuilder<'a> {
    builder: &'a mut ClassBuilder,
    state: &'static str,
}

impl<'a> StateBuilder<'a> {
    /// Create a new state builder
    pub fn new(builder: &'a mut ClassBuilder, state: &'static str) -> Self {
        Self { builder, state }
    }
    
    /// Add a class for this state
    pub fn class(mut self, class: &str) -> Self {
        let state_class = format!("{}:{}", self.state, class);
        self.builder.class(&state_class);
        self
    }
    
    /// Add padding for this state
    pub fn padding(self, value: SpacingValue) -> Self {
        self.class(&format!("p-{}", value.to_class_name()))
    }
    
    /// Add margin for this state
    pub fn margin(self, value: SpacingValue) -> Self {
        self.class(&format!("m-{}", value.to_class_name()))
    }
    
    /// Add background color for this state
    pub fn background_color(self, color: Color) -> Self {
        self.class(&format!("bg-{}", color.to_class_name()))
    }
    
    /// Add text color for this state
    pub fn text_color(self, color: Color) -> Self {
        self.class(&format!("text-{}", color.to_class_name()))
    }
}

/// State variant utilities
pub trait StateUtilities {
    /// Add classes for hover state
    fn hover<F>(self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder;
    
    /// Add classes for focus state
    fn focus<F>(self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder;
    
    /// Add classes for active state
    fn active<F>(self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder;
    
    /// Add classes for disabled state
    fn disabled<F>(self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder;
    
    /// Add classes for visited state
    fn visited<F>(self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder;
}

impl StateUtilities for ClassBuilder {
    fn hover<F>(mut self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder,
    {
        let mut state_builder = StateBuilder::new(&mut self, "hover");
        f(state_builder);
        self
    }
    
    fn focus<F>(mut self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder,
    {
        let mut state_builder = StateBuilder::new(&mut self, "focus");
        f(state_builder);
        self
    }
    
    fn active<F>(mut self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder,
    {
        let mut state_builder = StateBuilder::new(&mut self, "active");
        f(state_builder);
        self
    }
    
    fn disabled<F>(mut self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder,
    {
        let mut state_builder = StateBuilder::new(&mut self, "disabled");
        f(state_builder);
        self
    }
    
    fn visited<F>(mut self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder,
    {
        let mut state_builder = StateBuilder::new(&mut self, "visited");
        f(state_builder);
        self
    }
}
```

## 🧪 Testing Strategy

### 1. Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;
    
    #[test]
    fn test_padding_utilities() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Integer(4))
            .padding_x(SpacingValue::Integer(6))
            .padding_y(SpacingValue::Integer(2))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("px-6"));
        assert!(css_classes.contains("py-2"));
    }
    
    #[test]
    fn test_margin_utilities() {
        let classes = ClassBuilder::new()
            .margin(SpacingValue::Integer(8))
            .margin_x(SpacingValue::Integer(4))
            .margin_y(SpacingValue::Integer(2))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("m-8"));
        assert!(css_classes.contains("mx-4"));
        assert!(css_classes.contains("my-2"));
    }
    
    #[test]
    fn test_negative_margin_utilities() {
        let classes = ClassBuilder::new()
            .margin_negative(SpacingValue::Integer(4))
            .margin_x_negative(SpacingValue::Integer(2))
            .margin_y_negative(SpacingValue::Integer(1))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("-m-4"));
        assert!(css_classes.contains("-mx-2"));
        assert!(css_classes.contains("-my-1"));
    }
    
    #[test]
    fn test_color_utilities() {
        let classes = ClassBuilder::new()
            .text_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .background_color(Color::new(ColorPalette::Gray, ColorShade::Shade100))
            .border_color(Color::new(ColorPalette::Red, ColorShade::Shade300))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("text-blue-500"));
        assert!(css_classes.contains("bg-gray-100"));
        assert!(css_classes.contains("border-red-300"));
    }
    
    #[test]
    fn test_responsive_utilities() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Integer(4))
            .sm(|b| b.padding(SpacingValue::Integer(6)))
            .md(|b| b.padding(SpacingValue::Integer(8)))
            .lg(|b| b.padding(SpacingValue::Integer(10)))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("sm:p-6"));
        assert!(css_classes.contains("md:p-8"));
        assert!(css_classes.contains("lg:p-10"));
    }
    
    #[test]
    fn test_state_variants() {
        let classes = ClassBuilder::new()
            .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .hover(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade600)))
            .focus(|b| b.ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade500)))
            .active(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade700)))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-blue-500"));
        assert!(css_classes.contains("hover:bg-blue-600"));
        assert!(css_classes.contains("focus:ring-blue-500"));
        assert!(css_classes.contains("active:bg-blue-700"));
    }
}
```

### 2. Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::classes::ClassBuilder;
    
    #[test]
    fn test_complex_class_combination() {
        let classes = ClassBuilder::new()
            // Base styles
            .padding(SpacingValue::Integer(4))
            .margin(SpacingValue::Integer(2))
            .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .text_color(Color::new(ColorPalette::White, ColorShade::Shade500))
            .rounded_lg()
            .shadow_md()
            
            // Responsive styles
            .sm(|b| b.padding(SpacingValue::Integer(6)))
            .md(|b| b.padding(SpacingValue::Integer(8)))
            .lg(|b| b.padding(SpacingValue::Integer(10)))
            
            // State variants
            .hover(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade600)))
            .focus(|b| b.ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade500)))
            .active(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade700)))
            .disabled(|b| b.opacity(50))
            
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Verify base classes
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("m-2"));
        assert!(css_classes.contains("bg-blue-500"));
        assert!(css_classes.contains("text-white"));
        assert!(css_classes.contains("rounded-lg"));
        assert!(css_classes.contains("shadow-md"));
        
        // Verify responsive classes
        assert!(css_classes.contains("sm:p-6"));
        assert!(css_classes.contains("md:p-8"));
        assert!(css_classes.contains("lg:p-10"));
        
        // Verify state variants
        assert!(css_classes.contains("hover:bg-blue-600"));
        assert!(css_classes.contains("focus:ring-blue-500"));
        assert!(css_classes.contains("active:bg-blue-700"));
        assert!(css_classes.contains("disabled:opacity-50"));
    }
}
```

### 3. Property-Based Tests

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_spacing_value_properties(
            value in prop::sample::select(&[
                SpacingValue::Zero,
                SpacingValue::Px,
                SpacingValue::Fractional(0.5),
                SpacingValue::Fractional(1.5),
                SpacingValue::Fractional(2.5),
                SpacingValue::Fractional(3.5),
                SpacingValue::Integer(1),
                SpacingValue::Integer(2),
                SpacingValue::Integer(3),
                SpacingValue::Integer(4),
                SpacingValue::Integer(5),
                SpacingValue::Integer(6),
                SpacingValue::Integer(8),
                SpacingValue::Integer(10),
                SpacingValue::Integer(12),
                SpacingValue::Integer(16),
                SpacingValue::Integer(20),
                SpacingValue::Integer(24),
                SpacingValue::Integer(32),
                SpacingValue::Integer(40),
                SpacingValue::Integer(48),
                SpacingValue::Integer(56),
                SpacingValue::Integer(64),
                SpacingValue::Integer(72),
                SpacingValue::Integer(80),
                SpacingValue::Integer(96),
                SpacingValue::Auto,
                SpacingValue::Full,
                SpacingValue::Screen,
                SpacingValue::Min,
                SpacingValue::Max,
                SpacingValue::Fit,
            ])
        ) {
            // Test that CSS value is always valid
            let css_value = value.to_css_value();
            assert!(!css_value.is_empty());
            
            // Test that class name is always valid
            let class_name = value.to_class_name();
            assert!(!class_name.is_empty());
            
            // Test that class name doesn't contain invalid characters
            assert!(!class_name.contains(' '));
            assert!(!class_name.contains('\n'));
            assert!(!class_name.contains('\r'));
            assert!(!class_name.contains('\t'));
        }
    }
    
    proptest! {
        #[test]
        fn test_color_properties(
            palette in prop::sample::select(&[
                ColorPalette::Gray,
                ColorPalette::Slate,
                ColorPalette::Zinc,
                ColorPalette::Neutral,
                ColorPalette::Stone,
                ColorPalette::Red,
                ColorPalette::Rose,
                ColorPalette::Pink,
                ColorPalette::Orange,
                ColorPalette::Amber,
                ColorPalette::Yellow,
                ColorPalette::Lime,
                ColorPalette::Green,
                ColorPalette::Emerald,
                ColorPalette::Teal,
                ColorPalette::Cyan,
                ColorPalette::Sky,
                ColorPalette::Blue,
                ColorPalette::Indigo,
                ColorPalette::Violet,
                ColorPalette::Purple,
                ColorPalette::Fuchsia,
            ]),
            shade in prop::sample::select(&[
                ColorShade::Shade50,
                ColorShade::Shade100,
                ColorShade::Shade200,
                ColorShade::Shade300,
                ColorShade::Shade400,
                ColorShade::Shade500,
                ColorShade::Shade600,
                ColorShade::Shade700,
                ColorShade::Shade800,
                ColorShade::Shade900,
            ])
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
}
```

## 🚀 Performance Considerations

### 1. Lazy Evaluation

```rust
// Only generate CSS when needed
impl ClassSet {
    /// Convert to CSS classes with lazy evaluation
    pub fn to_css_classes(&self) -> String {
        // Use cached result if available
        if let Some(cached) = self.cached_css.as_ref() {
            return cached.clone();
        }
        
        // Generate CSS classes
        let mut classes = Vec::new();
        
        // Add base classes
        classes.extend(self.classes.iter().cloned());
        
        // Add responsive classes
        for (breakpoint, breakpoint_classes) in &self.responsive {
            for class in breakpoint_classes {
                classes.push(format!("{}:{}", breakpoint.prefix(), class));
            }
        }
        
        // Add conditional classes
        for (condition, condition_classes) in &self.conditional {
            for class in condition_classes {
                classes.push(format!("{}:{}", condition, class));
            }
        }
        
        // Sort and join
        classes.sort();
        let result = classes.join(" ");
        
        // Cache the result
        self.cached_css = Some(result.clone());
        
        result
    }
}
```

### 2. Intelligent Caching

```rust
// Cache frequently used class combinations
use std::collections::HashMap;
use std::sync::RwLock;

pub struct ClassCache {
    cache: RwLock<HashMap<String, String>>,
    max_size: usize,
}

impl ClassCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            max_size,
        }
    }
    
    pub fn get(&self, key: &str) -> Option<String> {
        self.cache.read().unwrap().get(key).cloned()
    }
    
    pub fn insert(&self, key: String, value: String) {
        let mut cache = self.cache.write().unwrap();
        
        // Evict oldest entries if cache is full
        if cache.len() >= self.max_size {
            let keys_to_remove: Vec<String> = cache.keys().take(self.max_size / 2).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }
        
        cache.insert(key, value);
    }
}
```

### 3. Memory Optimization

```rust
// Use string interning for common class names
use std::collections::HashMap;
use std::sync::RwLock;

pub struct StringInterner {
    strings: RwLock<HashMap<String, usize>>,
    reverse: RwLock<Vec<String>>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            strings: RwLock::new(HashMap::new()),
            reverse: RwLock::new(Vec::new()),
        }
    }
    
    pub fn intern(&self, s: &str) -> usize {
        // Check if string is already interned
        if let Some(&id) = self.strings.read().unwrap().get(s) {
            return id;
        }
        
        // Intern the string
        let mut strings = self.strings.write().unwrap();
        let mut reverse = self.reverse.write().unwrap();
        
        let id = reverse.len();
        strings.insert(s.to_string(), id);
        reverse.push(s.to_string());
        
        id
    }
    
    pub fn get(&self, id: usize) -> Option<String> {
        self.reverse.read().unwrap().get(id).cloned()
    }
}
```

## 📚 Documentation

### 1. API Documentation

```rust
/// Spacing utility values for padding and margin
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core::utilities::spacing::SpacingValue;
/// use tailwind_rs_core::classes::ClassBuilder;
/// 
/// let classes = ClassBuilder::new()
///     .padding(SpacingValue::Integer(4))
///     .padding_x(SpacingValue::Integer(6))
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpacingValue {
    /// Zero spacing (0)
    Zero,
    /// 1px spacing
    Px,
    /// Fractional spacing (0.5, 1.5, 2.5, 3.5)
    Fractional(f32),
    /// Integer spacing (1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 56, 64, 72, 80, 96)
    Integer(u32),
    /// Auto spacing
    Auto,
    /// Full spacing (100%)
    Full,
    /// Screen spacing (100vh)
    Screen,
    /// Min content spacing
    Min,
    /// Max content spacing
    Max,
    /// Fit content spacing
    Fit,
}
```

### 2. Usage Examples

```rust
/// # Examples
/// 
/// ## Basic Usage
/// 
/// ```rust
/// use tailwind_rs_core::classes::ClassBuilder;
/// use tailwind_rs_core::utilities::spacing::SpacingValue;
/// use tailwind_rs_core::utilities::colors::{Color, ColorPalette, ColorShade};
/// 
/// let classes = ClassBuilder::new()
///     .padding(SpacingValue::Integer(4))
///     .margin(SpacingValue::Integer(2))
///     .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
///     .text_color(Color::new(ColorPalette::White, ColorShade::Shade500))
///     .build();
/// ```
/// 
/// ## Responsive Usage
/// 
/// ```rust
/// let classes = ClassBuilder::new()
///     .padding(SpacingValue::Integer(4))
///     .sm(|b| b.padding(SpacingValue::Integer(6)))
///     .md(|b| b.padding(SpacingValue::Integer(8)))
///     .lg(|b| b.padding(SpacingValue::Integer(10)))
///     .build();
/// ```
/// 
/// ## State Variants
/// 
/// ```rust
/// let classes = ClassBuilder::new()
///     .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
///     .hover(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade600)))
///     .focus(|b| b.ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade500)))
///     .active(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade700)))
///     .build();
/// ```
```

## 🔧 Development Tools

### 1. Code Generation

```rust
// Generate utility classes from configuration
pub struct UtilityGenerator {
    config: UtilityConfig,
}

impl UtilityGenerator {
    pub fn generate_spacing_utilities(&self) -> String {
        let mut code = String::new();
        
        // Generate padding utilities
        code.push_str("pub trait PaddingUtilities {\n");
        for value in &self.config.spacing_values {
            code.push_str(&format!(
                "    fn padding_{}(self) -> Self;\n",
                value.to_method_name()
            ));
        }
        code.push_str("}\n\n");
        
        // Generate margin utilities
        code.push_str("pub trait MarginUtilities {\n");
        for value in &self.config.spacing_values {
            code.push_str(&format!(
                "    fn margin_{}(self) -> Self;\n",
                value.to_method_name()
            ));
        }
        code.push_str("}\n\n");
        
        code
    }
}
```

### 2. Validation Tools

```rust
// Validate utility class combinations
pub struct UtilityValidator {
    rules: ValidationRules,
}

impl UtilityValidator {
    pub fn validate_class_combination(&self, classes: &[String]) -> Result<(), ValidationError> {
        // Check for conflicting classes
        for class in classes {
            if let Some(conflicts) = self.rules.conflicts.get(class) {
                for conflict in conflicts {
                    if classes.contains(conflict) {
                        return Err(ValidationError::ConflictingClasses {
                            class: class.clone(),
                            conflict: conflict.clone(),
                        });
                    }
                }
            }
        }
        
        // Check for invalid combinations
        for combination in &self.rules.invalid_combinations {
            if combination.iter().all(|c| classes.contains(c)) {
                return Err(ValidationError::InvalidCombination {
                    classes: combination.clone(),
                });
            }
        }
        
        Ok(())
    }
}
```

---

**Last Updated**: January 2025  
**Version**: 1.0  
**Status**: Technical Implementation Guide
