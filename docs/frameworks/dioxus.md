# Dioxus Integration Guide

This guide covers the complete integration of `tailwind-rs` with Dioxus, following our Test-Driven Development approach and comprehensive testing strategy.

## 🎯 Overview

`tailwind-rs-dioxus` provides seamless integration between `tailwind-rs` and the Dioxus framework, offering:

- **Type-safe class generation** with compile-time validation
- **Component-based styling** with consistent design systems
- **Performance optimization** with intelligent caching
- **Comprehensive testing** with unit, integration, and E2E tests

## 🚀 Quick Start

### Installation
```toml
# Cargo.toml
[dependencies]
dioxus = "0.4"
tailwind-rs = "0.1.0"
tailwind-rs-dioxus = "0.1.0"
```

### Basic Usage
```rust
use dioxus::prelude::*;
use tailwind_rs::*;
use tailwind_rs_dioxus::*;

fn App(cx: Scope) -> Element {
    let classes = classes! {
        base: "min-h-screen bg-gray-100",
        typography: "font-sans",
    };
    
    cx.render(rsx! {
        div { class: classes,
            Header {}
            Main {}
            Footer {}
        }
    })
}

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header { class: "bg-white shadow-sm border-b px-6 py-4",
            h1 { class: "text-2xl font-bold text-gray-900",
                "My Dioxus App"
            }
        }
    })
}

fn Main(cx: Scope) -> Element {
    cx.render(rsx! {
        main { class: "container mx-auto px-6 py-8",
            Card {}
        }
    })
}

fn Card(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                "Welcome to tailwind-rs with Dioxus!"
            }
            p { class: "text-gray-600 mb-6",
                "This component demonstrates the power of type-safe, 
                performant Tailwind CSS integration for Dioxus."
            }
            Button { variant: ButtonVariant::Primary,
                "Get Started"
            }
        }
    })
}

fn main() {
    dioxus::launch(App);
}
```

## 🎨 Component Integration

### Button Component
```rust
use dioxus::prelude::*;
use tailwind_rs::*;
use tailwind_rs_dioxus::*;

#[derive(Debug, Clone, Copy, PartialEq, Props)]
pub struct ButtonProps {
    #[prop(default)]
    pub variant: ButtonVariant,
    #[prop(default)]
    pub disabled: bool,
    pub children: Element,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

fn Button(cx: Scope<ButtonProps>) -> Element {
    let classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500",
        variant: match cx.props.variant {
            ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
            ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
            ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
        },
        state: if cx.props.disabled {
            "opacity-50 cursor-not-allowed"
        } else {
            "cursor-pointer"
        },
    };
    
    cx.render(rsx! {
        button { 
            class: classes,
            disabled: cx.props.disabled,
            {cx.props.children}
        }
    })
}
```

### Input Component
```rust
use dioxus::prelude::*;
use tailwind_rs::*;
use tailwind_rs_dioxus::*;

#[derive(Debug, Clone, Copy, PartialEq, Props)]
pub struct InputProps {
    #[prop(default)]
    pub variant: InputVariant,
    #[prop(default)]
    pub placeholder: String,
    #[prop(default)]
    pub disabled: bool,
    #[prop(default)]
    pub value: String,
    #[prop(default)]
    pub onchange: EventHandler<FormEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputVariant {
    Default,
    Error,
    Success,
}

impl Default for InputVariant {
    fn default() -> Self {
        Self::Default
    }
}

fn Input(cx: Scope<InputProps>) -> Element {
    let classes = classes! {
        base: "block w-full rounded-md border shadow-sm focus:outline-none focus:ring-1",
        variant: match cx.props.variant {
            InputVariant::Default => "border-gray-300 focus:border-blue-500 focus:ring-blue-500",
            InputVariant::Error => "border-red-500 focus:border-red-500 focus:ring-red-500",
            InputVariant::Success => "border-green-500 focus:border-green-500 focus:ring-green-500",
        },
        state: if cx.props.disabled {
            "bg-gray-50 text-gray-500 cursor-not-allowed"
        } else {
            "bg-white text-gray-900"
        },
    };
    
    cx.render(rsx! {
        input { 
            class: classes,
            placeholder: cx.props.placeholder.clone(),
            disabled: cx.props.disabled,
            value: cx.props.value.clone(),
            onchange: cx.props.onchange,
        }
    })
}
```

### Card Component
```rust
use dioxus::prelude::*;
use tailwind_rs::*;
use tailwind_rs_dioxus::*;

#[derive(Debug, Clone, Copy, PartialEq, Props)]
pub struct CardProps {
    #[prop(default)]
    pub variant: CardVariant,
    pub children: Element,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardVariant {
    Default,
    Elevated,
    Outlined,
}

impl Default for CardVariant {
    fn default() -> Self {
        Self::Default
    }
}

fn Card(cx: Scope<CardProps>) -> Element {
    let classes = classes! {
        base: "rounded-lg p-6",
        variant: match cx.props.variant {
            CardVariant::Default => "bg-white border border-gray-200",
            CardVariant::Elevated => "bg-white shadow-lg",
            CardVariant::Outlined => "bg-white border-2 border-gray-300",
        },
    };
    
    cx.render(rsx! {
        div { class: classes,
            {cx.props.children}
        }
    })
}
```

## 🎭 State Management

### Counter Component
```rust
use dioxus::prelude::*;
use tailwind_rs::*;
use tailwind_rs_dioxus::*;

fn Counter(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    let is_loading = use_state(cx, || false);
    
    let increment = move |_| {
        if !*is_loading.get() {
            is_loading.set(true);
            count.set(*count.get() + 1);
            
            // Simulate async operation
            let is_loading = is_loading.clone();
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                is_loading.set(false);
            });
        }
    };
    
    let classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
        variant: if *is_loading.get() {
            "bg-gray-400 text-gray-700 cursor-not-allowed"
        } else if *count.get() > 5 {
            "bg-green-600 text-white hover:bg-green-700"
        } else {
            "bg-blue-600 text-white hover:bg-blue-700"
        },
        state: if *is_loading.get() {
            "opacity-50"
        } else {
            "focus:outline-none focus:ring-2 focus:ring-blue-500"
        },
    };
    
    cx.render(rsx! {
        button { 
            class: classes,
            onclick: increment,
            if *is_loading.get() {
                "Loading..."
            } else {
                format!("Count: {}", *count.get())
            }
        }
    })
}
```

### Theme Context
```rust
use dioxus::prelude::*;
use tailwind_rs::*;
use tailwind_rs_dioxus::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: Theme,
    pub set_theme: fn(Theme),
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ThemeProviderProps {
    pub children: Element,
}

fn ThemeProvider(cx: Scope<ThemeProviderProps>) -> Element {
    let theme = use_state(cx, || Theme::Light);
    
    let set_theme = move |new_theme: Theme| {
        theme.set(new_theme);
    };
    
    let context = ThemeContext {
        theme: *theme.get(),
        set_theme,
    };
    
    let container_classes = classes! {
        base: "min-h-screen transition-colors duration-200",
        theme: match *theme.get() {
            Theme::Light => "bg-white text-gray-900",
            Theme::Dark => "bg-gray-900 text-white",
        },
    };
    
    cx.render(rsx! {
        div { class: container_classes,
            ThemeToggle {}
            {cx.props.children}
        }
    })
}

fn ThemeToggle(cx: Scope) -> Element {
    let theme = use_state(cx, || Theme::Light);
    
    let toggle_theme = move |_| {
        theme.set(match *theme.get() {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        });
    };
    
    let button_classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
        theme: match *theme.get() {
            Theme::Light => "bg-gray-200 text-gray-900 hover:bg-gray-300",
            Theme::Dark => "bg-gray-700 text-white hover:bg-gray-600",
        },
    };
    
    cx.render(rsx! {
        button { 
            class: button_classes,
            onclick: toggle_theme,
            match *theme.get() {
                Theme::Light => "Switch to Dark",
                Theme::Dark => "Switch to Light",
            }
        }
    })
}
```

## 🎨 Advanced Theming

### Component Theming
```rust
use dioxus::prelude::*;
use tailwind_rs::*;
use tailwind_rs_dioxus::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ThemedButtonProps {
    #[prop(default)]
    pub theme: Option<AppTheme>,
    pub children: Element,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppTheme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self {
            primary_color: Color::Blue,
            secondary_color: Color::Gray,
            accent_color: Color::Green,
        }
    }
}

fn ThemedButton(cx: Scope<ThemedButtonProps>) -> Element {
    let app_theme = cx.props.theme.clone().unwrap_or_default();
    
    let classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
        background: app_theme.primary_color.background(600),
        text: app_theme.primary_color.text(600),
        hover: app_theme.primary_color.hover(700),
        focus: "focus:outline-none focus:ring-2",
        ring: app_theme.primary_color.ring(500),
    };
    
    cx.render(rsx! {
        button { class: classes,
            {cx.props.children}
        }
    })
}
```

### Responsive Design
```rust
use dioxus::prelude::*;
use tailwind_rs::*;
use tailwind_rs_dioxus::*;

fn ResponsiveCard(cx: Scope) -> Element {
    let classes = classes! {
        base: "bg-white rounded-lg shadow-md p-4",
        responsive: Responsive {
            sm: "p-6",
            md: "p-8",
            lg: "p-10",
        },
        typography: Responsive {
            sm: "text-sm",
            md: "text-base",
            lg: "text-lg",
        },
    };
    
    cx.render(rsx! {
        div { class: classes,
            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                "Responsive Card"
            }
            p { class: "text-gray-600",
                "This card adapts to different screen sizes with responsive padding and typography."
            }
        }
    })
}
```

## 🧪 Testing Dioxus Components

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;
    use tailwind_rs::*;

    #[test]
    fn test_button_component() {
        let button = Button::new()
            .variant(ButtonVariant::Primary)
            .disabled(false);
        
        let classes = button.generate_classes();
        
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
    }

    #[test]
    fn test_input_component() {
        let input = Input::new()
            .variant(InputVariant::Default)
            .placeholder("Enter text")
            .disabled(false);
        
        let classes = input.generate_classes();
        
        assert!(classes.contains("border-gray-300"));
        assert!(classes.contains("focus:border-blue-500"));
        assert!(classes.contains("bg-white"));
    }

    #[test]
    fn test_card_component() {
        let card = Card::new()
            .variant(CardVariant::Elevated);
        
        let classes = card.generate_classes();
        
        assert!(classes.contains("bg-white"));
        assert!(classes.contains("shadow-lg"));
        assert!(classes.contains("rounded-lg"));
        assert!(classes.contains("p-6"));
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
        let app = App::new();
        let header = Header::new();
        let main = Main::new();
        let card = Card::new();
        
        // Test component hierarchy
        assert!(app.contains(&header));
        assert!(app.contains(&main));
        assert!(main.contains(&card));
        
        // Test class generation
        let app_classes = app.generate_classes();
        let header_classes = header.generate_classes();
        let main_classes = main.generate_classes();
        let card_classes = card.generate_classes();
        
        assert!(!app_classes.is_empty());
        assert!(!header_classes.is_empty());
        assert!(!main_classes.is_empty());
        assert!(!card_classes.is_empty());
    }

    #[test]
    fn test_theme_integration() {
        let theme = AppTheme::default();
        let button = ThemedButton::new().theme(Some(theme.clone()));
        
        let classes = button.generate_classes();
        
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("hover:bg-blue-700"));
    }
}
```

### End-to-End Testing with Playwright
```typescript
import { test, expect } from '@playwright/test';

test.describe('Dioxus Integration', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/demo/dioxus');
    await page.waitForLoadState('networkidle');
  });

  test('should render Dioxus components correctly', async ({ page }) => {
    // Test header
    await expect(page.locator('header')).toBeVisible();
    await expect(page.locator('h1')).toContainText('My Dioxus App');
    
    // Test main content
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('[data-testid="card"]')).toBeVisible();
    
    // Test button
    await expect(page.locator('[data-testid="button"]')).toBeVisible();
    await expect(page.locator('[data-testid="button"]'))
      .toHaveClass(/bg-blue-600/);
  });

  test('should handle state management', async ({ page }) => {
    // Test counter
    await expect(page.locator('[data-testid="counter"]'))
      .toContainText('Count: 0');
    
    // Click counter
    await page.click('[data-testid="counter"]');
    
    // Wait for loading state
    await expect(page.locator('[data-testid="counter"]'))
      .toContainText('Loading...');
    
    // Wait for completion
    await expect(page.locator('[data-testid="counter"]'))
      .toContainText('Count: 1', { timeout: 2000 });
  });

  test('should handle theme switching', async ({ page }) => {
    // Test light theme
    await expect(page.locator('body')).toHaveClass(/bg-white/);
    
    // Switch to dark theme
    await page.click('[data-testid="theme-toggle"]');
    
    // Test dark theme
    await expect(page.locator('body')).toHaveClass(/bg-gray-900/);
    
    // Switch back to light theme
    await page.click('[data-testid="theme-toggle"]');
    
    // Test light theme again
    await expect(page.locator('body')).toHaveClass(/bg-white/);
  });

  test('should be responsive', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('[data-testid="responsive-card"]'))
      .toHaveClass(/p-4/);
    
    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator('[data-testid="responsive-card"]'))
      .toHaveClass(/md:p-6/);
    
    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('[data-testid="responsive-card"]'))
      .toHaveClass(/lg:p-8/);
  });
});
```

## 🚀 Performance Optimization

### Class Caching
```rust
use dioxus::prelude::*;
use tailwind_rs::*;
use tailwind_rs_dioxus::*;
use std::sync::LazyLock;

static CLASS_CACHE: LazyLock<ClassCache> = LazyLock::new(|| {
    ClassCache::new(1000)
});

#[derive(Debug, Clone, Copy, PartialEq, Props)]
pub struct OptimizedButtonProps {
    #[prop(default)]
    pub variant: ButtonVariant,
    pub children: Element,
}

fn OptimizedButton(cx: Scope<OptimizedButtonProps>) -> Element {
    let classes = use_memo(cx, |variant| {
        let cache_key = format!("button-{:?}", variant);
        
        CLASS_CACHE.get_or_generate(&cache_key, || {
            classes! {
                base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
                variant: match variant {
                    ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
                    ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
                    ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
                },
            }.build()
        })
    }, cx.props.variant);
    
    cx.render(rsx! {
        button { class: classes,
            {cx.props.children}
        }
    })
}
```

### Memoized Components
```rust
use dioxus::prelude::*;
use tailwind_rs::*;
use tailwind_rs_dioxus::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MemoizedCardProps {
    pub title: String,
    pub content: String,
}

fn MemoizedCard(cx: Scope<MemoizedCardProps>) -> Element {
    let classes = classes! {
        base: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
    };
    
    cx.render(rsx! {
        div { class: classes,
            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                {cx.props.title.clone()}
            }
            p { class: "text-gray-600",
                {cx.props.content.clone()}
            }
        }
    })
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ExpensiveListProps {
    pub items: Vec<String>,
}

fn ExpensiveList(cx: Scope<ExpensiveListProps>) -> Element {
    let memoized_items = use_memo(cx, |items| {
        items.iter().map(|item| {
            rsx! {
                MemoizedCard { 
                    title: item.clone(),
                    content: format!("Content for {}", item),
                }
            }
        }).collect::<Vec<_>>()
    }, cx.props.items.clone());
    
    cx.render(rsx! {
        div { class: "space-y-4",
            {memoized_items}
        }
    })
}
```

## 📚 Best Practices

### 1. Use Type-Safe Props
```rust
// Good: Type-safe props
#[derive(Debug, Clone, Copy, PartialEq, Props)]
pub struct ButtonProps {
    #[prop(default)]
    pub variant: ButtonVariant,
    #[prop(default)]
    pub size: ButtonSize,
    pub children: Element,
}

// Avoid: String-based props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ButtonProps {
    #[prop(default)]
    pub variant: String,
    #[prop(default)]
    pub size: String,
    pub children: Element,
}
```

### 2. Compose Components Logically
```rust
// Good: Logical composition
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        ThemeProvider {
            Header {}
            Main {}
            Footer {}
        }
    })
}

// Avoid: Flat structure
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Header {}
        Main {}
        Footer {}
    })
}
```

### 3. Use State Management Patterns
```rust
// Good: Proper state management
fn Counter(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    let is_loading = use_state(cx, || false);
    
    let increment = move |_| {
        if !*is_loading.get() {
            is_loading.set(true);
            count.set(*count.get() + 1);
            
            // Handle async operations
            let is_loading = is_loading.clone();
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                is_loading.set(false);
            });
        }
    };
    
    // Implementation
}

// Avoid: Direct state manipulation
fn Counter(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    
    let increment = move |_| {
        count.set(*count.get() + 1); // No loading state or error handling
    };
    
    // Implementation
}
```

### 4. Test All Variants
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
            assert!(classes.contains("px-4"));
            assert!(classes.contains("py-2"));
        }
    }
}
```

## 🎯 Next Steps

Now that you understand Dioxus integration:

1. **Explore [Advanced Features](./advanced/dynamic-styling.md)**
2. **Check out [Performance Optimization](./advanced/performance.md)**
3. **See [Example Projects](./examples/README.md)**
4. **Learn about [Testing Strategies](./testing.md)**

---

This Dioxus integration guide demonstrates how to build powerful, type-safe, and performant web applications using `tailwind-rs` with Dioxus. The integration follows our established ADRs and best practices, ensuring reliable and maintainable code.
