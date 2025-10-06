# 📐 Advanced Variants Architecture Design

## 📋 Overview

**Current Status**: ⚠️ Partially Implemented (40% Complete)  
**Priority**: High  
**Timeline**: 6-8 weeks  
**Dependencies**: Core variant system, CSS parser extensions

## 🎯 Mission

Implement advanced CSS variants including container queries, custom variants, and arbitrary value variants to match the full feature set of Tailwind CSS v4's variant system.

## 📊 Current State Analysis

### ✅ **What's Working**
- Basic responsive variants (sm, md, lg, xl, 2xl)
- State variants (hover, focus, active, disabled)
- Group variants (group-hover, group-focus, etc.)
- Dark mode variant

### ❌ **What's Missing**
- Container queries (`@container`)
- Custom variants (`@custom-variant`)
- Arbitrary variants (`[data-*]`, `[aria-*]`)
- Size variants (`@min-*`, `@max-*`)
- Orientation variants (`@portrait`, `@landscape`)
- Reduced motion variants (`@reduced-motion`)
- Contrast variants (`@high-contrast`, `@low-contrast`)
- Color scheme variants (`@light`, `@dark`)
- Motion variants (`@motion-safe`, `@motion-reduce`)

## 🏗️ Architecture Design

### **Container Queries**

#### **1. Container Query Parser**
```rust
pub struct ContainerQueryParser {
    containers: HashMap<String, ContainerDefinition>,
}

#[derive(Debug, Clone)]
pub struct ContainerDefinition {
    pub name: Option<String>,
    pub conditions: Vec<ContainerCondition>,
}

#[derive(Debug, Clone)]
pub enum ContainerCondition {
    Width { min: Option<String>, max: Option<String> },
    Height { min: Option<String>, max: Option<String> },
    AspectRatio(String),
    Orientation(Orientation),
}

impl ContainerQueryParser {
    pub fn parse_container_rule(&self, css: &str) -> Result<Vec<ContainerRule>, ParseError> {
        // Parse @container rules and extract definitions
        Ok(rules)
    }

    pub fn generate_container_css(&self, rules: &[ContainerRule]) -> String {
        // Generate CSS with container queries
        format!("@container {} {{\\n{}\\n}}", condition, content)
    }
}
```

#### **2. Container Variant Processor**
```rust
pub struct ContainerVariantProcessor {
    parser: ContainerQueryParser,
    container_stack: Vec<ContainerDefinition>,
}

impl ContainerVariantProcessor {
    pub fn process_container_variant(&mut self, class: &str) -> Result<ProcessedVariant, VariantError> {
        // Handle container variants like @container-md:flex
        // Parse container conditions
        // Generate appropriate CSS
        Ok(processed)
    }

    pub fn add_container_definition(&mut self, name: Option<&str>, conditions: &[ContainerCondition]) {
        // Register container definitions for later use
        let definition = ContainerDefinition {
            name: name.map(String::from),
            conditions: conditions.to_vec(),
        };
        self.container_stack.push(definition);
    }
}
```

### **Custom Variants**

#### **1. Custom Variant Definition**
```rust
#[derive(Debug, Clone)]
pub struct CustomVariant {
    pub name: String,
    pub selector: String,
    pub order: i32,
    pub compounds_with: Vec<VariantType>,
}

pub enum VariantType {
    Static,
    Dynamic,
    Responsive,
    State,
    Container,
}

pub struct CustomVariantManager {
    variants: HashMap<String, CustomVariant>,
    order_counter: i32,
}

impl CustomVariantManager {
    pub fn define_variant(&mut self, name: &str, selector: &str) -> Result<(), VariantError> {
        // Parse and validate custom variant
        // Assign ordering
        // Store definition
        Ok(())
    }

    pub fn apply_custom_variant(&self, class: &str, variant: &str) -> Result<String, VariantError> {
        // Apply custom variant to class
        // Generate CSS selector
        Ok(selector)
    }
}
```

#### **2. Custom Variant Parser**
```rust
pub struct CustomVariantParser {
    manager: CustomVariantManager,
}

impl CustomVariantParser {
    pub fn parse_custom_variant_definition(&mut self, css: &str) -> Result<(), ParseError> {
        // Parse @custom-variant definitions
        // Example: @custom-variant hover-visible (:hover:not(.hidden))
        Ok(())
    }

    pub fn expand_custom_variant(&self, class: &str) -> Result<Vec<String>, ExpandError> {
        // Expand classes using custom variants
        // Return all possible combinations
        Ok(expanded_classes)
    }
}
```

### **Arbitrary Variants**

#### **1. Arbitrary Variant Processor**
```rust
pub struct ArbitraryVariantProcessor {
    attribute_parser: AttributeParser,
    pseudo_parser: PseudoParser,
}

impl ArbitraryVariantProcessor {
    pub fn process_arbitrary_variant(&self, variant: &str) -> Result<ArbitraryVariant, VariantError> {
        // Handle variants like [data-visible=true]:flex
        // Parse attribute selectors
        // Validate syntax
        Ok(arbitrary_variant)
    }

    pub fn generate_arbitrary_selector(&self, variant: &ArbitraryVariant, base_selector: &str) -> String {
        // Generate CSS selector with arbitrary conditions
        format!("{}[data-visible=\"true\"] {}", variant.prefix, base_selector)
    }
}

#[derive(Debug)]
pub struct ArbitraryVariant {
    pub condition: String,
    pub prefix: String,
    pub value: Option<String>,
}
```

#### **2. Attribute Parser**
```rust
pub struct AttributeParser {
    valid_attributes: HashSet<String>,
}

impl AttributeParser {
    pub fn parse_attribute_selector(&self, selector: &str) -> Result<AttributeSelector, ParseError> {
        // Parse [data-visible=true], [aria-expanded], etc.
        // Validate attribute names and values
        Ok(attribute_selector)
    }

    pub fn validate_attribute(&self, name: &str, value: Option<&str>) -> bool {
        // Validate attribute syntax
        // Check for security issues
        true
    }
}
```

### **Size Variants**

#### **1. Size Query Processor**
```rust
pub struct SizeQueryProcessor {
    breakpoints: HashMap<String, String>,
}

impl SizeQueryProcessor {
    pub fn process_size_variant(&self, variant: &str) -> Result<String, VariantError> {
        // Handle @min-lg:flex, @max-md:grid, etc.
        // Convert to media queries
        Ok(media_query)
    }

    pub fn generate_size_media_query(&self, condition: &SizeCondition) -> String {
        match condition {
            SizeCondition::Min(breakpoint) => format!("(min-width: {})", self.breakpoints[breakpoint]),
            SizeCondition::Max(breakpoint) => format!("(max-width: {})", self.breakpoints[breakpoint]),
        }
    }
}

pub enum SizeCondition {
    Min(String),
    Orientation(Orientation),
    ReducedMotion,
    HighContrast,
    // ... other size conditions
}
```

## 🚀 Implementation Plan

### **Phase 1: Container Queries** (Week 1-2)

#### **Step 1.1: Container Query Parser**
- [ ] Implement @container rule parsing
- [ ] Add container definition storage
- [ ] Create container condition validation

#### **Step 1.2: Container Variants**
- [ ] Add container variant syntax (@container-md:)
- [ ] Implement container-aware CSS generation
- [ ] Add container query CSS output

#### **Step 1.3: Integration**
- [ ] Connect to variant processing pipeline
- [ ] Update CSS output for container queries
- [ ] Add container query testing

### **Phase 2: Custom Variants** (Week 3-4)

#### **Step 2.1: Custom Variant Definition**
- [ ] Implement @custom-variant directive parsing
- [ ] Add custom variant storage and management
- [ ] Create variant ordering system

#### **Step 2.2: Custom Variant Application**
- [ ] Implement custom variant expansion
- [ ] Add custom variant to class processing
- [ ] Update selector generation

#### **Step 2.3: Advanced Features**
- [ ] Add compound variant support
- [ ] Implement variant inheritance
- [ ] Add custom variant validation

### **Phase 3: Arbitrary Variants** (Week 5-6)

#### **Step 3.1: Arbitrary Variant Parser**
- [ ] Implement attribute selector parsing
- [ ] Add arbitrary variant validation
- [ ] Create security checks

#### **Step 3.2: Arbitrary Selector Generation**
- [ ] Implement arbitrary selector generation
- [ ] Add attribute selector support
- [ ] Update CSS output pipeline

#### **Step 3.3: Complex Arbitrary Variants**
- [ ] Add pseudo-class arbitrary variants
- [ ] Implement complex selector support
- [ ] Add arbitrary variant caching

### **Phase 4: Size & Environment Variants** (Week 7-8)

#### **Step 4.1: Size Query Variants**
- [ ] Implement @min- and @max- variants
- [ ] Add size condition parsing
- [ ] Generate appropriate media queries

#### **Step 4.2: Environment Variants**
- [ ] Add @portrait/@landscape variants
- [ ] Implement @reduced-motion support
- [ ] Add contrast and color scheme variants

#### **Step 4.3: Integration & Testing**
- [ ] Connect all variant types to main pipeline
- [ ] Add comprehensive variant testing
- [ ] Performance optimization

## 🧪 Testing Strategy

### **Unit Tests**
- [ ] Variant parsing accuracy
- [ ] Selector generation correctness
- [ ] Condition validation
- [ ] Error handling

### **Integration Tests**
- [ ] End-to-end variant processing
- [ ] CSS generation with complex variants
- [ ] Browser compatibility
- [ ] Performance impact

### **Compatibility Tests**
- [ ] Tailwind CSS variant compatibility
- [ ] Existing project migration
- [ ] Complex selector support
- [ ] Edge case handling

## 📊 Success Criteria

### **Functional Requirements**
- [ ] ✅ Container queries (@container) working
- [ ] ✅ Custom variants (@custom-variant) supported
- [ ] ✅ Arbitrary variants ([data-*]:class) working
- [ ] ✅ Size variants (@min-*, @max-*) implemented
- [ ] ✅ Environment variants (@portrait, @reduced-motion) added
- [ ] ✅ All variants generate correct CSS selectors

### **Performance Requirements**
- [ ] ✅ Variant processing < 10% of total build time
- [ ] ✅ CSS output size reasonable
- [ ] ✅ No performance regression
- [ ] ✅ Memory usage stable

### **Compatibility Requirements**
- [ ] ✅ Full Tailwind CSS variant compatibility
- [ ] ✅ Same CSS output as official implementation
- [ ] ✅ Same API surface area
- [ ] ✅ Same error messages and behavior

## 🔗 Dependencies

### **Required Before Implementation**
- ✅ Core variant system
- ✅ CSS selector generation
- ✅ Parser infrastructure
- ✅ Configuration system

### **Required During Implementation**
- 🔄 CSS parser extensions
- 🔄 Selector generation updates
- 🔄 Configuration system updates
- 🔄 Testing infrastructure

## 📈 Timeline & Milestones

| Week | Milestone | Deliverables |
|------|-----------|--------------|
| 1-2 | Container queries | @container support, container variants |
| 3-4 | Custom variants | @custom-variant directive, variant management |
| 5-6 | Arbitrary variants | Attribute selectors, complex variants |
| 7-8 | Size variants | @min/@max, environment variants, integration |

## 🚨 Risk Assessment

### **High Risk**
- **CSS Complexity**: Generating correct selectors for complex variants
- **Browser Compatibility**: Ensuring variants work across browsers
- **Performance**: Complex variant processing overhead

### **Mitigation Strategies**
- **Incremental Implementation**: Start with simple variants, add complexity
- **Extensive Testing**: Test all variants across browsers
- **Performance Monitoring**: Benchmark variant processing

## 📚 Related Documents

- [Variant API Reference](./api-reference.md)
- [Container Query Examples](./container-examples.md)
- [Custom Variant Guide](./custom-variants.md)

---

*This design document outlines a comprehensive variant system that will provide full compatibility with Tailwind CSS's advanced variant features while maintaining performance and correctness.*
