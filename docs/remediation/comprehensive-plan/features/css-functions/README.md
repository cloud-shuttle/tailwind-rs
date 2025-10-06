# 🎨 CSS Functions Architecture Design

## 📋 Overview

**Current Status**: ❌ Not Implemented (0% Complete)  
**Priority**: Critical  
**Timeline**: 6-8 weeks  
**Dependencies**: CSS parser, AST manipulation, plugin system

## 🎯 Mission

Implement core Tailwind CSS functions including `@apply`, `@layer`, `@import`, and `@tailwind` directives to provide complete compatibility with existing Tailwind CSS workflows and enable advanced CSS composition patterns.

## 📊 Current State Analysis

### ✅ **What's Working**
- Basic CSS generation pipeline
- Individual class processing
- CSS property generation

### ❌ **What's Missing**
- `@apply` directive for utility composition
- `@layer` directive for CSS cascade control
- `@import` directive for CSS modularization
- `@tailwind` directive parsing (basic)
- CSS AST manipulation for function processing
- Cascade layer management
- Import resolution and dependency tracking

## 🏗️ Architecture Design

### **@apply Directive**

#### **1. Apply Parser**
```rust
pub struct ApplyParser {
    class_parser: ClassParser,
    utility_resolver: UtilityResolver,
}

impl ApplyParser {
    pub fn parse_apply_rule(&self, css: &str) -> Result<Vec<CssProperty>, ApplyError> {
        // Parse @apply flex items-center justify-center
        // Resolve utility classes to properties
        // Return expanded properties
        Ok(properties)
    }

    pub fn validate_apply_classes(&self, classes: &[String]) -> Result<(), ValidationError> {
        // Validate that applied classes exist
        // Check for circular dependencies
        // Ensure no conflicting properties
        Ok(())
    }
}

pub struct UtilityResolver {
    utilities: HashMap<String, Vec<CssProperty>>,
    variants: HashMap<String, VariantDefinition>,
}

impl UtilityResolver {
    pub fn resolve_utility(&self, utility: &str) -> Result<Vec<CssProperty>, ResolveError> {
        // Resolve single utility to properties
        // Handle variants (hover:flex -> :hover flex properties)
        Ok(properties)
    }

    pub fn resolve_utilities(&self, utilities: &[String]) -> Result<Vec<CssProperty>, ResolveError> {
        // Resolve multiple utilities
        // Handle conflicts and precedence
        Ok(merged_properties)
    }
}
```

#### **2. Apply Processor**
```rust
pub struct ApplyProcessor {
    parser: ApplyParser,
    resolver: UtilityResolver,
}

impl ApplyProcessor {
    pub fn process_apply_directives(&self, css: &str) -> Result<String, ProcessError> {
        // Find all @apply directives
        // Parse and resolve utilities
        // Replace with expanded properties
        // Handle nesting and scoping
        Ok(processed_css)
    }

    pub fn expand_apply_rule(&self, apply_value: &str, scope: &CssScope) -> Result<String, ExpandError> {
        // Expand single @apply rule
        // Respect CSS scope and specificity
        Ok(expanded_css)
    }
}
```

### **@layer Directive**

#### **1. Layer Manager**
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CascadeLayer {
    Base,
    Components,
    Utilities,
    Custom(String),
}

pub struct LayerManager {
    layers: HashMap<CascadeLayer, Vec<CssRule>>,
    order: Vec<CascadeLayer>,
}

impl LayerManager {
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
            order: vec![CascadeLayer::Base, CascadeLayer::Components, CascadeLayer::Utilities],
        }
    }

    pub fn add_layer(&mut self, layer: CascadeLayer) {
        if !self.order.contains(&layer) {
            self.order.push(layer);
        }
    }

    pub fn add_rule_to_layer(&mut self, layer: CascadeLayer, rule: CssRule) {
        self.layers.entry(layer).or_insert_with(Vec::new).push(rule);
    }

    pub fn generate_layered_css(&self) -> String {
        let mut css = String::new();

        for layer in &self.order {
            if let Some(rules) = self.layers.get(layer) {
                css.push_str(&format!("@layer {} {{\n", layer_to_string(layer)));
                for rule in rules {
                    css.push_str(&format!("  {}\n", rule.to_css()));
                }
                css.push_str("}\n\n");
            }
        }

        css
    }
}

fn layer_to_string(layer: &CascadeLayer) -> &str {
    match layer {
        CascadeLayer::Base => "base",
        CascadeLayer::Components => "components",
        CascadeLayer::Utilities => "utilities",
        CascadeLayer::Custom(name) => name,
    }
}
```

#### **2. Layer Parser**
```rust
pub struct LayerParser {
    manager: LayerManager,
}

impl LayerParser {
    pub fn parse_layer_directive(&mut self, css: &str) -> Result<(), ParseError> {
        // Parse @layer base, components, utilities
        // Register layers in order
        Ok(())
    }

    pub fn parse_layer_block(&mut self, css: &str) -> Result<(), ParseError> {
        // Parse @layer utilities { .btn { ... } }
        // Add rules to appropriate layer
        Ok(())
    }
}
```

### **@import Directive**

#### **1. Import Resolver**
```rust
pub struct ImportResolver {
    base_path: PathBuf,
    resolved_imports: HashMap<String, String>,
    dependency_graph: DependencyGraph,
}

impl ImportResolver {
    pub fn resolve_import(&mut self, import_path: &str) -> Result<String, ResolveError> {
        // Resolve relative/absolute import paths
        // Handle CSS imports, Tailwind config imports
        // Prevent circular dependencies
        // Cache resolved content
        Ok(resolved_content)
    }

    pub fn resolve_css_import(&self, import_path: &str) -> Result<String, ResolveError> {
        // Resolve CSS @import statements
        // Handle Tailwind-specific imports
        Ok(css_content)
    }
}

pub struct DependencyGraph {
    nodes: HashMap<String, Vec<String>>, // file -> dependencies
}

impl DependencyGraph {
    pub fn add_dependency(&mut self, file: &str, dependency: &str) {
        self.nodes.entry(file.to_string()).or_insert_with(Vec::new).push(dependency.to_string());
    }

    pub fn detect_cycles(&self) -> Result<(), CycleError> {
        // Detect circular import dependencies
        Ok(())
    }
}
```

#### **2. Import Processor**
```rust
pub struct ImportProcessor {
    resolver: ImportResolver,
    processed_files: HashSet<String>,
}

impl ImportProcessor {
    pub fn process_imports(&mut self, css: &str) -> Result<String, ProcessError> {
        // Find all @import statements
        // Resolve and inline imported content
        // Maintain proper order and dependencies
        // Handle duplicate imports
        Ok(processed_css)
    }

    pub fn inline_import(&mut self, import_path: &str) -> Result<String, InlineError> {
        // Inline single import
        // Cache for performance
        // Handle different import types
        Ok(inlined_content)
    }
}
```

### **@tailwind Directive**

#### **1. Tailwind Directive Parser**
```rust
pub struct TailwindDirectiveParser {
    directives: Vec<TailwindDirective>,
}

#[derive(Debug, Clone)]
pub enum TailwindDirective {
    Base,
    Components,
    Utilities,
    Variants, // Legacy
    Screens,  // Legacy
}

impl TailwindDirectiveParser {
    pub fn parse_directives(&self, css: &str) -> Result<Vec<TailwindDirective>, ParseError> {
        // Parse @tailwind base, @tailwind components, etc.
        // Validate directive order and compatibility
        Ok(directives)
    }

    pub fn generate_tailwind_css(&self, directives: &[TailwindDirective], config: &Config) -> String {
        // Generate appropriate CSS for each directive
        // Include base styles, component classes, utilities
        let mut css = String::new();

        for directive in directives {
            match directive {
                TailwindDirective::Base => {
                    css.push_str(&self.generate_base_css());
                }
                TailwindDirective::Components => {
                    css.push_str(&self.generate_component_css(config));
                }
                TailwindDirective::Utilities => {
                    css.push_str(&self.generate_utility_css(config));
                }
                _ => {} // Legacy directives
            }
        }

        css
    }
}
```

## 🚀 Implementation Plan

### **Phase 1: @apply Directive** (Week 1-2)

#### **Step 1.1: Apply Parser**
- [ ] Implement @apply syntax parsing
- [ ] Create utility resolution system
- [ ] Add apply validation

#### **Step 1.2: Apply Processing**
- [ ] Implement property expansion
- [ ] Handle variant application
- [ ] Add conflict resolution

#### **Step 1.3: Integration**
- [ ] Connect to CSS processing pipeline
- [ ] Update CSS output generation
- [ ] Add comprehensive testing

### **Phase 2: @layer Directive** (Week 3-4)

#### **Step 2.1: Layer Management**
- [ ] Implement cascade layer system
- [ ] Add layer ordering and specificity
- [ ] Create layer CSS generation

#### **Step 2.2: Layer Parser**
- [ ] Parse @layer directives
- [ ] Handle layer blocks
- [ ] Add layer validation

#### **Step 2.3: CSS Organization**
- [ ] Organize CSS by layers
- [ ] Maintain proper cascade order
- [ ] Optimize layer output

### **Phase 3: @import Directive** (Week 5-6)

#### **Step 3.1: Import Resolution**
- [ ] Implement import path resolution
- [ ] Add dependency tracking
- [ ] Handle circular dependency detection

#### **Step 3.2: Import Processing**
- [ ] Inline imported CSS
- [ ] Cache resolved imports
- [ ] Handle different import types

#### **Step 3.3: Import Optimization**
- [ ] Deduplicate imports
- [ ] Optimize import order
- [ ] Add import validation

### **Phase 4: @tailwind Directive** (Week 7-8)

#### **Step 4.1: Tailwind Parser**
- [ ] Parse @tailwind directives
- [ ] Validate directive combinations
- [ ] Generate appropriate CSS

#### **Step 4.2: CSS Generation**
- [ ] Implement base CSS generation
- [ ] Add component CSS generation
- [ ] Create utility CSS generation

#### **Step 4.3: Integration & Optimization**
- [ ] Connect all directives to pipeline
- [ ] Optimize CSS output
- [ ] Add comprehensive testing

## 🧪 Testing Strategy

### **Unit Tests**
- [ ] Directive parsing accuracy
- [ ] CSS generation correctness
- [ ] Import resolution logic
- [ ] Layer management

### **Integration Tests**
- [ ] End-to-end CSS processing
- [ ] Complex @apply scenarios
- [ ] Layer cascade behavior
- [ ] Import dependency handling

### **Compatibility Tests**
- [ ] Tailwind CSS compatibility
- [ ] Existing project migration
- [ ] CSS output comparison
- [ ] Performance benchmarking

## 📊 Success Criteria

### **Functional Requirements**
- [ ] ✅ @apply directive working with utility classes
- [ ] ✅ @layer directive controlling CSS cascade
- [ ] ✅ @import directive resolving CSS dependencies
- [ ] ✅ @tailwind directives generating appropriate CSS
- [ ] ✅ All directives handle variants and complex selectors

### **Performance Requirements**
- [ ] ✅ CSS processing < 20% overhead
- [ ] ✅ Import resolution caching working
- [ ] ✅ Layer organization efficient
- [ ] ✅ No performance regression

### **Compatibility Requirements**
- [ ] ✅ Drop-in replacement for Tailwind directives
- [ ] ✅ Same CSS output as official implementation
- [ ] ✅ Same error handling and validation
- [ ] ✅ Same import resolution behavior

## 🔗 Dependencies

### **Required Before Implementation**
- ✅ CSS parser and AST
- ✅ Utility generation system
- ✅ Configuration system
- ✅ File system utilities

### **Required During Implementation**
- 🔄 Plugin system (for custom utilities)
- 🔄 CSS optimization (for output processing)
- 🔄 Content scanning (for import resolution)
- 🔄 Testing infrastructure

## 📈 Timeline & Milestones

| Week | Milestone | Deliverables |
|------|-----------|--------------|
| 1-2 | @apply implementation | Apply parsing, utility resolution, processing |
| 3-4 | @layer implementation | Layer management, parsing, CSS organization |
| 5-6 | @import implementation | Import resolution, processing, optimization |
| 7-8 | @tailwind implementation | Directive parsing, CSS generation, integration |

## 🚨 Risk Assessment

### **High Risk**
- **CSS Cascade Complexity**: Correct layer ordering and specificity
- **Import Resolution**: Complex dependency management
- **Performance Impact**: CSS processing overhead

### **Mitigation Strategies**
- **Incremental Testing**: Test each directive independently
- **Performance Monitoring**: Benchmark CSS processing pipeline
- **Compatibility Validation**: Extensive testing against real projects

## 📚 Related Documents

- [CSS Functions API](./api-reference.md)
- [@apply Examples](./apply-examples.md)
- [@layer Guide](./layer-guide.md)
- [Import Handling](./import-handling.md)

---

*This design document outlines a comprehensive CSS functions system that will provide complete compatibility with Tailwind CSS's directive-based workflow while maintaining high performance and correctness.*
