# 🎨 Tailwind-RS Leptos Demo v0.8.9

A comprehensive demo application showcasing the integration of **Tailwind-RS** with **Leptos v0.8.9**, featuring the latest API contracts and parser enhancements.

## ✨ Features Demonstrated

### 🔒 API Contracts System
- **Live Contract Validation**: Interactive demonstration of the API contracts system
- **Input/Output Validation**: Real-time validation of class building operations
- **Contract-Based Reliability**: Guaranteed API behavior with comprehensive error handling

### 🎨 New Transform Parsers
- **translate-x/y Classes**: `translate-x-1`, `translate-x-4`, `translate-y-2`, etc.
- **scale-x/y Classes**: `scale-x-50`, `scale-x-110`, `scale-y-95`, etc.
- **Performance**: O(1) HashMap-based lookups for optimal performance
- **Interactive Demo**: Hover effects showcasing the new transform capabilities

### 🚀 Modern Leptos v0.8.9 Features
- **Signal API**: Updated to use the new `signal()` function
- **Component Architecture**: Modern component patterns with proper typing
- **Reactive UI**: Full reactivity with state management
- **Performance**: Optimized rendering with Leptos' fine-grained reactivity

### 🎯 Tailwind-RS Integration
- **Framework Integration**: Seamless integration with Leptos components
- **Type Safety**: Compile-time validation of Tailwind classes
- **CSS Generation**: Real-time CSS generation from class combinations
- **Performance**: Optimized class processing and CSS output

## 🏗️ Architecture

```
examples/leptos-demo/
├── src/
│   └── main.rs          # Main demo application
├── Cargo.toml           # Dependencies and configuration
└── README.md           # This documentation
```

## 🚀 Running the Demo

### Prerequisites
- Rust 1.89.0 or later
- Node.js and npm (for building)

### Build and Run

```bash
# Navigate to the demo directory
cd examples/leptos-demo

# Build for development
cargo build

# Run the demo
cargo run

# Or build for web (WASM)
wasm-pack build --target web --out-dir pkg
```

### Browser Development

For web development with hot reloading:

```bash
# Install dependencies
npm install

# Start development server
npm run dev
```

## 📱 Demo Sections

### 1. Header & Hero
- Modern navigation and hero section
- Responsive design with Tailwind classes
- Gradient backgrounds and typography

### 2. Features Overview
- Feature cards showcasing Tailwind-RS capabilities
- Grid layouts and responsive design
- Icon integration with Tailwind utilities

### 3. 🔒 API Contracts Demo
- **Interactive Validation**: Click to run contract validation
- **Real-time Results**: See validation process and results
- **Error Handling**: Demonstrates comprehensive error reporting
- **Contract Process**: Shows input → validation → processing → output flow

### 4. 🎨 Transform Parsers Demo
- **Live Hover Effects**: See transform classes in action
- **Performance Metrics**: O(1) lookup times displayed
- **Supported Classes**: Complete list of implemented transform utilities
- **Visual Feedback**: Interactive demonstrations of scaling and translation

### 5. Interactive Demo
- Counter with state management
- Button hover effects using Tailwind-RS classes
- Reactive updates and user interaction

### 6. Footer
- Professional footer with links and branding
- Responsive grid layout

## 🔧 Technical Implementation

### API Contracts Integration

```rust
use tailwind_rs_core::api_contracts::{ClassBuilderContract, ApiVersion, ClassBuilderInput, ApiContract};

// Create contract instance
let contract = ClassBuilderContract::new(ApiVersion::V2_0_0);

// Validate input
contract.validate_input(&input)?;

// Process with guarantees
let output = contract.process(input)?;
contract.validate_output(&output)?;
```

### Transform Parsers Usage

The demo showcases the new O(1) performance parsers:

- `translate-x-*`: Horizontal translation (e.g., `translate-x-4` = `translateX(1rem)`)
- `translate-y-*`: Vertical translation (e.g., `translate-y-2` = `translateY(0.5rem)`)
- `scale-x-*`: Horizontal scaling (e.g., `scale-x-110` = `scaleX(1.1)`)
- `scale-y-*`: Vertical scaling (e.g., `scale-y-95` = `scaleY(0.95)`)

### Modern Leptos Patterns

```rust
// Updated signal API
let (count, set_count) = signal(0);
let (result, set_result) = signal(String::new());

// Reactive components with proper typing
#[component]
fn InteractiveComponent() -> impl IntoView {
    // Component logic here
}
```

## 🎯 Learning Outcomes

This demo helps developers understand:

1. **API Contracts**: How to use contract-based validation for reliable APIs
2. **Parser Performance**: The benefits of optimized O(1) parsers
3. **Modern Leptos**: Updated patterns for Leptos v0.8.9
4. **Tailwind-RS Integration**: Best practices for framework integration
5. **Reactive UI**: Building interactive applications with modern patterns

## 🔄 Development Workflow

The demo follows modern Rust web development practices:

- **Hot Reloading**: Fast iteration during development
- **Type Safety**: Compile-time guarantees throughout
- **Performance**: Optimized builds for production
- **Testing**: Comprehensive test coverage for reliability

## 📊 Performance Characteristics

- **CSS Generation**: Sub-millisecond class processing
- **Contract Validation**: Fast input/output validation
- **Parser Lookups**: O(1) HashMap-based class resolution
- **Bundle Size**: Minimal WASM output with tree-shaking

## 🤝 Contributing

This demo serves as a reference implementation for:

- API contracts usage patterns
- Parser integration best practices
- Modern Leptos development
- Tailwind-RS framework integration

## 📄 License

This demo is part of the Tailwind-RS project and follows the same MIT license.
