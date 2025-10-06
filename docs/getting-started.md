# 🚀 **Getting Started with tailwind-rs**

A comprehensive guide to getting started with the modern Tailwind CSS engine for Rust.

---

## 📦 **Installation**

Add `tailwind-rs` to your `Cargo.toml`:

```toml
[dependencies]
tailwind-rs-core = "0.15"
# For framework-specific integrations:
tailwind-rs-leptos = "0.15"  # For Leptos
tailwind-rs-dioxus = "0.15"  # For Dioxus
tailwind-rs-yew = "0.15"     # For Yew
```

---

## 🏁 **Quick Start**

### Basic CSS Generation

```rust
use tailwind_rs_core::CssGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a CSS generator
    let mut generator = CssGenerator::new();

    // Generate CSS for classes
    let css = generator.process_element_classes(&[
        "bg-blue-500",
        "text-white",
        "p-4",
        "rounded-lg"
    ])?;

    println!("{}", css);

    Ok(())
}
```

**Output:**
```css
.bg-blue-500 {
  background-color: rgb(59, 130, 246);
}
.text-white {
  color: rgb(255, 255, 255);
}
.p-4 {
  padding: 1rem;
}
.rounded-lg {
  border-radius: 0.5rem;
}
```

---

## 🎯 **Core Concepts**

### CSS Generation Architecture

`tailwind-rs` follows a **"One Class = One CSS Rule"** principle:

```rust
// Each class generates exactly one CSS rule
let css = generator.process_element_classes(&["bg-blue-500"])?;
// → .bg-blue-500 { background-color: rgb(59, 130, 246); }
```

### Element-Based Processing

Process multiple classes for a single HTML element:

```rust
let css = generator.process_element_classes(&[
    "flex",           // Display
    "items-center",   // Alignment
    "justify-center", // Justification
    "p-4",           // Spacing
    "bg-blue-500",   // Background
    "text-white",    // Text color
    "rounded-lg"     // Border radius
])?;
```

---

## 🎨 **Advanced Features**

### Container Queries

Build responsive designs based on container size:

```rust
let css = generator.process_element_classes(&[
    "bg-green-500",                    // Base style
    "@container-sm:bg-blue-500",       // Small containers
    "@container-md:min-w-300:bg-red-500" // Medium containers with custom condition
])?;
```

### Arbitrary Values

Use dynamic values with arbitrary value syntax:

```rust
let css = generator.process_element_classes(&[
    "[data-state=\"open\"]:bg-green-500",     // Data attributes
    "[aria-expanded=\"true\"]:text-blue-600", // ARIA attributes
    "top-[100px]",                           // Custom positioning
    "w-[50%]"                               // Custom sizing
])?;
```

### Device-Specific Variants

Target specific device capabilities:

```rust
let css = generator.process_element_classes(&[
    "motion-reduce:opacity-50",      // Reduced motion users
    "contrast-more:text-lg",         // High contrast preference
    "pointer-coarse:p-6",           // Touch devices
    "orientation-landscape:flex-row" // Landscape orientation
])?;
```

### Transform Combinations

Combine multiple transforms using CSS custom properties:

```rust
let css = generator.process_element_classes(&[
    "scale-110",      // Scale up
    "rotate-3",       // Rotate 3 degrees
    "translate-x-2",  // Move right
    "transform"       // Apply all transforms
])?;
```

---

## 🔧 **Framework Integrations**

### Leptos Integration

```rust
use leptos::*;
use tailwind_rs_leptos::tailwind_classes;

#[component]
pub fn MyButton(cx: Scope) -> impl IntoView {
    // Generate classes at compile time
    let button_classes = tailwind_classes!(
        "bg-blue-500",
        "hover:bg-blue-600",
        "text-white",
        "font-bold",
        "py-2",
        "px-4",
        "rounded",
        "transition-colors"
    );

    view! { cx,
        <button class=button_classes>
            "Click me!"
        </button>
    }
}
```

### Dioxus Integration

```rust
use dioxus::prelude::*;
use tailwind_rs_dioxus::tw;

fn MyComponent() -> Element {
    // Use the tw! macro for dynamic class generation
    let card_classes = tw!(
        "bg-white",
        "shadow-lg",
        "rounded-lg",
        "p-6",
        "max-w-sm",
        "mx-auto"
    );

    rsx! {
        div { class: card_classes,
            h2 { class: "text-xl font-bold mb-2",
                "Card Title"
            }
            p { class: "text-gray-600",
                "This is a beautiful card component."
            }
        }
    }
}
```

### Yew Integration

```rust
use yew::prelude::*;
use tailwind_rs_yew::classes;

pub struct Card;

impl Component for Card {
    type Message = ();
    type Properties = ();

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Generate classes with runtime processing
        let card_classes = classes!(
            "bg-white",
            "shadow-lg",
            "rounded-lg",
            "p-6",
            "max-w-sm"
        );

        html! {
            <div class={card_classes}>
                <h2 class="text-xl font-bold mb-2">
                    {"Card Title"}
                </h2>
                <p class="text-gray-600">
                    {"This is a beautiful card component."}
                </p>
            </div>
        }
    }
}
```

---

## 🎨 **Styling Patterns**

### Component-Based Styling

Create reusable component styles:

```rust
fn create_button_styles(variant: &str) -> Vec<&str> {
    match variant {
        "primary" => vec![
            "bg-blue-500", "hover:bg-blue-600", "text-white",
            "font-medium", "py-2", "px-4", "rounded-md",
            "transition-colors", "duration-200"
        ],
        "secondary" => vec![
            "bg-gray-200", "hover:bg-gray-300", "text-gray-800",
            "font-medium", "py-2", "px-4", "rounded-md",
            "transition-colors", "duration-200"
        ],
        _ => vec!["bg-gray-500", "text-white", "py-2", "px-4", "rounded"]
    }
}

let primary_button_css = generator.process_element_classes(&create_button_styles("primary"))?;
```

### Responsive Design

Build responsive layouts with breakpoints:

```rust
let responsive_css = generator.process_element_classes(&[
    // Mobile-first approach
    "flex-col",              // Mobile: column layout
    "sm:flex-row",          // Small screens: row layout
    "md:justify-between",    // Medium screens: space between
    "lg:gap-8",             // Large screens: larger gaps

    // Responsive spacing
    "p-4",                  // Mobile: small padding
    "md:p-6",               // Medium: medium padding
    "lg:p-8",               // Large: large padding

    // Responsive text
    "text-sm",              // Mobile: small text
    "md:text-base",         // Medium: base text
    "lg:text-lg"            // Large: large text
])?;
```

### Dark Mode Support

Implement dark mode with automatic switching:

```rust
let dark_mode_css = generator.process_element_classes(&[
    // Light mode (default)
    "bg-white", "text-gray-900",

    // Dark mode variants
    "dark:bg-gray-900", "dark:text-white",

    // Dark mode specific styling
    "dark:border-gray-700", "dark:hover:bg-gray-800"
])?;
```

---

## 🧪 **Testing & Development**

### Unit Testing

Test your CSS generation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs_core::CssGenerator;

    #[test]
    fn test_button_styling() {
        let mut generator = CssGenerator::new();
        let css = generator.process_element_classes(&[
            "bg-blue-500", "text-white", "p-4", "rounded"
        ]).unwrap();

        assert!(css.contains("background-color"));
        assert!(css.contains("color"));
        assert!(css.contains("padding"));
        assert!(css.contains("border-radius"));
    }

    #[test]
    fn test_responsive_design() {
        let mut generator = CssGenerator::new();
        let css = generator.process_element_classes(&[
            "flex", "sm:flex-row", "md:justify-center"
        ]).unwrap();

        assert!(css.contains("display: flex"));
        assert!(css.contains("@media"));
        assert!(css.contains("min-width"));
    }
}
```

### Development Workflow

1. **Setup your project:**
```bash
cargo new my-tailwind-app
cd my-tailwind-app
cargo add tailwind-rs-core
```

2. **Create your first component:**
```rust
// src/components.rs
use tailwind_rs_core::CssGenerator;

pub fn generate_component_css() -> String {
    let mut generator = CssGenerator::new();

    generator.process_element_classes(&[
        "bg-gradient-to-r", "from-blue-500", "to-purple-600",
        "text-white", "p-8", "rounded-xl", "shadow-2xl"
    ]).unwrap()
}
```

3. **Use in your application:**
```rust
// src/main.rs
mod components;

fn main() {
    let css = components::generate_component_css();
    println!("Generated CSS:\n{}", css);
}
```

---

## 🔧 **Configuration & Optimization**

### Custom Configuration

Configure the generator for your needs:

```rust
use tailwind_rs_core::{CssGenerator, CssGenerationConfig};
use std::collections::HashMap;

let config = CssGenerationConfig {
    memory_optimization: true,
    enable_container_queries: true,
    enable_arbitrary_values: true,
    custom_breakpoints: {
        let mut breakpoints = HashMap::new();
        breakpoints.insert(tailwind_rs_core::responsive::Breakpoint::Sm, "(min-width: 576px)".to_string());
        breakpoints.insert(tailwind_rs_core::responsive::Breakpoint::Md, "(min-width: 768px)".to_string());
        breakpoints
    },
    ..Default::default()
};

let generator = CssGenerator::with_config(config);
```

### Performance Optimization

Optimize for production use:

```rust
// Reuse generators across requests
let mut generator = CssGenerator::new();

// For web applications, cache generated CSS
let css_cache = std::collections::HashMap::new();

// Generate CSS once, reuse many times
let button_css = generator.process_element_classes(&[
    "bg-blue-500", "hover:bg-blue-600", "text-white"
])?;
```

---

## 📚 **Advanced Topics**

### CSS Functions (@apply, @layer, @import)

Use advanced CSS composition features:

```rust
use tailwind_rs_core::css_functions::CssFunctionsProcessor;

let mut processor = CssFunctionsProcessor::new();

// @apply directive
let applied_css = processor.process_apply("bg-blue-500 text-white p-4")?;

// @layer directive
let layered_css = processor.process_layer("components", ".btn { color: blue; }")?;

// @import directive
let imported_css = processor.process_import("url('styles.css')")?;
```

### Plugin System

Extend `tailwind-rs` with custom utilities:

```rust
use tailwind_rs_core::css_generator::plugin_system::{Plugin, PluginManager};

struct CustomGradientPlugin;

impl Plugin for CustomGradientPlugin {
    fn name(&self) -> &str { "custom-gradients" }

    fn register_utilities(&self, manager: &mut PluginManager) {
        // Register custom gradient utilities
        manager.register_utility("bg-rainbow", "background: linear-gradient(45deg, red, orange, yellow, green, blue, indigo, violet);");
    }
}

let mut generator = CssGenerator::new();
generator.register_plugin(Box::new(CustomGradientPlugin))?;
```

---

## 🚀 **Production Deployment**

### Build Optimization

```bash
# Optimized release build
cargo build --release --features production

# With framework features
cargo build --release --features "leptos dioxus yew"
```

### CI/CD Setup

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo check
      - run: cargo test
      - run: cargo build --release
```

---

## 📖 **Next Steps**

1. **Explore the API Reference** - Learn about all available features
2. **Try the Examples** - See practical usage patterns
3. **Join the Community** - Get help and share your projects
4. **Contribute** - Help improve `tailwind-rs`

Happy styling with `tailwind-rs`! 🎨✨
