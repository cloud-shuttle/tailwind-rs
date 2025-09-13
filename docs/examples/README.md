# Tailwind-RS Examples

This directory contains comprehensive examples demonstrating how to use Tailwind-RS with different frameworks and scenarios.

## 📚 Available Examples

### 🦀 Rust CLI Examples
- **[Basic Usage](./basic-usage.md)** - Getting started with Tailwind-RS
- **[Advanced Styling](./advanced-styling.md)** - Complex styling scenarios
- **[Component Library](./component-library.md)** - Building reusable components
- **[Performance Optimization](./performance-optimization.md)** - Optimizing for production

### 🌐 Web Framework Examples
- **[Leptos Integration](./leptos-integration.md)** - Full-stack Rust web apps
- **[Yew Integration](./yew-integration.md)** - Component-based web apps
- **[Dioxus Integration](./dioxus-integration.md)** - Cross-platform UI framework

### 🎨 WASM Examples
- **[WASM Demo](./wasm-demo.md)** - Browser-based Tailwind-RS
- **[WASM Performance](./wasm-performance.md)** - Optimizing WASM bundles
- **[WASM Integration](./wasm-integration.md)** - Integrating with existing web apps

### 🧪 Testing Examples
- **[Unit Testing](./unit-testing.md)** - Testing Tailwind-RS components
- **[Integration Testing](./integration-testing.md)** - End-to-end testing
- **[Property-Based Testing](./property-based-testing.md)** - Advanced testing strategies

## 🚀 Quick Start

### 1. Basic Usage
```rust
use tailwind_rs_core::classes::ClassBuilder;

let classes = ClassBuilder::new()
    .class("bg-blue-500")
    .class("text-white")
    .class("p-4")
    .class("rounded-lg")
    .build();

println!("Generated classes: {}", classes);
// Output: "bg-blue-500 text-white p-4 rounded-lg"
```

### 2. Conditional Classes
```rust
use tailwind_rs_core::classes::ClassBuilder;

let is_active = true;
let classes = ClassBuilder::new()
    .class("base-class")
    .class_if(is_active, "active-class")
    .class_if(!is_active, "inactive-class")
    .build();
```

### 3. Responsive Design
```rust
use tailwind_rs_core::classes::ClassBuilder;

let classes = ClassBuilder::new()
    .class("text-sm")
    .class("sm:text-base")
    .class("md:text-lg")
    .class("lg:text-xl")
    .build();
```

## 📖 Framework-Specific Guides

### Leptos (v0.8.8)
```rust
use leptos::prelude::*;
use tailwind_rs_core::classes::ClassBuilder;

#[component]
fn MyComponent() -> impl IntoView {
    let classes = ClassBuilder::new()
        .class("bg-blue-500")
        .class("text-white")
        .class("p-4")
        .build();
    
    view! {
        <div class=classes>
            "Hello, Tailwind-RS!"
        </div>
    }
}
```

### Yew (v0.21.0)
```rust
use yew::prelude::*;
use tailwind_rs_core::classes::ClassBuilder;

#[function_component]
fn MyComponent() -> Html {
    let classes = ClassBuilder::new()
        .class("bg-green-500")
        .class("text-white")
        .class("p-4")
        .build();
    
    html! {
        <div class={classes}>
            {"Hello, Tailwind-RS!"}
        </div>
    }
}
```

## 🎯 Best Practices

### 1. Performance
- Use `ClassBuilder` for dynamic class generation
- Cache frequently used class combinations
- Minimize WASM bundle size for web applications

### 2. Maintainability
- Create reusable component functions
- Use consistent naming conventions
- Document complex styling logic

### 3. Testing
- Test class generation logic
- Use property-based testing for edge cases
- Validate responsive behavior

## 🔧 Troubleshooting

### Common Issues

1. **WASM Build Errors**
   - Ensure you're using compatible versions
   - Check for conflicting dependencies
   - Use `no_std` features when needed

2. **Class Generation Issues**
   - Verify Tailwind CSS is properly configured
   - Check for typos in class names
   - Ensure proper escaping of dynamic content

3. **Performance Issues**
   - Profile your application
   - Use `wasm-opt` for optimization
   - Consider code splitting for large applications

## 📚 Additional Resources

- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [Leptos Documentation](https://leptos.dev/)
- [Yew Documentation](https://yew.rs/)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](../../CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.