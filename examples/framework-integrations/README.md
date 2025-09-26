# 🎨 Tailwind-RS Framework Integration Examples

This directory contains comprehensive examples demonstrating how to use Tailwind-RS with different Rust web frameworks and environments.

## 📚 **Available Examples**

### 🚀 **Leptos Integration**
**File:** `leptos_comprehensive_example.rs`

A complete example showcasing:
- **Reactive Components**: Signal-based state management
- **Dynamic Class Building**: Classes that respond to state changes
- **Dark Mode Support**: Theme switching with CSS classes
- **Interactive Elements**: Buttons with hover and active states
- **Advanced Styling**: Gradients, transforms, and animations
- **Component Architecture**: Reusable styled components

**Key Features:**
- Reactive class building with `ClassBuilder`
- State management with Leptos signals
- Dark mode toggle functionality
- Interactive counter with animations
- Custom button and card components

### ⚛️ **Yew Integration**
**File:** `yew_comprehensive_example.rs`

A complete example showcasing:
- **Component-Based Architecture**: Props and state management
- **Event Handling**: Click events and callbacks
- **Styling System**: Type-safe class building
- **Responsive Design**: Mobile-first approach
- **Performance**: Optimized rendering

**Key Features:**
- Component props and state management
- Event handling with callbacks
- Type-safe class building
- Responsive design patterns
- Performance optimizations

### 🌐 **Dioxus Integration**
**File:** `dioxus_comprehensive_example.rs`

A complete example showcasing:
- **Cross-Platform UI**: Desktop, web, and mobile support
- **Reactive State**: Signal-based reactivity
- **Component System**: Props and children
- **Styling**: Advanced CSS generation
- **Performance**: Optimized for all platforms

**Key Features:**
- Cross-platform compatibility
- Signal-based reactivity
- Component composition
- Advanced styling capabilities
- Platform-specific optimizations

### 🔧 **WASM Integration**
**File:** `wasm_comprehensive_example.rs`

A complete example showcasing:
- **Direct Browser Integration**: No framework dependencies
- **Performance**: Optimized WASM compilation
- **DOM Manipulation**: Direct browser API usage
- **CSS Injection**: Dynamic style generation
- **Event Handling**: JavaScript interop

**Key Features:**
- Direct browser API integration
- WASM-optimized performance
- Dynamic CSS generation
- JavaScript interop
- Event handling

### 🛠️ **CLI Integration**
**File:** `cli_comprehensive_example.rs`

A complete example showcasing:
- **File Scanning**: Recursive directory scanning
- **CSS Generation**: Automated CSS creation
- **Optimization**: CSS minification and optimization
- **Build Integration**: Project building workflows
- **Watch Mode**: File change monitoring

**Key Features:**
- File scanning and class extraction
- CSS generation and optimization
- Build system integration
- Watch mode for development
- Project analysis and reporting

## 🚀 **Getting Started**

### **Prerequisites**
- Rust 1.70+
- Cargo
- Framework-specific dependencies (Leptos, Yew, Dioxus)

### **Running Examples**

#### **Leptos Example**
```bash
# Add to Cargo.toml
[dependencies]
tailwind-rs-leptos = "0.15.0"
leptos = "0.8"

# Run the example
cargo run --example leptos_comprehensive_example
```

#### **Yew Example**
```bash
# Add to Cargo.toml
[dependencies]
tailwind-rs-yew = "0.15.0"
yew = "0.21"

# Run the example
cargo run --example yew_comprehensive_example
```

#### **Dioxus Example**
```bash
# Add to Cargo.toml
[dependencies]
tailwind-rs-dioxus = "0.15.0"
dioxus = "0.3"

# Run the example
cargo run --example dioxus_comprehensive_example
```

#### **WASM Example**
```bash
# Add to Cargo.toml
[dependencies]
tailwind-rs-wasm = "0.15.0"
wasm-bindgen = "0.2"
web-sys = "0.3"

# Build for WASM
cargo build --target wasm32-unknown-unknown --example wasm_comprehensive_example
```

#### **CLI Example**
```bash
# Add to Cargo.toml
[dependencies]
tailwind-rs-cli = "0.15.0"

# Run the example
cargo run --example cli_comprehensive_example -- scan src/ -r -v
```

## 🎯 **Key Concepts Demonstrated**

### **1. Class Building**
All examples show how to use `ClassBuilder` for type-safe class construction:

```rust
let classes = ClassBuilder::new()
    .class("px-4")
    .class("py-2")
    .class("bg-blue-600")
    .class("hover:bg-blue-700")
    .class("text-white")
    .class("rounded-lg")
    .build();
```

### **2. Reactive Styling**
Framework-specific reactive styling patterns:

**Leptos:**
```rust
let button_classes = move || {
    ClassBuilder::new()
        .class("px-6")
        .class("py-3")
        .class("bg-blue-600")
        .class("hover:bg-blue-700")
        .build()
};
```

**Yew:**
```rust
let button_classes = ClassBuilder::new()
    .class("px-6")
    .class("py-3")
    .class("bg-blue-600")
    .class("hover:bg-blue-700")
    .build();
```

**Dioxus:**
```rust
let button_classes = ClassBuilder::new()
    .class("px-6")
    .class("py-3")
    .class("bg-blue-600")
    .class("hover:bg-blue-700")
    .build();
```

### **3. Component Architecture**
Reusable styled components with props:

```rust
#[component]
fn StyledButton(
    children: Children,
    #[prop(optional)] variant: Option<String>,
) -> impl IntoView {
    let classes = ClassBuilder::new()
        .class("px-6")
        .class("py-3")
        .class("rounded-lg")
        .class("transition-colors")
        .build();
    
    view! { <button class=classes>{children()}</button> }
}
```

### **4. State Management**
Framework-specific state management patterns:

**Leptos (Signals):**
```rust
let (count, set_count) = create_signal(0);
let classes = move || {
    ClassBuilder::new()
        .class("text-6xl")
        .class("font-bold")
        .build()
};
```

**Yew (Hooks):**
```rust
let counter = use_state(|| 0);
let classes = ClassBuilder::new()
    .class("text-6xl")
    .class("font-bold")
    .build();
```

**Dioxus (Signals):**
```rust
let mut counter = use_signal(|| 0);
let classes = ClassBuilder::new()
    .class("text-6xl")
    .class("font-bold")
    .build();
```

## 🎨 **Styling Features Demonstrated**

### **Layout & Spacing**
- Container and grid systems
- Responsive breakpoints
- Spacing utilities (padding, margin)

### **Typography**
- Font sizes and weights
- Line heights and text colors
- Text alignment and decoration

### **Colors & Backgrounds**
- Color palettes and shades
- Gradient backgrounds
- Dark mode support

### **Effects & Animations**
- Hover and focus states
- Transform animations
- Transition effects
- Shadow and blur effects

### **Responsive Design**
- Mobile-first approach
- Breakpoint-specific styling
- Responsive utilities

## 🔧 **Advanced Features**

### **Dark Mode Support**
All examples include dark mode support:

```rust
let container_classes = ClassBuilder::new()
    .class("bg-white")
    .class("dark:bg-gray-800")
    .class("text-gray-900")
    .class("dark:text-white")
    .build();
```

### **Interactive States**
Hover, focus, and active states:

```rust
let button_classes = ClassBuilder::new()
    .class("bg-blue-600")
    .class("hover:bg-blue-700")
    .class("focus:bg-blue-800")
    .class("active:bg-blue-900")
    .build();
```

### **Transform Animations**
Scale, rotate, and translate effects:

```rust
let card_classes = ClassBuilder::new()
    .class("transform")
    .class("hover:scale-105")
    .class("active:scale-95")
    .class("transition-transform")
    .class("duration-200")
    .build();
```

## 📊 **Performance Considerations**

### **Class Building Performance**
- Efficient class construction
- Minimal memory allocation
- Optimized string operations

### **Framework Integration**
- Framework-specific optimizations
- Minimal overhead
- Fast compilation

### **WASM Performance**
- Optimized for browser execution
- Small bundle sizes
- Fast runtime performance

## 🚀 **Next Steps**

1. **Explore the Examples**: Run each example to see the features in action
2. **Customize Styling**: Modify the examples to match your design needs
3. **Build Your App**: Use the patterns shown to build your own application
4. **Contribute**: Add your own examples and improvements

## 📚 **Additional Resources**

- [Tailwind-RS Documentation](https://docs.rs/tailwind-rs-core)
- [Framework Documentation](https://leptos.dev/) | [Yew](https://yew.rs/) | [Dioxus](https://dioxuslabs.com/)
- [WASM Guide](https://rustwasm.github.io/docs/book/)
- [CLI Documentation](https://docs.rs/tailwind-rs-cli)

---

**🎉 Happy Styling with Tailwind-RS!** 🎨
