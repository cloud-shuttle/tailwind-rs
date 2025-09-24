# 🏗️ Architecture Overview

## 📋 Overview

This document outlines the core architecture and design principles for the Tailwind CSS v4.1 Rust implementation. It covers the fundamental concepts, design decisions, and architectural patterns that guide the entire implementation.

## 🎯 Design Principles

### 1. Type Safety First
- **Compile-time validation**: All utility classes are validated at compile time
- **Zero runtime errors**: Invalid class combinations are caught during compilation
- **IDE support**: Full autocomplete and error detection in IDEs

### 2. Performance Optimized
- **Zero runtime overhead**: Classes are generated at compile time
- **Intelligent caching**: Frequently used class combinations are cached
- **Memory efficient**: Minimal memory footprint with string interning

### 3. Framework Agnostic
- **Multiple framework support**: Works with Leptos, Yew, Dioxus, and more
- **Consistent API**: Same API across all framework integrations
- **Easy integration**: Simple to add to existing projects

### 4. Extensible Design
- **Plugin system**: Easy to add custom utilities
- **Theme customization**: Full theme system with custom values
- **Custom variants**: Support for custom state variants

### 5. Rust Native
- **No JavaScript dependencies**: Pure Rust implementation
- **Leverages Rust features**: Uses Rust's type system and macros
- **WASM compatible**: Works in browser environments

## 🏗️ Core Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    tailwind-rs-core                        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Classes   │  │  Utilities  │  │   Themes    │        │
│  │   System    │  │   System    │  │   System    │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ Validation  │  │ Responsive  │  │   Macros    │        │
│  │   System    │  │   System    │  │   System    │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────┐
│                Framework Integrations                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Leptos    │  │     Yew     │  │   Dioxus    │        │
│  │ Integration │  │ Integration │  │ Integration │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

### Core Components

#### 1. Class Management System
- **ClassBuilder**: Fluent API for building class combinations
- **ClassSet**: Internal representation of CSS classes
- **ClassCache**: Intelligent caching for performance

#### 2. Utility System
- **Spacing Utilities**: Padding, margin, gap utilities
- **Sizing Utilities**: Width, height, min/max utilities
- **Typography Utilities**: Font, text, line-height utilities
- **Color Utilities**: Complete color palette support
- **Layout Utilities**: Display, position, overflow utilities

#### 3. Theme System
- **Theme Configuration**: Customizable theme values
- **Color Palette**: Complete color system
- **Spacing Scale**: Consistent spacing values
- **Typography Scale**: Font sizes and weights

#### 4. Validation System
- **Compile-time Validation**: Type-safe class combinations
- **Runtime Validation**: Dynamic class validation
- **Error Reporting**: Detailed error messages and suggestions

#### 5. Responsive System
- **Breakpoint Management**: Responsive breakpoints
- **Responsive Utilities**: Breakpoint-specific classes
- **Mobile-first**: Mobile-first responsive design

#### 6. Macro System
- **Procedural Macros**: Code generation at compile time
- **Utility Macros**: Shortcuts for common patterns
- **Validation Macros**: Compile-time validation

## 🔧 Implementation Patterns

### 1. Builder Pattern
```rust
let classes = ClassBuilder::new()
    .padding(4)
    .margin(2)
    .background_color(Color::Blue(500))
    .text_color(Color::White)
    .build();
```

### 2. Trait-based Utilities
```rust
pub trait SpacingUtilities {
    fn padding(self, value: SpacingValue) -> Self;
    fn margin(self, value: SpacingValue) -> Self;
}

impl SpacingUtilities for ClassBuilder {
    // Implementation
}
```

### 3. Enum-based Values
```rust
#[derive(Debug, Clone, Copy)]
pub enum SpacingValue {
    Zero,
    Px,
    Fractional(f32),
    Integer(u32),
    Auto,
    Full,
}
```

### 4. Responsive Builders
```rust
let classes = ClassBuilder::new()
    .padding(4)
    .sm(|b| b.padding(6))
    .md(|b| b.padding(8))
    .lg(|b| b.padding(10))
    .build();
```

### 5. State Variants
```rust
let classes = ClassBuilder::new()
    .background_color(Color::Blue(500))
    .hover(|b| b.background_color(Color::Blue(600)))
    .focus(|b| b.ring_color(Color::Blue(500)))
    .build();
```

## 🎨 Design Decisions

### 1. Fluent API Design
**Decision**: Use fluent API for class building
**Rationale**: 
- More readable and intuitive
- Follows Rust conventions
- Easy to chain operations
- IDE-friendly with autocomplete

### 2. Enum-based Values
**Decision**: Use enums for utility values
**Rationale**:
- Type safety at compile time
- Clear value constraints
- Easy to extend
- Performance benefits

### 3. Trait-based Utilities
**Decision**: Organize utilities as traits
**Rationale**:
- Modular design
- Easy to extend
- Clear separation of concerns
- Reusable across different builders

### 4. Macro-based Generation
**Decision**: Use procedural macros for code generation
**Rationale**:
- Compile-time optimization
- Reduced runtime overhead
- Type safety
- Custom validation

### 5. Caching Strategy
**Decision**: Implement intelligent caching
**Rationale**:
- Performance optimization
- Reduced memory allocation
- Faster class generation
- Scalable for large applications

## 🔄 Data Flow

### 1. Class Building Flow
```
User Code → ClassBuilder → ClassSet → CSS Classes
     ↓           ↓           ↓           ↓
  Fluent API → Validation → Caching → Output
```

### 2. Responsive Flow
```
Base Classes → Responsive Builders → Breakpoint Classes → Final Output
     ↓              ↓                    ↓                ↓
  p-4, m-2 → sm:p-6, md:p-8 → lg:p-10 → Combined CSS
```

### 3. State Variant Flow
```
Base Classes → State Builders → State Classes → Final Output
     ↓             ↓               ↓             ↓
  bg-blue-500 → hover:bg-blue-600 → focus:ring-2 → Combined CSS
```

## 🚀 Performance Characteristics

### 1. Compile-time Optimization
- **Zero runtime overhead**: All classes generated at compile time
- **Dead code elimination**: Unused utilities are removed
- **Type checking**: Invalid combinations caught at compile time

### 2. Runtime Performance
- **Lazy evaluation**: Classes only generated when needed
- **Intelligent caching**: Frequently used combinations cached
- **Memory efficient**: Minimal allocations with string interning

### 3. Scalability
- **Large applications**: Handles thousands of class combinations
- **Memory usage**: Constant memory usage regardless of class count
- **Build time**: Fast compilation with incremental builds

## 🔒 Type Safety

### 1. Compile-time Validation
```rust
// This will fail at compile time
let classes = ClassBuilder::new()
    .padding(InvalidValue) // ❌ Compile error
    .build();
```

### 2. Runtime Validation
```rust
// This will fail at runtime with detailed error
let classes = ClassBuilder::new()
    .class("invalid-class") // ❌ Runtime error with suggestion
    .build();
```

### 3. IDE Support
- **Autocomplete**: Full autocomplete for all utilities
- **Error detection**: Real-time error detection
- **Documentation**: Inline documentation for all methods

## 🔧 Extensibility

### 1. Custom Utilities
```rust
pub trait CustomUtilities {
    fn custom_property(self, value: CustomValue) -> Self;
}
```

### 2. Custom Themes
```rust
let theme = Theme::new()
    .with_custom_colors(custom_colors)
    .with_custom_spacing(custom_spacing)
    .build();
```

### 3. Custom Variants
```rust
let classes = ClassBuilder::new()
    .custom_variant("dark", |b| b.background_color(Color::Black))
    .build();
```

## 📊 Metrics and Benchmarks

### 1. Performance Targets
- **Compile time**: < 100ms for typical applications
- **Runtime**: < 1ms for class generation
- **Memory**: < 1MB for 1000+ class combinations
- **Bundle size**: < 50KB for core library

### 2. Quality Metrics
- **Test coverage**: > 95% for all utilities
- **Type safety**: 100% compile-time validation
- **Documentation**: 100% API documentation
- **Examples**: Comprehensive usage examples

## 🔗 Integration Points

### 1. Framework Integrations
- **Leptos**: Signal-based reactive classes
- **Yew**: Hook-based class management
- **Dioxus**: State-based class updates

### 2. Build System Integration
- **Cargo**: Native Rust build system
- **WASM**: Browser compatibility
- **Node.js**: Server-side rendering

### 3. Development Tools
- **IDE plugins**: Enhanced development experience
- **Linting**: Custom linting rules
- **Formatting**: Code formatting support

---

**Next**: [02-project-structure.md](./02-project-structure.md) - Project organization and file structure
