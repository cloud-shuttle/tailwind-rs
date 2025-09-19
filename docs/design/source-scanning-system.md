# 🔍 Source Scanning System Design

> **Purpose**: Design the source file scanning system for class extraction  
> **File Size**: <300 lines per module  
> **Priority**: High (Phase 2)

## 📋 **Overview**

The source scanning system analyzes Rust source files to extract Tailwind class usage, enabling tree-shaking and optimization. It must parse Rust AST to find `ClassBuilder` usage patterns.

## 🏗️ **Architecture**

### **Core Components**

1. **AST Parser** (`ast_parser.rs`)
   - Parses Rust source files
   - Extracts class usage patterns
   - Handles macro expansions

2. **Class Scanner** (`class_scanner.rs`)
   - Scans parsed AST for class usage
   - Identifies responsive variants
   - Handles conditional classes

3. **Dependency Analyzer** (`dependency_analyzer.rs`)
   - Builds dependency graph
   - Identifies unused classes
   - Enables tree-shaking

## 🎯 **Design Requirements**

### **Functional Requirements**
- Parse Rust source files for class usage
- Extract class names from `ClassBuilder` calls
- Handle responsive and conditional variants
- Build dependency graph for optimization
- Support incremental scanning

### **Non-Functional Requirements**
- **Performance**: Scan 1000+ files in <5 seconds
- **Accuracy**: 99.9% class extraction accuracy
- **Memory**: <100MB peak memory usage
- **Reliability**: Handle malformed code gracefully

## 🔧 **Implementation Design**

### **AST Parser Module**

```rust
// ast_parser.rs (<300 lines)
use syn::{parse_file, visit::Visit, Expr, ExprCall, ExprMethodCall};

pub struct AstParser {
    classes: HashSet<String>,
    responsive_classes: HashMap<Breakpoint, HashSet<String>>,
    conditional_classes: HashMap<String, HashSet<String>>,
}

impl AstParser {
    pub fn new() -> Self;
    pub fn parse_file(&mut self, path: &Path) -> Result<()>;
    pub fn parse_content(&mut self, content: &str) -> Result<()>;
    pub fn get_classes(&self) -> &HashSet<String>;
    pub fn get_responsive_classes(&self) -> &HashMap<Breakpoint, HashSet<String>>;
}
```

### **Class Extraction Logic**

```rust
impl<'ast> Visit<'ast> for ClassVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        // Handle ClassBuilder method calls
        if let Some(ident) = node.method.get_ident() {
            match ident.to_string().as_str() {
                "class" => self.extract_class_call(node),
                "padding" => self.extract_spacing_call(node, "p"),
                "margin" => self.extract_spacing_call(node, "m"),
                "background_color" => self.extract_color_call(node, "bg"),
                "text_color" => self.extract_color_call(node, "text"),
                _ => {}
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
    
    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        // Handle direct class() calls
        if self.is_class_builder_call(node) {
            self.extract_direct_class_call(node);
        }
        syn::visit::visit_expr_call(self, node);
    }
}
```

### **Class Pattern Recognition**

```rust
impl ClassVisitor {
    fn extract_class_call(&mut self, node: &ExprMethodCall) {
        if let Some(expr) = node.args.first() {
            if let Ok(lit_str) = self.extract_string_literal(expr) {
                self.classes.insert(lit_str);
            }
        }
    }
    
    fn extract_spacing_call(&mut self, node: &ExprMethodCall, prefix: &str) {
        if let Some(expr) = node.args.first() {
            if let Ok(spacing_value) = self.extract_spacing_value(expr) {
                let class = format!("{}-{}", prefix, spacing_value);
                self.classes.insert(class);
            }
        }
    }
    
    fn extract_color_call(&mut self, node: &ExprMethodCall, prefix: &str) {
        if let Some(expr) = node.args.first() {
            if let Ok(color_value) = self.extract_color_value(expr) {
                let class = format!("{}-{}", prefix, color_value);
                self.classes.insert(class);
            }
        }
    }
}
```

## 🧪 **Testing Strategy**

### **Unit Tests**
- AST parsing accuracy
- Class extraction patterns
- Error handling for malformed code
- Performance benchmarks

### **Integration Tests**
- Full file scanning pipeline
- Dependency graph building
- Tree-shaking validation

### **Property-Based Tests**
- Random Rust code generation
- Class extraction validation
- Edge case handling

## 📊 **Performance Considerations**

### **Optimization Strategies**
- **Incremental parsing**: Only parse changed files
- **Parallel processing**: Scan multiple files concurrently
- **Caching**: Cache parsed AST results
- **Lazy evaluation**: Parse only when needed

### **Memory Management**
- **Streaming parser**: Process large files in chunks
- **AST cleanup**: Free unused AST nodes
- **String interning**: Reduce memory for repeated strings

## 🔄 **Integration Points**

### **Build System Integration**
```rust
impl TailwindBuilder {
    pub fn scan_source(self, path: &Path) -> Self {
        let mut scanner = ClassScanner::new();
        scanner.scan_directory(path).unwrap();
        
        self.classes = scanner.get_all_classes();
        self
    }
}
```

### **Tree-Shaking Integration**
```rust
impl TreeShaker {
    pub fn analyze_dependencies(&mut self, classes: &HashSet<String>) {
        let mut analyzer = DependencyAnalyzer::new();
        
        for class in classes {
            analyzer.add_class(class);
        }
        
        self.unused_classes = analyzer.find_unused_classes();
    }
}
```

## 🚀 **Implementation Phases**

### **Phase 1: Basic Parsing**
- Core AST parsing
- Basic class extraction
- File I/O operations

### **Phase 2: Advanced Features**
- Responsive variant detection
- Conditional class handling
- Macro expansion support

### **Phase 3: Optimization**
- Performance optimization
- Memory management
- Caching strategies

## 📈 **Success Metrics**

| Metric | Target | Measurement |
|--------|--------|-------------|
| Parsing Speed | <5s for 1000 files | Benchmark tests |
| Extraction Accuracy | 99.9% | Validation tests |
| Memory Usage | <100MB | Memory profiling |
| Test Coverage | >95% | Code coverage reports |

## 🔍 **Risk Mitigation**

### **Technical Risks**
- **AST parsing complexity**: Use proven `syn` crate
- **Performance**: Continuous benchmarking
- **Memory usage**: Memory profiling and optimization

### **Implementation Risks**
- **Scope creep**: Strict adherence to <300 line limit
- **Integration issues**: Early integration testing
- **Breaking changes**: Semantic versioning

## 🎯 **Example Usage**

### **Input Rust Code**
```rust
use tailwind_rs_core::ClassBuilder;

fn create_button() -> String {
    ClassBuilder::new()
        .class("px-4")
        .class("py-2")
        .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
        .text_color(Color::new(ColorPalette::White, ColorShade::Shade100))
        .responsive(Breakpoint::Md, |builder| {
            builder.class("px-6")
        })
        .to_string()
}
```

### **Extracted Classes**
```rust
HashSet::from([
    "px-4",
    "py-2", 
    "bg-blue-500",
    "text-white",
    "md:px-6"
])
```

---

**Next Steps**: Implement basic AST parsing module with core class extraction functionality.
