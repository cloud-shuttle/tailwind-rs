# 🎨 CSS Generation System Design

> **Purpose**: Design the core CSS generation system for Tailwind-RS  
> **File Size**: <300 lines per module  
> **Priority**: Critical (Phase 1)

## 📋 **Overview**

The CSS generation system is the core value proposition of Tailwind-RS. It must generate actual CSS files from Rust class definitions, not just class name strings.

## 🏗️ **Architecture**

### **Core Components**

1. **CSS Generator** (`css_generator.rs`)
   - Converts class names to CSS rules
   - Handles responsive variants
   - Manages CSS specificity

2. **CSS Optimizer** (`css_optimizer.rs`)
   - Removes unused CSS
   - Minifies output
   - Generates source maps

3. **CSS Builder** (`css_builder.rs`)
   - Orchestrates generation process
   - Manages file I/O
   - Handles build configuration

## 🎯 **Design Requirements**

### **Functional Requirements**
- Generate valid CSS from Tailwind class names
- Support responsive breakpoints
- Handle custom CSS properties
- Support CSS minification
- Generate source maps

### **Non-Functional Requirements**
- **Performance**: Generate CSS in <100ms for typical projects
- **Memory**: <50MB peak memory usage
- **Accuracy**: 100% compatibility with Tailwind CSS output
- **Reliability**: Handle edge cases gracefully

## 🔧 **Implementation Design**

### **CSS Generator Module**

```rust
// css_generator.rs (<300 lines)
pub struct CssGenerator {
    rules: HashMap<String, CssRule>,
    breakpoints: HashMap<Breakpoint, String>,
    custom_properties: HashMap<String, String>,
}

impl CssGenerator {
    pub fn new() -> Self;
    pub fn add_class(&mut self, class: &str) -> Result<()>;
    pub fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: &str) -> Result<()>;
    pub fn add_custom_property(&mut self, name: &str, value: &str);
    pub fn generate_css(&self) -> String;
    pub fn generate_minified_css(&self) -> String;
}
```

### **CSS Rule Structure**

```rust
pub struct CssRule {
    pub selector: String,
    pub properties: Vec<CssProperty>,
    pub media_query: Option<String>,
    pub specificity: u32,
}

pub struct CssProperty {
    pub name: String,
    pub value: String,
    pub important: bool,
}
```

### **Class to CSS Mapping**

```rust
impl CssGenerator {
    fn class_to_css_rule(&self, class: &str) -> Result<CssRule> {
        match class {
            "p-4" => Ok(CssRule {
                selector: ".".to_string() + class,
                properties: vec![CssProperty {
                    name: "padding".to_string(),
                    value: "1rem".to_string(),
                    important: false,
                }],
                media_query: None,
                specificity: 10,
            }),
            "bg-blue-500" => Ok(CssRule {
                selector: ".".to_string() + class,
                properties: vec![CssProperty {
                    name: "background-color".to_string(),
                    value: "#3b82f6".to_string(),
                    important: false,
                }],
                media_query: None,
                specificity: 10,
            }),
            _ => Err(TailwindError::UnknownClass(class.to_string())),
        }
    }
}
```

## 🧪 **Testing Strategy**

### **Unit Tests**
- Class to CSS rule conversion
- Responsive variant handling
- Custom property support
- CSS minification

### **Integration Tests**
- Full CSS generation pipeline
- File I/O operations
- Build system integration

### **Property-Based Tests**
- Random class combinations
- Edge case handling
- Performance validation

## 📊 **Performance Considerations**

### **Optimization Strategies**
- **Rule caching**: Cache generated CSS rules
- **Lazy evaluation**: Generate CSS only when needed
- **Incremental updates**: Only regenerate changed rules
- **Parallel processing**: Generate multiple files concurrently

### **Memory Management**
- **Rule deduplication**: Remove duplicate CSS rules
- **String interning**: Reduce memory usage for repeated strings
- **Garbage collection**: Clean up unused rules

## 🔄 **Integration Points**

### **Build System Integration**
```rust
impl TailwindBuilder {
    pub fn build(self) -> Result<()> {
        let mut generator = CssGenerator::new();
        
        // Scan source files for classes
        let classes = self.scan_classes()?;
        
        // Generate CSS rules
        for class in classes {
            generator.add_class(&class)?;
        }
        
        // Generate and write CSS file
        let css = generator.generate_css();
        std::fs::write(&self.output_path, css)?;
        
        Ok(())
    }
}
```

### **Framework Integration**
```rust
// Leptos integration
impl ClassBuilder {
    pub fn to_css_file(&self, path: &str) -> Result<()> {
        let mut generator = CssGenerator::new();
        generator.add_class(&self.to_string())?;
        
        let css = generator.generate_css();
        std::fs::write(path, css)?;
        
        Ok(())
    }
}
```

## 🚀 **Implementation Phases**

### **Phase 1: Basic Generation**
- Core CSS rule generation
- Basic class mapping
- File I/O operations

### **Phase 2: Advanced Features**
- Responsive variants
- Custom properties
- CSS minification

### **Phase 3: Optimization**
- Performance optimization
- Memory management
- Caching strategies

## 📈 **Success Metrics**

| Metric | Target | Measurement |
|--------|--------|-------------|
| CSS Generation Speed | <100ms | Benchmark tests |
| Memory Usage | <50MB | Memory profiling |
| CSS Accuracy | 100% | Comparison with Tailwind CSS |
| Test Coverage | >95% | Code coverage reports |

## 🔍 **Risk Mitigation**

### **Technical Risks**
- **CSS compatibility**: Comprehensive testing against Tailwind CSS
- **Performance**: Continuous benchmarking and optimization
- **Memory usage**: Memory profiling and optimization

### **Implementation Risks**
- **Scope creep**: Strict adherence to <300 line limit
- **Integration issues**: Early integration testing
- **Breaking changes**: Semantic versioning

---

**Next Steps**: Implement basic CSS generation module with core class mapping functionality.
