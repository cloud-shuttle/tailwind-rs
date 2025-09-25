# Enhanced Autoprefixer Integration Design

## Overview
This document outlines the design for an enhanced autoprefixer system that adds vendor prefixes to CSS properties based on browser compatibility data.

## Problem Statement
Our current PostCSS implementation has basic vendor prefixing support but lacks comprehensive browser compatibility data and intelligent prefixing logic.

## Solution Architecture

### Core Components

#### 1. Autoprefixer
```rust
// File: crates/tailwind-rs-postcss/src/autoprefixer.rs
pub struct Autoprefixer {
    browser_data: BrowserData,
    caniuse_data: CanIUseData,
    config: AutoprefixerConfig,
    cache: PrefixCache,
}

impl Autoprefixer {
    /// Add vendor prefixes based on browser support
    pub fn add_prefixes(&self, css: &str, browsers: &[String]) -> Result<String> {
        // 1. Parse CSS properties
        // 2. Check browser support
        // 3. Add necessary prefixes
        // 4. Optimize prefix usage
    }
    
    /// Add prefixes with advanced options
    pub fn add_prefixes_advanced(&self, css: &str, options: &PrefixOptions) -> Result<PrefixResult> {
        // Advanced prefixing with detailed results
    }
    
    /// Check if property needs prefixes
    pub fn needs_prefix(&self, property: &str, browsers: &[String]) -> bool {
        // Check if property needs vendor prefixes for given browsers
    }
}
```

#### 2. BrowserData
```rust
pub struct BrowserData {
    browsers: HashMap<String, BrowserInfo>,
    features: HashMap<String, FeatureSupport>,
    versions: HashMap<String, Vec<String>>,
}

impl BrowserData {
    /// Get browser support for a feature
    pub fn get_feature_support(&self, feature: &str, browser: &str) -> Option<SupportLevel> {
        // Get support level for specific feature and browser
    }
    
    /// Check if browser supports feature
    pub fn supports_feature(&self, feature: &str, browser: &str, version: &str) -> bool {
        // Check if specific browser version supports feature
    }
}
```

#### 3. PrefixGenerator
```rust
pub struct PrefixGenerator {
    prefixes: HashMap<String, Vec<String>>,
    config: GeneratorConfig,
}

impl PrefixGenerator {
    /// Generate vendor prefixes for property
    pub fn generate_prefixes(&self, property: &str, browsers: &[String]) -> Vec<String> {
        // Generate appropriate vendor prefixes
    }
    
    /// Optimize prefix usage
    pub fn optimize_prefixes(&self, prefixes: &[String]) -> Vec<String> {
        // Remove redundant prefixes
    }
}
```

### Data Structures

#### AutoprefixerConfig
```rust
#[derive(Debug, Clone)]
pub struct AutoprefixerConfig {
    pub browsers: Vec<String>,
    pub cascade: bool,
    pub add: bool,
    pub remove: bool,
    pub supports: bool,
    pub flexbox: FlexboxMode,
    pub grid: GridMode,
    pub stats: Option<BrowserStats>,
}

#[derive(Debug, Clone)]
pub enum FlexboxMode {
    No2009,
    No2012,
    All,
}

#[derive(Debug, Clone)]
pub enum GridMode {
    NoAutoplace,
    Autoplace,
}

#[derive(Debug, Clone)]
pub struct PrefixOptions {
    pub browsers: Vec<String>,
    pub cascade: bool,
    pub add: bool,
    pub remove: bool,
    pub supports: bool,
    pub flexbox: FlexboxMode,
    pub grid: GridMode,
    pub stats: Option<BrowserStats>,
}

#[derive(Debug, Clone)]
pub struct PrefixResult {
    pub prefixed_css: String,
    pub prefixes_added: HashMap<String, Vec<String>>,
    pub prefixes_removed: HashMap<String, Vec<String>>,
    pub statistics: PrefixStatistics,
}

#[derive(Debug, Clone)]
pub struct PrefixStatistics {
    pub original_size: usize,
    pub prefixed_size: usize,
    pub prefixes_added: usize,
    pub prefixes_removed: usize,
    pub properties_processed: usize,
}
```

### Browser Support Data

#### BrowserInfo
```rust
#[derive(Debug, Clone)]
pub struct BrowserInfo {
    pub name: String,
    pub versions: Vec<String>,
    pub current_version: String,
    pub support_level: SupportLevel,
}

#[derive(Debug, Clone)]
pub enum SupportLevel {
    Full,
    Partial,
    None,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct FeatureSupport {
    pub feature: String,
    pub browsers: HashMap<String, SupportInfo>,
}

#[derive(Debug, Clone)]
pub struct SupportInfo {
    pub version: String,
    pub support: SupportLevel,
    pub prefix: Option<String>,
    pub notes: Option<String>,
}
```

### Processing Pipeline

#### 1. CSS Property Analysis
```rust
impl Autoprefixer {
    fn analyze_css_properties(&self, css: &str) -> Vec<CSSProperty> {
        let mut properties = Vec::new();
        let property_pattern = Regex::new(r"([a-zA-Z-]+)\s*:\s*([^;]+);").unwrap();
        
        for cap in property_pattern.captures_iter(css) {
            let property_name = &cap[1];
            let property_value = &cap[2];
            
            properties.push(CSSProperty {
                name: property_name.to_string(),
                value: property_value.to_string(),
                needs_prefix: self.needs_prefix(property_name, &self.config.browsers),
            });
        }
        
        properties
    }
}
```

#### 2. Prefix Generation
```rust
impl Autoprefixer {
    fn generate_prefixes_for_property(&self, property: &CSSProperty) -> Vec<PrefixedProperty> {
        let mut prefixed_properties = Vec::new();
        
        if property.needs_prefix {
            let prefixes = self.get_prefixes_for_property(&property.name);
            
            for prefix in prefixes {
                let prefixed_name = format!("{}{}", prefix, property.name);
                prefixed_properties.push(PrefixedProperty {
                    name: prefixed_name,
                    value: property.value.clone(),
                    prefix: prefix.clone(),
                });
            }
        }
        
        // Add original property
        prefixed_properties.push(PrefixedProperty {
            name: property.name.clone(),
            value: property.value.clone(),
            prefix: String::new(),
        });
        
        prefixed_properties
    }
}
```

#### 3. Browser Compatibility Check
```rust
impl Autoprefixer {
    fn check_browser_compatibility(&self, property: &str, browsers: &[String]) -> bool {
        for browser in browsers {
            let support = self.browser_data.get_feature_support(property, browser);
            match support {
                Some(SupportLevel::Full) => continue,
                Some(SupportLevel::Partial) => return true, // Needs prefix
                Some(SupportLevel::None) => return true, // Needs prefix
                Some(SupportLevel::Unknown) => return true, // Assume needs prefix
                None => return true, // Unknown feature, assume needs prefix
            }
        }
        false
    }
}
```

### Advanced Features

#### 1. Intelligent Prefixing
```rust
impl Autoprefixer {
    fn intelligent_prefixing(&self, css: &str) -> Result<String> {
        let mut result = String::new();
        let mut in_rule = false;
        let mut current_rule = String::new();
        
        for line in css.lines() {
            if line.trim().ends_with('{') {
                in_rule = true;
                current_rule = line.to_string();
            } else if in_rule && line.trim() == "}" {
                current_rule.push_str(line);
                
                let prefixed_rule = self.prefix_rule(&current_rule)?;
                result.push_str(&prefixed_rule);
                result.push('\n');
                
                in_rule = false;
                current_rule.clear();
            } else if in_rule {
                current_rule.push_str(line);
                current_rule.push('\n');
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }
        
        Ok(result)
    }
}
```

#### 2. Prefix Optimization
```rust
impl Autoprefixer {
    fn optimize_prefixes(&self, css: &str) -> Result<String> {
        let mut optimized_css = String::new();
        let mut prefix_counts: HashMap<String, usize> = HashMap::new();
        
        // Count prefix usage
        for line in css.lines() {
            if line.contains('-webkit-') || line.contains('-moz-') || line.contains('-ms-') {
                let prefix = self.extract_prefix(line);
                *prefix_counts.entry(prefix).or_insert(0) += 1;
            }
        }
        
        // Remove redundant prefixes
        for line in css.lines() {
            let optimized_line = self.optimize_line(line, &prefix_counts);
            optimized_css.push_str(&optimized_line);
            optimized_css.push('\n');
        }
        
        Ok(optimized_css)
    }
}
```

#### 3. Flexbox and Grid Support
```rust
impl Autoprefixer {
    fn handle_flexbox(&self, css: &str) -> Result<String> {
        let mut result = css.to_string();
        
        match self.config.flexbox {
            FlexboxMode::No2009 => {
                // Remove old flexbox prefixes
                result = result.replace("-webkit-box-", "");
                result = result.replace("-moz-box-", "");
            },
            FlexboxMode::No2012 => {
                // Remove 2012 flexbox prefixes
                result = result.replace("-webkit-flex-", "");
                result = result.replace("-moz-flex-", "");
            },
            FlexboxMode::All => {
                // Keep all flexbox prefixes
            },
        }
        
        Ok(result)
    }
    
    fn handle_grid(&self, css: &str) -> Result<String> {
        let mut result = css.to_string();
        
        match self.config.grid {
            GridMode::NoAutoplace => {
                // Remove grid autoplace prefixes
                result = result.replace("-ms-grid-", "");
            },
            GridMode::Autoplace => {
                // Keep grid autoplace prefixes
            },
        }
        
        Ok(result)
    }
}
```

### Browser Data Integration

#### 1. Can I Use Data
```rust
impl Autoprefixer {
    fn load_caniuse_data(&self) -> Result<CanIUseData> {
        // Load Can I Use database
        let data = include_str!("../data/caniuse.json");
        let caniuse: CanIUseData = serde_json::from_str(data)?;
        Ok(caniuse)
    }
    
    fn get_browser_support(&self, feature: &str, browser: &str) -> Option<SupportLevel> {
        self.caniuse_data.get_support(feature, browser)
    }
}
```

#### 2. Custom Browser Stats
```rust
impl Autoprefixer {
    fn load_custom_stats(&self, stats: &BrowserStats) -> Result<()> {
        // Load custom browser statistics
        for (browser, usage) in &stats.usage {
            self.browser_data.update_browser_usage(browser, *usage);
        }
        Ok(())
    }
}
```

### Error Handling

#### AutoprefixerError
```rust
#[derive(Debug, thiserror::Error)]
pub enum AutoprefixerError {
    #[error("Invalid CSS syntax: {error}")]
    InvalidCSS { error: String },
    
    #[error("Unsupported browser: {browser}")]
    UnsupportedBrowser { browser: String },
    
    #[error("Failed to load browser data: {error}")]
    BrowserDataError { error: String },
    
    #[error("Prefix generation failed: {property}")]
    PrefixGenerationError { property: String },
}
```

### Testing Strategy

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_prefixing() {
        let autoprefixer = Autoprefixer::new(AutoprefixerConfig::default());
        let css = ".test { display: flex; }";
        let result = autoprefixer.add_prefixes(css, &["chrome 30".to_string()]);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("-webkit-"));
    }
    
    #[test]
    fn test_flexbox_prefixing() {
        let mut config = AutoprefixerConfig::default();
        config.flexbox = FlexboxMode::All;
        let autoprefixer = Autoprefixer::new(config);
        
        let css = ".container { display: flex; }";
        let result = autoprefixer.add_prefixes(css, &["ie 10".to_string()]);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("-ms-"));
    }
}
```

#### Integration Tests
```rust
#[test]
fn test_complex_prefixing_scenario() {
    let autoprefixer = Autoprefixer::new(AutoprefixerConfig::default());
    
    let css = r#"
        .container {
            display: flex;
            flex-direction: column;
            align-items: center;
        }
        
        .item {
            flex: 1;
            transform: translateX(10px);
        }
    "#;
    
    let browsers = vec![
        "chrome 30".to_string(),
        "firefox 25".to_string(),
        "safari 7".to_string(),
    ];
    
    let result = autoprefixer.add_prefixes(css, &browsers);
    assert!(result.is_ok());
    
    let prefixed = result.unwrap();
    assert!(prefixed.contains("-webkit-"));
    assert!(prefixed.contains("-moz-"));
    assert!(prefixed.contains("-ms-"));
}
```

### Performance Optimization

#### 1. Caching Strategy
```rust
pub struct PrefixCache {
    property_cache: HashMap<String, Vec<String>>,
    browser_cache: HashMap<String, SupportLevel>,
    css_cache: HashMap<String, String>,
}

impl PrefixCache {
    pub fn get_cached_prefixes(&self, property: &str) -> Option<&Vec<String>> {
        self.property_cache.get(property)
    }
    
    pub fn cache_prefixes(&mut self, property: String, prefixes: Vec<String>) {
        self.property_cache.insert(property, prefixes);
    }
}
```

#### 2. Parallel Processing
```rust
impl Autoprefixer {
    fn parallel_prefixing(&self, css: &str) -> Result<String> {
        let rules = self.split_css_into_rules(css);
        let prefixed_rules: Vec<String> = rules
            .par_iter()
            .map(|rule| self.prefix_rule(rule))
            .collect::<Result<Vec<String>>>()?;
        
        Ok(prefixed_rules.join("\n"))
    }
}
```

### Implementation Timeline

#### Week 1: Core Infrastructure
- [ ] Create Autoprefixer struct
- [ ] Implement browser data loading
- [ ] Basic prefix generation

#### Week 2: Advanced Features
- [ ] Implement intelligent prefixing
- [ ] Add flexbox and grid support
- [ ] Prefix optimization

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
serde_json = "1.0"
rayon = "1.0"  # For parallel processing
lazy_static = "1.0"  # For static data loading
```

### API Design

#### Public API
```rust
// Simple prefixing
pub fn add_prefixes(css: &str, browsers: &[String]) -> Result<String> {
    let autoprefixer = Autoprefixer::new(AutoprefixerConfig::default());
    autoprefixer.add_prefixes(css, browsers)
}

// Advanced prefixing with options
pub fn add_prefixes_with_options(
    css: &str, 
    browsers: &[String], 
    options: &PrefixOptions
) -> Result<PrefixResult> {
    let autoprefixer = Autoprefixer::new(AutoprefixerConfig::from_options(options));
    autoprefixer.add_prefixes_advanced(css, options)
}

// Configuration-based prefixing
pub fn add_prefixes_with_config(
    css: &str, 
    config: &AutoprefixerConfig
) -> Result<String> {
    let autoprefixer = Autoprefixer::new(config.clone());
    autoprefixer.add_prefixes(css, &config.browsers)
}
```

### Future Enhancements

#### Phase 2 Features
- [ ] Custom prefix patterns
- [ ] Advanced browser detection
- [ ] Real-time prefix updates
- [ ] Plugin system integration

#### Phase 3 Features
- [ ] Machine learning-based prefixing
- [ ] Advanced optimization algorithms
- [ ] Cloud-based prefix service
- [ ] Real-time browser support updates
