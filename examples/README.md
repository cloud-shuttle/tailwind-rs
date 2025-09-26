# 🎨 Tailwind-RS Examples

This directory contains comprehensive examples demonstrating Tailwind-RS usage across different scenarios and frameworks.

## 🚀 Quick Examples

### **String-Based vs Rust Objects Demo**
- **File**: [`string_vs_objects_demo.rs`](string_vs_objects_demo.rs)
- **Description**: Comprehensive comparison of string-based classes vs Rust objects
- **Features**: Performance benchmarks, real-world component patterns, API examples
- **Run**: `cargo run --example string_vs_objects_demo`

### **Framework Integration Examples**
- **Directory**: [`framework-integrations/`](framework-integrations/)
- **Description**: Examples for Leptos, Yew, and Dioxus integration
- **Features**: Complete working examples for each framework

## 🔧 Core Examples

### **CSS Generation Examples**
- [`comprehensive_css_generation_100_percent.rs`](comprehensive_css_generation_100_percent.rs) - Complete CSS generation
- [`css_generation_showcase.rs`](css_generation_showcase.rs) - CSS generation features
- [`demo_css_generation.rs`](demo_css_generation.rs) - Basic CSS generation demo
- [`seamless_css_generation.rs`](seamless_css_generation.rs) - Seamless CSS workflow

### **Tailwind v4.1 Features**
- [`tailwind_v4_1_13_features.rs`](tailwind_v4_1_13_features.rs) - Latest Tailwind features
- [`demo_phase2_features.rs`](demo_phase2_features.rs) - Phase 2 feature demonstrations

### **Directive Processing**
- [`tailwind_directive_processing.rs`](tailwind_directive_processing.rs) - Tailwind directive handling

## 🎯 Running Examples

### **Run a Specific Example**
```bash
# Run the string vs objects demo
cargo run --example string_vs_objects_demo

# Run CSS generation showcase
cargo run --example css_generation_showcase

# Run Tailwind v4.1 features
cargo run --example tailwind_v4_1_13_features
```

### **Run All Examples**
```bash
# Run all examples
cargo run --examples
```

## 📚 Example Categories

### **1. Basic Usage**
- Simple CSS generation
- Basic class building
- String-based approaches

### **2. Advanced Features**
- Complex component patterns
- Performance optimization
- Advanced CSS features

### **3. Framework Integration**
- Leptos integration examples
- Yew integration examples
- Dioxus integration examples

### **4. Performance & Optimization**
- Performance benchmarks
- Memory optimization
- Build optimization

## 🔗 Related Documentation

- **[Quick Start Guide](../docs/getting-started/quick-start.md)** - Get started with Tailwind-RS
- **[Framework Integration](../docs/frameworks/)** - Framework-specific guides
- **[API Reference](../docs/api/)** - Complete API documentation
- **[String vs Objects Demo](string_vs_objects_demo.rs)** - Comprehensive API comparison

## 🎨 Demo Applications

### **SSR Demo**
- **Location**: [`../demos/ssr-demo/`](../demos/ssr-demo/)
- **Description**: Server-side rendering with Rust HTTP server
- **Features**: Dynamic HTML generation, Tailwind-RS integration, real-time data

### **Leptos Demo**
- **Location**: [`../demos/leptos-demo/`](../demos/leptos-demo/)
- **Description**: Full Leptos application with Tailwind-RS
- **Features**: WASM support, interactive components, responsive design

## 🚀 Getting Started

1. **Choose Your Approach**:
   - **String-based**: Simple, fast, familiar
   - **Rust objects**: Type-safe, advanced, dynamic

2. **Pick a Framework**:
   - **Leptos**: Modern Rust web framework
   - **Yew**: Component-based web framework
   - **Dioxus**: Cross-platform UI framework

3. **Run Examples**:
   ```bash
   cargo run --example string_vs_objects_demo
   ```

4. **Explore Demos**:
   ```bash
   cd demos/ssr-demo && cargo run
   cd demos/leptos-demo && cargo run
   ```

## 📈 Performance Tips

- **String-based classes**: Fastest for simple cases
- **Rust objects**: Better for complex, dynamic styling
- **Hybrid approach**: Use strings for simple, objects for complex
- **Framework integration**: Choose based on your framework needs

---

**Happy coding with Tailwind-RS!** 🎨✨
