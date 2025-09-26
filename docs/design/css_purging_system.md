# CSS Purging System Design

## Overview
This document outlines the design for a comprehensive CSS purging system that removes unused CSS classes, essential for Tailwind CSS production builds.

## Problem Statement
Tailwind CSS generates thousands of utility classes, but most projects only use a small subset. Our PostCSS implementation needs to purge unused CSS to optimize bundle size.

## Solution Architecture

### Core Components

#### 1. CSSPurger
```rust
// File: crates/tailwind-rs-postcss/src/purger.rs
pub struct CSSPurger {
    used_classes: HashSet<String>,
    safelist: HashSet<String>,
    config: PurgeConfig,
    scanner: ContentScanner,
    optimizer: CSSOptimizer,
}

impl CSSPurger {
    /// Purge unused CSS from stylesheet
    pub fn purge(&self, css: &str, content: &[String]) -> Result<String> {
        // 1. Scan content for used classes
        // 2. Remove unused CSS rules
        // 3. Optimize remaining CSS
        // 4. Handle safelist exceptions
    }
    
    /// Scan content for class usage
    pub fn scan_content(&self, content: &str) -> HashSet<String> {
        // Extract all Tailwind classes from content
    }
    
    /// Purge with advanced options
    pub fn purge_advanced(&self, css: &str, options: &PurgeOptions) -> Result<PurgeResult> {
        // Advanced purging with detailed results
    }
}
```

#### 2. ContentScanner
```rust
pub struct ContentScanner {
    patterns: Vec<ClassPattern>,
    extractors: Vec<ContentExtractor>,
    config: ScannerConfig,
}

impl ContentScanner {
    /// Scan HTML content for classes
    pub fn scan_html(&self, html: &str) -> HashSet<String> {
        // Extract classes from HTML attributes
    }
    
    /// Scan JavaScript content for classes
    pub fn scan_javascript(&self, js: &str) -> HashSet<String> {
        // Extract classes from JS strings and templates
    }
    
    /// Scan Rust content for classes
    pub fn scan_rust(&self, rust: &str) -> HashSet<String> {
        // Extract classes from Rust string literals
    }
}
```

#### 3. ClassExtractor
```rust
pub struct ClassExtractor {
    patterns: Vec<Regex>,
    config: ExtractorConfig,
}

impl ClassExtractor {
    /// Extract classes from text using patterns
    pub fn extract_classes(&self, text: &str) -> HashSet<String> {
        // Use regex patterns to find class names
    }
    
    /// Extract classes from specific file types
    pub fn extract_from_file(&self, file_path: &str, content: &str) -> HashSet<String> {
        // File-type specific extraction
    }
}
```

### Data Structures

#### PurgeConfig
```rust
#[derive(Debug, Clone)]
pub struct PurgeConfig {
    pub content_paths: Vec<String>,
    pub safelist: Vec<String>,
    pub blocklist: Vec<String>,
    pub default_safelist: bool,
    pub preserve_html_elements: bool,
    pub keyframes: bool,
    pub font_face: bool,
    pub variables: bool,
}

#[derive(Debug, Clone)]
pub struct PurgeOptions {
    pub aggressive: bool,
    pub preserve_comments: bool,
    pub minify: bool,
    pub source_map: bool,
}

#[derive(Debug, Clone)]
pub struct PurgeResult {
    pub purged_css: String,
    pub removed_classes: HashSet<String>,
    pub kept_classes: HashSet<String>,
    pub statistics: PurgeStatistics,
}

#[derive(Debug, Clone)]
pub struct PurgeStatistics {
    pub original_size: usize,
    pub purged_size: usize,
    pub reduction_percentage: f64,
    pub classes_removed: usize,
    pub classes_kept: usize,
}
```

### Processing Pipeline

#### 1. Content Scanning
```rust
impl CSSPurger {
    fn scan_all_content(&self, content_paths: &[String]) -> Result<HashSet<String>> {
        let mut all_classes = HashSet::new();
        
        for path in content_paths {
            let content = self.read_content(path)?;
            let classes = self.scanner.scan_content(&content);
            all_classes.extend(classes);
        }
        
        Ok(all_classes)
    }
}
```

#### 2. Class Pattern Matching
```rust
impl ContentScanner {
    fn extract_classes_with_patterns(&self, content: &str) -> HashSet<String> {
        let mut classes = HashSet::new();
        
        // HTML class attributes
        let html_pattern = Regex::new(r#"class\s*=\s*["']([^"']+)["']"#).unwrap();
        for cap in html_pattern.captures_iter(content) {
            let class_attr = &cap[1];
            classes.extend(self.parse_class_attribute(class_attr));
        }
        
        // JavaScript template literals
        let js_pattern = Regex::new(r#"`([^`]*\$\{[^}]*\}[^`]*)`"#).unwrap();
        for cap in js_pattern.captures_iter(content) {
            let template = &cap[1];
            classes.extend(self.extract_classes_from_template(template));
        }
        
        // Rust string literals
        let rust_pattern = Regex::new(r#""([^"]*)"#).unwrap();
        for cap in rust_pattern.captures_iter(content) {
            let string_literal = &cap[1];
            classes.extend(self.extract_classes_from_string(string_literal));
        }
        
        classes
    }
}
```

#### 3. CSS Rule Filtering
```rust
impl CSSPurger {
    fn filter_css_rules(&self, css: &str, used_classes: &HashSet<String>) -> Result<String> {
        let mut filtered_css = String::new();
        let mut in_rule = false;
        let mut current_rule = String::new();
        
        for line in css.lines() {
            if line.trim().ends_with('{') {
                in_rule = true;
                current_rule = line.to_string();
            } else if in_rule && line.trim() == "}" {
                current_rule.push_str(line);
                
                if self.should_keep_rule(&current_rule, used_classes) {
                    filtered_css.push_str(&current_rule);
                    filtered_css.push('\n');
                }
                
                in_rule = false;
                current_rule.clear();
            } else if in_rule {
                current_rule.push_str(line);
                current_rule.push('\n');
            } else {
                // Handle @media, @keyframes, etc.
                if self.should_keep_at_rule(line, used_classes) {
                    filtered_css.push_str(line);
                    filtered_css.push('\n');
                }
            }
        }
        
        Ok(filtered_css)
    }
}
```

### Advanced Features

#### 1. Safelist Support
```rust
impl CSSPurger {
    fn apply_safelist(&self, used_classes: &mut HashSet<String>) {
        // Add safelist classes
        used_classes.extend(self.config.safelist.iter().cloned());
        
        // Add default safelist if enabled
        if self.config.default_safelist {
            used_classes.extend(self.get_default_safelist());
        }
        
        // Remove blocklist classes
        for blocked_class in &self.config.blocklist {
            used_classes.remove(blocked_class);
        }
    }
    
    fn get_default_safelist(&self) -> HashSet<String> {
        // Common classes that should always be kept
        vec![
            "html", "body", "head", "title",
            "h1", "h2", "h3", "h4", "h5", "h6",
            "p", "div", "span", "a", "img",
            "button", "input", "form", "label",
        ].into_iter().map(|s| s.to_string()).collect()
    }
}
```

#### 2. Dynamic Class Detection
```rust
impl ContentScanner {
    fn detect_dynamic_classes(&self, content: &str) -> HashSet<String> {
        let mut classes = HashSet::new();
        
        // Detect class concatenation patterns
        let concat_pattern = Regex::new(r#"class\s*=\s*["']([^"']*\+[^"']*)["']"#).unwrap();
        for cap in concat_pattern.captures_iter(content) {
            let expression = &cap[1];
            classes.extend(self.parse_class_expression(expression));
        }
        
        // Detect template string patterns
        let template_pattern = Regex::new(r#"`([^`]*\$\{[^}]*\}[^`]*)`"#).unwrap();
        for cap in template_pattern.captures_iter(content) {
            let template = &cap[1];
            classes.extend(self.extract_classes_from_template(template));
        }
        
        classes
    }
}
```

#### 3. Performance Optimization
```rust
impl CSSPurger {
    fn optimize_scanning(&self, content_paths: &[String]) -> Result<HashSet<String>> {
        let mut all_classes = HashSet::new();
        
        // Parallel scanning for large projects
        let chunks: Vec<Vec<String>> = content_paths
            .chunks(10)
            .map(|chunk| chunk.to_vec())
            .collect();
        
        let handles: Vec<_> = chunks.into_iter().map(|chunk| {
            let scanner = self.scanner.clone();
            std::thread::spawn(move || {
                let mut classes = HashSet::new();
                for path in chunk {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        classes.extend(scanner.scan_content(&content));
                    }
                }
                classes
            })
        }).collect();
        
        for handle in handles {
            let classes = handle.join().unwrap();
            all_classes.extend(classes);
        }
        
        Ok(all_classes)
    }
}
```

### Error Handling

#### PurgeError
```rust
#[derive(Debug, thiserror::Error)]
pub enum PurgeError {
    #[error("Failed to read content file: {path}")]
    FileReadError { path: String },
    
    #[error("Invalid CSS syntax: {error}")]
    InvalidCSS { error: String },
    
    #[error("Circular dependency in class resolution")]
    CircularDependency,
    
    #[error("Out of memory during purging")]
    OutOfMemory,
}
```

### Testing Strategy

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_purging() {
        let purger = CSSPurger::new(PurgeConfig::default());
        let css = ".unused { color: red; } .used { color: blue; }";
        let content = vec!["<div class=\"used\"></div>".to_string()];
        let result = purger.purge(css, &content);
        assert!(result.is_ok());
        assert!(!result.unwrap().contains("unused"));
    }
    
    #[test]
    fn test_safelist_purging() {
        let mut config = PurgeConfig::default();
        config.safelist = vec!["important".to_string()];
        let purger = CSSPurger::new(config);
        
        let css = ".important { color: red; }";
        let content = vec!["<div></div>".to_string()];
        let result = purger.purge(css, &content);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("important"));
    }
}
```

#### Integration Tests
```rust
#[test]
fn test_complex_purging_scenario() {
    let purger = CSSPurger::new(PurgeConfig::default());
    
    let css = r#"
        .btn { @apply px-4 py-2 rounded; }
        .btn-primary { @apply bg-blue-500 text-white; }
        .btn-secondary { @apply bg-gray-500 text-white; }
        .unused-class { color: red; }
    "#;
    
    let content = vec![
        r#"<button class="btn btn-primary">Click me</button>"#.to_string(),
        r#"const className = `btn ${isPrimary ? 'btn-primary' : 'btn-secondary'}`;"#.to_string(),
    ];
    
    let result = purger.purge(css, &content);
    assert!(result.is_ok());
    
    let purged = result.unwrap();
    assert!(purged.contains("btn"));
    assert!(purged.contains("btn-primary"));
    assert!(!purged.contains("unused-class"));
}
```

### Performance Benchmarks

#### Benchmark Targets
- **Small project** (< 100 files): < 1 second
- **Medium project** (100-1000 files): < 5 seconds  
- **Large project** (1000+ files): < 30 seconds
- **Memory usage**: < 100MB for large projects

#### Optimization Strategies
1. **Parallel scanning** for multiple files
2. **Incremental purging** for development
3. **Caching** for repeated scans
4. **Streaming** for large files

### Implementation Timeline

#### Week 1: Core Infrastructure
- [ ] Create CSSPurger struct
- [ ] Implement basic content scanning
- [ ] Basic CSS rule filtering

#### Week 2: Advanced Scanning
- [ ] Implement ContentScanner
- [ ] Add support for multiple file types
- [ ] Dynamic class detection

#### Week 3: Optimization & Safelist
- [ ] Implement safelist/blocklist
- [ ] Performance optimization
- [ ] Caching system

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
// Simple purging
pub fn purge_css(css: &str, content: &[String]) -> Result<String> {
    let purger = CSSPurger::new(PurgeConfig::default());
    purger.purge(css, content)
}

// Advanced purging with options
pub fn purge_css_advanced(
    css: &str, 
    content: &[String], 
    options: &PurgeOptions
) -> Result<PurgeResult> {
    let purger = CSSPurger::new(PurgeConfig::default());
    purger.purge_advanced(css, options)
}

// Configuration-based purging
pub fn purge_css_with_config(
    css: &str, 
    content: &[String], 
    config: &PurgeConfig
) -> Result<String> {
    let purger = CSSPurger::new(config.clone());
    purger.purge(css, content)
}
```

### Future Enhancements

#### Phase 2 Features
- [ ] Incremental purging
- [ ] Hot reloading support
- [ ] Advanced pattern matching
- [ ] Plugin system integration

#### Phase 3 Features
- [ ] Machine learning-based class prediction
- [ ] Advanced optimization algorithms
- [ ] Real-time purging
- [ ] Cloud-based purging service
