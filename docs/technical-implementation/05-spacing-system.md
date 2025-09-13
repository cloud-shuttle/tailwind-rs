# 📏 Spacing System

## 📋 Overview

The spacing system is one of the most fundamental parts of Tailwind CSS, providing utilities for padding, margin, and gap. This document outlines the complete implementation of the spacing system in Rust, including all spacing values, utilities, and responsive variants.

## 🎯 Spacing Values

### Core Spacing Scale

The spacing system is based on a consistent scale where each unit represents 0.25rem (4px by default).

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpacingValue {
    /// 0 spacing
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

### Spacing Value Implementation

```rust
impl SpacingValue {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            SpacingValue::Zero => "0".to_string(),
            SpacingValue::Px => "1px".to_string(),
            SpacingValue::Fractional(f) => format!("{}rem", f * 0.25),
            SpacingValue::Integer(i) => format!("{}rem", i * 0.25),
            SpacingValue::Auto => "auto".to_string(),
            SpacingValue::Full => "100%".to_string(),
            SpacingValue::Full => "100vh".to_string(),
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
    
    /// Get all valid spacing values
    pub fn all_values() -> Vec<SpacingValue> {
        vec![
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
        ]
    }
}
```

## 🎨 Padding Utilities

### Padding Trait

```rust
/// Trait for adding padding utilities to a class builder
pub trait PaddingUtilities {
    /// Add padding to all sides
    fn padding(self, value: SpacingValue) -> Self;
    
    /// Add horizontal padding (left and right)
    fn padding_x(self, value: SpacingValue) -> Self;
    
    /// Add vertical padding (top and bottom)
    fn padding_y(self, value: SpacingValue) -> Self;
    
    /// Add top padding
    fn padding_top(self, value: SpacingValue) -> Self;
    
    /// Add right padding
    fn padding_right(self, value: SpacingValue) -> Self;
    
    /// Add bottom padding
    fn padding_bottom(self, value: SpacingValue) -> Self;
    
    /// Add left padding
    fn padding_left(self, value: SpacingValue) -> Self;
    
    /// Add padding to start (left in LTR, right in RTL)
    fn padding_start(self, value: SpacingValue) -> Self;
    
    /// Add padding to end (right in LTR, left in RTL)
    fn padding_end(self, value: SpacingValue) -> Self;
}
```

### Padding Implementation

```rust
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
    
    fn padding_start(self, value: SpacingValue) -> Self {
        self.class(&format!("ps-{}", value.to_class_name()))
    }
    
    fn padding_end(self, value: SpacingValue) -> Self {
        self.class(&format!("pe-{}", value.to_class_name()))
    }
}
```

### Padding Usage Examples

```rust
// Basic padding
let classes = ClassBuilder::new()
    .padding(SpacingValue::Integer(4))        // p-4
    .build();

// Directional padding
let classes = ClassBuilder::new()
    .padding_x(SpacingValue::Integer(6))      // px-6
    .padding_y(SpacingValue::Integer(2))      // py-2
    .build();

// Individual sides
let classes = ClassBuilder::new()
    .padding_top(SpacingValue::Integer(8))    // pt-8
    .padding_right(SpacingValue::Integer(4))  // pr-4
    .padding_bottom(SpacingValue::Integer(2)) // pb-2
    .padding_left(SpacingValue::Integer(6))   // pl-6
    .build();

// Fractional values
let classes = ClassBuilder::new()
    .padding(SpacingValue::Fractional(1.5))   // p-1.5
    .padding_x(SpacingValue::Fractional(2.5)) // px-2.5
    .build();

// Special values
let classes = ClassBuilder::new()
    .padding(SpacingValue::Auto)              // p-auto
    .padding_x(SpacingValue::Full)            // px-full
    .build();
```

## 📐 Margin Utilities

### Margin Trait

```rust
/// Trait for adding margin utilities to a class builder
pub trait MarginUtilities {
    /// Add margin to all sides
    fn margin(self, value: SpacingValue) -> Self;
    
    /// Add horizontal margin (left and right)
    fn margin_x(self, value: SpacingValue) -> Self;
    
    /// Add vertical margin (top and bottom)
    fn margin_y(self, value: SpacingValue) -> Self;
    
    /// Add top margin
    fn margin_top(self, value: SpacingValue) -> Self;
    
    /// Add right margin
    fn margin_right(self, value: SpacingValue) -> Self;
    
    /// Add bottom margin
    fn margin_bottom(self, value: SpacingValue) -> Self;
    
    /// Add left margin
    fn margin_left(self, value: SpacingValue) -> Self;
    
    /// Add margin to start (left in LTR, right in RTL)
    fn margin_start(self, value: SpacingValue) -> Self;
    
    /// Add margin to end (right in LTR, left in RTL)
    fn margin_end(self, value: SpacingValue) -> Self;
    
    /// Add negative margin to all sides
    fn margin_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative horizontal margin
    fn margin_x_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative vertical margin
    fn margin_y_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative top margin
    fn margin_top_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative right margin
    fn margin_right_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative bottom margin
    fn margin_bottom_negative(self, value: SpacingValue) -> Self;
    
    /// Add negative left margin
    fn margin_left_negative(self, value: SpacingValue) -> Self;
}
```

### Margin Implementation

```rust
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
    
    fn margin_start(self, value: SpacingValue) -> Self {
        self.class(&format!("ms-{}", value.to_class_name()))
    }
    
    fn margin_end(self, value: SpacingValue) -> Self {
        self.class(&format!("me-{}", value.to_class_name()))
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
    
    fn margin_top_negative(self, value: SpacingValue) -> Self {
        self.class(&format!("-mt-{}", value.to_class_name()))
    }
    
    fn margin_right_negative(self, value: SpacingValue) -> Self {
        self.class(&format!("-mr-{}", value.to_class_name()))
    }
    
    fn margin_bottom_negative(self, value: SpacingValue) -> Self {
        self.class(&format!("-mb-{}", value.to_class_name()))
    }
    
    fn margin_left_negative(self, value: SpacingValue) -> Self {
        self.class(&format!("-ml-{}", value.to_class_name()))
    }
}
```

### Margin Usage Examples

```rust
// Basic margin
let classes = ClassBuilder::new()
    .margin(SpacingValue::Integer(4))         // m-4
    .build();

// Directional margin
let classes = ClassBuilder::new()
    .margin_x(SpacingValue::Integer(6))       // mx-6
    .margin_y(SpacingValue::Integer(2))       // my-2
    .build();

// Individual sides
let classes = ClassBuilder::new()
    .margin_top(SpacingValue::Integer(8))     // mt-8
    .margin_right(SpacingValue::Integer(4))   // mr-4
    .margin_bottom(SpacingValue::Integer(2))  // mb-2
    .margin_left(SpacingValue::Integer(6))    // ml-6
    .build();

// Negative margins
let classes = ClassBuilder::new()
    .margin_negative(SpacingValue::Integer(4))    // -m-4
    .margin_x_negative(SpacingValue::Integer(2))  // -mx-2
    .margin_y_negative(SpacingValue::Integer(1))  // -my-1
    .build();

// Special values
let classes = ClassBuilder::new()
    .margin(SpacingValue::Auto)                 // m-auto
    .margin_x(SpacingValue::Auto)               // mx-auto
    .build();
```

## 🔄 Gap Utilities

### Gap Trait

```rust
/// Trait for adding gap utilities to a class builder
pub trait GapUtilities {
    /// Add gap between grid/flex items
    fn gap(self, value: SpacingValue) -> Self;
    
    /// Add horizontal gap between grid/flex items
    fn gap_x(self, value: SpacingValue) -> Self;
    
    /// Add vertical gap between grid/flex items
    fn gap_y(self, value: SpacingValue) -> Self;
}
```

### Gap Implementation

```rust
impl GapUtilities for ClassBuilder {
    fn gap(self, value: SpacingValue) -> Self {
        self.class(&format!("gap-{}", value.to_class_name()))
    }
    
    fn gap_x(self, value: SpacingValue) -> Self {
        self.class(&format!("gap-x-{}", value.to_class_name()))
    }
    
    fn gap_y(self, value: SpacingValue) -> Self {
        self.class(&format!("gap-y-{}", value.to_class_name()))
    }
}
```

### Gap Usage Examples

```rust
// Basic gap
let classes = ClassBuilder::new()
    .gap(SpacingValue::Integer(4))          // gap-4
    .build();

// Directional gap
let classes = ClassBuilder::new()
    .gap_x(SpacingValue::Integer(6))        // gap-x-6
    .gap_y(SpacingValue::Integer(2))        // gap-y-2
    .build();

// Fractional gap
let classes = ClassBuilder::new()
    .gap(SpacingValue::Fractional(1.5))     // gap-1.5
    .gap_x(SpacingValue::Fractional(2.5))   // gap-x-2.5
    .build();
```

## 📱 Responsive Spacing

### Responsive Spacing Implementation

```rust
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

### Responsive Spacing Usage Examples

```rust
// Responsive padding
let classes = ClassBuilder::new()
    .padding(SpacingValue::Integer(4))                    // p-4
    .sm(|b| b.padding(SpacingValue::Integer(6)))          // sm:p-6
    .md(|b| b.padding(SpacingValue::Integer(8)))          // md:p-8
    .lg(|b| b.padding(SpacingValue::Integer(10)))         // lg:p-10
    .xl(|b| b.padding(SpacingValue::Integer(12)))         // xl:p-12
    .build();

// Responsive margin
let classes = ClassBuilder::new()
    .margin(SpacingValue::Integer(2))                     // m-2
    .sm(|b| b.margin(SpacingValue::Integer(4)))           // sm:m-4
    .md(|b| b.margin(SpacingValue::Integer(6)))           // md:m-6
    .lg(|b| b.margin(SpacingValue::Integer(8)))           // lg:m-8
    .build();

// Responsive gap
let classes = ClassBuilder::new()
    .gap(SpacingValue::Integer(2))                        // gap-2
    .sm(|b| b.gap(SpacingValue::Integer(4)))              // sm:gap-4
    .md(|b| b.gap(SpacingValue::Integer(6)))              // md:gap-6
    .lg(|b| b.gap(SpacingValue::Integer(8)))              // lg:gap-8
    .build();
```

## 🎭 State Variants

### State Variant Implementation

```rust
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
}
```

### State Variant Usage Examples

```rust
// Hover states
let classes = ClassBuilder::new()
    .padding(SpacingValue::Integer(4))                    // p-4
    .hover(|b| b.padding(SpacingValue::Integer(6)))       // hover:p-6
    .build();

// Focus states
let classes = ClassBuilder::new()
    .margin(SpacingValue::Integer(2))                     // m-2
    .focus(|b| b.margin(SpacingValue::Integer(4)))        // focus:m-4
    .build();

// Active states
let classes = ClassBuilder::new()
    .gap(SpacingValue::Integer(2))                        // gap-2
    .active(|b| b.gap(SpacingValue::Integer(4)))          // active:gap-4
    .build();

// Disabled states
let classes = ClassBuilder::new()
    .padding(SpacingValue::Integer(4))                    // p-4
    .disabled(|b| b.padding(SpacingValue::Integer(2)))    // disabled:p-2
    .build();
```

## 🧪 Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
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
    fn test_gap_utilities() {
        let classes = ClassBuilder::new()
            .gap(SpacingValue::Integer(4))
            .gap_x(SpacingValue::Integer(6))
            .gap_y(SpacingValue::Integer(2))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("gap-4"));
        assert!(css_classes.contains("gap-x-6"));
        assert!(css_classes.contains("gap-y-2"));
    }
    
    #[test]
    fn test_responsive_spacing() {
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
            .padding(SpacingValue::Integer(4))
            .hover(|b| b.padding(SpacingValue::Integer(6)))
            .focus(|b| b.padding(SpacingValue::Integer(8)))
            .active(|b| b.padding(SpacingValue::Integer(10)))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("hover:p-6"));
        assert!(css_classes.contains("focus:p-8"));
        assert!(css_classes.contains("active:p-10"));
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
        fn test_spacing_value_properties(
            value in prop::sample::select(&SpacingValue::all_values())
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
        fn test_spacing_utility_combinations(
            padding in prop::sample::select(&SpacingValue::all_values()),
            margin in prop::sample::select(&SpacingValue::all_values()),
            gap in prop::sample::select(&SpacingValue::all_values())
        ) {
            let classes = ClassBuilder::new()
                .padding(padding)
                .margin(margin)
                .gap(gap)
                .build();
            
            let css_classes = classes.to_css_classes();
            
            // Test that all classes are present
            assert!(css_classes.contains(&format!("p-{}", padding.to_class_name())));
            assert!(css_classes.contains(&format!("m-{}", margin.to_class_name())));
            assert!(css_classes.contains(&format!("gap-{}", gap.to_class_name())));
            
            // Test that CSS classes are valid
            assert!(!css_classes.is_empty());
            assert!(!css_classes.contains("  ")); // No double spaces
        }
    }
}
```

## 🚀 Performance Considerations

### 1. Lazy Evaluation
```rust
impl ClassSet {
    pub fn to_css_classes(&self) -> String {
        if let Some(cached) = self.cached_css.as_ref() {
            return cached.clone();
        }
        
        let result = self.generate_css_classes();
        self.cached_css = Some(result.clone());
        result
    }
}
```

### 2. String Interning
```rust
pub struct StringInterner {
    strings: HashMap<String, usize>,
    reverse: Vec<String>,
}

impl StringInterner {
    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(&id) = self.strings.get(s) {
            return id;
        }
        
        let id = self.reverse.len();
        self.strings.insert(s.to_string(), id);
        self.reverse.push(s.to_string());
        id
    }
}
```

### 3. Caching Strategy
```rust
pub struct ClassCache {
    cache: HashMap<String, String>,
    max_size: usize,
}

impl ClassCache {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.cache.get(key)
    }
    
    pub fn insert(&mut self, key: String, value: String) {
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }
        self.cache.insert(key, value);
    }
}
```

## 📊 Complete Spacing Reference

### Padding Classes
- `p-0`, `p-px`, `p-0.5`, `p-1`, `p-1.5`, `p-2`, `p-2.5`, `p-3`, `p-3.5`, `p-4`, `p-5`, `p-6`, `p-8`, `p-10`, `p-12`, `p-16`, `p-20`, `p-24`, `p-32`, `p-40`, `p-48`, `p-56`, `p-64`, `p-72`, `p-80`, `p-96`
- `px-*`, `py-*`, `pt-*`, `pr-*`, `pb-*`, `pl-*`, `ps-*`, `pe-*`

### Margin Classes
- `m-0`, `m-px`, `m-0.5`, `m-1`, `m-1.5`, `m-2`, `m-2.5`, `m-3`, `m-3.5`, `m-4`, `m-5`, `m-6`, `m-8`, `m-10`, `m-12`, `m-16`, `m-20`, `m-24`, `m-32`, `m-40`, `m-48`, `m-56`, `m-64`, `m-72`, `m-80`, `m-96`
- `mx-*`, `my-*`, `mt-*`, `mr-*`, `mb-*`, `ml-*`, `ms-*`, `me-*`
- `-m-*`, `-mx-*`, `-my-*`, `-mt-*`, `-mr-*`, `-mb-*`, `-ml-*`

### Gap Classes
- `gap-0`, `gap-px`, `gap-0.5`, `gap-1`, `gap-1.5`, `gap-2`, `gap-2.5`, `gap-3`, `gap-3.5`, `gap-4`, `gap-5`, `gap-6`, `gap-8`, `gap-10`, `gap-12`, `gap-16`, `gap-20`, `gap-24`, `gap-32`, `gap-40`, `gap-48`, `gap-56`, `gap-64`, `gap-72`, `gap-80`, `gap-96`
- `gap-x-*`, `gap-y-*`

### Special Values
- `auto`, `full`, `screen`, `min`, `max`, `fit`

---

**Next**: [06-sizing-system.md](./06-sizing-system.md) - Width and height utilities
