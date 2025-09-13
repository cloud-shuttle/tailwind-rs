# Leptos Integration Guide

This guide covers the complete integration of `tailwind-rs` with Leptos, following our Test-Driven Development approach and comprehensive testing strategy. Leptos is our primary framework, and we maintain the latest version support (ADR-006).

## 🎯 Overview

`tailwind-rs-leptos` provides seamless integration between `tailwind-rs` and the Leptos framework, offering:

- **Type-safe class generation** with compile-time validation
- **Reactive styling** that responds to Leptos signals
- **Component-based theming** with consistent design systems
- **Performance optimization** with intelligent caching
- **Comprehensive testing** with unit, integration, and E2E tests

## 🚀 Quick Start

### Installation
```toml
# Cargo.toml
[dependencies]
leptos = { version = "0.8.8", features = ["csr"] }
leptos_router = "0.8.8"
tailwind-rs = "0.1.0"
tailwind-rs-leptos = "0.1.0"

# Additional Leptos ecosystem crates (following ADR-009)
leptos-flow = "0.1.0"
leptos-forms-rs = "0.1.0"
leptos-helios = "0.1.0"
leptos-motion = "0.1.0"
leptos-query-rs = "0.1.0"
leptos-shadcn-ui = "0.1.0"
leptos-state = "0.1.0"
leptos-sync = "0.1.0"
leptos-ws-pro = "0.1.0"
leptos-next-metadata = "0.1.0"
radix-leptos = "0.1.0"
```

### Basic Usage
```rust
use leptos::*;
use tailwind_rs::*;
use tailwind_rs_leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let classes = classes! {
        base: "min-h-screen bg-gray-100",
        typography: "font-sans",
    };
    
    view! {
        <div class=classes>
            <Header />
            <Main />
            <Footer />
        </div>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-white shadow-sm border-b px-6 py-4">
            <h1 class="text-2xl font-bold text-gray-900">
                "My Leptos App"
            </h1>
        </header>
    }
}

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <main class="container mx-auto px-6 py-8">
            <Card />
        </main>
    }
}

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-md p-6 border border-gray-200">
            <h2 class="text-xl font-semibold text-gray-800 mb-4">
                "Welcome to tailwind-rs with Leptos!"
            </h2>
            <p class="text-gray-600 mb-6">
                "This component demonstrates the power of type-safe, 
                performant Tailwind CSS integration for Leptos."
            </p>
            <Button variant=ButtonVariant::Primary>
                "Get Started"
            </Button>
        </div>
    }
}

fn main() {
    mount_to_body(App)
}
```

## 🎨 Component Integration

### Button Component
```rust
use leptos::*;
use tailwind_rs::*;
use tailwind_rs_leptos::*;

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

#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] disabled: bool,
    children: Children,
) -> impl IntoView {
    let classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500",
        variant: match variant {
            ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
            ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
            ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
        },
        state: if disabled {
            "opacity-50 cursor-not-allowed"
        } else {
            "cursor-pointer"
        },
    };
    
    view! {
        <button class=classes disabled=disabled>
            {children()}
        </button>
    }
}
```

### Input Component
```rust
use leptos::*;
use tailwind_rs::*;
use tailwind_rs_leptos::*;

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

#[component]
pub fn Input(
    #[prop(optional)] variant: InputVariant,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let classes = classes! {
        base: "block w-full rounded-md border shadow-sm focus:outline-none focus:ring-1",
        variant: match variant {
            InputVariant::Default => "border-gray-300 focus:border-blue-500 focus:ring-blue-500",
            InputVariant::Error => "border-red-500 focus:border-red-500 focus:ring-red-500",
            InputVariant::Success => "border-green-500 focus:border-green-500 focus:ring-green-500",
        },
        state: if disabled {
            "bg-gray-50 text-gray-500 cursor-not-allowed"
        } else {
            "bg-white text-gray-900"
        },
    };
    
    view! {
        <input 
            class=classes
            placeholder=placeholder
            disabled=disabled
        />
    }
}
```

### Card Component
```rust
use leptos::*;
use tailwind_rs::*;
use tailwind_rs_leptos::*;

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

#[component]
pub fn Card(
    #[prop(optional)] variant: CardVariant,
    children: Children,
) -> impl IntoView {
    let classes = classes! {
        base: "rounded-lg p-6",
        variant: match variant {
            CardVariant::Default => "bg-white border border-gray-200",
            CardVariant::Elevated => "bg-white shadow-lg",
            CardVariant::Outlined => "bg-white border-2 border-gray-300",
        },
    };
    
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}
```

## 🎭 Reactive Styling

### Signal-Based Classes
```rust
use leptos::*;
use tailwind_rs::*;
use tailwind_rs_leptos::*;

#[component]
pub fn ReactiveButton() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (is_loading, set_loading) = create_signal(false);
    
    // Reactive classes based on signals
    let classes = move || {
        classes! {
            base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
            variant: if is_loading.get() {
                "bg-gray-400 text-gray-700 cursor-not-allowed"
            } else if count.get() > 5 {
                "bg-green-600 text-white hover:bg-green-700"
            } else {
                "bg-blue-600 text-white hover:bg-blue-700"
            },
            state: if is_loading.get() {
                "opacity-50"
            } else {
                "focus:outline-none focus:ring-2 focus:ring-blue-500"
            },
        }.build()
    };
    
    let handle_click = move |_| {
        if !is_loading.get() {
            set_loading.set(true);
            set_count.update(|c| *c += 1);
            
            // Simulate async operation
            set_timeout(move || {
                set_loading.set(false);
            }, 1000);
        }
    };
    
    view! {
        <button class=classes on:click=handle_click>
            {move || if is_loading.get() {
                "Loading..."
            } else {
                format!("Count: {}", count.get())
            }}
        </button>
    }
}
```

### Dynamic Theme Switching
```rust
use leptos::*;
use tailwind_rs::*;
use tailwind_rs_leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let (theme, set_theme) = create_signal(Theme::Light);
    
    // Provide theme context
    provide_context(theme);
    provide_context(set_theme);
    
    let container_classes = move || {
        classes! {
            base: "min-h-screen transition-colors duration-200",
            theme: match theme.get() {
                Theme::Light => "bg-white text-gray-900",
                Theme::Dark => "bg-gray-900 text-white",
            },
        }.build()
    };
    
    view! {
        <div class=container_classes>
            <ThemeToggle />
            {children()}
        </div>
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = use_context::<ReadSignal<Theme>>().unwrap();
    let set_theme = use_context::<WriteSignal<Theme>>().unwrap();
    
    let toggle_theme = move |_| {
        set_theme.update(|t| *t = match *t {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        });
    };
    
    let button_classes = move || {
        classes! {
            base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
            theme: match theme.get() {
                Theme::Light => "bg-gray-200 text-gray-900 hover:bg-gray-300",
                Theme::Dark => "bg-gray-700 text-white hover:bg-gray-600",
            },
        }.build()
    };
    
    view! {
        <button class=button_classes on:click=toggle_theme>
            {move || match theme.get() {
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
use leptos::*;
use tailwind_rs::*;
use tailwind_rs_leptos::*;

#[derive(Debug, Clone)]
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

#[component]
pub fn ThemedButton(
    #[prop(optional)] theme: Option<AppTheme>,
    children: Children,
) -> impl IntoView {
    let app_theme = theme.unwrap_or_default();
    
    let classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
        background: app_theme.primary_color.background(600),
        text: app_theme.primary_color.text(600),
        hover: app_theme.primary_color.hover(700),
        focus: "focus:outline-none focus:ring-2",
        ring: app_theme.primary_color.ring(500),
    };
    
    view! {
        <button class=classes>
            {children()}
        </button>
    }
}
```

### Responsive Design
```rust
use leptos::*;
use tailwind_rs::*;
use tailwind_rs_leptos::*;

#[component]
pub fn ResponsiveCard() -> impl IntoView {
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
    
    view! {
        <div class=classes>
            <h2 class="text-xl font-semibold text-gray-800 mb-4">
                "Responsive Card"
            </h2>
            <p class="text-gray-600">
                "This card adapts to different screen sizes with responsive padding and typography."
            </p>
        </div>
    }
}
```

## 🧪 Testing Leptos Components

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
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

test.describe('Leptos Integration', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/demo/leptos');
    await page.waitForLoadState('networkidle');
  });

  test('should render Leptos components correctly', async ({ page }) => {
    // Test header
    await expect(page.locator('header')).toBeVisible();
    await expect(page.locator('h1')).toContainText('My Leptos App');
    
    // Test main content
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('[data-testid="card"]')).toBeVisible();
    
    // Test button
    await expect(page.locator('[data-testid="button"]')).toBeVisible();
    await expect(page.locator('[data-testid="button"]'))
      .toHaveClass(/bg-blue-600/);
  });

  test('should handle reactive styling', async ({ page }) => {
    // Test reactive button
    await expect(page.locator('[data-testid="reactive-button"]'))
      .toContainText('Count: 0');
    
    // Click button
    await page.click('[data-testid="reactive-button"]');
    
    // Wait for loading state
    await expect(page.locator('[data-testid="reactive-button"]'))
      .toContainText('Loading...');
    
    // Wait for completion
    await expect(page.locator('[data-testid="reactive-button"]'))
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
use leptos::*;
use tailwind_rs::*;
use tailwind_rs_leptos::*;
use std::sync::LazyLock;

static CLASS_CACHE: LazyLock<ClassCache> = LazyLock::new(|| {
    ClassCache::new(1000)
});

#[component]
pub fn OptimizedButton(
    #[prop(optional)] variant: ButtonVariant,
    children: Children,
) -> impl IntoView {
    let classes = move || {
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
    };
    
    view! {
        <button class=classes>
            {children()}
        </button>
    }
}
```

### Memoized Components
```rust
use leptos::*;
use tailwind_rs::*;
use tailwind_rs_leptos::*;

#[component]
pub fn MemoizedCard(
    #[prop(optional)] title: String,
    #[prop(optional)] content: String,
) -> impl IntoView {
    let classes = classes! {
        base: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
    };
    
    view! {
        <div class=classes>
            <h2 class="text-xl font-semibold text-gray-800 mb-4">
                {title}
            </h2>
            <p class="text-gray-600">
                {content}
            </p>
        </div>
    }
}

// Use memoization for expensive components
#[component]
pub fn ExpensiveList(items: Vec<String>) -> impl IntoView {
    let memoized_items = create_memo(move |_| {
        items.iter().map(|item| {
            view! {
                <MemoizedCard 
                    title=item.clone()
                    content=format!("Content for {}", item)
                />
            }
        }).collect::<Vec<_>>()
    });
    
    view! {
        <div class="space-y-4">
            {memoized_items}
        </div>
    }
}
```

## 📚 Best Practices

### 1. Use Type-Safe Props
```rust
// Good: Type-safe props
#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    children: Children,
) -> impl IntoView {
    // Implementation
}

// Avoid: String-based props
#[component]
pub fn Button(
    #[prop(optional)] variant: String,
    #[prop(optional)] size: String,
    children: Children,
) -> impl IntoView {
    // Implementation
}
```

### 2. Compose Components Logically
```rust
// Good: Logical composition
#[component]
pub fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <Header />
            <Main />
            <Footer />
        </ThemeProvider>
    }
}

// Avoid: Flat structure
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Header />
        <Main />
        <Footer />
    }
}
```

### 3. Use Reactive Patterns
```rust
// Good: Reactive classes
let classes = move || {
    classes! {
        base: "px-4 py-2 rounded-md",
        variant: if is_loading.get() {
            "bg-gray-400 cursor-not-allowed"
        } else {
            "bg-blue-600 hover:bg-blue-700"
        },
    }.build()
};

// Avoid: Static classes
let classes = "px-4 py-2 rounded-md bg-blue-600 hover:bg-blue-700";
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

Now that you understand Leptos integration:

1. **Explore [Advanced Features](./advanced/dynamic-styling.md)**
2. **Check out [Performance Optimization](./advanced/performance.md)**
3. **See [Example Projects](./examples/README.md)**
4. **Learn about [Testing Strategies](./testing.md)**

---

This Leptos integration guide demonstrates how to build powerful, type-safe, and performant web applications using `tailwind-rs` with Leptos. The integration follows our established ADRs and best practices, ensuring reliable and maintainable code.

