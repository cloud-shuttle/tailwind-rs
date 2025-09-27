# 🚀 Getting Started with Tailwind-RS 2025

## Overview

This guide will help you get started with Tailwind-RS, the production-ready Rust implementation of Tailwind CSS. Follow these steps to set up your development environment and create your first Tailwind-RS application.

## 📋 Prerequisites

### System Requirements

- **Rust**: Version 1.70 or higher
- **Cargo**: Latest stable version
- **Node.js**: Version 18 or higher (for PostCSS integration)
- **Git**: For version control

### Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

## 🎯 Quick Start

### 1. Create a New Project

```bash
# Create a new Rust project
cargo new my-tailwind-app
cd my-tailwind-app
```

### 2. Add Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tailwind-rs-core = "0.15.4"
tailwind-rs-leptos = "0.15.4"  # For Leptos
# tailwind-rs-yew = "0.15.4"   # For Yew
# tailwind-rs-dioxus = "0.15.4" # For Dioxus

# Framework dependencies (choose one)
leptos = "0.8"
# yew = "0.21"
# dioxus = "0.4"
```

### 3. Basic Usage

Create a simple example in `src/main.rs`:

```rust
use tailwind_rs_core::css_generator::CssGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a CSS generator
    let mut generator = CssGenerator::new();
    
    // Add Tailwind classes
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    generator.add_class("p-4")?;
    generator.add_class("rounded-lg")?;
    
    // Generate CSS
    let css = generator.generate_css();
    
    println!("Generated CSS:");
    println!("{}", css);
    
    Ok(())
}
```

### 4. Run Your Application

```bash
cargo run
```

## 🎨 Framework Integration

### Leptos Integration

#### 1. Update Cargo.toml

```toml
[dependencies]
tailwind-rs-leptos = "0.15.4"
leptos = "0.8"
leptos_dom = "0.8"
leptos_server = "0.8"
```

#### 2. Create a Leptos Component

```rust
use tailwind_rs_leptos::*;
use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100">
            <header class="bg-white shadow">
                <div class="max-w-7xl mx-auto py-6 px-4">
                    <h1 class="text-3xl font-bold text-gray-900">
                        "Tailwind-RS with Leptos"
                    </h1>
                </div>
            </header>
            <main class="max-w-7xl mx-auto py-6 px-4">
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-xl font-semibold mb-4">"Welcome"</h2>
                    <p class="text-gray-600">
                        "This is a Tailwind-RS application built with Leptos."
                    </p>
                    <button class="mt-4 bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded">
                        "Click me"
                    </button>
                </div>
            </main>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
```

### Yew Integration

#### 1. Update Cargo.toml

```toml
[dependencies]
tailwind-rs-yew = "0.15.4"
yew = "0.21"
```

#### 2. Create a Yew Component

```rust
use tailwind_rs_yew::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="min-h-screen bg-gray-100">
            <header class="bg-white shadow">
                <div class="max-w-7xl mx-auto py-6 px-4">
                    <h1 class="text-3xl font-bold text-gray-900">
                        {"Tailwind-RS with Yew"}
                    </h1>
                </div>
            </header>
            <main class="max-w-7xl mx-auto py-6 px-4">
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-xl font-semibold mb-4">{"Welcome"}</h2>
                    <p class="text-gray-600">
                        {"This is a Tailwind-RS application built with Yew."}
                    </p>
                    <button class="mt-4 bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded">
                        {"Click me"}
                    </button>
                </div>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

### Dioxus Integration

#### 1. Update Cargo.toml

```toml
[dependencies]
tailwind-rs-dioxus = "0.15.4"
dioxus = "0.4"
```

#### 2. Create a Dioxus Component

```rust
use tailwind_rs_dioxus::*;
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100",
            header { class: "bg-white shadow",
                div { class: "max-w-7xl mx-auto py-6 px-4",
                    h1 { class: "text-3xl font-bold text-gray-900",
                        "Tailwind-RS with Dioxus"
                    }
                }
            }
            main { class: "max-w-7xl mx-auto py-6 px-4",
                div { class: "bg-white rounded-lg shadow p-6",
                    h2 { class: "text-xl font-semibold mb-4", "Welcome" }
                    p { class: "text-gray-600",
                        "This is a Tailwind-RS application built with Dioxus."
                    }
                    button { class: "mt-4 bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded",
                        "Click me"
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
```

## 🔧 Advanced Configuration

### Theme Customization

#### 1. Create a Custom Theme

```rust
use tailwind_rs_core::theme::ThemeConfig;

let theme = ThemeConfig::new("my-custom-theme");

// Customize spacing
theme.spacing.add_spacing("custom", Spacing::px(24));

// Customize colors
theme.color_palettes.push(ColorPalette::new("brand", vec![
    ("primary", Color::hex("#3b82f6")),
    ("secondary", Color::hex("#64748b")),
]));

// Validate theme
theme.validate()?;
```

#### 2. Use Custom Theme

```rust
use tailwind_rs_core::css_generator::CssGenerator;

let mut generator = CssGenerator::with_theme(theme);

// Use custom classes
generator.add_class("p-custom")?; // Uses custom spacing
generator.add_class("bg-brand-primary")?; // Uses custom color
```

### Performance Optimization

#### 1. Enable Performance Monitoring

```rust
use tailwind_rs_core::utilities::advanced_performance_optimization::PerformanceMonitor;

let monitor = PerformanceMonitor::new();

// Monitor performance
let cpu_usage = monitor.get_metric("cpu_usage");
let memory_usage = monitor.get_metric("memory_usage");

if let Some(usage) = cpu_usage {
    if *usage > 80.0 {
        println!("High CPU usage: {}%", usage);
    }
}
```

#### 2. Optimize Memory Usage

```rust
use tailwind_rs_core::utilities::advanced_performance_optimization::MemoryOptimizer;

let optimizer = MemoryOptimizer::new()
    .max_memory_usage(50 * 1024 * 1024) // 50MB
    .gc_threshold(0.8);

optimizer.optimize_memory();
```

## 📚 Examples and Tutorials

### Example 1: Simple Card Component

```rust
use tailwind_rs_core::css_generator::CssGenerator;

fn create_card_css() -> Result<String, Box<dyn std::error::Error>> {
    let mut generator = CssGenerator::new();
    
    // Card container
    generator.add_class("bg-white")?;
    generator.add_class("rounded-lg")?;
    generator.add_class("shadow-md")?;
    generator.add_class("p-6")?;
    
    // Card title
    generator.add_class("text-xl")?;
    generator.add_class("font-semibold")?;
    generator.add_class("text-gray-900")?;
    generator.add_class("mb-4")?;
    
    // Card content
    generator.add_class("text-gray-600")?;
    generator.add_class("leading-relaxed")?;
    
    Ok(generator.generate_css())
}
```

### Example 2: Responsive Grid

```rust
use tailwind_rs_core::css_generator::CssGenerator;

fn create_responsive_grid() -> Result<String, Box<dyn std::error::Error>> {
    let mut generator = CssGenerator::new();
    
    // Grid container
    generator.add_class("grid")?;
    generator.add_class("grid-cols-1")?;
    generator.add_class("md:grid-cols-2")?;
    generator.add_class("lg:grid-cols-3")?;
    generator.add_class("gap-6")?;
    
    // Grid items
    generator.add_class("bg-white")?;
    generator.add_class("rounded-lg")?;
    generator.add_class("shadow-md")?;
    generator.add_class("p-6")?;
    
    Ok(generator.generate_css())
}
```

### Example 3: Interactive Button

```rust
use tailwind_rs_core::css_generator::CssGenerator;

fn create_interactive_button() -> Result<String, Box<dyn std::error::Error>> {
    let mut generator = CssGenerator::new();
    
    // Button base styles
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    generator.add_class("font-bold")?;
    generator.add_class("py-2")?;
    generator.add_class("px-4")?;
    generator.add_class("rounded")?;
    
    // Hover effects
    generator.add_class("hover:bg-blue-600")?;
    generator.add_class("transition-colors")?;
    generator.add_class("duration-200")?;
    
    // Focus styles
    generator.add_class("focus:outline-none")?;
    generator.add_class("focus:ring-2")?;
    generator.add_class("focus:ring-blue-300")?;
    
    Ok(generator.generate_css())
}
```

## 🚀 Deployment

### 1. Build for Production

```bash
# Build optimized version
cargo build --release

# For WASM targets
cargo build --target wasm32-unknown-unknown --release
```

### 2. Docker Deployment

Create a `Dockerfile`:

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/my-tailwind-app /usr/local/bin/
CMD ["my-tailwind-app"]
```

### 3. GitHub Actions

Create `.github/workflows/ci.yml`:

```yaml
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
      - name: Run tests
        run: cargo test
      - name: Run benchmarks
        run: cargo bench
```

## 🔍 Troubleshooting

### Common Issues

#### 1. Compilation Errors

```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update
```

#### 2. Performance Issues

```bash
# Run with optimizations
cargo run --release

# Profile the application
cargo bench
```

#### 3. Framework Integration Issues

```bash
# Check framework compatibility
cargo check --package tailwind-rs-leptos
cargo check --package tailwind-rs-yew
cargo check --package tailwind-rs-dioxus
```

### Getting Help

- **Documentation**: Check the comprehensive API reference
- **Examples**: Review the example applications
- **Community**: Join the Tailwind-RS community
- **Issues**: Report issues on GitHub

## 📈 Next Steps

### 1. Explore Advanced Features

- **Theme System**: Customize your design system
- **Performance Optimization**: Optimize for production
- **Framework Integration**: Build with your preferred framework
- **Testing**: Add comprehensive tests

### 2. Contribute to the Project

- **Report Issues**: Help improve the project
- **Submit PRs**: Contribute new features
- **Documentation**: Help improve documentation
- **Examples**: Share your examples

### 3. Production Deployment

- **Performance Monitoring**: Monitor in production
- **Error Handling**: Implement robust error handling
- **Caching**: Implement caching strategies
- **Scaling**: Plan for scale

---

*Last Updated: January 2025*
*Version: 0.15.4*
