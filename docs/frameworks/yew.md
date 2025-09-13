# Yew Integration Guide

This guide covers the complete integration of `tailwind-rs` with Yew, following our Test-Driven Development approach and comprehensive testing strategy.

## 🎯 Overview

`tailwind-rs-yew` provides seamless integration between `tailwind-rs` and the Yew framework, offering:

- **Type-safe class generation** with compile-time validation
- **Component-based styling** with consistent design systems
- **Performance optimization** with intelligent caching
- **Comprehensive testing** with unit, integration, and E2E tests

## 🚀 Quick Start

### Installation
```toml
# Cargo.toml
[dependencies]
yew = "0.21"
tailwind-rs = "0.1.0"
tailwind-rs-yew = "0.1.0"
wasm-bindgen = "0.2.101"
web-sys = "0.3.77"
```

### Basic Usage
```rust
use yew::prelude::*;
use tailwind_rs::*;
use tailwind_rs_yew::*;

#[function_component]
pub fn App() -> Html {
    let classes = classes! {
        base: "min-h-screen bg-gray-100",
        typography: "font-sans",
    };
    
    html! {
        <div class={classes}>
            <Header />
            <Main />
            <Footer />
        </div>
    }
}

#[function_component]
pub fn Header() -> Html {
    html! {
        <header class="bg-white shadow-sm border-b px-6 py-4">
            <h1 class="text-2xl font-bold text-gray-900">
                {"My Yew App"}
            </h1>
        </header>
    }
}

#[function_component]
pub fn Main() -> Html {
    html! {
        <main class="container mx-auto px-6 py-8">
            <Card />
        </main>
    }
}

#[function_component]
pub fn Card() -> Html {
    html! {
        <div class="bg-white rounded-lg shadow-md p-6 border border-gray-200">
            <h2 class="text-xl font-semibold text-gray-800 mb-4">
                {"Welcome to tailwind-rs with Yew!"}
            </h2>
            <p class="text-gray-600 mb-6">
                {"This component demonstrates the power of type-safe, 
                performant Tailwind CSS integration for Yew."}
            </p>
            <Button variant={ButtonVariant::Primary}>
                {"Get Started"}
            </Button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

## 🎨 Component Integration

### Button Component
```rust
use yew::prelude::*;
use tailwind_rs::*;
use tailwind_rs_yew::*;

#[derive(Debug, Clone, Copy, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub disabled: bool,
    pub children: Children,
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

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500",
        variant: match props.variant {
            ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
            ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
            ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
        },
        state: if props.disabled {
            "opacity-50 cursor-not-allowed"
        } else {
            "cursor-pointer"
        },
    };
    
    html! {
        <button class={classes} disabled={props.disabled}>
            {props.children.clone()}
        </button>
    }
}
```

### Input Component
```rust
use yew::prelude::*;
use tailwind_rs::*;
use tailwind_rs_yew::*;

#[derive(Debug, Clone, Copy, PartialEq, Properties)]
pub struct InputProps {
    #[prop_or_default]
    pub variant: InputVariant,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<Event>,
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

#[function_component]
pub fn Input(props: &InputProps) -> Html {
    let classes = classes! {
        base: "block w-full rounded-md border shadow-sm focus:outline-none focus:ring-1",
        variant: match props.variant {
            InputVariant::Default => "border-gray-300 focus:border-blue-500 focus:ring-blue-500",
            InputVariant::Error => "border-red-500 focus:border-red-500 focus:ring-red-500",
            InputVariant::Success => "border-green-500 focus:border-green-500 focus:ring-green-500",
        },
        state: if props.disabled {
            "bg-gray-50 text-gray-500 cursor-not-allowed"
        } else {
            "bg-white text-gray-900"
        },
    };
    
    html! {
        <input 
            class={classes}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            value={props.value.clone()}
            onchange={props.onchange.clone()}
        />
    }
}
```

### Card Component
```rust
use yew::prelude::*;
use tailwind_rs::*;
use tailwind_rs_yew::*;

#[derive(Debug, Clone, Copy, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub variant: CardVariant,
    pub children: Children,
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

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    let classes = classes! {
        base: "rounded-lg p-6",
        variant: match props.variant {
            CardVariant::Default => "bg-white border border-gray-200",
            CardVariant::Elevated => "bg-white shadow-lg",
            CardVariant::Outlined => "bg-white border-2 border-gray-300",
        },
    };
    
    html! {
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}
```

## 🎭 State Management

### Counter Component
```rust
use yew::prelude::*;
use tailwind_rs::*;
use tailwind_rs_yew::*;

#[function_component]
pub fn Counter() -> Html {
    let count = use_state(|| 0);
    let is_loading = use_state(|| false);
    
    let increment = {
        let count = count.clone();
        let is_loading = is_loading.clone();
        
        Callback::from(move |_| {
            if !*is_loading {
                is_loading.set(true);
                count.set(*count + 1);
                
                // Simulate async operation
                let is_loading = is_loading.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(1000).await;
                    is_loading.set(false);
                });
            }
        })
    };
    
    let classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
        variant: if *is_loading {
            "bg-gray-400 text-gray-700 cursor-not-allowed"
        } else if *count > 5 {
            "bg-green-600 text-white hover:bg-green-700"
        } else {
            "bg-blue-600 text-white hover:bg-blue-700"
        },
        state: if *is_loading {
            "opacity-50"
        } else {
            "focus:outline-none focus:ring-2 focus:ring-blue-500"
        },
    };
    
    html! {
        <button class={classes} onclick={increment}>
            {if *is_loading {
                "Loading..."
            } else {
                format!("Count: {}", *count)
            }}
        </button>
    }
}
```

### Theme Context
```rust
use yew::prelude::*;
use tailwind_rs::*;
use tailwind_rs_yew::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: Theme,
    pub set_theme: Callback<Theme>,
}

impl PartialEq for ThemeContext {
    fn eq(&self, other: &Self) -> bool {
        self.theme == other.theme
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ThemeProviderProps {
    pub children: Children,
}

#[function_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme = use_state(|| Theme::Light);
    
    let set_theme = {
        let theme = theme.clone();
        Callback::from(move |new_theme: Theme| {
            theme.set(new_theme);
        })
    };
    
    let context = ThemeContext {
        theme: (*theme).clone(),
        set_theme,
    };
    
    let container_classes = classes! {
        base: "min-h-screen transition-colors duration-200",
        theme: match *theme {
            Theme::Light => "bg-white text-gray-900",
            Theme::Dark => "bg-gray-900 text-white",
        },
    };
    
    html! {
        <ContextProvider<ThemeContext> context={context}>
            <div class={container_classes}>
                <ThemeToggle />
                {props.children.clone()}
            </div>
        </ContextProvider<ThemeContext>>
    }
}

#[function_component]
pub fn ThemeToggle() -> Html {
    let context = use_context::<ThemeContext>().unwrap();
    
    let toggle_theme = {
        let set_theme = context.set_theme.clone();
        Callback::from(move |_| {
            let new_theme = match context.theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
            set_theme.emit(new_theme);
        })
    };
    
    let button_classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
        theme: match context.theme {
            Theme::Light => "bg-gray-200 text-gray-900 hover:bg-gray-300",
            Theme::Dark => "bg-gray-700 text-white hover:bg-gray-600",
        },
    };
    
    html! {
        <button class={button_classes} onclick={toggle_theme}>
            {match context.theme {
                Theme::Light => "Switch to Dark",
                Theme::Dark => "Switch to Light",
            }}
        </button>
    }
}
```

## 🎨 Advanced Theming

### Component Theming
```rust
use yew::prelude::*;
use tailwind_rs::*;
use tailwind_rs_yew::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ThemedButtonProps {
    #[prop_or_default]
    pub theme: Option<AppTheme>,
    pub children: Children,
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

#[function_component]
pub fn ThemedButton(props: &ThemedButtonProps) -> Html {
    let app_theme = props.theme.clone().unwrap_or_default();
    
    let classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
        background: app_theme.primary_color.background(600),
        text: app_theme.primary_color.text(600),
        hover: app_theme.primary_color.hover(700),
        focus: "focus:outline-none focus:ring-2",
        ring: app_theme.primary_color.ring(500),
    };
    
    html! {
        <button class={classes}>
            {props.children.clone()}
        </button>
    }
}
```

### Responsive Design
```rust
use yew::prelude::*;
use tailwind_rs::*;
use tailwind_rs_yew::*;

#[function_component]
pub fn ResponsiveCard() -> Html {
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
    
    html! {
        <div class={classes}>
            <h2 class="text-xl font-semibold text-gray-800 mb-4">
                {"Responsive Card"}
            </h2>
            <p class="text-gray-600">
                {"This card adapts to different screen sizes with responsive padding and typography."}
            </p>
        </div>
    }
}
```

## 🧪 Testing Yew Components

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use yew::prelude::*;
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

test.describe('Yew Integration', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/demo/yew');
    await page.waitForLoadState('networkidle');
  });

  test('should render Yew components correctly', async ({ page }) => {
    // Test header
    await expect(page.locator('header')).toBeVisible();
    await expect(page.locator('h1')).toContainText('My Yew App');
    
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
use yew::prelude::*;
use tailwind_rs::*;
use tailwind_rs_yew::*;
use std::sync::LazyLock;

static CLASS_CACHE: LazyLock<ClassCache> = LazyLock::new(|| {
    ClassCache::new(1000)
});

#[derive(Debug, Clone, Copy, PartialEq, Properties)]
pub struct OptimizedButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    pub children: Children,
}

#[function_component]
pub fn OptimizedButton(props: &OptimizedButtonProps) -> Html {
    let classes = use_memo(
        |variant| {
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
        },
        props.variant,
    );
    
    html! {
        <button class={(*classes).clone()}>
            {props.children.clone()}
        </button>
    }
}
```

### Memoized Components
```rust
use yew::prelude::*;
use tailwind_rs::*;
use tailwind_rs_yew::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MemoizedCardProps {
    pub title: String,
    pub content: String,
}

#[function_component]
pub fn MemoizedCard(props: &MemoizedCardProps) -> Html {
    let classes = classes! {
        base: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
    };
    
    html! {
        <div class={classes}>
            <h2 class="text-xl font-semibold text-gray-800 mb-4">
                {props.title.clone()}
            </h2>
            <p class="text-gray-600">
                {props.content.clone()}
            </p>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ExpensiveListProps {
    pub items: Vec<String>,
}

#[function_component]
pub fn ExpensiveList(props: &ExpensiveListProps) -> Html {
    let memoized_items = use_memo(
        |items| {
            items.iter().map(|item| {
                html! {
                    <MemoizedCard 
                        title={item.clone()}
                        content={format!("Content for {}", item)}
                    />
                }
            }).collect::<Vec<_>>()
        },
        props.items.clone(),
    );
    
    html! {
        <div class="space-y-4">
            {(*memoized_items).clone()}
        </div>
    }
}
```

## 📚 Best Practices

### 1. Use Type-Safe Props
```rust
// Good: Type-safe props
#[derive(Debug, Clone, Copy, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    pub children: Children,
}

// Avoid: String-based props
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: String,
    #[prop_or_default]
    pub size: String,
    pub children: Children,
}
```

### 2. Compose Components Logically
```rust
// Good: Logical composition
#[function_component]
pub fn App() -> Html {
    html! {
        <ThemeProvider>
            <Header />
            <Main />
            <Footer />
        </ThemeProvider>
    }
}

// Avoid: Flat structure
#[function_component]
pub fn App() -> Html {
    html! {
        <Header />
        <Main />
        <Footer />
    }
}
```

### 3. Use State Management Patterns
```rust
// Good: Proper state management
#[function_component]
pub fn Counter() -> Html {
    let count = use_state(|| 0);
    let is_loading = use_state(|| false);
    
    let increment = {
        let count = count.clone();
        let is_loading = is_loading.clone();
        
        Callback::from(move |_| {
            if !*is_loading {
                is_loading.set(true);
                count.set(*count + 1);
                
                // Handle async operations
                let is_loading = is_loading.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(1000).await;
                    is_loading.set(false);
                });
            }
        })
    };
    
    // Implementation
}

// Avoid: Direct state manipulation
#[function_component]
pub fn Counter() -> Html {
    let count = use_state(|| 0);
    
    let increment = {
        let count = count.clone();
        Callback::from(move |_| {
            count.set(*count + 1); // No loading state or error handling
        })
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

Now that you understand Yew integration:

1. **Explore [Advanced Features](./advanced/dynamic-styling.md)**
2. **Check out [Performance Optimization](./advanced/performance.md)**
3. **See [Example Projects](./examples/README.md)**
4. **Learn about [Testing Strategies](./testing.md)**

---

This Yew integration guide demonstrates how to build powerful, type-safe, and performant web applications using `tailwind-rs` with Yew. The integration follows our established ADRs and best practices, ensuring reliable and maintainable code.

