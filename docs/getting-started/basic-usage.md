# Basic Usage Guide

This guide covers the fundamental concepts and usage patterns for `tailwind-rs`, following our Test-Driven Development approach and comprehensive testing strategy.

## 🎯 Core Concepts

### Type-Safe Class Generation
`tailwind-rs` provides type-safe class generation that prevents runtime errors and provides excellent IDE support.

```rust
use tailwind_rs::*;

// Basic class generation
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium",
    variant: "bg-blue-600 text-white hover:bg-blue-700",
    state: "focus:outline-none focus:ring-2 focus:ring-blue-500",
};

// The classes! macro validates all class names at compile time
// Invalid classes will cause compilation errors
```

### Dynamic Styling
Unlike traditional Tailwind integration, `tailwind-rs` supports dynamic class generation at runtime.

```rust
use tailwind_rs::*;

// Dynamic color generation
let color = Color::Blue;
let intensity = 600;

let classes = classes! {
    background: color.background(intensity),
    text: color.text(),
    hover: color.hover(700),
};

// This generates: "bg-blue-600 text-white hover:bg-blue-700"
```

## 🧪 TDD Development Example

Following our TDD approach, let's build a button component step by step.

### Step 1: Write Failing Test (Red)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_component() {
        // Given
        let button = Button::new()
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Medium);
        
        // When
        let classes = button.generate_classes();
        
        // Then
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
    }
}
```

### Step 2: Implement Minimal Code (Green)
```rust
use tailwind_rs::*;

#[derive(Debug, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

#[derive(Debug, Clone)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

pub struct Button {
    variant: ButtonVariant,
    size: ButtonSize,
}

impl Button {
    pub fn new() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
        }
    }
    
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
    
    pub fn generate_classes(&self) -> String {
        let mut classes = classes! {
            base: "rounded-md font-medium transition-colors duration-200",
        };
        
        // Add variant classes
        match self.variant {
            ButtonVariant::Primary => {
                classes = classes.variant("bg-blue-600 text-white hover:bg-blue-700");
            }
            ButtonVariant::Secondary => {
                classes = classes.variant("bg-gray-200 text-gray-900 hover:bg-gray-300");
            }
            ButtonVariant::Danger => {
                classes = classes.variant("bg-red-600 text-white hover:bg-red-700");
            }
        }
        
        // Add size classes
        match self.size {
            ButtonSize::Small => {
                classes = classes.variant("px-2 py-1 text-sm");
            }
            ButtonSize::Medium => {
                classes = classes.variant("px-4 py-2 text-base");
            }
            ButtonSize::Large => {
                classes = classes.variant("px-6 py-3 text-lg");
            }
        }
        
        classes.build()
    }
}
```

### Step 3: Refactor and Improve
```rust
impl Button {
    pub fn generate_classes(&self) -> String {
        classes! {
            base: "rounded-md font-medium transition-colors duration-200 focus:outline-none focus:ring-2",
            variant: self.variant.classes(),
            size: self.size.classes(),
            state: "focus:ring-blue-500",
        }.build()
    }
}

impl ButtonVariant {
    fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
            ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
            ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
        }
    }
}

impl ButtonSize {
    fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Small => "px-2 py-1 text-sm",
            ButtonSize::Medium => "px-4 py-2 text-base",
            ButtonSize::Large => "px-6 py-3 text-lg",
        }
    }
}
```

## 🎨 Framework Integration

### Leptos Integration
```rust
use leptos::*;
use tailwind_rs::*;

#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    children: Children,
) -> impl IntoView {
    let classes = classes! {
        base: "rounded-md font-medium transition-colors duration-200",
        variant: variant.classes(),
        size: size.classes(),
        state: "focus:outline-none focus:ring-2 focus:ring-blue-500",
    };
    
    view! {
        <button class=classes>
            {children()}
        </button>
    }
}

// Usage
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="p-6 space-y-4">
            <Button variant=ButtonVariant::Primary size=ButtonSize::Medium>
                "Primary Button"
            </Button>
            
            <Button variant=ButtonVariant::Secondary size=ButtonSize::Large>
                "Secondary Button"
            </Button>
            
            <Button variant=ButtonVariant::Danger size=ButtonSize::Small>
                "Danger Button"
            </Button>
        </div>
    }
}
```

### Yew Integration
```rust
use yew::prelude::*;
use tailwind_rs::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    pub children: Children,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let classes = classes! {
        base: "rounded-md font-medium transition-colors duration-200",
        variant: props.variant.classes(),
        size: props.size.classes(),
        state: "focus:outline-none focus:ring-2 focus:ring-blue-500",
    };
    
    html! {
        <button class={classes}>
            {props.children.clone()}
        </button>
    }
}
```

## 🎭 Responsive Design

### Responsive Classes
```rust
use tailwind_rs::*;

let classes = classes! {
    base: "text-sm",
    responsive: Responsive {
        sm: "text-base",
        md: "text-lg",
        lg: "text-xl",
        xl: "text-2xl",
    },
};

// Generates: "text-sm sm:text-base md:text-lg lg:text-xl xl:text-2xl"
```

### Breakpoint System
```rust
use tailwind_rs::*;

#[derive(Debug, Clone)]
pub struct ResponsiveClasses {
    pub mobile: String,
    pub tablet: String,
    pub desktop: String,
}

impl ResponsiveClasses {
    pub fn new() -> Self {
        Self {
            mobile: String::new(),
            tablet: String::new(),
            desktop: String::new(),
        }
    }
    
    pub fn mobile(mut self, classes: &str) -> Self {
        self.mobile = classes.to_string();
        self
    }
    
    pub fn tablet(mut self, classes: &str) -> Self {
        self.tablet = classes.to_string();
        self
    }
    
    pub fn desktop(mut self, classes: &str) -> Self {
        self.desktop = classes.to_string();
        self
    }
    
    pub fn build(self) -> String {
        let mut result = Vec::new();
        
        if !self.mobile.is_empty() {
            result.push(self.mobile);
        }
        
        if !self.tablet.is_empty() {
            result.push(format!("md:{}", self.tablet));
        }
        
        if !self.desktop.is_empty() {
            result.push(format!("lg:{}", self.desktop));
        }
        
        result.join(" ")
    }
}
```

## 🎨 Theme System

### Color System
```rust
use tailwind_rs::*;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
    Gray,
}

impl Color {
    pub fn background(&self, intensity: u8) -> String {
        match self {
            Color::Blue => format!("bg-blue-{}", intensity),
            Color::Green => format!("bg-green-{}", intensity),
            Color::Red => format!("bg-red-{}", intensity),
            Color::Yellow => format!("bg-yellow-{}", intensity),
            Color::Purple => format!("bg-purple-{}", intensity),
            Color::Gray => format!("bg-gray-{}", intensity),
        }
    }
    
    pub fn text(&self) -> &'static str {
        match self {
            Color::Blue | Color::Green | Color::Red | Color::Purple => "text-white",
            Color::Yellow => "text-black",
            Color::Gray => "text-gray-900",
        }
    }
    
    pub fn hover(&self, intensity: u8) -> String {
        match self {
            Color::Blue => format!("hover:bg-blue-{}", intensity),
            Color::Green => format!("hover:bg-green-{}", intensity),
            Color::Red => format!("hover:bg-red-{}", intensity),
            Color::Yellow => format!("hover:bg-yellow-{}", intensity),
            Color::Purple => format!("hover:bg-purple-{}", intensity),
            Color::Gray => format!("hover:bg-gray-{}", intensity),
        }
    }
}
```

### Theme Application
```rust
use tailwind_rs::*;

#[derive(Debug, Clone)]
pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            primary_color: Color::Blue,
            secondary_color: Color::Gray,
            accent_color: Color::Green,
        }
    }
    
    pub fn primary_color(mut self, color: Color) -> Self {
        self.primary_color = color;
        self
    }
    
    pub fn secondary_color(mut self, color: Color) -> Self {
        self.secondary_color = color;
        self
    }
    
    pub fn accent_color(mut self, color: Color) -> Self {
        self.accent_color = color;
        self
    }
    
    pub fn apply_to_button(&self, variant: ButtonVariant) -> String {
        match variant {
            ButtonVariant::Primary => {
                classes! {
                    base: "rounded-md font-medium transition-colors duration-200",
                    background: self.primary_color.background(600),
                    text: self.primary_color.text(),
                    hover: self.primary_color.hover(700),
                }.build()
            }
            ButtonVariant::Secondary => {
                classes! {
                    base: "rounded-md font-medium transition-colors duration-200",
                    background: self.secondary_color.background(200),
                    text: self.secondary_color.text(),
                    hover: self.secondary_color.hover(300),
                }.build()
            }
            ButtonVariant::Danger => {
                classes! {
                    base: "rounded-md font-medium transition-colors duration-200",
                    background: self.accent_color.background(600),
                    text: self.accent_color.text(),
                    hover: self.accent_color.hover(700),
                }.build()
            }
        }
    }
}
```

## 🧪 Testing Your Components

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_class_generation() {
        // Given
        let button = Button::new()
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Medium);
        
        // When
        let classes = button.generate_classes();
        
        // Then
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
    }

    #[test]
    fn test_theme_application() {
        // Given
        let theme = Theme::new()
            .primary_color(Color::Green)
            .secondary_color(Color::Purple);
        
        // When
        let primary_classes = theme.apply_to_button(ButtonVariant::Primary);
        let secondary_classes = theme.apply_to_button(ButtonVariant::Secondary);
        
        // Then
        assert!(primary_classes.contains("bg-green-600"));
        assert!(secondary_classes.contains("bg-purple-200"));
    }
}
```

### Integration Testing
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_component_integration() {
        // Given
        let theme = Theme::new();
        let button = Button::new()
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Large);
        
        // When
        let classes = button.generate_classes();
        let themed_classes = theme.apply_to_button(ButtonVariant::Primary);
        
        // Then
        assert!(!classes.is_empty());
        assert!(!themed_classes.is_empty());
        assert_ne!(classes, themed_classes);
    }
}
```

### End-to-End Testing with Playwright
```typescript
import { test, expect } from '@playwright/test';

test.describe('Button Component', () => {
  test('should render with correct classes', async ({ page }) => {
    await page.goto('/demo/button');
    
    // Test primary button
    await expect(page.locator('[data-testid="primary-button"]'))
      .toHaveClass(/bg-blue-600/);
    
    await expect(page.locator('[data-testid="primary-button"]'))
      .toHaveClass(/text-white/);
    
    // Test secondary button
    await expect(page.locator('[data-testid="secondary-button"]'))
      .toHaveClass(/bg-gray-200/);
    
    // Test danger button
    await expect(page.locator('[data-testid="danger-button"]'))
      .toHaveClass(/bg-red-600/);
  });

  test('should respond to user interactions', async ({ page }) => {
    await page.goto('/demo/button');
    
    // Test hover effects
    await page.hover('[data-testid="primary-button"]');
    await expect(page.locator('[data-testid="primary-button"]'))
      .toHaveClass(/hover:bg-blue-700/);
    
    // Test focus effects
    await page.focus('[data-testid="primary-button"]');
    await expect(page.locator('[data-testid="primary-button"]'))
      .toHaveClass(/focus:ring-2/);
  });
});
```

## 🚀 Performance Optimization

### Class Caching
```rust
use std::collections::HashMap;
use std::sync::Arc;

pub struct ClassCache {
    cache: Arc<HashMap<String, String>>,
}

impl ClassCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(HashMap::new()),
        }
    }
    
    pub fn get_or_generate<F>(&self, key: &str, generator: F) -> String
    where
        F: FnOnce() -> String,
    {
        if let Some(cached) = self.cache.get(key) {
            return cached.clone();
        }
        
        let generated = generator();
        // Note: In a real implementation, you'd need proper synchronization
        // This is simplified for demonstration
        generated
    }
}
```

### Lazy Class Generation
```rust
use std::sync::LazyLock;

static CLASS_CACHE: LazyLock<ClassCache> = LazyLock::new(|| {
    ClassCache::new()
});

pub fn get_cached_classes(key: &str, generator: impl FnOnce() -> String) -> String {
    CLASS_CACHE.get_or_generate(key, generator)
}
```

## 📚 Best Practices

### 1. Use Type-Safe Enums
```rust
// Good: Type-safe variants
#[derive(Debug, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

// Avoid: String-based variants
pub struct Button {
    variant: String, // This can lead to runtime errors
}
```

### 2. Compose Classes Logically
```rust
// Good: Logical composition
let classes = classes! {
    base: "rounded-md font-medium transition-colors",
    variant: variant.classes(),
    size: size.classes(),
    state: "focus:outline-none focus:ring-2",
};

// Avoid: String concatenation
let classes = format!("{} {} {}", base, variant, size);
```

### 3. Test All Variants
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_button_variants() {
        for variant in [ButtonVariant::Primary, ButtonVariant::Secondary, ButtonVariant::Danger] {
            let button = Button::new().variant(variant);
            let classes = button.generate_classes();
            
            assert!(!classes.is_empty());
            assert!(classes.contains("rounded-md"));
        }
    }
}
```

### 4. Use Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_class_generation_properties(
        variant in prop::sample::select(&[ButtonVariant::Primary, ButtonVariant::Secondary, ButtonVariant::Danger]),
        size in prop::sample::select(&[ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large])
    ) {
        let button = Button::new().variant(variant).size(size);
        let classes = button.generate_classes();
        
        // Properties that should always hold
        prop_assert!(!classes.is_empty());
        prop_assert!(classes.contains("rounded-md"));
        prop_assert!(classes.contains("font-medium"));
    }
}
```

## 🎯 Next Steps

Now that you understand the basics:

1. **Explore [Advanced Features](./advanced/dynamic-styling.md)**
2. **Check out [Framework Integration](./frameworks/leptos.md)**
3. **Learn about [Performance Optimization](./advanced/performance.md)**
4. **See [Example Projects](./examples/README.md)**

---

This guide demonstrates the core concepts of `tailwind-rs` following our TDD approach and comprehensive testing strategy. The type-safe, performant class generation system provides a solid foundation for building reliable Rust web applications.

