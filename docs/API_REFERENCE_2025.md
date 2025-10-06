# 📚 Tailwind-RS API Reference 2025

## Overview

This document provides a comprehensive API reference for the Tailwind-RS ecosystem, covering all public APIs, their usage, and examples. The API now includes comprehensive contract-based validation for API stability and reliability.

## 🔒 API Contracts System

### Contract-Based API Design

All major APIs in Tailwind-RS implement the `ApiContract` trait, providing:

- **Input Validation**: Type-safe validation of API inputs
- **Processing**: Reliable transformation of inputs to outputs
- **Output Validation**: Guaranteed output format compliance
- **Error Handling**: Comprehensive error reporting with specific error types

### Available Contracts

- **ClassBuilderContract**: Validates class building operations
- **CssGeneratorContract**: Validates CSS generation operations
- **ThemeContract**: Validates theme configuration operations
- **ValidationContract**: Validates class validation operations

### Example Usage

```rust
use tailwind_rs_core::api_contracts::{ClassBuilderContract, ApiVersion, ClassBuilderInput};

let contract = ClassBuilderContract::new(ApiVersion::V2_0_0);
let input = ClassBuilderInput {
    classes: vec!["p-4".to_string(), "m-2".to_string()],
    responsive: vec![],
    conditional: vec![],
    custom: vec![],
};

// Validate input
contract.validate_input(&input)?;

// Process with validation
let output = contract.process(input)?;
contract.validate_output(&output)?;
```

### Contract Testing Framework

```rust
use tailwind_rs_core::api_contracts::{ContractTester, TestCase};

let mut tester = ContractTester::new();
tester.add_test_case(TestCase {
    name: "class_builder_test".to_string(),
    input: "test_input".to_string(),
    expected_output: "test_output".to_string(),
    should_fail: false,
});

let results = tester.run_tests()?;
println!("Tests passed: {}/{}", results.passed_tests, results.total_tests);
```

### Runtime Contract Validation

```rust
use tailwind_rs_core::api_contracts::{ContractValidator, ClassBuilderContract, ApiVersion};

let mut validator = ContractValidator::new();
let contract = ClassBuilderContract::new(ApiVersion::V2_0_0);
validator.add_contract("class_builder".to_string(), contract);

// Validate API calls at runtime
validator.validate_call("class_builder", your_input_data)?;
```

## 🎨 Core API (`tailwind-rs-core`)

### CSS Generator

#### `CssGenerator`

The main class for generating CSS from Tailwind classes.

```rust
use tailwind_rs_core::css_generator::CssGenerator;

// Create a new generator
let mut generator = CssGenerator::new();

// Add classes
generator.add_class("bg-blue-500")?;
generator.add_class("text-white")?;
generator.add_class("p-4")?;

// Generate CSS
let css = generator.generate_css();
```

#### Methods

- `new() -> CssGenerator`: Create a new CSS generator
- `add_class(class: &str) -> Result<(), TailwindError>`: Add a Tailwind class
- `generate_css() -> String`: Generate CSS output
- `get_classes() -> Vec<String>`: Get all added classes
- `clear()`: Clear all classes

### Class Builder

#### `ClassBuilder`

Build and manage sets of Tailwind classes.

```rust
use tailwind_rs_core::classes::ClassBuilder;

// Create a class builder
let builder = ClassBuilder::new()
    .class("bg-blue-500")
    .class("text-white")
    .class("p-4")
    .class("rounded-lg");

// Build the class set
let class_set = builder.build();

// Get CSS classes
let css_classes = class_set.to_css_classes();
```

#### Methods

- `new() -> ClassBuilder`: Create a new class builder
- `class(class: &str) -> ClassBuilder`: Add a class
- `build() -> ClassSet`: Build the class set

### Theme System

#### `ThemeConfig`

Manage theme configuration and customization.

```rust
use tailwind_rs_core::theme::ThemeConfig;

// Create a default theme
let theme = ThemeConfig::default();

// Create a custom theme
let custom_theme = ThemeConfig::new("my-theme");

// Access theme components
let spacing = theme.spacing();
let colors = theme.color_palettes();
```

#### Methods

- `new(name: &str) -> ThemeConfig`: Create a new theme
- `default() -> ThemeConfig`: Get default theme
- `spacing() -> &SpacingScale`: Get spacing configuration
- `color_palettes() -> &Vec<ColorPalette>`: Get color palettes
- `validate() -> Result<(), TailwindError>`: Validate theme configuration

### Validation System

#### `ClassValidator`

Validate Tailwind classes and detect conflicts.

```rust
use tailwind_rs_core::validation::{ClassValidator, ValidationRules};

// Create a validator
let rules = ValidationRules::new();
let validator = ClassValidator::new(rules);

// Validate a class
let result = validator.validate_class("bg-blue-500");
```

#### Methods

- `new(rules: ValidationRules) -> ClassValidator`: Create a new validator
- `validate_class(class: &str) -> ValidationResult`: Validate a single class
- `validate_classes(classes: &[String]) -> Vec<ValidationResult>`: Validate multiple classes

## 🔧 Framework Integration

### Leptos Integration (`tailwind-rs-leptos`)

#### Basic Usage

```rust
use tailwind_rs_leptos::*;
use leptos::*;

#[component]
fn MyComponent() -> impl IntoView {
    view! {
        <div class="bg-blue-500 text-white p-4 rounded-lg">
            "Hello, Tailwind-RS!"
        </div>
    }
}
```

#### Advanced Usage

```rust
use tailwind_rs_leptos::*;
use leptos::*;

#[component]
fn AdvancedComponent() -> impl IntoView {
    let (is_hovered, set_hovered) = create_signal(false);
    
    view! {
        <div 
            class=move || {
                if is_hovered.get() {
                    "bg-blue-600 text-white p-4 rounded-lg transition-colors"
                } else {
                    "bg-blue-500 text-white p-4 rounded-lg transition-colors"
                }
            }
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| set_hovered.set(false)
        >
            "Interactive Component"
        </div>
    }
}
```

### Yew Integration (`tailwind-rs-yew`)

#### Basic Usage

```rust
use tailwind_rs_yew::*;
use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
    html! {
        <div class="bg-blue-500 text-white p-4 rounded-lg">
            {"Hello, Tailwind-RS!"}
        </div>
    }
}
```

#### Advanced Usage

```rust
use tailwind_rs_yew::*;
use yew::prelude::*;

#[function_component]
fn AdvancedComponent() -> Html {
    let hovered = use_state(|| false);
    
    let onmouseenter = {
        let hovered = hovered.clone();
        Callback::from(move |_| hovered.set(true))
    };
    
    let onmouseleave = {
        let hovered = hovered.clone();
        Callback::from(move |_| hovered.set(false))
    };
    
    html! {
        <div 
            class={if *hovered {
                "bg-blue-600 text-white p-4 rounded-lg transition-colors"
            } else {
                "bg-blue-500 text-white p-4 rounded-lg transition-colors"
            }}
            {onmouseenter}
            {onmouseleave}
        >
            {"Interactive Component"}
        </div>
    }
}
```

### Dioxus Integration (`tailwind-rs-dioxus`)

#### Basic Usage

```rust
use tailwind_rs_dioxus::*;
use dioxus::prelude::*;

#[component]
fn MyComponent() -> Element {
    rsx! {
        div { class: "bg-blue-500 text-white p-4 rounded-lg",
            "Hello, Tailwind-RS!"
        }
    }
}
```

#### Advanced Usage

```rust
use tailwind_rs_dioxus::*;
use dioxus::prelude::*;

#[component]
fn AdvancedComponent() -> Element {
    let mut hovered = use_signal(|| false);
    
    rsx! {
        div { 
            class: if hovered() {
                "bg-blue-600 text-white p-4 rounded-lg transition-colors"
            } else {
                "bg-blue-500 text-white p-4 rounded-lg transition-colors"
            },
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            "Interactive Component"
        }
    }
}
```

## 🎨 New Parser APIs

### Basic Transforms Parser

#### `BasicTransformsParser`

Handles `translate-x-*` and `translate-y-*` classes.

```rust
use tailwind_rs_core::css_generator::parsers::BasicTransformsParser;

let parser = BasicTransformsParser::new();

// Parse translate classes
let result = parser.parse_class("translate-x-1");
let result = parser.parse_class("translate-y-2");
let result = parser.parse_class("translate-x-px");
```

#### Supported Classes

- `translate-x-{value}`: Horizontal translation
- `translate-y-{value}`: Vertical translation
- Values: `0`, `1`, `2`, `4`, `8`, `px`, `0.5`, `1.5`, `2.5`, `3.5`

### Scale Parser

#### `ScaleParser`

Handles `scale-x-*` and `scale-y-*` classes.

```rust
use tailwind_rs_core::css_generator::parsers::ScaleParser;

let parser = ScaleParser::new();

// Parse scale classes
let result = parser.parse_class("scale-x-50");
let result = parser.parse_class("scale-y-75");
let result = parser.parse_class("scale-x-100");
```

#### Supported Classes

- `scale-x-{value}`: Horizontal scaling
- `scale-y-{value}`: Vertical scaling
- Values: `50`, `75`, `90`, `95`, `100`, `105`, `110`, `125`, `150`

## 🔧 Advanced Features

### Performance Optimization

#### `PerformanceMonitor`

Monitor and optimize performance.

```rust
use tailwind_rs_core::utilities::advanced_performance_optimization::PerformanceMonitor;

let monitor = PerformanceMonitor::new();

// Get performance metrics
let cpu_usage = monitor.get_metric("cpu_usage");
let memory_usage = monitor.get_metric("memory_usage");
```

#### `BundleSplitter`

Split CSS bundles for optimal loading.

```rust
use tailwind_rs_core::utilities::advanced_performance_optimization::BundleSplitter;

let splitter = BundleSplitter::new()
    .max_chunk_size(1024 * 1024) // 1MB
    .split_strategy(SplitStrategy::FeatureBased);

let chunks = splitter.split_css(css);
```

### Memory Optimization

#### `MemoryOptimizer`

Optimize memory usage.

```rust
use tailwind_rs_core::utilities::advanced_performance_optimization::MemoryOptimizer;

let optimizer = MemoryOptimizer::new()
    .max_memory_usage(50 * 1024 * 1024) // 50MB
    .gc_threshold(0.8);

optimizer.optimize_memory();
```

## 📊 Error Handling

### `TailwindError`

Comprehensive error handling for all operations.

```rust
use tailwind_rs_core::error::TailwindError;

match generator.add_class("invalid-class") {
    Ok(_) => println!("Class added successfully"),
    Err(TailwindError::InvalidClass(class)) => {
        println!("Invalid class: {}", class);
    }
    Err(TailwindError::ParserError(msg)) => {
        println!("Parser error: {}", msg);
    }
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

### Error Types

- `InvalidClass(String)`: Invalid Tailwind class
- `ParserError(String)`: Parser-specific error
- `Theme(String)`: Theme-related error
- `Validation(String)`: Validation error
- `IO(String)`: File I/O error

## 🚀 Best Practices

## 🎯 **ADVANCED FEATURES API REFERENCE**

### Container Queries

Container queries allow responsive design based on container size rather than viewport size.

```rust
use tailwind_rs_core::{CssGenerator, Result};

// Create generator
let mut generator = CssGenerator::new();

// Add container query classes
let css = generator.process_element_classes(&[
    "@container-sm:bg-green-500",
    "@container-md:min-w-300:text-blue-600",
    "@container-lg:flex-col"
])?;
```

**Container Query Syntax:**
- `@container-{size}:{utility}` - Basic container queries
- `@container-{name}:{condition}:{utility}` - Named containers with conditions

**Supported Sizes:**
- `sm`, `md`, `lg`, `xl`, `2xl` - Standard breakpoints
- Custom conditions: `min-w-300`, `max-h-500`, etc.

### Arbitrary Value Variants

Arbitrary values allow dynamic class generation with custom values.

```rust
let css = generator.process_element_classes(&[
    "[data-state=\"open\"]:bg-green-500",
    "[aria-expanded=\"true\"]:text-blue-600",
    "[data-count=\"5\"]:opacity-100"
])?;
```

**Supported Arbitrary Variants:**
- Data attributes: `[data-*]:{utility}`
- ARIA attributes: `[aria-*]:{utility}`
- Custom attributes: `[custom-attr]:{utility}`

### Device-Specific Variants

Device-specific variants target specific device capabilities and preferences.

```rust
let css = generator.process_element_classes(&[
    "motion-reduce:opacity-50",      // Reduced motion preference
    "contrast-more:text-lg",         // High contrast preference
    "pointer-coarse:p-6",           // Touch devices
    "hover-none:hidden",            // No hover capability
    "orientation-landscape:flex-row" // Landscape orientation
])?;
```

**Device Variants:**
- **Motion**: `motion-reduce`, `motion-safe`
- **Contrast**: `contrast-more`, `contrast-less`, `contrast-custom`
- **Pointer**: `pointer-coarse`, `pointer-fine`, `pointer-none`
- **Hover**: `hover-none`, `hover-hover`
- **Color Gamut**: `color-gamut-srgb`, `color-gamut-p3`, `color-gamut-rec2020`
- **Orientation**: `orientation-portrait`, `orientation-landscape`
- **Media**: `print`, `screen`

### CSS Functions (@apply, @layer, @import)

CSS functions enable advanced CSS composition and organization.

#### @apply Directive

```rust
use tailwind_rs_core::css_functions::CssFunctionsProcessor;

let mut processor = CssFunctionsProcessor::new();

// Apply utility classes within custom CSS
let css = processor.process_apply("bg-blue-500 text-white p-4")?;
// Returns: "background-color: #3b82f6; color: #ffffff; padding: 1rem;"
```

#### @layer Directive

```rust
// Organize CSS into cascade layers
let css = processor.process_layer("components", ".btn { color: blue; }")?;
// Returns: "@layer components { .btn { color: blue; } }"
```

**Supported Layers:**
- `base` - Reset and base styles
- `components` - Component styles
- `utilities` - Utility classes

#### @import Directive

```rust
// Process CSS imports
let css = processor.process_import("url('https://fonts.googleapis.com/css2?family=Inter&display=swap')")?;
// Returns: "@import url('https://fonts.googleapis.com/css2?family=Inter&display=swap');"
```

### Transform System

Advanced transform system using CSS custom properties for combination support.

```rust
let css = generator.process_element_classes(&[
    "scale-110",      // --tw-scale-x: 1.1; --tw-scale-y: 1.1;
    "rotate-3",       // --tw-rotate: 3deg;
    "translate-x-2",  // --tw-translate-x: 0.5rem;
    "transform"       // Applies all transforms via var(--tw-transform)
])?;
```

**Transform Properties:**
- Scale: `scale-{value}`, `scale-x-{value}`, `scale-y-{value}`
- Rotate: `rotate-{value}`, `rotate-x-{value}`, etc.
- Translate: `translate-{value}`, `translate-x-{value}`, etc.
- Skew: `skew-{value}`, `skew-x-{value}`, etc.
- Origin: `origin-{value}`

### Plugin System

Extensible plugin system for custom utilities and features.

```rust
use tailwind_rs_core::css_generator::plugin_system::{Plugin, PluginManager};

// Implement custom plugin
struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn name(&self) -> &str { "custom" }
    fn register_utilities(&self, _manager: &mut PluginManager) {
        // Register custom utilities
    }
}

// Register plugin
let mut generator = CssGenerator::new();
generator.register_plugin(Box::new(CustomPlugin))?;
```

## 📚 **PERFORMANCE & OPTIMIZATION**

### Performance Monitoring

```rust
use std::time::Instant;

let start = Instant::now();
// Your CSS generation code
let duration = start.elapsed();
println!("CSS generation took: {:?}", duration);
```

### Memory Optimization

```rust
// Enable memory optimization
let config = CssGenerationConfig {
    memory_optimization: true,
    ..Default::default()
};
let generator = CssGenerator::with_config(config);

// Clear unused data
generator.clear_cache();
```

### Caching Strategies

```rust
// Rule caching for repeated classes
let mut generator = CssGenerator::new();

// First generation (creates cache)
let css1 = generator.process_element_classes(&["bg-blue-500", "text-white"])?;

// Second generation (uses cache)
let css2 = generator.process_element_classes(&["bg-blue-500", "text-white"])?;

// Results are identical but second call is faster
assert_eq!(css1, css2);
```

## 🔧 **FRAMEWORK INTEGRATIONS**

### Leptos Integration

```rust
use tailwind_rs_leptos::tailwind_classes;

// In your Leptos component
#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView {
    let classes = tailwind_classes!("bg-blue-500 hover:bg-blue-600 p-4");

    view! { cx,
        <div class=classes>
            "Hello World"
        </div>
    }
}
```

### Dioxus Integration

```rust
use tailwind_rs_dioxus::tw;

// In your Dioxus component
fn MyComponent() -> Element {
    let classes = tw!("flex items-center justify-center p-4");

    rsx! {
        div { class: classes,
            "Hello World"
        }
    }
}
```

### Yew Integration

```rust
use tailwind_rs_yew::classes;

// In your Yew component
pub struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let classes = classes!("bg-red-500 text-white p-4");

        html! {
            <div class={classes}>
                {"Hello World"}
            </div>
        }
    }
}
```

## 🧪 **TESTING & QUALITY ASSURANCE**

### Integration Testing

```rust
#[cfg(test)]
mod tests {
    use tailwind_rs_core::CssGenerator;

    #[test]
    fn test_css_generation() {
        let mut generator = CssGenerator::new();
        let css = generator.process_element_classes(&["bg-blue-500", "text-white"]).unwrap();

        assert!(css.contains("background-color"));
        assert!(css.contains("color"));
    }
}
```

### Performance Testing

```rust
#[test]
fn benchmark_css_generation() {
    let mut generator = CssGenerator::new();

    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _css = generator.process_element_classes(&["bg-blue-500"]).unwrap();
    }
    let duration = start.elapsed();

    assert!(duration < std::time::Duration::from_millis(100));
}
```

## 📋 **MIGRATION GUIDE**

### From Legacy API

```rust
// Old API
let mut generator = CssGenerator::new();
generator.add_class("bg-blue-500")?;

// New API (recommended)
let css = generator.process_element_classes(&["bg-blue-500"])?;
```

### Configuration Updates

```rust
// New configuration options
let config = CssGenerationConfig {
    memory_optimization: true,
    custom_breakpoints: HashMap::new(),
    enable_container_queries: true,
    enable_arbitrary_values: true,
    ..Default::default()
};
```

## 🚀 **PRODUCTION DEPLOYMENT**

### Build Optimization

```bash
# Release build with optimizations
cargo build --release --features production

# With custom feature flags
cargo build --release --features "leptos,dioxus,yew"
```

### CI/CD Integration

```yaml
# .github/workflows/release.yml
- name: Build and test
  run: |
    cargo check
    cargo test
    cargo build --release
    cargo doc --no-deps
```

### Performance Benchmarking

```rust
// Continuous performance monitoring
#[cfg(feature = "benchmarks")]
mod benchmarks {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn benchmark_css_generation(c: &mut Criterion) {
        c.bench_function("css_generation", |b| {
            b.iter(|| {
                let mut generator = CssGenerator::new();
                black_box(generator.process_element_classes(&["bg-blue-500"]))
            })
        });
    }

    criterion_group!(benches, benchmark_css_generation);
    criterion_main!(benches);
}
```
4. **Test Integration**: Test framework integration thoroughly

## 📚 Examples

### Complete Example

```rust
use tailwind_rs_core::css_generator::CssGenerator;
use tailwind_rs_core::classes::ClassBuilder;
use tailwind_rs_core::theme::ThemeConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a theme
    let theme = ThemeConfig::new("my-theme");
    
    // Create a CSS generator
    let mut generator = CssGenerator::new();
    
    // Add classes
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    generator.add_class("p-4")?;
    generator.add_class("rounded-lg")?;
    generator.add_class("hover:bg-blue-600")?;
    
    // Generate CSS
    let css = generator.generate_css();
    
    println!("Generated CSS: {}", css);
    
    Ok(())
}
```

### Framework Example (Leptos)

```rust
use tailwind_rs_leptos::*;
use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100">
            <header class="bg-white shadow">
                <div class="max-w-7xl mx-auto py-6 px-4">
                    <h1 class="text-3xl font-bold text-gray-900">
                        "Tailwind-RS App"
                    </h1>
                </div>
            </header>
            <main class="max-w-7xl mx-auto py-6 px-4">
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-xl font-semibold mb-4">"Welcome"</h2>
                    <p class="text-gray-600">
                        "This is a Tailwind-RS application built with Leptos."
                    </p>
                </div>
            </main>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
```

---

*Last Updated: January 2025*
*API Version: 0.15.4*
