# Installation Guide

This guide will help you install and set up `tailwind-rs` in your Rust web project, following our established standards and best practices.

## 📋 Prerequisites

### System Requirements
- **Rust**: 1.70+ (latest stable recommended)
- **Cargo**: Latest version

### Development Tools
- **VS Code** with rust-analyzer (recommended)
- **Git** for version control

## 🚀 Quick Installation

### 1. Create New Project
```bash
# Create a new Rust project
cargo new my-tailwind-app
cd my-tailwind-app

# Initialize as a workspace if needed
cargo init --lib
```

### 2. Add Dependencies
```toml
# Cargo.toml
[package]
name = "my-tailwind-app"
version = "0.1.0"
edition = "2021"
rust-version = "1.70"

[dependencies]
# Core tailwind-rs
tailwind-rs-core = "0.8.1"

# Framework-specific integration (choose one)
tailwind-rs-leptos = "0.8.1"  # For Leptos
# tailwind-rs-yew = "0.8.1"   # For Yew
# tailwind-rs-dioxus = "0.8.1" # For Dioxus

# Framework dependencies
leptos = { version = "0.6", features = ["csr"] }
leptos_router = "0.6"

# Supporting crates
wasm-bindgen = "0.2"
web-sys = "0.3"
console_error_panic_hook = "0.1"
console_log = "0.2"
log = "0.4"
serde = { version = "1.0", features = ["derive"] }

[dev-dependencies]
# Testing dependencies
criterion = { version = "0.5", features = ["html_reports"] }
proptest = "1.4"
mockall = "0.12"
wasm-bindgen-test = "0.3"
```

### 3. Basic Usage
```rust
// src/main.rs
use tailwind_rs_core::*;

fn main() {
    let classes = ClassBuilder::new()
        .padding(SpacingValue::Integer(4))
        .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
        .text_color(Color::new(ColorPalette::White, ColorShade::Shade500))
        .rounded_lg()
        .build();
    
    println!("Generated classes: {}", classes.to_string());
    // Output: "p-4 bg-blue-500 text-white rounded-lg"
}
```

### 4. Run Tests
```bash
# Run the test suite
cargo test

# Run with verbose output
cargo test --verbose
```

## 🎯 Framework-Specific Installation

### Leptos Installation
```toml
# Cargo.toml - Leptos configuration
[dependencies]
leptos = { version = "0.6", features = ["csr"] }
leptos_router = "0.6"
tailwind-rs-core = "0.8.1"
tailwind-rs-leptos = "0.8.1"
```

### Yew Installation
```toml
# Cargo.toml - Yew configuration
[dependencies]
yew = "0.21"
tailwind-rs-core = "0.8.1"
tailwind-rs-yew = "0.8.1"
wasm-bindgen = "0.2"
web-sys = "0.3"
```

### Dioxus Installation
```toml
# Cargo.toml - Dioxus configuration
[dependencies]
dioxus = "0.4"
tailwind-rs-core = "0.8.1"
tailwind-rs-dioxus = "0.8.1"
```

## 🧪 Testing

### Basic Testing
```rust
// tests/basic_tests.rs
#[cfg(test)]
mod tests {
    use tailwind_rs_core::*;

    #[test]
    fn test_class_generation() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Integer(4))
            .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .build();
        
        assert!(classes.to_string().contains("p-4"));
        assert!(classes.to_string().contains("bg-blue-500"));
    }
}
```

## 🔧 Development Configuration

### VS Code Configuration
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll": true
  }
}
```

## 🎯 Verification

### Test Installation
```bash
# Run the test suite
cargo test

# Check code quality
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check

# Build the project
cargo build --release
```

### Verify Framework Integration
```rust
// src/main.rs - Test basic functionality
use leptos::*;
use tailwind_rs_leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let classes = ClassBuilder::new()
        .min_height(SpacingValue::Screen)
        .background_color(Color::new(ColorPalette::Gray, ColorShade::Shade100))
        .font_family(FontFamily::Sans)
        .build();
    
    view! {
        <div class=classes.to_string()>
            <h1 class="text-2xl font-bold text-gray-900">
                "Tailwind-rs is working!"
            </h1>
        </div>
    }
}

fn main() {
    mount_to_body(App)
}
```

## 🔧 Troubleshooting

### Common Issues

#### Build Errors
```bash
# Clear cargo cache
cargo clean
rm -rf target/

# Update dependencies
cargo update

# Rebuild
cargo build
```

#### Rust Version Issues
```bash
# Update Rust
rustup update stable

# Set default toolchain
rustup default stable
```

### Getting Help
- **GitHub Issues**: Report bugs and request features
- **Documentation**: Comprehensive guides and examples

## 📚 Next Steps

After successful installation:

1. **Read the [Quick Start Guide](quick-start.md)**
2. **Explore the [API Reference](../api/core.md)**
3. **Check out [Examples](../examples/README.md)**

---

Congratulations! You've successfully installed `tailwind-rs` v0.8.1 and are ready to build amazing Rust web applications with type-safe, performant Tailwind CSS integration.

