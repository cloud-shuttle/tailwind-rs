# Basic Usage Examples

This guide provides simple, practical examples to get you started with `tailwind-rs`.

## Prerequisites

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js and PNPM (for Playwright tests)
curl -fsSL https://get.pnpm.io/install.sh | sh
pnpm install
```

## Project Setup

### 1. Create New Project
```bash
cargo new my-tailwind-app
cd my-tailwind-app
```

### 2. Add Dependencies

**Cargo.toml**
```toml
[package]
name = "my-tailwind-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.6"
tailwind-rs = "0.1.0"
tailwind-rs-leptos = "0.1.0"

[build-dependencies]
tailwind-rs-cli = "0.1.0"
```

### 3. Create Build Script

**build.rs**
```rust
fn main() {
    tailwind_rs_cli::build()
        .input("src/**/*.rs")
        .output("dist/styles.css")
        .watch(true)
        .run()
        .expect("Failed to build Tailwind CSS");
}
```

### 4. Create Tailwind Configuration

**tailwind.toml**
```toml
[build]
input = ["src/**/*.rs"]
output = "dist/styles.css"
watch = true

[theme]
[theme.colors]
primary = "#3b82f6"
secondary = "#64748b"
```

## Basic Component Examples

### Simple Button

**src/components/button.rs**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    let button_classes = classes!(
        "bg-blue-500",
        "hover:bg-blue-700",
        "text-white",
        "font-bold",
        "py-2",
        "px-4",
        "rounded"
    );
    
    view! {
        <button class=button_classes>
            {children()}
        </button>
    }
}
```

### Card Component

**src/components/card.rs**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    let card_classes = classes!(
        "bg-white",
        "shadow-lg",
        "rounded-lg",
        "p-6",
        "max-w-sm"
    );
    
    view! {
        <div class=card_classes>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(children: Children) -> impl IntoView {
    let title_classes = classes!(
        "text-xl",
        "font-bold",
        "text-gray-800",
        "mb-4"
    );
    
    view! {
        <h2 class=title_classes>
            {children()}
        </h2>
    }
}

#[component]
pub fn CardContent(children: Children) -> impl IntoView {
    let content_classes = classes!(
        "text-gray-600",
        "leading-relaxed"
    );
    
    view! {
        <div class=content_classes>
            {children()}
        </div>
    }
}
```

### Input Component

**src/components/input.rs**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn Input(
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] input_type: Option<String>,
) -> impl IntoView {
    let input_classes = classes!(
        "w-full",
        "px-3",
        "py-2",
        "border",
        "border-gray-300",
        "rounded-md",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500",
        "focus:border-transparent"
    );
    
    view! {
        <input
            class=input_classes
            type=input_type.unwrap_or_else(|| "text".to_string())
            placeholder=placeholder.unwrap_or_else(|| "".to_string())
        />
    }
}
```

## Main Application

**src/main.rs**
```rust
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeFile;

mod components;

use components::{Button, Card, CardTitle, CardContent, Input};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100 py-8">
            <div class="container mx-auto px-4">
                <h1 class="text-3xl font-bold text-center mb-8 text-gray-800">
                    "Welcome to tailwind-rs"
                </h1>
                
                <div class="max-w-md mx-auto">
                    <Card>
                        <CardTitle>"Basic Example"</CardTitle>
                        <CardContent>
                            <p class="mb-4">
                                "This is a simple example of tailwind-rs in action."
                            </p>
                            
                            <div class="space-y-4">
                                <Input 
                                    placeholder="Enter your name"
                                    input_type="text"
                                />
                                
                                <Button>
                                    "Click me!"
                                </Button>
                            </div>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    }
}

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(ServeFile::new("dist/styles.css"))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
```

## Testing Examples

### Unit Tests

**src/components/button.rs** (continued)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_button_classes() {
        let button = Button(|| view! { "Test" });
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("py-2"));
        assert!(classes.contains("px-4"));
        assert!(classes.contains("rounded"));
    }
}
```

### Integration Tests

**tests/integration_tests.rs**
```rust
use my_tailwind_app::*;
use tailwind_rs::testing::*;

#[test]
fn test_app_rendering() {
    let app = create_test_app(|| view! { <App /> });
    let html = render_to_string(app);
    
    assert!(html.contains("Welcome to tailwind-rs"));
    assert!(html.contains("bg-gray-100"));
    assert!(html.contains("container mx-auto"));
}

#[test]
fn test_card_component() {
    let card = Card(|| view! { "Test content" });
    let html = render_to_string(card);
    
    assert!(html.contains("bg-white"));
    assert!(html.contains("shadow-lg"));
    assert!(html.contains("rounded-lg"));
    assert!(html.contains("p-6"));
}
```

### Playwright E2E Tests

**tests/basic-usage.spec.ts**
```typescript
import { test, expect } from '@playwright/test';

test('basic usage example renders correctly', async ({ page }) => {
  await page.goto('/');
  
  // Check page title
  const title = page.locator('h1');
  await expect(title).toHaveText('Welcome to tailwind-rs');
  
  // Check card component
  const card = page.locator('.bg-white.shadow-lg.rounded-lg');
  await expect(card).toBeVisible();
  
  // Check button component
  const button = page.locator('button');
  await expect(button).toBeVisible();
  await expect(button).toHaveText('Click me!');
  
  // Check input component
  const input = page.locator('input[type="text"]');
  await expect(input).toBeVisible();
  await expect(input).toHaveAttribute('placeholder', 'Enter your name');
});

test('button hover state works', async ({ page }) => {
  await page.goto('/');
  
  const button = page.locator('button');
  await button.hover();
  
  const styles = await button.evaluate((el) => {
    const computed = getComputedStyle(el);
    return {
      backgroundColor: computed.backgroundColor
    };
  });
  
  expect(styles.backgroundColor).toBe('rgb(37, 99, 235)'); // blue-700
});

test('input focus state works', async ({ page }) => {
  await page.goto('/');
  
  const input = page.locator('input[type="text"]');
  await input.focus();
  
  const styles = await input.evaluate((el) => {
    const computed = getComputedStyle(el);
    return {
      outline: computed.outline,
      boxShadow: computed.boxShadow
    };
  });
  
  expect(styles.outline).toBe('none');
  expect(styles.boxShadow).toContain('rgb(59, 130, 246)'); // blue-500 ring
});
```

## Running the Examples

### Development Mode
```bash
# Start the development server
cargo run

# In another terminal, start the CSS watcher
cargo run --bin tailwind-watch
```

### Production Build
```bash
# Build the application
cargo build --release

# Build CSS
cargo run --bin tailwind-build
```

### Running Tests
```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run Playwright tests
pnpm test:e2e
```

## Key Concepts Demonstrated

### 1. Type-Safe Class Generation
```rust
use tailwind_rs::classes;

let button_classes = classes!(
    "bg-blue-500",
    "hover:bg-blue-700",
    "text-white"
);
```

### 2. Component Composition
```rust
<Card>
    <CardTitle>"Title"</CardTitle>
    <CardContent>"Content"</CardContent>
</Card>
```

### 3. Conditional Styling
```rust
let classes = if is_active {
    classes!("bg-blue-500", "text-white")
} else {
    classes!("bg-gray-200", "text-gray-700")
};
```

### 4. Responsive Design
```rust
use tailwind_rs::responsive;

let responsive_classes = classes!(
    "grid",
    "grid-cols-1",
    responsive::md("grid-cols-2"),
    responsive::lg("grid-cols-3")
);
```

## Next Steps

1. 🎨 Explore [Button Components](./button-components.md)
2. 🧪 Learn [Testing Patterns](./unit-testing.md)
3. 🎯 Try [Framework-Specific Examples](./leptos-examples.md)
4. 🏗️ Build [Real-World Applications](./todo-app.md)

## Additional Resources

- [Getting Started Guide](../getting-started.md)
- [API Reference](../api/README.md)
- [Testing Guidelines](../testing.md)
- [Architecture Documentation](../architecture.md)

