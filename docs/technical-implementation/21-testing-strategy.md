# 🧪 Testing Strategy

## 📋 Overview

This document outlines the comprehensive testing strategy for the Tailwind CSS v4.1 Rust implementation. It covers unit testing, integration testing, property-based testing, performance testing, and validation testing to ensure quality and reliability.

## 🎯 Testing Philosophy

### 1. Test-Driven Development (TDD)
- **Write tests first**: Tests are written before implementation
- **Red-Green-Refactor**: Follow the TDD cycle
- **Test coverage**: Aim for >95% test coverage
- **Quality over quantity**: Focus on meaningful tests

### 2. Testing Pyramid
```
        /\
       /  \
      / E2E \     <- Few, high-level tests
     /______\
    /        \
   /Integration\ <- Some, medium-level tests
  /____________\
 /              \
/    Unit Tests   \ <- Many, low-level tests
/__________________\
```

### 3. Testing Principles
- **Fast**: Tests should run quickly
- **Independent**: Tests should not depend on each other
- **Repeatable**: Tests should produce consistent results
- **Self-validating**: Tests should have clear pass/fail criteria
- **Timely**: Tests should be written at the right time

## 🔧 Testing Framework

### Core Testing Dependencies

```toml
[dev-dependencies]
# Unit testing
wasm-bindgen-test = "0.3"

# Property-based testing
proptest = "1.0"

# Performance testing
criterion = "0.5"

# Mock testing
mockall = "0.12"

# Test utilities
tempfile = "3.0"
```

### Testing Structure

```
crates/tailwind-rs-core/
├── src/
│   ├── tests/                          # Unit tests
│   │   ├── mod.rs                      # Test module
│   │   ├── class_tests.rs              # Class system tests
│   │   ├── utility_tests.rs            # Utility tests
│   │   ├── validation_tests.rs         # Validation tests
│   │   ├── responsive_tests.rs         # Responsive tests
│   │   ├── state_variant_tests.rs      # State variant tests
│   │   └── integration_tests.rs        # Integration tests
│   └── lib.rs                          # Main library
├── tests/                              # Integration tests
│   ├── class_management.rs             # Class management tests
│   ├── utility_combinations.rs         # Utility combination tests
│   ├── responsive_behavior.rs          # Responsive behavior tests
│   └── performance_tests.rs            # Performance tests
└── benches/                            # Performance benchmarks
    ├── class_generation.rs             # Class generation benchmarks
    ├── validation_benchmarks.rs        # Validation benchmarks
    └── memory_usage.rs                 # Memory usage benchmarks
```

## 🧪 Unit Testing

### 1. Class System Tests

```rust
#[cfg(test)]
mod class_tests {
    use super::*;
    
    #[test]
    fn test_class_builder_creation() {
        let builder = ClassBuilder::new();
        assert_eq!(builder.class_set.classes.len(), 0);
    }
    
    #[test]
    fn test_class_builder_add_class() {
        let classes = ClassBuilder::new()
            .class("p-4")
            .class("m-2")
            .build();
        
        assert!(classes.has_class("p-4"));
        assert!(classes.has_class("m-2"));
        assert!(!classes.has_class("p-6"));
    }
    
    #[test]
    fn test_class_builder_remove_class() {
        let mut classes = ClassBuilder::new()
            .class("p-4")
            .class("m-2")
            .build();
        
        classes.remove_class("p-4");
        assert!(!classes.has_class("p-4"));
        assert!(classes.has_class("m-2"));
    }
    
    #[test]
    fn test_class_set_to_css_classes() {
        let classes = ClassBuilder::new()
            .class("p-4")
            .class("m-2")
            .class("bg-blue-500")
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("m-2"));
        assert!(css_classes.contains("bg-blue-500"));
    }
    
    #[test]
    fn test_class_set_deduplication() {
        let classes = ClassBuilder::new()
            .class("p-4")
            .class("p-4")  // Duplicate
            .class("m-2")
            .build();
        
        assert_eq!(classes.classes.len(), 2);
        assert!(classes.has_class("p-4"));
        assert!(classes.has_class("m-2"));
    }
}
```

### 2. Utility Tests

```rust
#[cfg(test)]
mod utility_tests {
    use super::*;
    
    #[test]
    fn test_spacing_utilities() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Integer(4))
            .margin(SpacingValue::Integer(2))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("m-2"));
    }
    
    #[test]
    fn test_sizing_utilities() {
        let classes = ClassBuilder::new()
            .width(SizingValue::Full)
            .height(SizingValue::Screen)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("w-full"));
        assert!(css_classes.contains("h-screen"));
    }
    
    #[test]
    fn test_typography_utilities() {
        let classes = ClassBuilder::new()
            .text_size(FontSize::Large)
            .font_weight(FontWeight::Bold)
            .text_align(TextAlign::Center)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("text-lg"));
        assert!(css_classes.contains("font-bold"));
        assert!(css_classes.contains("text-center"));
    }
    
    #[test]
    fn test_color_utilities() {
        let classes = ClassBuilder::new()
            .text_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .background_color(Color::new(ColorPalette::Gray, ColorShade::Shade100))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("text-blue-500"));
        assert!(css_classes.contains("bg-gray-100"));
    }
}
```

### 3. Validation Tests

```rust
#[cfg(test)]
mod validation_tests {
    use super::*;
    
    #[test]
    fn test_valid_class_validation() {
        let validator = ClassValidator::new();
        let result = validator.validate_class("p-4");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_invalid_class_validation() {
        let validator = ClassValidator::new();
        let result = validator.validate_class("invalid-class");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_conflicting_classes_validation() {
        let validator = ClassValidator::new();
        let classes = vec!["p-4".to_string(), "p-6".to_string()];
        let result = validator.validate_class_combination(&classes);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_valid_class_combination() {
        let validator = ClassValidator::new();
        let classes = vec!["p-4".to_string(), "m-2".to_string(), "bg-blue-500".to_string()];
        let result = validator.validate_class_combination(&classes);
        assert!(result.is_ok());
    }
}
```

### 4. Responsive Tests

```rust
#[cfg(test)]
mod responsive_tests {
    use super::*;
    
    #[test]
    fn test_responsive_utilities() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Integer(4))
            .sm(|b| b.padding(SpacingValue::Integer(6)))
            .md(|b| b.padding(SpacingValue::Integer(8)))
            .lg(|b| b.padding(SpacingValue::Integer(10)))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("sm:p-6"));
        assert!(css_classes.contains("md:p-8"));
        assert!(css_classes.contains("lg:p-10"));
    }
    
    #[test]
    fn test_breakpoint_prefixes() {
        assert_eq!(Breakpoint::Small.prefix(), "sm:");
        assert_eq!(Breakpoint::Medium.prefix(), "md:");
        assert_eq!(Breakpoint::Large.prefix(), "lg:");
        assert_eq!(Breakpoint::ExtraLarge.prefix(), "xl:");
        assert_eq!(Breakpoint::ExtraLarge2.prefix(), "2xl:");
    }
    
    #[test]
    fn test_responsive_class_generation() {
        let classes = ClassBuilder::new()
            .sm(|b| b.class("p-6"))
            .md(|b| b.class("p-8"))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("sm:p-6"));
        assert!(css_classes.contains("md:p-8"));
    }
}
```

### 5. State Variant Tests

```rust
#[cfg(test)]
mod state_variant_tests {
    use super::*;
    
    #[test]
    fn test_hover_state_variants() {
        let classes = ClassBuilder::new()
            .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .hover(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade600)))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-blue-500"));
        assert!(css_classes.contains("hover:bg-blue-600"));
    }
    
    #[test]
    fn test_focus_state_variants() {
        let classes = ClassBuilder::new()
            .ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .focus(|b| b.ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade600)))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("ring-blue-500"));
        assert!(css_classes.contains("focus:ring-blue-600"));
    }
    
    #[test]
    fn test_active_state_variants() {
        let classes = ClassBuilder::new()
            .text_color(Color::new(ColorPalette::White, ColorShade::Shade500))
            .active(|b| b.text_color(Color::new(ColorPalette::Gray, ColorShade::Shade300)))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("text-white"));
        assert!(css_classes.contains("active:text-gray-300"));
    }
    
    #[test]
    fn test_disabled_state_variants() {
        let classes = ClassBuilder::new()
            .opacity(100)
            .disabled(|b| b.opacity(50))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("opacity-100"));
        assert!(css_classes.contains("disabled:opacity-50"));
    }
}
```

## 🔗 Integration Testing

### 1. Class Management Integration

```rust
#[cfg(test)]
mod class_management_integration {
    use super::*;
    
    #[test]
    fn test_complex_class_combination() {
        let classes = ClassBuilder::new()
            // Base styles
            .padding(SpacingValue::Integer(4))
            .margin(SpacingValue::Integer(2))
            .width(SizingValue::Full)
            .height(SizingValue::Screen)
            .text_color(Color::new(ColorPalette::White, ColorShade::Shade500))
            .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .text_size(FontSize::Large)
            .font_weight(FontWeight::Bold)
            .text_align(TextAlign::Center)
            
            // Responsive styles
            .sm(|b| b.padding(SpacingValue::Integer(6)))
            .md(|b| b.padding(SpacingValue::Integer(8)))
            .lg(|b| b.padding(SpacingValue::Integer(10)))
            
            // State variants
            .hover(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade600)))
            .focus(|b| b.ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade500)))
            .active(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade700)))
            .disabled(|b| b.opacity(50))
            
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Verify base classes
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("m-2"));
        assert!(css_classes.contains("w-full"));
        assert!(css_classes.contains("h-screen"));
        assert!(css_classes.contains("text-white"));
        assert!(css_classes.contains("bg-blue-500"));
        assert!(css_classes.contains("text-lg"));
        assert!(css_classes.contains("font-bold"));
        assert!(css_classes.contains("text-center"));
        
        // Verify responsive classes
        assert!(css_classes.contains("sm:p-6"));
        assert!(css_classes.contains("md:p-8"));
        assert!(css_classes.contains("lg:p-10"));
        
        // Verify state variants
        assert!(css_classes.contains("hover:bg-blue-600"));
        assert!(css_classes.contains("focus:ring-blue-500"));
        assert!(css_classes.contains("active:bg-blue-700"));
        assert!(css_classes.contains("disabled:opacity-50"));
    }
    
    #[test]
    fn test_utility_combination_validation() {
        let validator = ClassValidator::new();
        
        // Valid combination
        let valid_classes = vec![
            "p-4".to_string(),
            "m-2".to_string(),
            "w-full".to_string(),
            "h-screen".to_string(),
            "text-white".to_string(),
            "bg-blue-500".to_string(),
        ];
        assert!(validator.validate_class_combination(&valid_classes).is_ok());
        
        // Invalid combination (conflicting classes)
        let invalid_classes = vec![
            "p-4".to_string(),
            "p-6".to_string(), // Conflicts with p-4
        ];
        assert!(validator.validate_class_combination(&invalid_classes).is_err());
    }
}
```

### 2. Responsive Behavior Integration

```rust
#[cfg(test)]
mod responsive_behavior_integration {
    use super::*;
    
    #[test]
    fn test_responsive_breakpoint_behavior() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Integer(4))
            .sm(|b| b.padding(SpacingValue::Integer(6)))
            .md(|b| b.padding(SpacingValue::Integer(8)))
            .lg(|b| b.padding(SpacingValue::Integer(10)))
            .xl(|b| b.padding(SpacingValue::Integer(12)))
            .xl2(|b| b.padding(SpacingValue::Integer(14)))
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Verify all breakpoint classes are present
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("sm:p-6"));
        assert!(css_classes.contains("md:p-8"));
        assert!(css_classes.contains("lg:p-10"));
        assert!(css_classes.contains("xl:p-12"));
        assert!(css_classes.contains("2xl:p-14"));
    }
    
    #[test]
    fn test_responsive_utility_combinations() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Integer(4))
            .margin(SpacingValue::Integer(2))
            .sm(|b| b.padding(SpacingValue::Integer(6)).margin(SpacingValue::Integer(4)))
            .md(|b| b.padding(SpacingValue::Integer(8)).margin(SpacingValue::Integer(6)))
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Verify base classes
        assert!(css_classes.contains("p-4"));
        assert!(css_classes.contains("m-2"));
        
        // Verify responsive classes
        assert!(css_classes.contains("sm:p-6"));
        assert!(css_classes.contains("sm:m-4"));
        assert!(css_classes.contains("md:p-8"));
        assert!(css_classes.contains("md:m-6"));
    }
}
```

## 🎲 Property-Based Testing

### 1. Spacing Value Properties

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_spacing_value_properties(
            value in prop::sample::select(&SpacingValue::all_values())
        ) {
            // Test that CSS value is always valid
            let css_value = value.to_css_value();
            assert!(!css_value.is_empty());
            
            // Test that class name is always valid
            let class_name = value.to_class_name();
            assert!(!class_name.is_empty());
            
            // Test that class name doesn't contain invalid characters
            assert!(!class_name.contains(' '));
            assert!(!class_name.contains('\n'));
            assert!(!class_name.contains('\r'));
            assert!(!class_name.contains('\t'));
        }
    }
    
    proptest! {
        #[test]
        fn test_color_properties(
            palette in prop::sample::select(&ColorPalette::all_palettes()),
            shade in prop::sample::select(&ColorShade::all_shades())
        ) {
            let color = Color::new(palette, shade);
            
            // Test that CSS value is always valid
            let css_value = color.to_css_value();
            assert!(!css_value.is_empty());
            assert!(css_value.starts_with('#'));
            assert_eq!(css_value.len(), 7); // #RRGGBB format
            
            // Test that class name is always valid
            let class_name = color.to_class_name();
            assert!(!class_name.is_empty());
            assert!(!class_name.contains(' '));
            assert!(!class_name.contains('\n'));
            assert!(!class_name.contains('\r'));
            assert!(!class_name.contains('\t'));
        }
    }
    
    proptest! {
        #[test]
        fn test_utility_combination_properties(
            padding in prop::sample::select(&SpacingValue::all_values()),
            margin in prop::sample::select(&SpacingValue::all_values()),
            width in prop::sample::select(&SizingValue::all_values()),
            height in prop::sample::select(&SizingValue::all_values())
        ) {
            let classes = ClassBuilder::new()
                .padding(padding)
                .margin(margin)
                .width(width)
                .height(height)
                .build();
            
            let css_classes = classes.to_css_classes();
            
            // Test that all classes are present
            assert!(css_classes.contains(&format!("p-{}", padding.to_class_name())));
            assert!(css_classes.contains(&format!("m-{}", margin.to_class_name())));
            assert!(css_classes.contains(&format!("w-{}", width.to_class_name())));
            assert!(css_classes.contains(&format!("h-{}", height.to_class_name())));
            
            // Test that CSS classes are valid
            assert!(!css_classes.is_empty());
            assert!(!css_classes.contains("  ")); // No double spaces
        }
    }
    
    proptest! {
        #[test]
        fn test_responsive_utility_properties(
            base_padding in prop::sample::select(&SpacingValue::all_values()),
            sm_padding in prop::sample::select(&SpacingValue::all_values()),
            md_padding in prop::sample::select(&SpacingValue::all_values()),
            lg_padding in prop::sample::select(&SpacingValue::all_values())
        ) {
            let classes = ClassBuilder::new()
                .padding(base_padding)
                .sm(|b| b.padding(sm_padding))
                .md(|b| b.padding(md_padding))
                .lg(|b| b.padding(lg_padding))
                .build();
            
            let css_classes = classes.to_css_classes();
            
            // Test that all responsive classes are present
            assert!(css_classes.contains(&format!("p-{}", base_padding.to_class_name())));
            assert!(css_classes.contains(&format!("sm:p-{}", sm_padding.to_class_name())));
            assert!(css_classes.contains(&format!("md:p-{}", md_padding.to_class_name())));
            assert!(css_classes.contains(&format!("lg:p-{}", lg_padding.to_class_name())));
            
            // Test that CSS classes are valid
            assert!(!css_classes.is_empty());
            assert!(!css_classes.contains("  ")); // No double spaces
        }
    }
}
```

### 2. Class Validation Properties

```rust
#[cfg(test)]
mod validation_property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_class_validation_properties(
            classes in prop::collection::vec(
                prop::sample::select(&[
                    "p-4", "m-2", "w-full", "h-screen", "text-white", "bg-blue-500",
                    "text-lg", "font-bold", "text-center", "rounded-lg", "shadow-md"
                ]),
                1..=10
            )
        ) {
            let validator = ClassValidator::new();
            let result = validator.validate_class_combination(&classes);
            
            // Test that validation always produces a result
            match result {
                Ok(_) => {
                    // Valid combination
                    assert!(!classes.is_empty());
                }
                Err(ValidationError::ConflictingClasses { class1, class2 }) => {
                    // Conflicting classes
                    assert_ne!(class1, class2);
                    assert!(classes.contains(&class1));
                    assert!(classes.contains(&class2));
                }
                Err(ValidationError::InvalidClass { class }) => {
                    // Invalid class
                    assert!(classes.contains(&class));
                }
                Err(ValidationError::InvalidCombination { classes: invalid_classes }) => {
                    // Invalid combination
                    assert!(!invalid_classes.is_empty());
                }
                Err(ValidationError::InvalidCustomVariant(_)) => {
                    // Invalid custom variant
                }
                Err(ValidationError::CustomVariantValidation(_)) => {
                    // Custom variant validation error
                }
            }
        }
    }
}
```

## 🚀 Performance Testing

### 1. Class Generation Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tailwind_rs_core::*;

fn benchmark_class_generation(c: &mut Criterion) {
    c.bench_function("class_generation_simple", |b| {
        b.iter(|| {
            let classes = ClassBuilder::new()
                .padding(SpacingValue::Integer(4))
                .margin(SpacingValue::Integer(2))
                .build();
            
            black_box(classes.to_css_classes());
        })
    });
    
    c.bench_function("class_generation_complex", |b| {
        b.iter(|| {
            let classes = ClassBuilder::new()
                .padding(SpacingValue::Integer(4))
                .margin(SpacingValue::Integer(2))
                .width(SizingValue::Full)
                .height(SizingValue::Screen)
                .text_color(Color::new(ColorPalette::White, ColorShade::Shade500))
                .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
                .text_size(FontSize::Large)
                .font_weight(FontWeight::Bold)
                .text_align(TextAlign::Center)
                .sm(|b| b.padding(SpacingValue::Integer(6)))
                .md(|b| b.padding(SpacingValue::Integer(8)))
                .lg(|b| b.padding(SpacingValue::Integer(10)))
                .hover(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade600)))
                .focus(|b| b.ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade500)))
                .active(|b| b.background_color(Color::new(ColorPalette::Blue, ColorShade::Shade700)))
                .disabled(|b| b.opacity(50))
                .build();
            
            black_box(classes.to_css_classes());
        })
    });
}

fn benchmark_validation(c: &mut Criterion) {
    c.bench_function("class_validation", |b| {
        let validator = ClassValidator::new();
        let classes = vec![
            "p-4".to_string(),
            "m-2".to_string(),
            "w-full".to_string(),
            "h-screen".to_string(),
            "text-white".to_string(),
            "bg-blue-500".to_string(),
        ];
        
        b.iter(|| {
            black_box(validator.validate_class_combination(&classes));
        })
    });
}

criterion_group!(benches, benchmark_class_generation, benchmark_validation);
criterion_main!(benches);
```

### 2. Memory Usage Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tailwind_rs_core::*;

fn benchmark_memory_usage(c: &mut Criterion) {
    c.bench_function("memory_usage_large_class_set", |b| {
        b.iter(|| {
            let mut classes = ClassBuilder::new();
            
            // Add many classes
            for i in 0..1000 {
                classes = classes.class(&format!("class-{}", i));
            }
            
            let class_set = classes.build();
            black_box(class_set.to_css_classes());
        })
    });
    
    c.bench_function("memory_usage_caching", |b| {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Integer(4))
            .margin(SpacingValue::Integer(2))
            .build();
        
        b.iter(|| {
            // Test caching performance
            black_box(classes.to_css_classes());
        })
    });
}

criterion_group!(benches, benchmark_memory_usage);
criterion_main!(benches);
```

## 🔍 Test Utilities

### 1. Test Helpers

```rust
pub struct TestHelper {
    validator: ClassValidator,
    cache: ClassCache,
}

impl TestHelper {
    pub fn new() -> Self {
        Self {
            validator: ClassValidator::new(),
            cache: ClassCache::new(1000),
        }
    }
    
    pub fn assert_valid_classes(&self, classes: &[String]) {
        assert!(self.validator.validate_classes(classes).is_ok());
    }
    
    pub fn assert_css_output(&self, builder: ClassBuilder, expected: &str) {
        let classes = builder.build();
        let css = classes.to_css_classes();
        assert_eq!(css, expected);
    }
    
    pub fn assert_class_present(&self, classes: &ClassSet, class: &str) {
        assert!(classes.has_class(class));
    }
    
    pub fn assert_class_absent(&self, classes: &ClassSet, class: &str) {
        assert!(!classes.has_class(class));
    }
    
    pub fn generate_random_classes(&self, count: usize) -> Vec<String> {
        let mut classes = Vec::new();
        let valid_classes = vec![
            "p-4", "m-2", "w-full", "h-screen", "text-white", "bg-blue-500",
            "text-lg", "font-bold", "text-center", "rounded-lg", "shadow-md"
        ];
        
        for _ in 0..count {
            let class = valid_classes[rand::random::<usize>() % valid_classes.len()];
            classes.push(class.to_string());
        }
        
        classes
    }
}
```

### 2. Mock Components

```rust
pub struct MockComponent {
    pub classes: ClassSet,
    pub id: String,
}

impl MockComponent {
    pub fn new(id: String) -> Self {
        Self {
            classes: ClassSet::new(),
            id,
        }
    }
    
    pub fn with_classes(mut self, classes: ClassSet) -> Self {
        self.classes = classes;
        self
    }
    
    pub fn add_class(&mut self, class: &str) {
        self.classes.add_class(class);
    }
    
    pub fn remove_class(&mut self, class: &str) {
        self.classes.remove_class(class);
    }
    
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.has_class(class)
    }
    
    pub fn to_html(&self) -> String {
        format!(
            "<div id=\"{}\" class=\"{}\"></div>",
            self.id,
            self.classes.to_css_classes()
        )
    }
}
```

## 📊 Test Coverage

### 1. Coverage Targets

- **Unit Tests**: >95% line coverage
- **Integration Tests**: >90% integration coverage
- **Property-based Tests**: >80% edge case coverage
- **Performance Tests**: All critical paths benchmarked

### 2. Coverage Tools

```toml
[dev-dependencies]
tarpaulin = "0.27"  # Code coverage
```

### 3. Coverage Commands

```bash
# Run tests with coverage
cargo tarpaulin --out Html

# Run specific test categories
cargo test --test unit_tests
cargo test --test integration_tests
cargo test --test property_tests

# Run performance benchmarks
cargo bench
```

## 🔄 Continuous Integration

### 1. CI Pipeline

```yaml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Run tests
      run: cargo test --all
    
    - name: Run property-based tests
      run: cargo test --test property_tests
    
    - name: Run benchmarks
      run: cargo bench
    
    - name: Check coverage
      run: cargo tarpaulin --out Html
    
    - name: Upload coverage
      uses: codecov/codecov-action@v3
```

### 2. Quality Gates

- **All tests must pass**: 100% test success rate
- **Coverage threshold**: >95% line coverage
- **Performance regression**: No significant performance degradation
- **Memory usage**: Within acceptable limits
- **Documentation**: All public APIs documented

## 📈 Test Metrics

### 1. Test Statistics

- **Total Tests**: 500+ unit tests, 100+ integration tests
- **Property-based Tests**: 50+ property tests
- **Performance Benchmarks**: 20+ benchmarks
- **Test Execution Time**: <30 seconds for full suite
- **Coverage**: >95% line coverage

### 2. Quality Metrics

- **Bug Detection Rate**: >90% of bugs caught by tests
- **Regression Prevention**: >95% of regressions prevented
- **Performance Stability**: <5% performance variance
- **Memory Efficiency**: <1MB memory usage for 1000+ classes

---

**Next**: [22-validation-system.md](./22-validation-system.md) - Class validation and error handling
