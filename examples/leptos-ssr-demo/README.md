# Tailwind-RS Leptos SSR Demo

A Server-Side Rendering demo showcasing Tailwind-RS integration with Leptos v0.8.9, featuring API contracts and modern parsers.

## 🚀 Features

- **Server-Side Rendering**: Full SSR with Leptos and Axum
- **API Contracts**: Type-safe validation with contract-based guarantees
- **Transform Parsers**: O(1) HashMap-based parsers for translate/scale utilities
- **CSS Generation**: Dynamic CSS generation from Tailwind-RS classes
- **Interactive Components**: Real-time counter and form interactions

## 🏗️ Architecture

### Components
- **HomePage**: Overview with interactive counter and form
- **ApiContractsPage**: Demonstrates contract-based validation
- **TransformParsersPage**: Shows transform utility parsing performance

### Technical Stack
- **Leptos v0.8.9**: Modern reactive web framework
- **Axum**: High-performance web server
- **Tailwind-RS**: Type-safe CSS generation
- **Rust**: Systems programming language

## 📦 Installation

```bash
# Build the demo
cargo build --release

# Run the server
cargo run --release
```

## 🌐 Usage

1. **Start the server**:
   ```bash
   cargo run --release
   ```

2. **Open your browser**:
   Navigate to `http://localhost:3001`

3. **Explore features**:
   - Interactive counter with reactive updates
   - API contract validation demos
   - Transform parser performance showcases

## 🔧 Current Status

The SSR demo is **fully functional and production-ready** with:
- ✅ Full Leptos component architecture
- ✅ API contracts integration
- ✅ Transform parsers demonstration
- ✅ CSS generation pipeline
- ✅ Server-side rendering with inline CSS
- ✅ Axum HTTP server with proper routing
- ✅ Tailwind-RS CSS generation working

## 🎯 Demo Highlights

- **Server-Side Rendering**: All content is rendered on the server for optimal performance
- **API Contracts**: Demonstrates guaranteed API stability through contract validation
- **Modern Parsers**: Shows the new O(1) performance parsers for transform utilities
- **Type Safety**: Full Rust type safety throughout the application

## 📊 Performance

- **Initial Load**: Server-rendered HTML provides instant content
- **Bundle Size**: Optimized with tree-shaking and minimal dependencies
- **Parser Performance**: O(1) lookup for transform utilities
- **Memory Usage**: Efficient reactive system with automatic cleanup

## 🔄 Development

The demo showcases the complete Tailwind-RS ecosystem:
- Core CSS generation engine
- API contract validation system
- Modern parser implementations
- Framework integration patterns

This represents a production-ready example of modern web development with Rust, combining the performance of systems programming with the ergonomics of reactive web frameworks.
