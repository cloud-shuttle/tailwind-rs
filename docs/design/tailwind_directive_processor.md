# @tailwind Directive Processing System Design

## Overview
This document outlines the design for processing Tailwind CSS directives (`@tailwind base`, `@tailwind components`, `@tailwind utilities`) in our PostCSS implementation.

## Problem Statement
Our current PostCSS implementation doesn't understand Tailwind's core directives, which are essential for Tailwind CSS functionality.

## Solution Architecture

### Core Components

#### 1. TailwindProcessor
```rust
// File: crates/tailwind-rs-postcss/src/tailwind_processor.rs
pub struct TailwindProcessor {
    base_styles: String,
    component_styles: String,
    utility_styles: String,
    config: TailwindConfig,
    cache: ProcessorCache,
}

impl TailwindProcessor {
    /// Process @tailwind directives in CSS
    pub fn process_directives(&self, css: &str) -> Result<String> {
        // 1. Parse @tailwind directives
        // 2. Replace with actual CSS
        // 3. Handle @apply directives
        // 4. Process @layer directives
    }
    
    /// Process @apply directive
    pub fn process_apply(&self, css: &str) -> Result<String> {
        // Convert @apply classes to actual CSS
    }
    
    /// Process @layer directive
    pub fn process_layer(&self, css: &str) -> Result<String> {
        // Handle @layer base, components, utilities
    }
}
```

#### 2. Directive Parser
```rust
pub struct DirectiveParser {
    patterns: Vec<DirectivePattern>,
    config: ParserConfig,
}

impl DirectiveParser {
    /// Parse @tailwind directives
    pub fn parse_tailwind_directives(&self, css: &str) -> Vec<TailwindDirective> {
        // Parse @tailwind base, components, utilities
    }
    
    /// Parse @apply directives
    pub fn parse_apply_directives(&self, css: &str) -> Vec<ApplyDirective> {
        // Parse @apply class-name;
    }
    
    /// Parse @layer directives
    pub fn parse_layer_directives(&self, css: &str) -> Vec<LayerDirective> {
        // Parse @layer base, components, utilities
    }
}
```

#### 3. CSS Injector
```rust
pub struct CSSInjector {
    base_css: String,
    components_css: String,
    utilities_css: String,
    config: InjectionConfig,
}

impl CSSInjector {
    /// Inject base styles
    pub fn inject_base(&self, css: &str) -> Result<String> {
        // Replace @tailwind base; with actual base CSS
    }
    
    /// Inject component styles
    pub fn inject_components(&self, css: &str) -> Result<String> {
        // Replace @tailwind components; with component CSS
    }
    
    /// Inject utility styles
    pub fn inject_utilities(&self, css: &str) -> Result<String> {
        // Replace @tailwind utilities; with utility CSS
    }
}
```

### Data Structures

#### TailwindDirective
```rust
#[derive(Debug, Clone)]
pub enum TailwindDirective {
    Base,
    Components,
    Utilities,
}

#[derive(Debug, Clone)]
pub struct ApplyDirective {
    pub classes: Vec<String>,
    pub selector: String,
}

#[derive(Debug, Clone)]
pub enum LayerDirective {
    Base,
    Components,
    Utilities,
    Custom(String),
}
```

### Processing Pipeline

#### 1. Directive Detection
```rust
impl TailwindProcessor {
    fn detect_directives(&self, css: &str) -> Vec<DirectiveMatch> {
        let patterns = vec![
            r"@tailwind\s+base;",
            r"@tailwind\s+components;",
            r"@tailwind\s+utilities;",
            r"@apply\s+([^;]+);",
            r"@layer\s+([^;]+);",
        ];
        
        // Use regex to find all directive matches
    }
}
```

#### 2. CSS Generation
```rust
impl TailwindProcessor {
    fn generate_css_for_directive(&self, directive: &TailwindDirective) -> String {
        match directive {
            TailwindDirective::Base => self.generate_base_css(),
            TailwindDirective::Components => self.generate_components_css(),
            TailwindDirective::Utilities => self.generate_utilities_css(),
        }
    }
}
```

#### 3. @apply Processing
```rust
impl TailwindProcessor {
    fn process_apply_directive(&self, apply: &ApplyDirective) -> Result<String> {
        let mut css = String::new();
        
        for class in &apply.classes {
            let class_css = self.resolve_class_to_css(class)?;
            css.push_str(&format!("{} {{ {} }}\n", apply.selector, class_css));
        }
        
        Ok(css)
    }
}
```

### Configuration

#### TailwindConfig
```rust
#[derive(Debug, Clone)]
pub struct TailwindConfig {
    pub base_styles: bool,
    pub component_styles: bool,
    pub utility_styles: bool,
    pub apply_processing: bool,
    pub layer_processing: bool,
    pub custom_directives: Vec<CustomDirective>,
}
```

### Error Handling

#### TailwindProcessorError
```rust
#[derive(Debug, thiserror::Error)]
pub enum TailwindProcessorError {
    #[error("Invalid @tailwind directive: {directive}")]
    InvalidDirective { directive: String },
    
    #[error("Unknown class in @apply: {class}")]
    UnknownClass { class: String },
    
    #[error("Circular @apply dependency: {class}")]
    CircularDependency { class: String },
    
    #[error("Invalid @layer directive: {layer}")]
    InvalidLayer { layer: String },
}
```

### Performance Considerations

#### Caching Strategy
```rust
pub struct ProcessorCache {
    directive_cache: HashMap<String, String>,
    apply_cache: HashMap<String, String>,
    layer_cache: HashMap<String, String>,
}

impl ProcessorCache {
    pub fn get_cached_directive(&self, directive: &str) -> Option<&String> {
        self.directive_cache.get(directive)
    }
    
    pub fn cache_directive(&mut self, directive: String, css: String) {
        self.directive_cache.insert(directive, css);
    }
}
```

### Testing Strategy

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tailwind_directive_processing() {
        let processor = TailwindProcessor::new();
        let css = "@tailwind base; @tailwind components; @tailwind utilities;";
        let result = processor.process_directives(css);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_apply_directive_processing() {
        let processor = TailwindProcessor::new();
        let css = ".btn { @apply bg-blue-500 text-white px-4 py-2; }";
        let result = processor.process_apply(css);
        assert!(result.is_ok());
    }
}
```

#### Integration Tests
```rust
#[test]
fn test_full_tailwind_processing() {
    let processor = TailwindProcessor::new();
    let input = r#"
        @tailwind base;
        @tailwind components;
        @tailwind utilities;
        
        .btn {
            @apply bg-blue-500 text-white px-4 py-2 rounded;
        }
    "#;
    
    let result = processor.process_directives(input);
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert!(output.contains("html"));
    assert!(output.contains("body"));
    assert!(output.contains(".btn"));
}
```

### Implementation Timeline

#### Week 1: Core Infrastructure
- [ ] Create TailwindProcessor struct
- [ ] Implement directive detection
- [ ] Basic directive parsing

#### Week 2: CSS Generation
- [ ] Implement base CSS generation
- [ ] Implement components CSS generation
- [ ] Implement utilities CSS generation

#### Week 3: @apply Processing
- [ ] Implement @apply directive parsing
- [ ] Implement class resolution
- [ ] Handle @apply edge cases

#### Week 4: Testing & Optimization
- [ ] Comprehensive unit tests
- [ ] Integration tests
- [ ] Performance optimization
- [ ] Documentation

### Dependencies

#### New Dependencies
```toml
# Cargo.toml additions
regex = "1.0"
lazy_static = "1.0"
```

#### Existing Dependencies
- `serde` for serialization
- `thiserror` for error handling
- `tokio` for async processing

### API Design

#### Public API
```rust
// Main entry point
pub fn process_tailwind_css(css: &str, config: &TailwindConfig) -> Result<String> {
    let processor = TailwindProcessor::new(config);
    processor.process_directives(css)
}

// Advanced usage
pub fn process_tailwind_css_with_cache(
    css: &str, 
    config: &TailwindConfig,
    cache: &mut ProcessorCache
) -> Result<String> {
    let processor = TailwindProcessor::with_cache(config, cache);
    processor.process_directives(css)
}
```

### Future Enhancements

#### Phase 2 Features
- [ ] Custom directive support
- [ ] Plugin integration
- [ ] Advanced caching
- [ ] Performance monitoring

#### Phase 3 Features
- [ ] Hot reloading
- [ ] Incremental processing
- [ ] Advanced optimization
