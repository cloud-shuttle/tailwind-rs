# 🔨 Build System Design

> **Purpose**: Design the production build system for Tailwind-RS  
> **File Size**: <300 lines per module  
> **Priority**: High (Phase 2)

## 📋 **Overview**

The build system orchestrates the entire CSS generation pipeline, from source scanning to final CSS output. It replaces the current stubbed implementation with a fully functional system.

## 🏗️ **Architecture**

### **Core Components**

1. **Build System** (`build_system.rs`)
   - Orchestrates the build pipeline
   - Manages build configuration
   - Handles file I/O operations

2. **Tree Shaker** (`tree_shaker.rs`)
   - Removes unused CSS classes
   - Optimizes bundle size
   - Handles dependency analysis

3. **Build Pipeline** (`build_pipeline.rs`)
   - Manages build steps
   - Handles error recovery
   - Provides progress reporting

## 🎯 **Design Requirements**

### **Functional Requirements**
- Scan source files for class usage
- Generate CSS from used classes
- Remove unused CSS (tree-shaking)
- Minify output CSS
- Generate source maps
- Support watch mode

### **Non-Functional Requirements**
- **Performance**: Build in <10 seconds for typical projects
- **Memory**: <200MB peak memory usage
- **Reliability**: Handle build failures gracefully
- **Incremental**: Support incremental builds

## 🔧 **Implementation Design**

### **Build System Module**

```rust
// build_system.rs (<300 lines)
pub struct BuildSystem {
    config: BuildConfig,
    scanner: ClassScanner,
    generator: CssGenerator,
    optimizer: CssOptimizer,
    tree_shaker: TreeShaker,
}

impl BuildSystem {
    pub fn new(config: BuildConfig) -> Self;
    pub fn build(&mut self) -> Result<BuildResult>;
    pub fn watch(&mut self) -> Result<()>;
    pub fn clean(&self) -> Result<()>;
    pub fn get_stats(&self) -> BuildStats;
}
```

### **Build Pipeline**

```rust
impl BuildSystem {
    pub fn build(&mut self) -> Result<BuildResult> {
        let start_time = std::time::Instant::now();
        
        // Step 1: Scan source files
        let classes = self.scan_classes()?;
        
        // Step 2: Generate CSS
        let css = self.generate_css(&classes)?;
        
        // Step 3: Tree-shake unused classes
        let optimized_css = self.tree_shake(css, &classes)?;
        
        // Step 4: Minify CSS
        let minified_css = self.minify_css(optimized_css)?;
        
        // Step 5: Write output files
        self.write_output_files(minified_css)?;
        
        Ok(BuildResult {
            duration: start_time.elapsed(),
            classes_found: classes.len(),
            css_size: minified_css.len(),
            unused_classes: self.get_unused_classes(),
        })
    }
}
```

### **Tree Shaking Implementation**

```rust
// tree_shaker.rs (<300 lines)
pub struct TreeShaker {
    used_classes: HashSet<String>,
    all_classes: HashSet<String>,
    dependency_graph: HashMap<String, Vec<String>>,
}

impl TreeShaker {
    pub fn new() -> Self;
    pub fn add_used_class(&mut self, class: &str);
    pub fn add_dependency(&mut self, class: &str, dependency: &str);
    pub fn shake(&mut self, css: &str) -> Result<String>;
    pub fn get_unused_classes(&self) -> HashSet<String>;
}
```

### **Watch Mode Implementation**

```rust
impl BuildSystem {
    pub fn watch(&mut self) -> Result<()> {
        use notify::{Watcher, RecursiveMode, Event, EventKind};
        
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::watcher(tx, Duration::from_secs(1))?;
        
        // Watch source directories
        for input_path in &self.config.input {
            watcher.watch(input_path, RecursiveMode::Recursive)?;
        }
        
        // Process file changes
        loop {
            match rx.recv() {
                Ok(Event { kind: EventKind::Modify(_), paths, .. }) => {
                    if self.should_rebuild(&paths) {
                        println!("Files changed, rebuilding...");
                        self.build()?;
                    }
                }
                Err(e) => return Err(TailwindError::WatchError(e.to_string())),
                _ => {}
            }
        }
    }
}
```

## 🧪 **Testing Strategy**

### **Unit Tests**
- Build pipeline steps
- Tree-shaking logic
- File I/O operations
- Error handling

### **Integration Tests**
- Full build pipeline
- Watch mode functionality
- Configuration handling
- Performance benchmarks

### **Property-Based Tests**
- Random class combinations
- Build configuration validation
- Edge case handling

## 📊 **Performance Considerations**

### **Optimization Strategies**
- **Parallel processing**: Scan files concurrently
- **Incremental builds**: Only rebuild changed files
- **Caching**: Cache build artifacts
- **Lazy evaluation**: Generate CSS only when needed

### **Memory Management**
- **Streaming**: Process large files in chunks
- **Cleanup**: Free unused resources
- **Pooling**: Reuse objects where possible

## 🔄 **Integration Points**

### **TailwindBuilder Integration**
```rust
impl TailwindBuilder {
    pub fn build(self) -> Result<()> {
        let config = BuildConfig {
            input: self.input_paths,
            output: self.output_path,
            watch: self.watch_mode,
            minify: self.minify,
            source_maps: self.source_maps,
            purge: self.purge,
            additional_css: self.additional_css,
            postcss_plugins: self.postcss_plugins,
        };
        
        let mut build_system = BuildSystem::new(config);
        let result = build_system.build()?;
        
        println!("Build completed in {:?}", result.duration);
        println!("Generated {} classes, {} bytes CSS", 
                 result.classes_found, result.css_size);
        
        Ok(())
    }
}
```

### **CLI Integration**
```rust
// CLI command implementation
pub fn build_command(config_path: Option<PathBuf>) -> Result<()> {
    let config = if let Some(path) = config_path {
        TailwindConfig::from_file(path)?
    } else {
        TailwindConfig::default()
    };
    
    let mut build_system = BuildSystem::new(config.build);
    build_system.build()?;
    
    Ok(())
}
```

## 🚀 **Implementation Phases**

### **Phase 1: Basic Build System**
- Core build pipeline
- Basic file I/O
- Simple CSS generation

### **Phase 2: Advanced Features**
- Tree-shaking
- Minification
- Source maps

### **Phase 3: Optimization**
- Watch mode
- Incremental builds
- Performance optimization

## 📈 **Success Metrics**

| Metric | Target | Measurement |
|--------|--------|-------------|
| Build Speed | <10s for typical project | Benchmark tests |
| Memory Usage | <200MB | Memory profiling |
| CSS Size Reduction | >50% with tree-shaking | Size comparison |
| Test Coverage | >95% | Code coverage reports |

## 🔍 **Risk Mitigation**

### **Technical Risks**
- **Build complexity**: Modular design with clear interfaces
- **Performance**: Continuous benchmarking
- **Memory usage**: Memory profiling and optimization

### **Implementation Risks**
- **Scope creep**: Strict adherence to <300 line limit
- **Integration issues**: Early integration testing
- **Breaking changes**: Semantic versioning

## 🎯 **Example Usage**

### **Basic Build**
```rust
let config = BuildConfig {
    input: vec!["src/**/*.rs".to_string()],
    output: "dist/styles.css".to_string(),
    watch: false,
    minify: true,
    source_maps: true,
    purge: true,
    additional_css: vec![],
    postcss_plugins: vec![],
};

let mut build_system = BuildSystem::new(config);
let result = build_system.build()?;

println!("Build completed: {} classes, {} bytes", 
         result.classes_found, result.css_size);
```

### **Watch Mode**
```rust
let mut build_system = BuildSystem::new(config);
build_system.watch()?; // Runs indefinitely, rebuilding on changes
```

---

**Next Steps**: Implement basic build system module with core pipeline functionality.
