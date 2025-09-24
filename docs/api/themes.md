# Theme System API Reference

This document provides comprehensive API reference for the theme system in `tailwind-rs`, following our Test-Driven Development approach and comprehensive testing strategy.

## 🎨 Core Theme Types

### Theme
The primary theme type that defines the overall design system.

```rust
pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub background_color: Color,
    pub text_color: Color,
    pub border_color: Color,
    pub success_color: Color,
    pub warning_color: Color,
    pub error_color: Color,
    pub info_color: Color,
    pub spacing: SpacingScale,
    pub typography: TypographyScale,
    pub shadows: ShadowScale,
    pub borders: BorderScale,
    pub animations: AnimationScale,
}
```

#### Methods

##### `new() -> Self`
Creates a new theme with default values.

```rust
let theme = Theme::new();
```

**Example:**
```rust
use tailwind_rs::*;

let theme = Theme::new();
assert_eq!(theme.primary_color, Color::Blue);
assert_eq!(theme.secondary_color, Color::Gray);
```

##### `primary_color(self, color: Color) -> Self`
Sets the primary color for the theme.

```rust
let theme = Theme::new().primary_color(Color::Green);
```

**Example:**
```rust
use tailwind_rs::*;

let theme = Theme::new().primary_color(Color::Green);
assert_eq!(theme.primary_color, Color::Green);
```

##### `secondary_color(self, color: Color) -> Self`
Sets the secondary color for the theme.

```rust
let theme = Theme::new().secondary_color(Color::Purple);
```

**Example:**
```rust
use tailwind_rs::*;

let theme = Theme::new().secondary_color(Color::Purple);
assert_eq!(theme.secondary_color, Color::Purple);
```

##### `accent_color(self, color: Color) -> Self`
Sets the accent color for the theme.

```rust
let theme = Theme::new().accent_color(Color::Orange);
```

**Example:**
```rust
use tailwind_rs::*;

let theme = Theme::new().accent_color(Color::Orange);
assert_eq!(theme.accent_color, Color::Orange);
```

##### `apply_to_component(&self, component: &dyn ThemedComponent) -> String`
Applies the theme to a component.

```rust
let theme = Theme::new();
let component = Button::new();
let themed_classes = theme.apply_to_component(&component);
```

**Example:**
```rust
use tailwind_rs::*;

let theme = Theme::new().primary_color(Color::Green);
let button = Button::new().variant(ButtonVariant::Primary);
let classes = theme.apply_to_component(&button);

assert!(classes.contains("bg-green-600"));
```

### Color
Represents colors in the theme system.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Slate,
    Gray,
    Zinc,
    Neutral,
    Stone,
    Red,
    Orange,
    Amber,
    Yellow,
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    Sky,
    Blue,
    Indigo,
    Violet,
    Purple,
    Fuchsia,
    Pink,
    Rose,
}
```

#### Methods

##### `background(&self, intensity: u8) -> String`
Generates background color class.

```rust
let color = Color::Blue;
let class = color.background(600); // "bg-blue-600"
```

**Example:**
```rust
use tailwind_rs::*;

let color = Color::Blue;
assert_eq!(color.background(600), "bg-blue-600");
assert_eq!(color.background(700), "bg-blue-700");
```

##### `text(&self, intensity: u8) -> String`
Generates text color class.

```rust
let color = Color::Blue;
let class = color.text(600); // "text-blue-600"
```

**Example:**
```rust
use tailwind_rs::*;

let color = Color::Blue;
assert_eq!(color.text(600), "text-blue-600");
assert_eq!(color.text(700), "text-blue-700");
```

##### `border(&self, intensity: u8) -> String`
Generates border color class.

```rust
let color = Color::Blue;
let class = color.border(600); // "border-blue-600"
```

**Example:**
```rust
use tailwind_rs::*;

let color = Color::Blue;
assert_eq!(color.border(600), "border-blue-600");
assert_eq!(color.border(700), "border-blue-700");
```

##### `ring(&self, intensity: u8) -> String`
Generates ring color class.

```rust
let color = Color::Blue;
let class = color.ring(600); // "ring-blue-600"
```

**Example:**
```rust
use tailwind_rs::*;

let color = Color::Blue;
assert_eq!(color.ring(600), "ring-blue-600");
assert_eq!(color.ring(700), "ring-blue-700");
```

##### `hover(&self, intensity: u8) -> String`
Generates hover color class.

```rust
let color = Color::Blue;
let class = color.hover(700); // "hover:bg-blue-700"
```

**Example:**
```rust
use tailwind_rs::*;

let color = Color::Blue;
assert_eq!(color.hover(700), "hover:bg-blue-700");
assert_eq!(color.hover(800), "hover:bg-blue-800");
```

##### `focus(&self, intensity: u8) -> String`
Generates focus color class.

```rust
let color = Color::Blue;
let class = color.focus(500); // "focus:ring-blue-500"
```

**Example:**
```rust
use tailwind_rs::*;

let color = Color::Blue;
assert_eq!(color.focus(500), "focus:ring-blue-500");
assert_eq!(color.focus(600), "focus:ring-blue-600");
```

## 🎯 Component Theming

### ThemedComponent
Trait for components that support theming.

```rust
pub trait ThemedComponent {
    fn base_classes(&self) -> &str;
    fn apply_theme(&self, theme: &Theme) -> String;
    fn theme_variants(&self) -> Vec<ThemeVariant>;
}
```

#### Methods

##### `base_classes(&self) -> &str`
Returns the base classes for the component.

```rust
impl ThemedComponent for Button {
    fn base_classes(&self) -> &str {
        "px-4 py-2 rounded-md font-medium transition-colors duration-200"
    }
}
```

##### `apply_theme(&self, theme: &Theme) -> String`
Applies the theme to the component.

```rust
impl ThemedComponent for Button {
    fn apply_theme(&self, theme: &Theme) -> String {
        match self.variant {
            ButtonVariant::Primary => {
                classes! {
                    base: self.base_classes(),
                    background: theme.primary_color.background(600),
                    text: theme.primary_color.text(600),
                    hover: theme.primary_color.hover(700),
                }.build()
            }
            ButtonVariant::Secondary => {
                classes! {
                    base: self.base_classes(),
                    background: theme.secondary_color.background(200),
                    text: theme.secondary_color.text(900),
                    hover: theme.secondary_color.hover(300),
                }.build()
            }
        }
    }
}
```

##### `theme_variants(&self) -> Vec<ThemeVariant>`
Returns available theme variants for the component.

```rust
impl ThemedComponent for Button {
    fn theme_variants(&self) -> Vec<ThemeVariant> {
        vec![
            ThemeVariant::Primary,
            ThemeVariant::Secondary,
            ThemeVariant::Danger,
            ThemeVariant::Success,
        ]
    }
}
```

### ThemeVariant
Represents different theme variants for components.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeVariant {
    Primary,
    Secondary,
    Danger,
    Success,
    Warning,
    Info,
    Light,
    Dark,
}
```

#### Methods

##### `color(&self) -> Color`
Returns the associated color for the variant.

```rust
impl ThemeVariant {
    pub fn color(&self) -> Color {
        match self {
            ThemeVariant::Primary => Color::Blue,
            ThemeVariant::Secondary => Color::Gray,
            ThemeVariant::Danger => Color::Red,
            ThemeVariant::Success => Color::Green,
            ThemeVariant::Warning => Color::Yellow,
            ThemeVariant::Info => Color::Cyan,
            ThemeVariant::Light => Color::Gray,
            ThemeVariant::Dark => Color::Gray,
        }
    }
}
```

##### `intensity(&self) -> u8`
Returns the default intensity for the variant.

```rust
impl ThemeVariant {
    pub fn intensity(&self) -> u8 {
        match self {
            ThemeVariant::Primary => 600,
            ThemeVariant::Secondary => 200,
            ThemeVariant::Danger => 600,
            ThemeVariant::Success => 600,
            ThemeVariant::Warning => 500,
            ThemeVariant::Info => 600,
            ThemeVariant::Light => 100,
            ThemeVariant::Dark => 800,
        }
    }
}
```

## 🎨 Spacing System

### SpacingScale
Defines the spacing scale for the theme.

```rust
pub struct SpacingScale {
    pub xs: String,    // 0.25rem
    pub sm: String,    // 0.5rem
    pub md: String,    // 1rem
    pub lg: String,    // 1.5rem
    pub xl: String,    // 2rem
    pub xxl: String,   // 3rem
    pub xxxl: String,  // 4rem
}
```

#### Methods

##### `new() -> Self`
Creates a new spacing scale with default values.

```rust
let spacing = SpacingScale::new();
```

**Example:**
```rust
use tailwind_rs::*;

let spacing = SpacingScale::new();
assert_eq!(spacing.xs, "0.25rem");
assert_eq!(spacing.sm, "0.5rem");
assert_eq!(spacing.md, "1rem");
```

##### `custom(xs: &str, sm: &str, md: &str, lg: &str, xl: &str, xxl: &str, xxxl: &str) -> Self`
Creates a custom spacing scale.

```rust
let spacing = SpacingScale::custom(
    "0.125rem", // xs
    "0.25rem",  // sm
    "0.5rem",   // md
    "1rem",     // lg
    "2rem",     // xl
    "4rem",     // xxl
    "8rem",     // xxxl
);
```

##### `get(&self, size: SpacingSize) -> &str`
Gets spacing value for a specific size.

```rust
let spacing = SpacingScale::new();
let value = spacing.get(SpacingSize::Md); // "1rem"
```

### SpacingSize
Represents different spacing sizes.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpacingSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Xxxl,
}
```

## 🎨 Typography System

### TypographyScale
Defines the typography scale for the theme.

```rust
pub struct TypographyScale {
    pub font_family: FontFamily,
    pub font_sizes: FontSizeScale,
    pub font_weights: FontWeightScale,
    pub line_heights: LineHeightScale,
    pub letter_spacing: LetterSpacingScale,
}
```

#### Methods

##### `new() -> Self`
Creates a new typography scale with default values.

```rust
let typography = TypographyScale::new();
```

##### `font_family(self, family: FontFamily) -> Self`
Sets the font family for the typography scale.

```rust
let typography = TypographyScale::new()
    .font_family(FontFamily::Sans);
```

### FontFamily
Represents different font families.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontFamily {
    Sans,
    Serif,
    Mono,
    Custom(String),
}
```

#### Methods

##### `class(&self) -> &str`
Returns the CSS class for the font family.

```rust
impl FontFamily {
    pub fn class(&self) -> &str {
        match self {
            FontFamily::Sans => "font-sans",
            FontFamily::Serif => "font-serif",
            FontFamily::Mono => "font-mono",
            FontFamily::Custom(name) => name,
        }
    }
}
```

### FontSizeScale
Defines font sizes for the typography scale.

```rust
pub struct FontSizeScale {
    pub xs: String,    // 0.75rem
    pub sm: String,    // 0.875rem
    pub base: String,  // 1rem
    pub lg: String,    // 1.125rem
    pub xl: String,    // 1.25rem
    pub xxl: String,   // 1.5rem
    pub xxxl: String,  // 1.875rem
}
```

#### Methods

##### `new() -> Self`
Creates a new font size scale with default values.

```rust
let font_sizes = FontSizeScale::new();
```

##### `get(&self, size: FontSize) -> &str`
Gets font size value for a specific size.

```rust
let font_sizes = FontSizeScale::new();
let size = font_sizes.get(FontSize::Base); // "1rem"
```

### FontSize
Represents different font sizes.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontSize {
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    Xxl,
    Xxxl,
}
```

## 🎨 Shadow System

### ShadowScale
Defines the shadow scale for the theme.

```rust
pub struct ShadowScale {
    pub sm: String,    // 0 1px 2px 0 rgb(0 0 0 / 0.05)
    pub base: String,  // 0 1px 3px 0 rgb(0 0 0 / 0.1)
    pub md: String,    // 0 4px 6px -1px rgb(0 0 0 / 0.1)
    pub lg: String,    // 0 10px 15px -3px rgb(0 0 0 / 0.1)
    pub xl: String,    // 0 20px 25px -5px rgb(0 0 0 / 0.1)
    pub xxl: String,   // 0 25px 50px -12px rgb(0 0 0 / 0.25)
}
```

#### Methods

##### `new() -> Self`
Creates a new shadow scale with default values.

```rust
let shadows = ShadowScale::new();
```

##### `get(&self, size: ShadowSize) -> &str`
Gets shadow value for a specific size.

```rust
let shadows = ShadowScale::new();
let shadow = shadows.get(ShadowSize::Md);
```

### ShadowSize
Represents different shadow sizes.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShadowSize {
    Sm,
    Base,
    Md,
    Lg,
    Xl,
    Xxl,
}
```

## 🎨 Border System

### BorderScale
Defines the border scale for the theme.

```rust
pub struct BorderScale {
    pub width: BorderWidthScale,
    pub radius: BorderRadiusScale,
    pub style: BorderStyleScale,
}
```

#### Methods

##### `new() -> Self`
Creates a new border scale with default values.

```rust
let borders = BorderScale::new();
```

### BorderWidthScale
Defines border widths.

```rust
pub struct BorderWidthScale {
    pub none: String,  // 0px
    pub sm: String,    // 1px
    pub base: String,  // 2px
    pub md: String,    // 4px
    pub lg: String,    // 8px
}
```

### BorderRadiusScale
Defines border radius values.

```rust
pub struct BorderRadiusScale {
    pub none: String,  // 0px
    pub sm: String,    // 0.125rem
    pub base: String,  // 0.25rem
    pub md: String,    // 0.375rem
    pub lg: String,    // 0.5rem
    pub xl: String,    // 0.75rem
    pub xxl: String,   // 1rem
    pub full: String,  // 9999px
}
```

## 🎨 Animation System

### AnimationScale
Defines the animation scale for the theme.

```rust
pub struct AnimationScale {
    pub duration: DurationScale,
    pub easing: EasingScale,
    pub delay: DelayScale,
}
```

#### Methods

##### `new() -> Self`
Creates a new animation scale with default values.

```rust
let animations = AnimationScale::new();
```

### DurationScale
Defines animation durations.

```rust
pub struct DurationScale {
    pub fast: String,    // 150ms
    pub base: String,    // 200ms
    pub slow: String,    // 300ms
    pub slower: String,  // 500ms
    pub slowest: String, // 1000ms
}
```

### EasingScale
Defines animation easing functions.

```rust
pub struct EasingScale {
    pub linear: String,      // linear
    pub ease_in: String,     // ease-in
    pub ease_out: String,    // ease-out
    pub ease_in_out: String, // ease-in-out
}
```

## 🎨 Theme Presets

### ThemePreset
Predefined theme configurations.

```rust
pub enum ThemePreset {
    Default,
    Dark,
    Light,
    HighContrast,
    Minimal,
    Colorful,
    Professional,
    Playful,
}
```

#### Methods

##### `create() -> Theme`
Creates a theme from the preset.

```rust
let theme = ThemePreset::Dark.create();
```

**Example:**
```rust
use tailwind_rs::*;

let dark_theme = ThemePreset::Dark.create();
assert_eq!(dark_theme.background_color, Color::Gray);
assert_eq!(dark_theme.text_color, Color::Gray);

let light_theme = ThemePreset::Light.create();
assert_eq!(light_theme.background_color, Color::Gray);
assert_eq!(light_theme.text_color, Color::Gray);
```

## 🧪 Testing Theme System

### Theme Testing Utilities
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new()
            .primary_color(Color::Blue)
            .secondary_color(Color::Gray);
        
        assert_eq!(theme.primary_color, Color::Blue);
        assert_eq!(theme.secondary_color, Color::Gray);
    }

    #[test]
    fn test_color_generation() {
        let color = Color::Blue;
        assert_eq!(color.background(600), "bg-blue-600");
        assert_eq!(color.text(600), "text-blue-600");
        assert_eq!(color.border(600), "border-blue-600");
        assert_eq!(color.ring(600), "ring-blue-600");
    }

    #[test]
    fn test_component_theming() {
        let theme = Theme::new().primary_color(Color::Green);
        let button = Button::new().variant(ButtonVariant::Primary);
        let classes = theme.apply_to_component(&button);
        
        assert!(classes.contains("bg-green-600"));
    }

    #[test]
    fn test_theme_presets() {
        let dark_theme = ThemePreset::Dark.create();
        let light_theme = ThemePreset::Light.create();
        
        assert_ne!(dark_theme.background_color, light_theme.background_color);
    }
}
```

### Integration Testing
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_theme_system_integration() {
        let theme = Theme::new()
            .primary_color(Color::Blue)
            .secondary_color(Color::Gray)
            .accent_color(Color::Green);
        
        let button = Button::new().variant(ButtonVariant::Primary);
        let input = Input::new().variant(InputVariant::Default);
        
        let button_classes = theme.apply_to_component(&button);
        let input_classes = theme.apply_to_component(&input);
        
        assert!(button_classes.contains("bg-blue-600"));
        assert!(input_classes.contains("border-gray-300"));
    }
}
```

## 📚 Examples

### Basic Theme Usage
```rust
use tailwind_rs::*;

// Create a custom theme
let theme = Theme::new()
    .primary_color(Color::Blue)
    .secondary_color(Color::Gray)
    .accent_color(Color::Green);

// Apply theme to components
let button = Button::new().variant(ButtonVariant::Primary);
let classes = theme.apply_to_component(&button);

// Use theme colors directly
let color = Color::Blue;
let classes = classes! {
    base: "px-4 py-2 rounded-md",
    background: color.background(600),
    text: color.text(600),
    hover: color.hover(700),
};
```

### Advanced Theme Usage
```rust
use tailwind_rs::*;

// Create a comprehensive theme
let theme = Theme::new()
    .primary_color(Color::Blue)
    .secondary_color(Color::Gray)
    .accent_color(Color::Green)
    .spacing(SpacingScale::custom(
        "0.125rem", // xs
        "0.25rem",  // sm
        "0.5rem",   // md
        "1rem",     // lg
        "2rem",     // xl
        "4rem",     // xxl
        "8rem",     // xxxl
    ))
    .typography(TypographyScale::new()
        .font_family(FontFamily::Sans)
    );

// Apply theme to multiple components
let components = vec![
    Button::new().variant(ButtonVariant::Primary),
    Input::new().variant(InputVariant::Default),
    Card::new().variant(CardVariant::Default),
];

for component in components {
    let classes = theme.apply_to_component(&component);
    // Use classes...
}
```

### Theme Preset Usage
```rust
use tailwind_rs::*;

// Use predefined themes
let dark_theme = ThemePreset::Dark.create();
let light_theme = ThemePreset::Light.create();
let professional_theme = ThemePreset::Professional.create();

// Apply different themes based on context
let theme = if is_dark_mode {
    dark_theme
} else {
    light_theme
};

let button = Button::new().variant(ButtonVariant::Primary);
let classes = theme.apply_to_component(&button);
```

---

This theme system API reference provides comprehensive documentation for all theme-related functionality in `tailwind-rs`. The theme system is designed with flexibility, type safety, and performance in mind, following our established ADRs and best practices.

