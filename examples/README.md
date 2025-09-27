# 🚀 Tailwind-RS Examples

This directory contains working demo applications showcasing Tailwind-RS integration with various Rust web frameworks.

## 📁 Available Examples

### 1. Leptos Demo (`leptos-demo/`)

A modern web application built with Leptos and Tailwind-RS, featuring:

- **Responsive Design**: Mobile-first responsive layout
- **Interactive Components**: Reactive components with state management
- **Modern UI**: Beautiful gradient backgrounds and smooth transitions
- **Performance**: Optimized for speed and efficiency

#### Features Demonstrated

- Framework integration with Leptos
- Responsive grid layouts
- Interactive buttons with hover effects
- Modern card components
- Gradient backgrounds
- Smooth transitions and animations

#### Running the Demo

```bash
cd examples/leptos-demo
cargo run
```

### 2. Yew Demo (`yew-demo/`)

A component-based application built with Yew and Tailwind-RS, featuring:

- **Component Architecture**: Modular, reusable components
- **State Management**: Efficient state handling with Yew
- **Type Safety**: Full compile-time type safety
- **Performance**: Optimized rendering and updates

#### Features Demonstrated

- Framework integration with Yew
- Component-based architecture
- State management with hooks
- Interactive elements
- Responsive design patterns
- Modern CSS utilities

#### Running the Demo

```bash
cd examples/yew-demo
cargo run
```

### 3. Dioxus Demo (`dioxus-demo/`)

A modern web application built with Dioxus and Tailwind-RS, featuring:

- **Modern Syntax**: Clean, readable component syntax
- **Reactive Updates**: Efficient reactivity system
- **Performance**: Optimized for modern web development
- **Developer Experience**: Excellent tooling and debugging

#### Features Demonstrated

- Framework integration with Dioxus
- Modern component syntax
- Reactive state management
- Interactive user interfaces
- Responsive design
- Performance optimizations

#### Running the Demo

```bash
cd examples/dioxus-demo
cargo run
```

## 🎯 What Each Demo Shows

### Common Features

All demos showcase:

1. **Framework Integration**: How Tailwind-RS integrates seamlessly with each framework
2. **Responsive Design**: Mobile-first responsive layouts
3. **Interactive Elements**: Buttons, forms, and interactive components
4. **Modern CSS**: Latest Tailwind CSS features and utilities
5. **Performance**: Optimized for speed and efficiency
6. **Type Safety**: Full compile-time type safety with Rust

### Framework-Specific Features

#### Leptos Demo
- **Reactive Components**: Signal-based reactivity
- **Server-Side Rendering**: SSR capabilities
- **Modern Syntax**: Clean, modern component syntax
- **Performance**: Optimized for speed

#### Yew Demo
- **Component Architecture**: Modular, reusable components
- **State Management**: Efficient state handling
- **Hooks**: Modern React-like hooks
- **Performance**: Optimized rendering

#### Dioxus Demo
- **Modern Syntax**: Clean, readable component syntax
- **Reactive Updates**: Efficient reactivity system
- **Performance**: Optimized for modern web development
- **Developer Experience**: Excellent tooling

## 🚀 Getting Started

### Prerequisites

- **Rust**: Version 1.70 or higher
- **Cargo**: Latest stable version
- **Node.js**: Version 18 or higher (for some examples)

### Running the Examples

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-org/tailwind-rs.git
   cd tailwind-rs
   ```

2. **Choose a demo**:
   ```bash
   # Leptos demo
   cd examples/leptos-demo
   cargo run
   
   # Yew demo
   cd examples/yew-demo
   cargo run
   
   # Dioxus demo
   cd examples/dioxus-demo
   cargo run
   ```

3. **Open in browser**:
   - The demos will typically run on `http://localhost:8080`
   - Check the console output for the exact URL

## 🔧 Customization

### Adding New Components

Each demo can be extended with new components:

```rust
// Example: Adding a new component
#[component]
fn MyComponent() -> impl IntoView {
    view! {
        <div class="bg-blue-500 text-white p-4 rounded-lg">
            "My Custom Component"
        </div>
    }
}
```

### Styling with Tailwind-RS

All demos use Tailwind-RS classes for styling:

```rust
// Example: Using Tailwind classes
<div class="bg-blue-500 text-white p-4 rounded-lg hover:bg-blue-600 transition-colors">
    "Styled with Tailwind-RS"
</div>
```

### State Management

Each framework has its own state management patterns:

#### Leptos
```rust
let (count, set_count) = create_signal(0);
```

#### Yew
```rust
let counter = use_state(|| 0);
```

#### Dioxus
```rust
let mut counter = use_signal(|| 0);
```

## 📚 Learning Resources

### Documentation

- **API Reference**: Comprehensive API documentation
- **Getting Started**: Step-by-step setup guide
- **Best Practices**: Production-ready guidelines
- **Examples**: More example applications

### Community

- **GitHub**: Source code and issues
- **Discord**: Real-time community support
- **Twitter**: Latest updates and news
- **Reddit**: Community discussions

## 🎯 Next Steps

### After Running the Demos

1. **Explore the Code**: Study the component implementations
2. **Modify Components**: Try changing styles and behavior
3. **Add Features**: Implement new functionality
4. **Build Your App**: Use the patterns in your own projects

### Production Deployment

1. **Optimize for Production**: Use release builds
2. **Add Testing**: Implement comprehensive tests
3. **Monitor Performance**: Use performance monitoring
4. **Deploy**: Deploy to your preferred platform

## 🤝 Contributing

### Adding New Examples

1. **Create a new directory**: `examples/your-framework-demo/`
2. **Add Cargo.toml**: Include necessary dependencies
3. **Implement components**: Create your demo components
4. **Update README**: Document your example
5. **Submit PR**: Share your contribution

### Improving Existing Examples

1. **Fork the repository**
2. **Make your changes**
3. **Test thoroughly**
4. **Submit a pull request**

## 📄 License

All examples are provided under the same license as the main Tailwind-RS project.

---

*Last Updated: January 2025*
*Examples Version: 0.15.4*