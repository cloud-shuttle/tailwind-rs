# Getting Started with `tailwind-rs`

Get up and running with `tailwind-rs` in under 5 minutes. This guide will walk you through setting up a new project and creating your first styled components.

## 🚀 Quick Setup

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Cargo
- Your preferred Rust web framework (Leptos, Yew, or Dioxus)

### 1. Create a New Project

```bash
# Create a new Rust project
cargo new my-tailwind-app
cd my-tailwind-app

# Add your web framework (example with Leptos)
cargo add leptos
cargo add tailwind-rs
cargo add tailwind-rs-leptos
```

### 2. Configure Cargo.toml

```toml
[package]
name = "my-tailwind-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.6"
tailwind-rs = "0.1"
tailwind-rs-leptos = "0.1"

[build-dependencies]
tailwind-rs-build = "0.1"
```

### 3. Create Build Script

Create `build.rs` in your project root:

```rust
use tailwind_rs_build::TailwindBuilder;

fn main() {
    TailwindBuilder::new()
        .scan_source("src/")
        .generate_css("dist/styles.css")
        .optimize()
        .build();
}
```

### 4. Your First Component

Create `src/main.rs`:

```rust
use leptos::*;
use tailwind_rs::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class=classes! {
            layout: "min-h-screen bg-gray-100",
            typography: "font-sans",
        }>
            <Header />
            <Main />
            <Footer />
        </div>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class=classes! {
            layout: "bg-white shadow-sm border-b",
            spacing: "px-6 py-4",
        }>
            <h1 class=classes! {
                typography: "text-2xl font-bold text-gray-900",
            }>
                "My Tailwind App"
            </h1>
        </header>
    }
}

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <main class=classes! {
            layout: "container mx-auto",
            spacing: "px-6 py-8",
        }>
            <Card />
        </main>
    }
}

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div class=classes! {
            layout: "bg-white rounded-lg shadow-md",
            spacing: "p-6",
            border: "border border-gray-200",
        }>
            <h2 class=classes! {
                typography: "text-xl font-semibold text-gray-800 mb-4",
            }>
                "Welcome to tailwind-rs!"
            </h2>
            <p class=classes! {
                typography: "text-gray-600 mb-6",
            }>
                "This component demonstrates the power of type-safe, 
                performant Tailwind CSS integration for Rust web frameworks."
            </p>
            <Button variant=ButtonVariant::Primary>
                "Get Started"
            </Button>
        </div>
    }
}

#[component]
pub fn Button(#[prop(optional)] variant: ButtonVariant) -> impl IntoView {
    let classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
        variant: match variant {
            ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700 focus:ring-2 focus:ring-blue-500",
            ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-2 focus:ring-gray-500",
            ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700 focus:ring-2 focus:ring-red-500",
        },
        state: "focus:outline-none",
    };
    
    view! { <button class=classes>"Click me"</button> }
}

#[derive(Clone, Copy, PartialEq)]
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
pub fn Footer() -> impl IntoView {
    view! {
        <footer class=classes! {
            layout: "bg-gray-800 text-white",
            spacing: "px-6 py-4",
            typography: "text-center text-sm",
        }>
            "Built with ❤️ using tailwind-rs"
        </footer>
    }
}

fn main() {
    mount_to_body(App)
}
```

### 5. Run Your Application

```bash
# Build and run
cargo run

# Or for development with hot reloading
cargo watch -x run
```

## 🎯 What Just Happened?

Let's break down what we've accomplished:

### 1. **Type-Safe Classes**
```rust
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium",
    variant: "bg-blue-600 text-white hover:bg-blue-700",
    state: "focus:outline-none focus:ring-2",
};
```

The `classes!` macro provides:
- ✅ **Compile-time validation** - Invalid classes cause compilation errors
- ✅ **IDE autocomplete** - Full IntelliSense support
- ✅ **Organized styling** - Logical grouping of related classes
- ✅ **Performance** - Optimized class generation

### 2. **Dynamic Styling**
```rust
variant: match variant {
    ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
    ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
    ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
},
```

You can now:
- ✅ **Generate classes at runtime** - No more static analysis limitations
- ✅ **Use conditional styling** - Match expressions, if statements, etc.
- ✅ **Create reusable components** - Variants and props for flexibility

### 3. **Automatic CSS Generation**
The build script automatically:
- ✅ **Scans your Rust code** - Finds all used classes
- ✅ **Generates optimized CSS** - Only includes classes you actually use
- ✅ **Tree-shakes unused styles** - Minimal bundle size
- ✅ **Integrates with your build** - No separate CSS build step

## 🎨 Next Steps

Now that you have a basic setup, explore these advanced features:

### 1. **Responsive Design**
```rust
let classes = classes! {
    responsive: Responsive {
        sm: "text-sm",
        md: "text-base",
        lg: "text-lg",
        xl: "text-xl",
    },
};
```

### 2. **Theme System**
```rust
let classes = classes! {
    theme: Theme::Primary,
    variant: Variant::Solid,
    size: Size::Medium,
};
```

### 3. **Dynamic Colors**
```rust
let color = Color::Blue;
let classes = classes! {
    background: color.background(600),
    text: color.text(),
    hover: color.hover(700),
};
```

## 🔧 Troubleshooting

### Common Issues

#### Classes Not Appearing
- **Check build script** - Ensure `build.rs` is properly configured
- **Verify scanning** - Make sure source directory is correct
- **Check CSS output** - Verify `dist/styles.css` is generated

#### Compilation Errors
- **Invalid classes** - Check for typos in class names
- **Missing dependencies** - Ensure all required crates are added
- **Version conflicts** - Use compatible versions of framework and tailwind-rs

#### Performance Issues
- **Enable optimization** - Use `.optimize()` in build script
- **Check bundle size** - Monitor generated CSS file size
- **Use tree-shaking** - Remove unused classes automatically

### Getting Help

- **GitHub Issues** - Report bugs and request features
- **Discord Community** - Get help from other developers
- **Documentation** - Comprehensive guides and examples
- **Stack Overflow** - Tag questions with `tailwind-rs`

## 📚 What's Next?

Ready to dive deeper? Check out these guides:

- **[API Reference](./api/core.md)** - Complete class reference
- **[Framework Integration](./frameworks/leptos.md)** - Framework-specific guides
- **[Advanced Features](./advanced/dynamic-styling.md)** - Dynamic styling and theming
- **[Example Projects](./examples/README.md)** - Complete example applications

---

Congratulations! You've successfully set up `tailwind-rs` and created your first styled components. The type-safe, performant Tailwind integration you've been waiting for is now at your fingertips.

