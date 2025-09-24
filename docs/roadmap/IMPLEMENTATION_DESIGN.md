# 🎨 Tailwind-RS v4.1 Implementation Design

## 📋 Architecture Overview

This document outlines the technical design and implementation strategy for achieving 100% Tailwind CSS v4.1 parity in Tailwind-RS.

## 🏗️ Core Design Principles

### **1. Type Safety First**
- All utilities are compile-time validated
- Zero runtime errors for invalid classes
- Full IDE autocomplete and IntelliSense support

### **2. Performance Optimized**
- Zero runtime overhead for class generation
- Intelligent caching for repeated operations
- Minimal bundle size impact

### **3. Framework Agnostic**
- Works with Leptos, Yew, Dioxus, and any Rust web framework
- No framework-specific dependencies
- Consistent API across all integrations

### **4. Extensible Architecture**
- Easy to add new utilities and variants
- Plugin system for ecosystem compatibility
- Custom variant support

## 🎯 Phase 1: Critical Missing Features

### **1.1 Text Shadow Utilities**

#### **Design Pattern**
```rust
// Core enum for text shadow values
#[derive(Debug, Clone, PartialEq)]
pub enum TextShadow {
    None,
    Sm,        // text-shadow-sm
    Default,   // text-shadow
    Lg,        // text-shadow-lg
    Xl,        // text-shadow-xl
    Custom(String), // text-shadow-[...]
}

// Implementation in ClassBuilder
impl ClassBuilder {
    pub fn text_shadow(&mut self, shadow: TextShadow) -> &mut Self {
        match shadow {
            TextShadow::None => self.add_class("text-shadow-none"),
            TextShadow::Sm => self.add_class("text-shadow-sm"),
            TextShadow::Default => self.add_class("text-shadow"),
            TextShadow::Lg => self.add_class("text-shadow-lg"),
            TextShadow::Xl => self.add_class("text-shadow-xl"),
            TextShadow::Custom(value) => self.add_class(&format!("text-shadow-[{}]", value)),
        }
        self
    }
}
```

#### **CSS Generation**
```rust
// CSS output for each variant
impl TextShadow {
    pub fn to_css(&self) -> String {
        match self {
            TextShadow::None => "text-shadow: none;".to_string(),
            TextShadow::Sm => "text-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);".to_string(),
            TextShadow::Default => "text-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);".to_string(),
            TextShadow::Lg => "text-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);".to_string(),
            TextShadow::Xl => "text-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);".to_string(),
            TextShadow::Custom(value) => format!("text-shadow: {};", value),
        }
    }
}
```

#### **Testing Strategy**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_shadow_generation() {
        let mut builder = ClassBuilder::new();
        builder.text_shadow(TextShadow::Sm);
        
        let classes = builder.build();
        assert!(classes.contains("text-shadow-sm"));
    }

    #[test]
    fn test_text_shadow_css() {
        let shadow = TextShadow::Sm;
        let css = shadow.to_css();
        assert!(css.contains("text-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05)"));
    }
}
```

### **1.2 Mask Utilities**

#### **Design Pattern**
```rust
// Mask clip utilities
#[derive(Debug, Clone, PartialEq)]
pub enum MaskClip {
    None,
    Border,
    Padding,
    Content,
    Text,
}

// Mask image utilities
#[derive(Debug, Clone, PartialEq)]
pub enum MaskImage {
    None,
    LinearGradient(String),
    RadialGradient(String),
    Custom(String),
}

// Mask mode utilities
#[derive(Debug, Clone, PartialEq)]
pub enum MaskMode {
    Alpha,
    Luminance,
    Custom(String),
}
```

#### **Implementation in ClassBuilder**
```rust
impl ClassBuilder {
    pub fn mask_clip(&mut self, clip: MaskClip) -> &mut Self {
        match clip {
            MaskClip::None => self.add_class("mask-clip-none"),
            MaskClip::Border => self.add_class("mask-clip-border"),
            MaskClip::Padding => self.add_class("mask-clip-padding"),
            MaskClip::Content => self.add_class("mask-clip-content"),
            MaskClip::Text => self.add_class("mask-clip-text"),
        }
        self
    }

    pub fn mask_image(&mut self, image: MaskImage) -> &mut Self {
        match image {
            MaskImage::None => self.add_class("mask-image-none"),
            MaskImage::LinearGradient(value) => self.add_class(&format!("mask-image-[{}]", value)),
            MaskImage::RadialGradient(value) => self.add_class(&format!("mask-image-[{}]", value)),
            MaskImage::Custom(value) => self.add_class(&format!("mask-image-[{}]", value)),
        }
        self
    }
}
```

### **1.3 Enhanced Backdrop Filters**

#### **Design Pattern**
```rust
// Backdrop blur utilities
#[derive(Debug, Clone, PartialEq)]
pub enum BackdropBlur {
    None,
    Sm,      // backdrop-blur-sm
    Default, // backdrop-blur
    Md,      // backdrop-blur-md
    Lg,      // backdrop-blur-lg
    Xl,      // backdrop-blur-xl
    Custom(String),
}

// Backdrop brightness utilities
#[derive(Debug, Clone, PartialEq)]
pub enum BackdropBrightness {
    None,
    Dim,     // backdrop-brightness-50
    Default, // backdrop-brightness-100
    Bright,  // backdrop-brightness-150
    Custom(String),
}
```

#### **CSS Generation**
```rust
impl BackdropBlur {
    pub fn to_css(&self) -> String {
        match self {
            BackdropBlur::None => "backdrop-filter: none;".to_string(),
            BackdropBlur::Sm => "backdrop-filter: blur(4px);".to_string(),
            BackdropBlur::Default => "backdrop-filter: blur(8px);".to_string(),
            BackdropBlur::Md => "backdrop-filter: blur(12px);".to_string(),
            BackdropBlur::Lg => "backdrop-filter: blur(16px);".to_string(),
            BackdropBlur::Xl => "backdrop-filter: blur(24px);".to_string(),
            BackdropBlur::Custom(value) => format!("backdrop-filter: blur({});", value),
        }
    }
}
```

### **1.4 Container Queries**

#### **Design Pattern**
```rust
// Container query utilities
#[derive(Debug, Clone, PartialEq)]
pub enum ContainerQuery {
    Sm,    // @container (min-width: 20rem)
    Md,    // @container (min-width: 28rem)
    Lg,    // @container (min-width: 32rem)
    Xl,    // @container (min-width: 48rem)
    Custom(String),
}

// Container query variants
#[derive(Debug, Clone, PartialEq)]
pub enum ContainerVariant {
    Sm(ContainerQuery),
    Md(ContainerQuery),
    Lg(ContainerQuery),
    Xl(ContainerQuery),
}
```

#### **CSS Generation**
```rust
impl ContainerQuery {
    pub fn to_css_rule(&self) -> String {
        match self {
            ContainerQuery::Sm => "@container (min-width: 20rem) {".to_string(),
            ContainerQuery::Md => "@container (min-width: 28rem) {".to_string(),
            ContainerQuery::Lg => "@container (min-width: 32rem) {".to_string(),
            ContainerQuery::Xl => "@container (min-width: 48rem) {".to_string(),
            ContainerQuery::Custom(value) => format!("@container ({}) {{", value),
        }
    }
}
```

## 🎯 Phase 2: Advanced Layout Features

### **2.1 CSS Grid Subgrid**

#### **Design Pattern**
```rust
// Grid template columns with subgrid support
#[derive(Debug, Clone, PartialEq)]
pub enum GridTemplateColumns {
    Subgrid,
    Repeat(usize, String),
    Custom(String),
}

// Grid template rows with subgrid support
#[derive(Debug, Clone, PartialEq)]
pub enum GridTemplateRows {
    Subgrid,
    Repeat(usize, String),
    Custom(String),
}
```

#### **Implementation**
```rust
impl ClassBuilder {
    pub fn grid_template_columns(&mut self, columns: GridTemplateColumns) -> &mut Self {
        match columns {
            GridTemplateColumns::Subgrid => self.add_class("grid-cols-subgrid"),
            GridTemplateColumns::Repeat(count, value) => {
                self.add_class(&format!("grid-cols-repeat-{}-{}", count, value))
            }
            GridTemplateColumns::Custom(value) => {
                self.add_class(&format!("grid-cols-[{}]", value))
            }
        }
        self
    }
}
```

### **2.2 Logical Properties**

#### **Design Pattern**
```rust
// Logical property utilities
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalProperty {
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

// Logical spacing utilities
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalSpacing {
    InlineStart(SpacingValue),
    InlineEnd(SpacingValue),
    BlockStart(SpacingValue),
    BlockEnd(SpacingValue),
}
```

#### **CSS Generation**
```rust
impl LogicalSpacing {
    pub fn to_css(&self) -> String {
        match self {
            LogicalSpacing::InlineStart(value) => format!("padding-inline-start: {};", value.to_css()),
            LogicalSpacing::InlineEnd(value) => format!("padding-inline-end: {};", value.to_css()),
            LogicalSpacing::BlockStart(value) => format!("padding-block-start: {};", value.to_css()),
            LogicalSpacing::BlockEnd(value) => format!("padding-block-end: {};", value.to_css()),
        }
    }
}
```

## 🎯 Phase 3: Modern CSS Features

### **3.1 Cascade Layers**

#### **Design Pattern**
```rust
// Cascade layer utilities
#[derive(Debug, Clone, PartialEq)]
pub enum CascadeLayer {
    Base,
    Components,
    Utilities,
    Custom(String),
}

// Layer management
pub struct LayerManager {
    layers: Vec<CascadeLayer>,
    current_layer: Option<CascadeLayer>,
}
```

#### **Implementation**
```rust
impl LayerManager {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            current_layer: None,
        }
    }

    pub fn add_layer(&mut self, layer: CascadeLayer) {
        self.layers.push(layer);
    }

    pub fn set_current_layer(&mut self, layer: CascadeLayer) {
        self.current_layer = Some(layer);
    }

    pub fn generate_css(&self) -> String {
        let mut css = String::new();
        for layer in &self.layers {
            css.push_str(&format!("@layer {} {{\n", layer.to_string()));
            // Add layer-specific CSS here
            css.push_str("}\n");
        }
        css
    }
}
```

### **3.2 Registered Custom Properties**

#### **Design Pattern**
```rust
// Custom property utilities
#[derive(Debug, Clone, PartialEq)]
pub struct CustomProperty {
    name: String,
    value: String,
    fallback: Option<String>,
}

// Property registry
pub struct PropertyRegistry {
    properties: HashMap<String, CustomProperty>,
}
```

#### **Implementation**
```rust
impl PropertyRegistry {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn register(&mut self, property: CustomProperty) {
        self.properties.insert(property.name.clone(), property);
    }

    pub fn generate_css(&self) -> String {
        let mut css = String::new();
        css.push_str(":root {\n");
        for (_, property) in &self.properties {
            css.push_str(&format!("  {}: {};", property.name, property.value));
            if let Some(fallback) = &property.fallback {
                css.push_str(&format!(" /* fallback: {} */", fallback));
            }
            css.push_str("\n");
        }
        css.push_str("}\n");
        css
    }
}
```

## 🎯 Phase 4: Enhanced Developer Experience

### **4.1 Advanced Plugin System**

#### **Design Pattern**
```rust
// Plugin trait
pub trait TailwindPlugin {
    fn name(&self) -> &str;
    fn utilities(&self) -> Vec<UtilityClass>;
    fn components(&self) -> Vec<ComponentClass>;
    fn variants(&self) -> Vec<VariantClass>;
}

// Plugin manager
pub struct PluginManager {
    plugins: Vec<Box<dyn TailwindPlugin>>,
}
```

#### **Implementation**
```rust
impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: vec![],
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn TailwindPlugin>) {
        self.plugins.push(plugin);
    }

    pub fn generate_utilities(&self) -> Vec<UtilityClass> {
        let mut utilities = Vec::new();
        for plugin in &self.plugins {
            utilities.extend(plugin.utilities());
        }
        utilities
    }
}
```

### **4.2 Enhanced Validation**

#### **Design Pattern**
```rust
// Validation error types
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidClass(String),
    ConflictingClasses(Vec<String>),
    MissingDependency(String),
    UnsupportedVariant(String),
}

// Validation context
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationContext {
    ClassGeneration,
    FrameworkIntegration,
    PluginSystem,
    CustomVariant,
}
```

#### **Implementation**
```rust
pub struct ClassValidator {
    valid_classes: HashSet<String>,
    conflicting_classes: HashMap<String, Vec<String>>,
    dependencies: HashMap<String, Vec<String>>,
}

impl ClassValidator {
    pub fn new() -> Self {
        Self {
            valid_classes: HashSet::new(),
            conflicting_classes: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    pub fn validate_class(&self, class: &str) -> Result<(), ValidationError> {
        if !self.valid_classes.contains(class) {
            return Err(ValidationError::InvalidClass(class.to_string()));
        }
        Ok(())
    }

    pub fn validate_class_combination(&self, classes: &[String]) -> Result<(), ValidationError> {
        for class in classes {
            if let Some(conflicts) = self.conflicting_classes.get(class) {
                for conflict in conflicts {
                    if classes.contains(&conflict.to_string()) {
                        return Err(ValidationError::ConflictingClasses(vec![
                            class.clone(),
                            conflict.clone(),
                        ]));
                    }
                }
            }
        }
        Ok(())
    }
}
```

## 🎯 Phase 5: Testing & Documentation

### **5.1 Comprehensive Test Suite**

#### **Test Categories**
```rust
// Unit tests for each utility
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_text_shadow_generation() {
        // Test text shadow utility generation
    }

    #[test]
    fn test_mask_utilities() {
        // Test mask utility generation
    }
}

// Integration tests for framework compatibility
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_leptos_integration() {
        // Test Leptos framework integration
    }

    #[test]
    fn test_yew_integration() {
        // Test Yew framework integration
    }
}

// Performance tests
#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_bundle_size() {
        // Test bundle size constraints
    }

    #[test]
    fn test_runtime_performance() {
        // Test runtime performance
    }
}
```

### **5.2 Documentation Updates**

#### **Documentation Structure**
```
docs/
├── roadmap/
│   ├── TAILWIND_V4_1_PARITY_ROADMAP.md
│   └── IMPLEMENTATION_DESIGN.md
├── features/
│   ├── text-shadow.md
│   ├── mask-utilities.md
│   ├── container-queries.md
│   └── modern-css.md
├── migration/
│   ├── v3-to-v4.md
│   └── feature-comparison.md
└── examples/
    ├── text-shadow-examples.md
    ├── mask-examples.md
    └── container-query-examples.md
```

## 🚀 Implementation Strategy

### **Development Workflow**
1. **Feature Branch**: Create branch for each phase
2. **Implementation**: Implement features according to design
3. **Testing**: Add comprehensive tests
4. **Documentation**: Update documentation
5. **Review**: Code review and testing
6. **Merge**: Merge to main branch

### **Quality Assurance**
1. **Unit Tests**: 100% coverage for new utilities
2. **Integration Tests**: Framework compatibility
3. **Performance Tests**: Bundle size and runtime performance
4. **Visual Tests**: Cross-browser compatibility

### **Release Strategy**
1. **Phase 1**: Release critical features (2-3 weeks)
2. **Phase 2**: Release advanced layout features (2-3 weeks)
3. **Phase 3**: Release modern CSS features (2-3 weeks)
4. **Phase 4**: Release enhanced developer experience (1-2 weeks)
5. **Phase 5**: Release testing and documentation (1-2 weeks)

This design provides a comprehensive technical foundation for achieving 100% Tailwind CSS v4.1 parity in Tailwind-RS.
