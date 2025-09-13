# Tailwind Rust Library Documentation

Welcome to the comprehensive documentation for `tailwind-rs` - the first-class Tailwind CSS integration for Rust web frameworks.

## 📚 Documentation Overview

This documentation is organized to help you get started quickly and dive deep into advanced features as needed.

### 🚀 Quick Start
- **[Getting Started Guide](./getting-started.md)** - Set up `tailwind-rs` in minutes
- **[Installation](./installation.md)** - Detailed installation instructions
- **[Basic Usage](./basic-usage.md)** - Your first components with `tailwind-rs`

### 🎯 Framework Integration
- **[Leptos Integration](./frameworks/leptos.md)** - Complete Leptos integration guide
- **[Yew Integration](./frameworks/yew.md)** - Yew-specific setup and examples
- **[Dioxus Integration](./frameworks/dioxus.md)** - Dioxus integration patterns
- **[Generic Web Framework](./frameworks/generic.md)** - Support for other frameworks

### 📖 API Reference
- **[Core API](./api/core.md)** - Core classes and utilities
- **[Macros Reference](./api/macros.md)** - Procedural macros documentation
- **[Theme System](./api/themes.md)** - Theme and variant system
- **[Responsive Design](./api/responsive.md)** - Responsive utilities and breakpoints

### 🎨 Advanced Features
- **[Dynamic Styling](./advanced/dynamic-styling.md)** - Runtime class generation
- **[Theme Customization](./advanced/themes.md)** - Custom themes and design tokens
- **[Performance Optimization](./advanced/performance.md)** - Bundle optimization and caching
- **[Build Integration](./advanced/build.md)** - Advanced build configuration

### 🔄 Migration & Examples
- **[Migration Guide](./migration/README.md)** - Migrating from existing solutions
- **[Example Projects](./examples/README.md)** - Complete example applications
- **[Best Practices](./best-practices.md)** - Recommended patterns and conventions

### 🛠️ Development
- **[Contributing](./contributing.md)** - How to contribute to the project
- **[Architecture](./architecture.md)** - Technical architecture and design decisions
- **[Testing](./testing.md)** - Testing strategies and examples
- **[Release Process](./release-process.md)** - How we release new versions

## 🎯 What is `tailwind-rs`?

`tailwind-rs` is a comprehensive Tailwind CSS integration for Rust web frameworks that addresses the fundamental issues with current Tailwind integrations. Built following our **Test-Driven Development (TDD) first approach** and comprehensive testing pyramid strategy.

### ❌ Current Problems
- **Unreliable class detection** - Classes in `.rs` files often missed
- **Build process fragmentation** - CSS and WASM builds don't coordinate
- **No dynamic styling** - Can't generate classes at runtime
- **Poor performance** - Large CSS bundles, slow runtime
- **No type safety** - Runtime errors for invalid classes

### ✅ Our Solution
- **🔍 Intelligent detection** - Rust AST parsing for accurate class detection
- **⚡ Performance optimized** - Tree-shaking, minimal bundles
- **🛡️ Type safe** - Compile-time validation and IDE support
- **🎨 Dynamic styling** - Runtime class generation and theming
- **🔧 Seamless integration** - Works with Leptos, Yew, Dioxus, and more
- **🧪 Comprehensive testing** - 100% test coverage with unit, integration, and E2E tests
- **🎭 Playwright validated** - All demos and features tested with Playwright

## 🚀 Quick Example

```rust
use tailwind_rs::*;

#[component]
pub fn Button(variant: ButtonVariant) -> impl IntoView {
    let classes = classes! {
        base: "px-4 py-2 rounded-md font-medium transition-colors",
        variant: match variant {
            ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
            ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
            ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
        },
        responsive: "sm:text-sm md:text-base lg:text-lg",
        state: "focus:outline-none focus:ring-2 focus:ring-blue-500",
    };
    
    view! { <button class=classes>"Click me"</button> }
}
```

## 🎯 Key Benefits

- **Always works** - No more missing classes or build issues
- **Type safe** - Compile-time validation prevents runtime errors
- **Performance** - Smaller bundles, faster runtime
- **Developer experience** - Full IDE support with autocomplete
- **Flexible** - Dynamic styling and comprehensive theming
- **Battle-tested** - Comprehensive test coverage with TDD approach
- **Demo-ready** - All features validated with Playwright testing
- **Production-ready** - Built with latest Rust standards and best practices

## 📞 Getting Help

- **GitHub Issues** - Report bugs and request features
- **Discord Community** - Chat with other developers
- **Stack Overflow** - Tag questions with `tailwind-rs`
- **Documentation** - Comprehensive guides and examples

## 🏆 Why Choose `tailwind-rs`?

### vs Current Tailwind Integration
- ✅ **Reliability** - Always works, no build issues
- ✅ **Performance** - Smaller bundles, faster runtime
- ✅ **Type Safety** - Compile-time validation
- ✅ **Developer Experience** - Better IDE support

### vs CSS-in-JS Libraries
- ✅ **Familiarity** - Uses Tailwind's proven design system
- ✅ **Ecosystem** - Leverages existing Tailwind plugins
- ✅ **Community** - Large Tailwind community
- ✅ **Documentation** - Extensive Tailwind docs

### vs Custom CSS Solutions
- ✅ **Productivity** - Faster development
- ✅ **Consistency** - Design system enforcement
- ✅ **Maintenance** - Less custom CSS to maintain
- ✅ **Scalability** - Better for large teams

---

Ready to get started? Check out our [Getting Started Guide](./getting-started.md) or jump straight to [Installation](./installation.md).
