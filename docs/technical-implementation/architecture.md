# Architecture and Design Decisions

This document outlines the technical architecture and design decisions for `tailwind-rs`, following our Architecture Decision Records (ADRs) and best practices.

## 🏗️ System Architecture

### High-Level Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                    tailwind-rs Ecosystem                    │
├─────────────────────────────────────────────────────────────┤
│  Core Engine (tailwind-rs-core)                            │
│  ├── AST Parser & Class Detection                          │
│  ├── Type-Safe Class Generation                            │
│  ├── CSS Optimization & Tree-Shaking                       │
│  └── Runtime Class Validation                              │
├─────────────────────────────────────────────────────────────┤
│  Framework Integrations                                     │
│  ├── tailwind-rs-leptos                                    │
│  ├── tailwind-rs-yew                                       │
│  ├── tailwind-rs-dioxus                                    │
│  └── tailwind-rs-generic                                   │
├─────────────────────────────────────────────────────────────┤
│  Build Tools & CLI                                          │
│  ├── tailwind-rs-build                                     │
│  ├── tailwind-rs-cli                                       │
│  └── tailwind-rs-macros                                    │
├─────────────────────────────────────────────────────────────┤
│  Testing & Quality Assurance                               │
│  ├── Unit Tests (70%)                                      │
│  ├── Integration Tests (20%)                               │
│  └── E2E Tests with Playwright (10%)                       │
└─────────────────────────────────────────────────────────────┘
```

## 🎯 Core Design Principles

### 1. Test-Driven Development (ADR-001)
Every component is built following TDD principles:
- **Red**: Write failing test first
- **Green**: Implement minimal code
- **Refactor**: Improve while keeping tests green

### 2. Testing Pyramid Strategy (ADR-002)
- **Unit Tests (70%)**: Fast, isolated tests for individual functions
- **Integration Tests (20%)**: Component interaction testing
- **E2E Tests (10%)**: Complete user workflows with Playwright

### 3. Rust Coding Standards (ADR-007)
- **Latest Rust**: Always use latest stable (1.89.0+)
- **Edition 2021**: Modern Rust features and idioms
- **Performance**: Optimized for speed and memory efficiency
- **Safety**: Leverage Rust's type system for compile-time guarantees

## 🔧 Core Components

### 1. AST Parser and Class Detection Engine

```rust
// Core parsing engine
pub struct TailwindParser {
    ast_cache: HashMap<String, ParsedAst>,
    class_registry: ClassRegistry,
    validation_rules: ValidationRules,
}

impl TailwindParser {
    /// Parse Rust source code and extract Tailwind classes
    pub fn parse_source(&mut self, source: &str) -> Result<ParsedClasses, ParseError> {
        // 1. Parse Rust AST
        let ast = syn::parse_file(source)?;
        
        // 2. Extract class usage patterns
        let class_usage = self.extract_class_usage(&ast)?;
        
        // 3. Validate classes against Tailwind spec
        let validated_classes = self.validate_classes(class_usage)?;
        
        // 4. Cache results for performance
        self.cache_results(&validated_classes);
        
        Ok(validated_classes)
    }
    
    /// Extract classes from various Rust patterns
    fn extract_class_usage(&self, ast: &syn::File) -> Result<ClassUsage, ParseError> {
        let mut usage = ClassUsage::new();
        
        // Extract from classes! macro
        self.extract_from_macros(ast, &mut usage)?;
        
        // Extract from string literals
        self.extract_from_strings(ast, &mut usage)?;
        
        // Extract from dynamic generation
        self.extract_from_dynamic(ast, &mut usage)?;
        
        Ok(usage)
    }
}
```

### 2. Type-Safe Class Generation System

```rust
// Type-safe class generation
#[derive(Debug, Clone)]
pub struct ClassBuilder {
    base_classes: Vec<String>,
    variant_classes: Vec<String>,
    responsive_classes: HashMap<Breakpoint, Vec<String>>,
    state_classes: HashMap<State, Vec<String>>,
}

impl ClassBuilder {
    /// Create a new class builder
    pub fn new() -> Self {
        Self {
            base_classes: Vec::new(),
            variant_classes: Vec::new(),
            responsive_classes: HashMap::new(),
            state_classes: HashMap::new(),
        }
    }
    
    /// Add base classes
    pub fn base(mut self, classes: &str) -> Self {
        self.base_classes.extend(self.parse_classes(classes));
        self
    }
    
    /// Add variant classes
    pub fn variant(mut self, classes: &str) -> Self {
        self.variant_classes.extend(self.parse_classes(classes));
        self
    }
    
    /// Add responsive classes
    pub fn responsive(mut self, breakpoint: Breakpoint, classes: &str) -> Self {
        self.responsive_classes
            .entry(breakpoint)
            .or_insert_with(Vec::new)
            .extend(self.parse_classes(classes));
        self
    }
    
    /// Build final class string
    pub fn build(self) -> String {
        let mut result = Vec::new();
        
        // Add base classes
        result.extend(self.base_classes);
        
        // Add variant classes
        result.extend(self.variant_classes);
        
        // Add responsive classes
        for (breakpoint, classes) in self.responsive_classes {
            let prefixed_classes = classes.into_iter()
                .map(|class| format!("{}:{}", breakpoint.prefix(), class));
            result.extend(prefixed_classes);
        }
        
        // Add state classes
        for (state, classes) in self.state_classes {
            let prefixed_classes = classes.into_iter()
                .map(|class| format!("{}:{}", state.prefix(), class));
            result.extend(prefixed_classes);
        }
        
        result.join(" ")
    }
}

// Procedural macro for compile-time class generation
#[proc_macro]
pub fn classes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClassMacroInput);
    
    // Validate classes at compile time
    let validated_classes = validate_classes_compile_time(&input.classes);
    
    // Generate optimized code
    let generated_code = generate_class_code(&validated_classes);
    
    generated_code.into()
}
```

### 3. CSS Optimization and Tree-Shaking

```rust
// CSS optimization engine
pub struct CssOptimizer {
    used_classes: HashSet<String>,
    unused_classes: HashSet<String>,
    optimization_level: OptimizationLevel,
}

impl CssOptimizer {
    /// Optimize CSS by removing unused classes
    pub fn optimize(&mut self, css: &str, used_classes: &[String]) -> Result<String, OptimizationError> {
        // 1. Parse CSS
        let parsed_css = self.parse_css(css)?;
        
        // 2. Identify used classes
        self.used_classes = used_classes.iter().cloned().collect();
        
        // 3. Remove unused classes
        let optimized_css = self.remove_unused_classes(parsed_css)?;
        
        // 4. Minify CSS
        let minified_css = self.minify_css(optimized_css)?;
        
        Ok(minified_css)
    }
    
    /// Generate CSS from used classes only
    pub fn generate_optimized_css(&self, classes: &[String]) -> Result<String, GenerationError> {
        let mut css_rules = Vec::new();
        
        for class in classes {
            if let Some(rule) = self.generate_css_rule(class)? {
                css_rules.push(rule);
            }
        }
        
        Ok(css_rules.join("\n"))
    }
}
```

### 4. Runtime Class Validation

```rust
// Runtime validation system
pub struct ClassValidator {
    valid_classes: HashSet<String>,
    validation_rules: ValidationRules,
    error_reporter: ErrorReporter,
}

impl ClassValidator {
    /// Validate a class name at runtime
    pub fn validate_class(&self, class_name: &str) -> Result<(), ValidationError> {
        // Check if class exists in Tailwind spec
        if !self.valid_classes.contains(class_name) {
            return Err(ValidationError::InvalidClass(class_name.to_string()));
        }
        
        // Check for conflicts
        if let Some(conflict) = self.check_conflicts(class_name) {
            return Err(ValidationError::ClassConflict(class_name.to_string(), conflict));
        }
        
        // Check for deprecated classes
        if self.is_deprecated(class_name) {
            self.error_reporter.warn_deprecated_class(class_name);
        }
        
        Ok(())
    }
    
    /// Validate multiple classes
    pub fn validate_classes(&self, classes: &[String]) -> Result<(), ValidationError> {
        for class in classes {
            self.validate_class(class)?;
        }
        Ok(())
    }
}
```

## 🎨 Framework Integration Architecture

### Leptos Integration

```rust
// tailwind-rs-leptos integration
use leptos::*;
use tailwind_rs::*;

/// Leptos-specific class generation
pub struct LeptosClassGenerator {
    reactive_classes: HashMap<String, ReadSignal<String>>,
    class_cache: HashMap<String, String>,
}

impl LeptosClassGenerator {
    /// Create reactive classes for Leptos
    pub fn create_reactive_classes<F>(
        &mut self,
        class_fn: F,
    ) -> ReadSignal<String>
    where
        F: Fn() -> String + 'static,
    {
        let (classes, set_classes) = create_signal(class_fn());
        
        // Store reactive classes
        let key = format!("reactive_{}", self.reactive_classes.len());
        self.reactive_classes.insert(key, classes);
        
        classes
    }
    
    /// Generate classes for Leptos components
    pub fn generate_component_classes(
        &self,
        component: &dyn LeptosComponent,
    ) -> Result<String, GenerationError> {
        let mut builder = ClassBuilder::new();
        
        // Add component-specific classes
        builder = builder.base(component.base_classes());
        
        // Add variant classes
        if let Some(variant) = component.variant() {
            builder = builder.variant(variant.classes());
        }
        
        // Add responsive classes
        for (breakpoint, classes) in component.responsive_classes() {
            builder = builder.responsive(breakpoint, classes);
        }
        
        Ok(builder.build())
    }
}

// Leptos component trait
pub trait LeptosComponent {
    fn base_classes(&self) -> &str;
    fn variant(&self) -> Option<&dyn ComponentVariant>;
    fn responsive_classes(&self) -> HashMap<Breakpoint, String>;
}
```

### Yew Integration

```rust
// tailwind-rs-yew integration
use yew::prelude::*;
use tailwind_rs::*;

/// Yew-specific class generation
pub struct YewClassGenerator {
    class_properties: HashMap<String, String>,
    event_handlers: HashMap<String, Callback<Event>>,
}

impl YewClassGenerator {
    /// Generate classes for Yew components
    pub fn generate_yew_classes(
        &self,
        props: &dyn YewComponentProps,
    ) -> Result<String, GenerationError> {
        let mut builder = ClassBuilder::new();
        
        // Add props-based classes
        builder = builder.base(props.base_classes());
        
        // Add conditional classes
        for (condition, classes) in props.conditional_classes() {
            if condition {
                builder = builder.variant(classes);
            }
        }
        
        Ok(builder.build())
    }
}

// Yew component props trait
pub trait YewComponentProps {
    fn base_classes(&self) -> &str;
    fn conditional_classes(&self) -> Vec<(bool, &str)>;
}
```

## 🧪 Testing Architecture

### Unit Testing Framework

```rust
// Unit testing framework
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_class_generation() {
        // Given
        let builder = ClassBuilder::new()
            .base("px-4 py-2")
            .variant("bg-blue-600 text-white");
        
        // When
        let classes = builder.build();
        
        // Then
        assert!(classes.contains("px-4"));
        assert!(classes.contains("bg-blue-600"));
    }

    // Property-based testing
    proptest! {
        #[test]
        fn test_class_validation_properties(class_name in "[a-zA-Z0-9-]+") {
            let validator = ClassValidator::new();
            let result = validator.validate_class(&class_name);
            
            // Properties that should always hold
            match result {
                Ok(_) | Err(ValidationError::InvalidClass(_)) => {
                    prop_assert!(true);
                }
                _ => prop_assert!(false, "Unexpected error type"),
            }
        }
    }
}
```

### Integration Testing Framework

```rust
// Integration testing framework
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_framework_integration() {
        // Test Leptos integration
        let leptos_generator = LeptosClassGenerator::new();
        let leptos_classes = leptos_generator.generate_component_classes(&TestComponent);
        assert!(leptos_classes.contains("leptos-specific"));
        
        // Test Yew integration
        let yew_generator = YewClassGenerator::new();
        let yew_classes = yew_generator.generate_yew_classes(&TestProps);
        assert!(yew_classes.contains("yew-specific"));
    }

    #[tokio::test]
    async fn test_async_integration() {
        // Test async functionality
        let async_generator = AsyncClassGenerator::new();
        let classes = async_generator.generate_classes().await;
        assert!(!classes.is_empty());
    }
}
```

### End-to-End Testing with Playwright

```typescript
// E2E testing framework
import { test, expect } from '@playwright/test';

test.describe('Tailwind-rs E2E Testing', () => {
  test('should demonstrate complete workflow', async ({ page }) => {
    await page.goto('/demo');
    
    // Test class generation
    await page.click('[data-testid="generate-classes"]');
    await expect(page.locator('[data-testid="generated-classes"]'))
      .toBeVisible();
    
    // Test dynamic styling
    await page.selectOption('[data-testid="color-selector"]', 'green');
    await page.click('[data-testid="apply-styles"]');
    
    await expect(page.locator('[data-testid="styled-element"]'))
      .toHaveClass(/bg-green-600/);
    
    // Test performance
    const startTime = Date.now();
    await page.click('[data-testid="performance-test"]');
    const endTime = Date.now();
    
    expect(endTime - startTime).toBeLessThan(1000);
  });
});
```

## 🚀 Performance Architecture

### Optimization Strategies

```rust
// Performance optimization
pub struct PerformanceOptimizer {
    class_cache: LruCache<String, String>,
    css_cache: LruCache<String, String>,
    optimization_level: OptimizationLevel,
}

impl PerformanceOptimizer {
    /// Optimize class generation performance
    pub fn optimize_class_generation(&mut self, classes: &[String]) -> String {
        // Check cache first
        let cache_key = self.generate_cache_key(classes);
        if let Some(cached) = self.class_cache.get(&cache_key) {
            return cached.clone();
        }
        
        // Generate optimized classes
        let optimized = self.generate_optimized_classes(classes);
        
        // Cache result
        self.class_cache.put(cache_key, optimized.clone());
        
        optimized
    }
    
    /// Optimize CSS generation performance
    pub fn optimize_css_generation(&mut self, css: &str) -> String {
        // Check cache first
        if let Some(cached) = self.css_cache.get(css) {
            return cached.clone();
        }
        
        // Optimize CSS
        let optimized = self.optimize_css(css);
        
        // Cache result
        self.css_cache.put(css.to_string(), optimized.clone());
        
        optimized
    }
}
```

### Memory Management

```rust
// Memory-efficient class storage
pub struct ClassStorage {
    classes: Vec<Box<str>>,  // Use Box<str> for memory efficiency
    indices: HashMap<String, usize>,
    memory_pool: MemoryPool,
}

impl ClassStorage {
    /// Store class efficiently
    pub fn store_class(&mut self, class: &str) -> usize {
        if let Some(&index) = self.indices.get(class) {
            return index;
        }
        
        // Allocate from memory pool
        let boxed_class = self.memory_pool.allocate(class);
        let index = self.classes.len();
        
        self.classes.push(boxed_class);
        self.indices.insert(class.to_string(), index);
        
        index
    }
    
    /// Get class by index
    pub fn get_class(&self, index: usize) -> Option<&str> {
        self.classes.get(index).map(|s| s.as_ref())
    }
}
```

## 🔒 Security Architecture

### Input Validation

```rust
// Security validation
pub struct SecurityValidator {
    allowed_classes: HashSet<String>,
    dangerous_patterns: Vec<Regex>,
    sanitization_rules: SanitizationRules,
}

impl SecurityValidator {
    /// Validate and sanitize class input
    pub fn validate_and_sanitize(&self, input: &str) -> Result<String, SecurityError> {
        // Check for dangerous patterns
        for pattern in &self.dangerous_patterns {
            if pattern.is_match(input) {
                return Err(SecurityError::DangerousPattern(input.to_string()));
            }
        }
        
        // Sanitize input
        let sanitized = self.sanitize_input(input)?;
        
        // Validate against allowed classes
        if !self.allowed_classes.contains(&sanitized) {
            return Err(SecurityError::UnauthorizedClass(sanitized));
        }
        
        Ok(sanitized)
    }
    
    /// Sanitize input to prevent injection attacks
    fn sanitize_input(&self, input: &str) -> Result<String, SecurityError> {
        // Remove potentially dangerous characters
        let sanitized = input
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>();
        
        if sanitized.is_empty() {
            return Err(SecurityError::EmptyInput);
        }
        
        Ok(sanitized)
    }
}
```

## 📊 Monitoring and Observability

### Metrics Collection

```rust
// Metrics collection
pub struct MetricsCollector {
    performance_metrics: HashMap<String, PerformanceMetric>,
    error_metrics: HashMap<String, ErrorMetric>,
    usage_metrics: HashMap<String, UsageMetric>,
}

impl MetricsCollector {
    /// Record performance metric
    pub fn record_performance(&mut self, operation: &str, duration: Duration) {
        let metric = self.performance_metrics
            .entry(operation.to_string())
            .or_insert_with(PerformanceMetric::new);
        
        metric.record_duration(duration);
    }
    
    /// Record error metric
    pub fn record_error(&mut self, error_type: &str, error: &dyn std::error::Error) {
        let metric = self.error_metrics
            .entry(error_type.to_string())
            .or_insert_with(ErrorMetric::new);
        
        metric.record_error(error);
    }
    
    /// Generate metrics report
    pub fn generate_report(&self) -> MetricsReport {
        MetricsReport {
            performance: self.performance_metrics.clone(),
            errors: self.error_metrics.clone(),
            usage: self.usage_metrics.clone(),
            timestamp: Utc::now(),
        }
    }
}
```

## 🔄 Build System Architecture

### Cargo Integration

```rust
// build.rs - Build system integration
use tailwind_rs_build::TailwindBuilder;

fn main() {
    TailwindBuilder::new()
        .scan_source("src/")
        .generate_css("dist/styles.css")
        .optimize()
        .enable_tree_shaking()
        .enable_minification()
        .build()
        .expect("Failed to build Tailwind CSS");
}
```

### CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
      
      - name: Run tests
        run: cargo test --verbose
      
      - name: Run Playwright tests
        run: pnpm run test:e2e
      
      - name: Generate coverage
        run: cargo tarpaulin --out Html
```

## 🎯 Future Architecture Considerations

### Scalability
- **Horizontal scaling**: Support for multiple build workers
- **Caching**: Distributed caching for large projects
- **Incremental builds**: Only rebuild changed components

### Extensibility
- **Plugin system**: Support for custom class generators
- **Theme system**: Advanced theming and customization
- **Framework support**: Easy integration with new frameworks

### Performance
- **Parallel processing**: Multi-threaded class generation
- **Memory optimization**: Efficient memory usage patterns
- **Bundle optimization**: Advanced tree-shaking and minification

---

This architecture ensures that `tailwind-rs` is built with the highest standards of quality, performance, and maintainability while following our established ADRs and best practices.

