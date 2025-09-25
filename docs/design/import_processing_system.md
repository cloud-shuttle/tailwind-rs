# @import Processing System Design

## Overview
This document outlines the design for a comprehensive @import statement processing system that handles CSS imports, resolves dependencies, and optimizes import usage.

## Problem Statement
Our current PostCSS implementation doesn't handle @import statements, which are essential for modular CSS development and Tailwind CSS integration.

## Solution Architecture

### Core Components

#### 1. ImportProcessor
```rust
// File: crates/tailwind-rs-postcss/src/import_processor.rs
pub struct ImportProcessor {
    resolver: ImportResolver,
    cache: ImportCache,
    config: ImportConfig,
    dependency_graph: DependencyGraph,
}

impl ImportProcessor {
    /// Process @import statements in CSS
    pub fn process_imports(&self, css: &str, base_path: &str) -> Result<String> {
        // 1. Find @import statements
        // 2. Resolve file paths
        // 3. Inline imports
        // 4. Handle circular dependencies
    }
    
    /// Process imports with advanced options
    pub fn process_imports_advanced(&self, css: &str, options: &ImportOptions) -> Result<ImportResult> {
        // Advanced import processing with detailed results
    }
    
    /// Resolve import path
    pub fn resolve_import_path(&self, import_path: &str, base_path: &str) -> Result<String> {
        // Resolve relative and absolute import paths
    }
}
```

#### 2. ImportResolver
```rust
pub struct ImportResolver {
    search_paths: Vec<String>,
    extensions: Vec<String>,
    config: ResolverConfig,
}

impl ImportResolver {
    /// Resolve import path to file path
    pub fn resolve(&self, import_path: &str, base_path: &str) -> Result<String> {
        // Resolve import path to actual file path
    }
    
    /// Check if import exists
    pub fn import_exists(&self, import_path: &str, base_path: &str) -> bool {
        // Check if import file exists
    }
    
    /// Get import content
    pub fn get_import_content(&self, import_path: &str, base_path: &str) -> Result<String> {
        // Read import file content
    }
}
```

#### 3. DependencyGraph
```rust
pub struct DependencyGraph {
    nodes: HashMap<String, ImportNode>,
    edges: HashMap<String, Vec<String>>,
    visited: HashSet<String>,
}

impl DependencyGraph {
    /// Add import dependency
    pub fn add_dependency(&mut self, from: &str, to: &str) -> Result<()> {
        // Add dependency edge to graph
    }
    
    /// Check for circular dependencies
    pub fn has_circular_dependency(&self, start: &str) -> Result<Vec<String>> {
        // Detect circular dependencies
    }
    
    /// Get import order
    pub fn get_import_order(&self, start: &str) -> Result<Vec<String>> {
        // Get correct import order (topological sort)
    }
}
```

### Data Structures

#### ImportConfig
```rust
#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub search_paths: Vec<String>,
    pub extensions: Vec<String>,
    pub inline_imports: bool,
    pub preserve_imports: bool,
    pub optimize_imports: bool,
    pub handle_circular: CircularHandling,
    pub max_depth: usize,
}

#[derive(Debug, Clone)]
pub enum CircularHandling {
    Error,
    Warn,
    Ignore,
}

#[derive(Debug, Clone)]
pub struct ImportOptions {
    pub search_paths: Vec<String>,
    pub extensions: Vec<String>,
    pub inline_imports: bool,
    pub preserve_imports: bool,
    pub optimize_imports: bool,
    pub handle_circular: CircularHandling,
    pub max_depth: usize,
    pub source_map: bool,
}

#[derive(Debug, Clone)]
pub struct ImportResult {
    pub processed_css: String,
    pub imports_processed: Vec<ImportInfo>,
    pub dependencies: Vec<String>,
    pub circular_dependencies: Vec<String>,
    pub statistics: ImportStatistics,
}

#[derive(Debug, Clone)]
pub struct ImportInfo {
    pub original_path: String,
    pub resolved_path: String,
    pub content: String,
    pub size: usize,
    pub processed: bool,
}

#[derive(Debug, Clone)]
pub struct ImportStatistics {
    pub total_imports: usize,
    pub processed_imports: usize,
    pub skipped_imports: usize,
    pub total_size: usize,
    pub processing_time: Duration,
}
```

### Processing Pipeline

#### 1. Import Detection
```rust
impl ImportProcessor {
    fn detect_imports(&self, css: &str) -> Vec<ImportStatement> {
        let mut imports = Vec::new();
        let import_pattern = Regex::new(r#"@import\s+(?:url\()?["']?([^"')]+)["']?\)?;"#).unwrap();
        
        for (line_num, line) in css.lines().enumerate() {
            if let Some(cap) = import_pattern.captures(line) {
                let import_path = &cap[1];
                imports.push(ImportStatement {
                    line_number: line_num + 1,
                    import_path: import_path.to_string(),
                    media_query: self.extract_media_query(line),
                });
            }
        }
        
        imports
    }
}
```

#### 2. Path Resolution
```rust
impl ImportResolver {
    fn resolve_import_path(&self, import_path: &str, base_path: &str) -> Result<String> {
        // Handle different import path types
        if import_path.starts_with("http://") || import_path.starts_with("https://") {
            return Ok(import_path.to_string()); // External URL
        }
        
        if import_path.starts_with("//") {
            return Ok(format!("https:{}", import_path)); // Protocol-relative URL
        }
        
        if import_path.starts_with('/') {
            // Absolute path
            return Ok(import_path.to_string());
        }
        
        // Relative path
        let base_dir = std::path::Path::new(base_path).parent()
            .ok_or(ImportError::InvalidBasePath)?;
        
        let resolved_path = base_dir.join(import_path);
        
        // Try different extensions
        for ext in &self.extensions {
            let path_with_ext = format!("{}{}", resolved_path.display(), ext);
            if std::path::Path::new(&path_with_ext).exists() {
                return Ok(path_with_ext);
            }
        }
        
        // Try without extension
        if std::path::Path::new(&resolved_path).exists() {
            return Ok(resolved_path.to_string_lossy().to_string());
        }
        
        Err(ImportError::ImportNotFound { path: import_path.to_string() })
    }
}
```

#### 3. Import Inlining
```rust
impl ImportProcessor {
    fn inline_import(&self, import: &ImportStatement, base_path: &str) -> Result<String> {
        let resolved_path = self.resolver.resolve(&import.import_path, base_path)?;
        let content = self.resolver.get_import_content(&import.import_path, base_path)?;
        
        // Recursively process imports in the imported file
        let processed_content = if self.config.inline_imports {
            self.process_imports(&content, &resolved_path)?
        } else {
            content
        };
        
        // Wrap in media query if needed
        if let Some(media) = &import.media_query {
            Ok(format!("@media {} {{\n{}\n}}", media, processed_content))
        } else {
            Ok(processed_content)
        }
    }
}
```

### Advanced Features

#### 1. Circular Dependency Detection
```rust
impl DependencyGraph {
    fn detect_circular_dependencies(&self, start: &str) -> Result<Vec<String>> {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        let mut circular_paths = Vec::new();
        
        self.dfs_circular_detection(
            start, 
            &mut visited, 
            &mut recursion_stack, 
            &mut circular_paths
        )?;
        
        Ok(circular_paths)
    }
    
    fn dfs_circular_detection(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
        circular_paths: &mut Vec<String>,
    ) -> Result<()> {
        if recursion_stack.contains(node) {
            return Err(ImportError::CircularDependency { path: node.to_string() });
        }
        
        if visited.contains(node) {
            return Ok(());
        }
        
        visited.insert(node.to_string());
        recursion_stack.insert(node.to_string());
        
        if let Some(dependencies) = self.edges.get(node) {
            for dependency in dependencies {
                self.dfs_circular_detection(dependency, visited, recursion_stack, circular_paths)?;
            }
        }
        
        recursion_stack.remove(node);
        Ok(())
    }
}
```

#### 2. Import Optimization
```rust
impl ImportProcessor {
    fn optimize_imports(&self, css: &str) -> Result<String> {
        let mut optimized_css = String::new();
        let mut import_map: HashMap<String, String> = HashMap::new();
        
        // Collect all imports
        let imports = self.detect_imports(css);
        for import in imports {
            let content = self.inline_import(&import, "")?;
            import_map.insert(import.import_path, content);
        }
        
        // Remove duplicate imports
        let mut seen_imports = HashSet::new();
        for line in css.lines() {
            if line.trim().starts_with("@import") {
                let import_path = self.extract_import_path(line);
                if !seen_imports.contains(&import_path) {
                    seen_imports.insert(import_path);
                    optimized_css.push_str(line);
                    optimized_css.push('\n');
                }
            } else {
                optimized_css.push_str(line);
                optimized_css.push('\n');
            }
        }
        
        Ok(optimized_css)
    }
}
```

#### 3. Source Map Integration
```rust
impl ImportProcessor {
    fn process_imports_with_source_map(&self, css: &str, base_path: &str) -> Result<(String, SourceMap)> {
        let mut processed_css = String::new();
        let mut source_map = SourceMap::new();
        
        for line in css.lines() {
            if line.trim().starts_with("@import") {
                let import_path = self.extract_import_path(line);
                let resolved_path = self.resolver.resolve(&import_path, base_path)?;
                let content = self.resolver.get_import_content(&import_path, base_path)?;
                
                // Add to source map
                source_map.add_source(&resolved_path, &content);
                
                processed_css.push_str(&content);
                processed_css.push('\n');
            } else {
                processed_css.push_str(line);
                processed_css.push('\n');
            }
        }
        
        Ok((processed_css, source_map))
    }
}
```

### Error Handling

#### ImportError
```rust
#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    #[error("Import not found: {path}")]
    ImportNotFound { path: String },
    
    #[error("Circular dependency detected: {path}")]
    CircularDependency { path: String },
    
    #[error("Invalid base path: {path}")]
    InvalidBasePath { path: String },
    
    #[error("Import depth exceeded: {depth}")]
    ImportDepthExceeded { depth: usize },
    
    #[error("Failed to read import file: {path}")]
    FileReadError { path: String },
}
```

### Testing Strategy

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_import_processing() {
        let processor = ImportProcessor::new(ImportConfig::default());
        let css = "@import 'styles.css';";
        let result = processor.process_imports(css, "/path/to/base");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_circular_dependency_detection() {
        let processor = ImportProcessor::new(ImportConfig::default());
        let css = "@import 'circular.css';";
        let result = processor.process_imports(css, "/path/to/base");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ImportError::CircularDependency { .. }));
    }
}
```

#### Integration Tests
```rust
#[test]
fn test_complex_import_scenario() {
    let processor = ImportProcessor::new(ImportConfig::default());
    
    let css = r#"
        @import 'base.css';
        @import 'components.css';
        @import 'utilities.css';
        
        .custom {
            color: red;
        }
    "#;
    
    let result = processor.process_imports(css, "/path/to/base");
    assert!(result.is_ok());
    
    let processed = result.unwrap();
    assert!(processed.contains("html"));
    assert!(processed.contains("body"));
    assert!(processed.contains(".custom"));
}
```

### Performance Optimization

#### 1. Caching Strategy
```rust
pub struct ImportCache {
    file_cache: HashMap<String, String>,
    dependency_cache: HashMap<String, Vec<String>>,
    processed_cache: HashMap<String, String>,
}

impl ImportCache {
    pub fn get_cached_content(&self, path: &str) -> Option<&String> {
        self.file_cache.get(path)
    }
    
    pub fn cache_content(&mut self, path: String, content: String) {
        self.file_cache.insert(path, content);
    }
}
```

#### 2. Parallel Processing
```rust
impl ImportProcessor {
    fn parallel_import_processing(&self, css: &str, base_path: &str) -> Result<String> {
        let imports = self.detect_imports(css);
        let processed_imports: Vec<String> = imports
            .par_iter()
            .map(|import| self.inline_import(import, base_path))
            .collect::<Result<Vec<String>>>()?;
        
        Ok(processed_imports.join("\n"))
    }
}
```

### Implementation Timeline

#### Week 1: Core Infrastructure
- [ ] Create ImportProcessor struct
- [ ] Implement basic import detection
- [ ] Path resolution system

#### Week 2: Advanced Features
- [ ] Circular dependency detection
- [ ] Import optimization
- [ ] Source map integration

#### Week 3: Performance & Caching
- [ ] Implement caching system
- [ ] Parallel processing
- [ ] Memory optimization

#### Week 4: Testing & Documentation
- [ ] Comprehensive test suite
- [ ] Performance benchmarks
- [ ] Documentation and examples

### Dependencies

#### New Dependencies
```toml
# Cargo.toml additions
regex = "1.0"
walkdir = "2.3"
rayon = "1.0"  # For parallel processing
```

### API Design

#### Public API
```rust
// Simple import processing
pub fn process_imports(css: &str, base_path: &str) -> Result<String> {
    let processor = ImportProcessor::new(ImportConfig::default());
    processor.process_imports(css, base_path)
}

// Advanced import processing with options
pub fn process_imports_with_options(
    css: &str, 
    base_path: &str, 
    options: &ImportOptions
) -> Result<ImportResult> {
    let processor = ImportProcessor::new(ImportConfig::from_options(options));
    processor.process_imports_advanced(css, options)
}

// Configuration-based import processing
pub fn process_imports_with_config(
    css: &str, 
    base_path: &str, 
    config: &ImportConfig
) -> Result<String> {
    let processor = ImportProcessor::new(config.clone());
    processor.process_imports(css, base_path)
}
```

### Future Enhancements

#### Phase 2 Features
- [ ] HTTP import support
- [ ] Advanced dependency resolution
- [ ] Import bundling
- [ ] Plugin system integration

#### Phase 3 Features
- [ ] Real-time import watching
- [ ] Advanced optimization algorithms
- [ ] Cloud-based import service
- [ ] Machine learning-based import optimization
