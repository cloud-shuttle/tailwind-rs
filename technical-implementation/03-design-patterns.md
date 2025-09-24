# 🎨 Design Patterns

## 📋 Overview

This document outlines the design patterns and implementation conventions used throughout the Tailwind CSS v4.1 Rust implementation. These patterns ensure consistency, maintainability, and extensibility across the entire codebase.

## 🏗️ Core Design Patterns

### 1. Builder Pattern

The builder pattern is the foundation of our class building system, providing a fluent API for constructing CSS classes.

#### Implementation
```rust
pub struct ClassBuilder {
    class_set: ClassSet,
}

impl ClassBuilder {
    pub fn new() -> Self {
        Self {
            class_set: ClassSet::new(),
        }
    }
    
    pub fn class(mut self, class: &str) -> Self {
        self.class_set.add_class(class);
        self
    }
    
    pub fn build(self) -> ClassSet {
        self.class_set
    }
}
```

#### Usage
```rust
let classes = ClassBuilder::new()
    .class("p-4")
    .class("m-2")
    .class("bg-blue-500")
    .build();
```

#### Benefits
- **Fluent API**: Easy to read and write
- **Type Safety**: Compile-time validation
- **Extensible**: Easy to add new methods
- **IDE Support**: Full autocomplete

### 2. Trait-based Utilities

Utilities are organized as traits to provide modular, extensible functionality.

#### Implementation
```rust
pub trait SpacingUtilities {
    fn padding(self, value: SpacingValue) -> Self;
    fn margin(self, value: SpacingValue) -> Self;
}

impl SpacingUtilities for ClassBuilder {
    fn padding(self, value: SpacingValue) -> Self {
        self.class(&format!("p-{}", value.to_class_name()))
    }
    
    fn margin(self, value: SpacingValue) -> Self {
        self.class(&format!("m-{}", value.to_class_name()))
    }
}
```

#### Usage
```rust
let classes = ClassBuilder::new()
    .padding(SpacingValue::Integer(4))
    .margin(SpacingValue::Integer(2))
    .build();
```

#### Benefits
- **Modularity**: Clear separation of concerns
- **Extensibility**: Easy to add new utilities
- **Reusability**: Traits can be implemented by different types
- **Organization**: Logical grouping of related functionality

### 3. Enum-based Values

Utility values are represented as enums to ensure type safety and clear constraints.

#### Implementation
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpacingValue {
    Zero,
    Px,
    Fractional(f32),
    Integer(u32),
    Auto,
    Full,
    Screen,
    Min,
    Max,
    Fit,
}

impl SpacingValue {
    pub fn to_class_name(&self) -> String {
        match self {
            SpacingValue::Zero => "0".to_string(),
            SpacingValue::Px => "px".to_string(),
            SpacingValue::Fractional(f) => format!("{}", f),
            SpacingValue::Integer(i) => i.to_string(),
            SpacingValue::Auto => "auto".to_string(),
            SpacingValue::Full => "full".to_string(),
            SpacingValue::Screen => "screen".to_string(),
            SpacingValue::Min => "min".to_string(),
            SpacingValue::Max => "max".to_string(),
            SpacingValue::Fit => "fit".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            SpacingValue::Zero => "0".to_string(),
            SpacingValue::Px => "1px".to_string(),
            SpacingValue::Fractional(f) => format!("{}rem", f * 0.25),
            SpacingValue::Integer(i) => format!("{}rem", i * 0.25),
            SpacingValue::Auto => "auto".to_string(),
            SpacingValue::Full => "100%".to_string(),
            SpacingValue::Screen => "100vh".to_string(),
            SpacingValue::Min => "min-content".to_string(),
            SpacingValue::Max => "max-content".to_string(),
            SpacingValue::Fit => "fit-content".to_string(),
        }
    }
}
```

#### Benefits
- **Type Safety**: Invalid values caught at compile time
- **Clear Constraints**: Obvious what values are allowed
- **Performance**: No runtime validation needed
- **Extensibility**: Easy to add new values

### 4. Responsive Builder Pattern

Responsive utilities use a nested builder pattern for breakpoint-specific classes.

#### Implementation
```rust
pub struct ResponsiveBuilder<'a> {
    builder: &'a mut ClassBuilder,
    breakpoint: Breakpoint,
}

impl<'a> ResponsiveBuilder<'a> {
    pub fn new(builder: &'a mut ClassBuilder, breakpoint: Breakpoint) -> Self {
        Self { builder, breakpoint }
    }
    
    pub fn class(mut self, class: &str) -> Self {
        let responsive_class = format!("{}:{}", self.breakpoint.prefix(), class);
        self.builder.class(&responsive_class);
        self
    }
    
    pub fn padding(self, value: SpacingValue) -> Self {
        self.class(&format!("p-{}", value.to_class_name()))
    }
}

pub trait ResponsiveUtilities {
    fn sm<F>(self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder;
}

impl ResponsiveUtilities for ClassBuilder {
    fn sm<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder,
    {
        let mut responsive_builder = ResponsiveBuilder::new(&mut self, Breakpoint::Small);
        f(responsive_builder);
        self
    }
}
```

#### Usage
```rust
let classes = ClassBuilder::new()
    .padding(SpacingValue::Integer(4))
    .sm(|b| b.padding(SpacingValue::Integer(6)))
    .md(|b| b.padding(SpacingValue::Integer(8)))
    .build();
```

#### Benefits
- **Intuitive**: Natural syntax for responsive design
- **Type Safe**: Compile-time validation of breakpoints
- **Flexible**: Can chain multiple responsive utilities
- **Consistent**: Same pattern for all responsive utilities

### 5. State Variant Pattern

State variants use a similar nested builder pattern for pseudo-classes.

#### Implementation
```rust
pub struct StateBuilder<'a> {
    builder: &'a mut ClassBuilder,
    state: &'static str,
}

impl<'a> StateBuilder<'a> {
    pub fn new(builder: &'a mut ClassBuilder, state: &'static str) -> Self {
        Self { builder, state }
    }
    
    pub fn class(mut self, class: &str) -> Self {
        let state_class = format!("{}:{}", self.state, class);
        self.builder.class(&state_class);
        self
    }
    
    pub fn background_color(self, color: Color) -> Self {
        self.class(&format!("bg-{}", color.to_class_name()))
    }
}

pub trait StateUtilities {
    fn hover<F>(self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder;
}

impl StateUtilities for ClassBuilder {
    fn hover<F>(mut self, f: F) -> Self
    where
        F: FnOnce(StateBuilder) -> StateBuilder,
    {
        let mut state_builder = StateBuilder::new(&mut self, "hover");
        f(state_builder);
        self
    }
}
```

#### Usage
```rust
let classes = ClassBuilder::new()
    .background_color(Color::Blue(500))
    .hover(|b| b.background_color(Color::Blue(600)))
    .focus(|b| b.ring_color(Color::Blue(500)))
    .build();
```

#### Benefits
- **Intuitive**: Natural syntax for state variants
- **Type Safe**: Compile-time validation of states
- **Flexible**: Can chain multiple state utilities
- **Consistent**: Same pattern for all state variants

## 🎯 Implementation Conventions

### 1. Naming Conventions

#### Structs and Enums
- **PascalCase**: `ClassBuilder`, `SpacingValue`, `ColorPalette`
- **Descriptive**: Names should clearly indicate purpose
- **Consistent**: Similar concepts use similar naming patterns

#### Methods and Functions
- **snake_case**: `padding()`, `margin()`, `background_color()`
- **Verb-based**: Methods that perform actions use verbs
- **Consistent**: Similar operations use similar names

#### Constants
- **SCREAMING_SNAKE_CASE**: `DEFAULT_SPACING`, `MAX_BREAKPOINT`
- **Descriptive**: Names should clearly indicate purpose
- **Grouped**: Related constants are grouped together

### 2. Error Handling

#### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum TailwindError {
    #[error("Invalid class: {class}")]
    InvalidClass { class: String },
    
    #[error("Conflicting classes: {class1} and {class2}")]
    ConflictingClasses { class1: String, class2: String },
    
    #[error("Validation error: {message}")]
    Validation { message: String },
}
```

#### Error Handling Patterns
```rust
impl ClassBuilder {
    pub fn class(self, class: &str) -> Result<Self, TailwindError> {
        if !is_valid_class(class) {
            return Err(TailwindError::InvalidClass {
                class: class.to_string(),
            });
        }
        
        Ok(self.class_unchecked(class))
    }
}
```

### 3. Documentation Standards

#### Function Documentation
```rust
/// Add padding to all sides of an element
/// 
/// # Arguments
/// 
/// * `value` - The spacing value to apply
/// 
/// # Examples
/// 
/// ```rust
/// let classes = ClassBuilder::new()
///     .padding(SpacingValue::Integer(4))
///     .build();
/// ```
/// 
/// # Returns
/// 
/// A new `ClassBuilder` with the padding class added
pub fn padding(self, value: SpacingValue) -> Self {
    self.class(&format!("p-{}", value.to_class_name()))
}
```

#### Trait Documentation
```rust
/// Trait for adding spacing utilities to a class builder
/// 
/// This trait provides methods for adding padding and margin utilities
/// to any type that implements it. The utilities follow Tailwind CSS
/// conventions and provide type-safe spacing values.
/// 
/// # Examples
/// 
/// ```rust
/// let classes = ClassBuilder::new()
///     .padding(SpacingValue::Integer(4))
///     .margin(SpacingValue::Integer(2))
///     .build();
/// ```
pub trait SpacingUtilities {
    /// Add padding to all sides
    fn padding(self, value: SpacingValue) -> Self;
    
    /// Add margin to all sides
    fn margin(self, value: SpacingValue) -> Self;
}
```

### 4. Testing Patterns

#### Unit Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_padding_utility() {
        let classes = ClassBuilder::new()
            .padding(SpacingValue::Integer(4))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("p-4"));
    }
    
    #[test]
    fn test_margin_utility() {
        let classes = ClassBuilder::new()
            .margin(SpacingValue::Integer(2))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("m-2"));
    }
}
```

#### Property-based Testing
```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_spacing_value_properties(
            value in prop::sample::select(&[
                SpacingValue::Zero,
                SpacingValue::Px,
                SpacingValue::Integer(1),
                SpacingValue::Integer(4),
                SpacingValue::Integer(8),
            ])
        ) {
            let css_value = value.to_css_value();
            assert!(!css_value.is_empty());
            
            let class_name = value.to_class_name();
            assert!(!class_name.is_empty());
        }
    }
}
```

## 🔧 Macro Patterns

### 1. Utility Generation Macros

#### Implementation
```rust
#[proc_macro_derive(TailwindUtility)]
pub fn derive_tailwind_utility(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        impl #name {
            pub fn to_class_name(&self) -> String {
                match self {
                    #(Self::#variants => #class_names.to_string()),*
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

#### Usage
```rust
#[derive(TailwindUtility)]
pub enum SpacingValue {
    #[tailwind("0")]
    Zero,
    #[tailwind("px")]
    Px,
    #[tailwind("4")]
    Integer4,
}
```

### 2. Builder Macros

#### Implementation
```rust
#[proc_macro]
pub fn class_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClassBuilderInput);
    
    let expanded = quote! {
        ClassBuilder::new()
            #(.class(#input.classes))*
            .build()
    };
    
    TokenStream::from(expanded)
}
```

#### Usage
```rust
let classes = class_builder! {
    "p-4",
    "m-2",
    "bg-blue-500"
};
```

## 🚀 Performance Patterns

### 1. Lazy Evaluation

#### Implementation
```rust
impl ClassSet {
    pub fn to_css_classes(&self) -> String {
        if let Some(cached) = self.cached_css.as_ref() {
            return cached.clone();
        }
        
        let result = self.generate_css_classes();
        self.cached_css = Some(result.clone());
        result
    }
}
```

### 2. String Interning

#### Implementation
```rust
pub struct StringInterner {
    strings: HashMap<String, usize>,
    reverse: Vec<String>,
}

impl StringInterner {
    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(&id) = self.strings.get(s) {
            return id;
        }
        
        let id = self.reverse.len();
        self.strings.insert(s.to_string(), id);
        self.reverse.push(s.to_string());
        id
    }
}
```

### 3. Caching Strategy

#### Implementation
```rust
pub struct ClassCache {
    cache: HashMap<String, String>,
    max_size: usize,
}

impl ClassCache {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.cache.get(key)
    }
    
    pub fn insert(&mut self, key: String, value: String) {
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }
        self.cache.insert(key, value);
    }
}
```

## 🔄 Extension Patterns

### 1. Custom Utilities

#### Implementation
```rust
pub trait CustomUtilities {
    fn custom_property(self, value: CustomValue) -> Self;
}

impl CustomUtilities for ClassBuilder {
    fn custom_property(self, value: CustomValue) -> Self {
        self.class(&format!("custom-{}", value.to_class_name()))
    }
}
```

### 2. Plugin System

#### Implementation
```rust
pub trait UtilityPlugin {
    fn name(&self) -> &str;
    fn utilities(&self) -> Vec<Box<dyn Utility>>;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn UtilityPlugin>>,
}

impl PluginManager {
    pub fn register_plugin(&mut self, plugin: Box<dyn UtilityPlugin>) {
        self.plugins.push(plugin);
    }
}
```

## 📊 Quality Patterns

### 1. Validation Patterns

#### Implementation
```rust
pub struct ValidationRules {
    valid_classes: HashSet<String>,
    conflicts: HashMap<String, Vec<String>>,
    invalid_combinations: Vec<Vec<String>>,
}

impl ValidationRules {
    pub fn validate_class_combination(&self, classes: &[String]) -> Result<(), ValidationError> {
        for class in classes {
            if !self.valid_classes.contains(class) {
                return Err(ValidationError::InvalidClass {
                    class: class.clone(),
                });
            }
        }
        
        Ok(())
    }
}
```

### 2. Testing Patterns

#### Implementation
```rust
pub struct TestHelper {
    validator: ClassValidator,
    cache: ClassCache,
}

impl TestHelper {
    pub fn assert_valid_classes(&self, classes: &[String]) {
        assert!(self.validator.validate_classes(classes).is_ok());
    }
    
    pub fn assert_css_output(&self, builder: ClassBuilder, expected: &str) {
        let classes = builder.build();
        let css = classes.to_css_classes();
        assert_eq!(css, expected);
    }
}
```

---

**Next**: [04-utility-classes.md](./04-utility-classes.md) - Utility class implementation patterns
