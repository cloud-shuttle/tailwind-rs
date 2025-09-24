# 📁 Project Structure

## 📋 Overview

This document outlines the complete project structure for the Tailwind CSS v4.1 Rust implementation. It covers the organization of crates, modules, and files to ensure maintainability and scalability.

## 🏗️ Workspace Structure

```
tailwind-rs/
├── Cargo.toml                          # Workspace configuration
├── README.md                           # Project overview
├── TAILWIND_V4_IMPLEMENTATION_PLAN.md  # High-level implementation plan
├── docs/                               # Documentation
│   ├── technical-implementation/       # Technical implementation guides
│   ├── api-reference/                  # API documentation
│   └── examples/                       # Usage examples
├── crates/                             # Core crates
│   ├── tailwind-rs-core/              # Core library
│   ├── tailwind-rs-macros/            # Procedural macros
│   ├── tailwind-rs-leptos/            # Leptos integration
│   ├── tailwind-rs-yew/               # Yew integration
│   ├── tailwind-rs-dioxus/            # Dioxus integration
│   └── tailwind-rs-testing/           # Testing utilities
├── examples/                           # Example applications
├── tests/                              # Integration tests
└── benches/                            # Performance benchmarks
```

## 📦 Core Crates

### 1. tailwind-rs-core
**Purpose**: Core library with all utility classes and systems

```
crates/tailwind-rs-core/
├── Cargo.toml                          # Crate configuration
├── src/
│   ├── lib.rs                          # Main library entry point
│   ├── classes.rs                      # Class management system
│   ├── error.rs                        # Error types and handling
│   ├── validation.rs                   # Class validation system
│   ├── theme.rs                        # Theme system
│   ├── responsive.rs                   # Responsive utilities
│   ├── custom_variant.rs               # Custom variant system
│   ├── performance.rs                  # Performance optimization
│   ├── utils.rs                        # Utility functions
│   ├── utilities/                      # Utility class implementations
│   │   ├── mod.rs                      # Module declarations
│   │   ├── spacing.rs                  # Padding, margin utilities
│   │   ├── sizing.rs                   # Width, height utilities
│   │   ├── typography.rs               # Font, text utilities
│   │   ├── colors.rs                   # Color system
│   │   ├── layout.rs                   # Display, position utilities
│   │   ├── flexbox.rs                  # Flexbox utilities
│   │   ├── grid.rs                     # Grid utilities
│   │   ├── backgrounds.rs              # Background utilities
│   │   ├── borders.rs                  # Border utilities
│   │   ├── effects.rs                  # Shadow, opacity utilities
│   │   ├── filters.rs                  # Filter utilities
│   │   ├── transitions.rs              # Transition utilities
│   │   ├── transforms.rs               # Transform utilities
│   │   └── interactivity.rs            # Interactive utilities
│   ├── macros/                         # Macro implementations
│   │   ├── mod.rs                      # Macro module
│   │   ├── utility_macro.rs            # Utility generation macro
│   │   ├── builder_macro.rs            # Builder pattern macro
│   │   └── validation_macro.rs         # Validation macro
│   ├── tests/                          # Unit tests
│   │   ├── mod.rs                      # Test module
│   │   ├── class_tests.rs              # Class system tests
│   │   ├── utility_tests.rs            # Utility tests
│   │   ├── validation_tests.rs         # Validation tests
│   │   └── integration_tests.rs        # Integration tests
│   └── examples/                       # Example usage
│       ├── basic_usage.rs              # Basic usage examples
│       ├── responsive_example.rs       # Responsive examples
│       ├── state_variants.rs           # State variant examples
│       └── custom_utilities.rs         # Custom utility examples
├── tests/                              # Integration tests
│   ├── class_management.rs             # Class management tests
│   ├── utility_combinations.rs         # Utility combination tests
│   └── performance_tests.rs            # Performance tests
└── benches/                            # Performance benchmarks
    ├── class_generation.rs             # Class generation benchmarks
    ├── validation_benchmarks.rs        # Validation benchmarks
    └── memory_usage.rs                 # Memory usage benchmarks
```

### 2. tailwind-rs-macros
**Purpose**: Procedural macros for compile-time code generation

```
crates/tailwind-rs-macros/
├── Cargo.toml                          # Crate configuration
├── src/
│   ├── lib.rs                          # Macro library entry point
│   ├── utility_macro.rs                # Utility generation macro
│   ├── builder_macro.rs                # Builder pattern macro
│   ├── validation_macro.rs             # Validation macro
│   ├── responsive_macro.rs             # Responsive macro
│   ├── state_macro.rs                  # State variant macro
│   └── utils.rs                        # Macro utilities
├── tests/                              # Macro tests
│   ├── utility_macro_tests.rs          # Utility macro tests
│   ├── builder_macro_tests.rs          # Builder macro tests
│   └── validation_macro_tests.rs       # Validation macro tests
└── examples/                           # Macro examples
    ├── custom_utility.rs               # Custom utility macro example
    └── responsive_macro.rs             # Responsive macro example
```

### 3. tailwind-rs-leptos
**Purpose**: Leptos 0.8.8 integration

```
crates/tailwind-rs-leptos/
├── Cargo.toml                          # Crate configuration
├── src/
│   ├── lib.rs                          # Library entry point
│   ├── components.rs                   # Leptos components
│   ├── signal_manager.rs               # Signal-based class management
│   ├── reactive_builder.rs             # Reactive class builder
│   ├── dynamic_classes.rs              # Dynamic class management
│   ├── utils.rs                        # Utility functions
│   └── tests/                          # Unit tests
│       ├── component_tests.rs          # Component tests
│       ├── signal_tests.rs             # Signal tests
│       └── reactive_tests.rs           # Reactive tests
├── tests/                              # Integration tests
│   ├── leptos_integration.rs           # Leptos integration tests
│   └── reactive_behavior.rs            # Reactive behavior tests
└── examples/                           # Leptos examples
    ├── basic_component.rs              # Basic component example
    ├── reactive_styling.rs             # Reactive styling example
    └── dynamic_classes.rs              # Dynamic classes example
```

### 4. tailwind-rs-yew
**Purpose**: Yew integration

```
crates/tailwind-rs-yew/
├── Cargo.toml                          # Crate configuration
├── src/
│   ├── lib.rs                          # Library entry point
│   ├── components.rs                   # Yew components
│   ├── hooks.rs                        # Yew hooks
│   ├── class_manager.rs                # Class management
│   ├── props.rs                        # Component props
│   ├── utils.rs                        # Utility functions
│   └── tests/                          # Unit tests
│       ├── component_tests.rs          # Component tests
│       ├── hook_tests.rs               # Hook tests
│       └── class_manager_tests.rs      # Class manager tests
├── tests/                              # Integration tests
│   ├── yew_integration.rs              # Yew integration tests
│   └── hook_behavior.rs                # Hook behavior tests
└── examples/                           # Yew examples
    ├── basic_component.rs              # Basic component example
    ├── hook_usage.rs                   # Hook usage example
    └── dynamic_styling.rs              # Dynamic styling example
```

### 5. tailwind-rs-dioxus
**Purpose**: Dioxus integration

```
crates/tailwind-rs-dioxus/
├── Cargo.toml                          # Crate configuration
├── src/
│   ├── lib.rs                          # Library entry point
│   ├── components.rs                   # Dioxus components
│   ├── state_manager.rs                # State-based class management
│   ├── class_builder.rs                # Class builder
│   ├── utils.rs                        # Utility functions
│   └── tests/                          # Unit tests
│       ├── component_tests.rs          # Component tests
│       ├── state_tests.rs              # State tests
│       └── builder_tests.rs            # Builder tests
├── tests/                              # Integration tests
│   ├── dioxus_integration.rs           # Dioxus integration tests
│   └── state_behavior.rs               # State behavior tests
└── examples/                           # Dioxus examples
    ├── basic_component.rs              # Basic component example
    ├── state_management.rs             # State management example
    └── dynamic_classes.rs              # Dynamic classes example
```

### 6. tailwind-rs-testing
**Purpose**: Testing utilities and helpers

```
crates/tailwind-rs-testing/
├── Cargo.toml                          # Crate configuration
├── src/
│   ├── lib.rs                          # Library entry point
│   ├── test_helpers.rs                 # Test helper functions
│   ├── mock_components.rs              # Mock components
│   ├── test_utilities.rs               # Test utilities
│   ├── property_tests.rs               # Property-based testing
│   └── benchmarks.rs                   # Benchmark utilities
├── tests/                              # Testing tests
│   ├── helper_tests.rs                 # Helper function tests
│   └── mock_tests.rs                   # Mock component tests
└── examples/                           # Testing examples
    ├── property_testing.rs             # Property testing example
    └── benchmark_example.rs            # Benchmark example
```

## 📁 Documentation Structure

```
docs/
├── README.md                           # Documentation overview
├── technical-implementation/           # Technical implementation guides
│   ├── README.md                       # Implementation guide index
│   ├── 01-architecture-overview.md     # Architecture overview
│   ├── 02-project-structure.md         # Project structure (this file)
│   ├── 03-design-patterns.md           # Design patterns
│   ├── 04-utility-classes.md           # Utility class implementation
│   ├── 05-spacing-system.md            # Spacing system
│   ├── 06-sizing-system.md             # Sizing system
│   ├── 07-typography-system.md         # Typography system
│   ├── 08-color-system.md              # Color system
│   ├── 09-layout-system.md             # Layout system
│   ├── 10-flexbox-system.md            # Flexbox system
│   ├── 11-grid-system.md               # Grid system
│   ├── 12-background-system.md         # Background system
│   ├── 13-border-system.md             # Border system
│   ├── 14-effects-system.md            # Effects system
│   ├── 15-filter-system.md             # Filter system
│   ├── 16-transition-system.md         # Transition system
│   ├── 17-transform-system.md          # Transform system
│   ├── 18-interactivity-system.md      # Interactivity system
│   ├── 19-responsive-system.md         # Responsive system
│   ├── 20-state-variants.md            # State variants
│   ├── 21-testing-strategy.md          # Testing strategy
│   ├── 22-validation-system.md         # Validation system
│   ├── 23-property-based-testing.md    # Property-based testing
│   ├── 24-performance-optimization.md  # Performance optimization
│   ├── 25-caching-strategy.md          # Caching strategy
│   ├── 26-memory-optimization.md       # Memory optimization
│   ├── 27-macro-system.md              # Macro system
│   ├── 28-code-generation.md           # Code generation
│   ├── 29-development-workflow.md      # Development workflow
│   ├── 30-api-documentation.md         # API documentation
│   ├── 31-usage-examples.md            # Usage examples
│   ├── 32-migration-guide.md           # Migration guide
│   ├── 33-leptos-integration.md        # Leptos integration
│   ├── 34-yew-integration.md           # Yew integration
│   └── 35-dioxus-integration.md        # Dioxus integration
├── api-reference/                      # API documentation
│   ├── README.md                       # API reference index
│   ├── core/                           # Core API reference
│   ├── leptos/                         # Leptos API reference
│   ├── yew/                            # Yew API reference
│   └── dioxus/                         # Dioxus API reference
└── examples/                           # Usage examples
    ├── README.md                       # Examples index
    ├── basic-usage/                    # Basic usage examples
    ├── responsive/                     # Responsive examples
    ├── state-variants/                 # State variant examples
    ├── custom-utilities/               # Custom utility examples
    └── framework-integration/          # Framework integration examples
```

## 🔧 Configuration Files

### 1. Workspace Cargo.toml
```toml
[workspace]
resolver = "2"
members = [
    "crates/tailwind-rs-core",
    "crates/tailwind-rs-macros",
    "crates/tailwind-rs-leptos",
    "crates/tailwind-rs-yew",
    "crates/tailwind-rs-dioxus",
    "crates/tailwind-rs-testing",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Tailwind RS Contributors"]
license = "MIT"
repository = "https://github.com/tailwind-rs/tailwind-rs"
homepage = "https://tailwind-rs.dev"
documentation = "https://docs.tailwind-rs.dev"
keywords = ["tailwind", "css", "rust", "web", "styling"]
categories = ["web-programming", "css"]

[workspace.dependencies]
# Core dependencies
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "1.0"
toml = "0.8"
glob = "0.3"

# Testing dependencies
wasm-bindgen-test = "0.3"
proptest = "1.0"
criterion = "0.5"

# Framework dependencies
leptos = { version = "0.8", features = ["csr", "hydrate", "ssr"] }
yew = { version = "0.21", features = ["csr"] }
dioxus = { version = "0.4", features = ["web"] }
```

### 2. Core Crate Cargo.toml
```toml
[package]
name = "tailwind-rs-core"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
documentation.workspace = true
keywords.workspace = true
categories.workspace = true
description = "Core types and utilities for tailwind-rs"

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
toml = { workspace = true }
glob = { workspace = true }
regex = "1.10"
lru = "0.12"
tokio = { version = "1.0", features = ["sync", "rt", "rt-multi-thread", "test-util"] }

[dev-dependencies]
tailwind-rs-testing = { workspace = true }
wasm-bindgen-test = { workspace = true }
proptest = { workspace = true }

[lib]
name = "tailwind_rs_core"
path = "src/lib.rs"

[[example]]
name = "basic_usage"
path = "examples/basic_usage.rs"

[[example]]
name = "responsive_example"
path = "examples/responsive_example.rs"

[[example]]
name = "state_variants"
path = "examples/state_variants.rs"

[[example]]
name = "custom_utilities"
path = "examples/custom_utilities.rs"
```

## 🧪 Testing Structure

### 1. Unit Tests
- **Location**: `src/tests/` in each crate
- **Purpose**: Test individual functions and methods
- **Coverage**: > 95% for all utilities

### 2. Integration Tests
- **Location**: `tests/` in each crate
- **Purpose**: Test component interactions
- **Coverage**: All major use cases

### 3. Property-based Tests
- **Location**: `crates/tailwind-rs-testing/src/property_tests.rs`
- **Purpose**: Test utility combinations and edge cases
- **Tool**: proptest

### 4. Performance Tests
- **Location**: `benches/` in each crate
- **Purpose**: Benchmark performance
- **Tool**: criterion

## 📊 Build Configuration

### 1. Development Build
```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p tailwind-rs-core

# Build with tests
cargo build --tests
```

### 2. Testing
```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p tailwind-rs-core

# Run with output
cargo test -- --nocapture
```

### 3. Documentation
```bash
# Generate documentation
cargo doc --open

# Generate documentation for specific crate
cargo doc -p tailwind-rs-core --open
```

### 4. Benchmarks
```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench class_generation
```

## 🔄 Development Workflow

### 1. Feature Development
1. Create feature branch
2. Implement in appropriate crate
3. Add tests
4. Update documentation
5. Run CI checks
6. Submit pull request

### 2. Release Process
1. Update version numbers
2. Update changelog
3. Run full test suite
4. Generate documentation
5. Create release tag
6. Publish to crates.io

### 3. CI/CD Pipeline
- **Build**: Compile all crates
- **Test**: Run all tests
- **Lint**: Check code quality
- **Format**: Check code formatting
- **Security**: Check for vulnerabilities
- **Documentation**: Generate docs
- **Benchmarks**: Run performance tests

---

**Next**: [03-design-patterns.md](./03-design-patterns.md) - Implementation patterns and conventions
