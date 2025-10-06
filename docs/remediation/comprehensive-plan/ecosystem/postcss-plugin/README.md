# 📦 PostCSS Plugin Architecture Design

## 📋 Overview

**Current Status**: ❌ Not Implemented (0% Complete)  
**Priority**: Critical  
**Timeline**: 4-5 weeks  
**Dependencies**: CLI tool, WASM build, Node.js bindings

## 🎯 Mission

Create a PostCSS plugin that provides drop-in compatibility with existing Tailwind CSS setups, allowing developers to replace `@tailwindcss/postcss` with `@tailwind-rs/postcss` without changing their build configuration.

## 📊 Current State Analysis

### ✅ **What's Working**
- Core CSS generation pipeline is functional
- Basic configuration parsing exists
- File system operations are available

### ❌ **What's Missing**
- No Node.js bindings or WASM build
- No PostCSS plugin interface
- No integration with PostCSS ecosystem
- No lazy loading or on-demand generation
- No PostCSS-specific optimization

## 🏗️ Architecture Design

### **PostCSS Integration**

#### **1. Plugin Interface**
```typescript
// @tailwind-rs/postcss
import type { Plugin } from 'postcss'

interface TailwindRsOptions {
  content?: string[]
  config?: string
  base?: string
  includePreflight?: boolean
  respectPrefix?: boolean
  respectImportant?: boolean
}

declare function tailwindRs(options?: TailwindRsOptions): Plugin

export = tailwindRs
```

#### **2. PostCSS Plugin Implementation**
```typescript
import { Plugin } from 'postcss'
import { generateCss } from '@tailwind-rs/wasm'

export default function tailwindRs(options: TailwindRsOptions = {}): Plugin {
  return {
    postcssPlugin: '@tailwind-rs/postcss',

    Once(root, { result }) {
      // 1. Parse @tailwind directives
      // 2. Extract content configuration
      // 3. Scan content files for classes
      // 4. Generate CSS using WASM
      // 5. Replace @tailwind directives with generated CSS
      return generateCss(options)
    }
  }
}
```

### **WASM Build Architecture**

#### **1. WASM Module Interface**
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TailwindRsWasm {
    generator: CssGenerator,
    config: Config,
}

#[wasm_bindgen]
impl TailwindRsWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(config_json: &str) -> Result<TailwindRsWasm, JsValue> {
        // Initialize with configuration
    }

    #[wasm_bindgen]
    pub fn generate_css(&mut self, content: &[String]) -> Result<String, JsValue> {
        // Generate CSS from content
    }

    #[wasm_bindgen]
    pub fn generate_css_with_config(
        &mut self,
        content: &[String],
        config_override: &str
    ) -> Result<String, JsValue> {
        // Generate with config override
    }
}

#[wasm_bindgen]
pub fn generate_css_from_config(config_json: &str, content: &[String]) -> Result<String, JsValue> {
    // One-shot CSS generation
}
```

#### **2. WASM Build Configuration**
```toml
# Cargo.toml additions
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = "0.3"

[dependencies.tailwind-rs-core]
path = "../tailwind-rs-core"
```

### **Content Processing**

#### **1. Directive Parser**
```rust
pub struct DirectiveParser {
    directives: Vec<TailwindDirective>,
}

#[derive(Debug)]
pub enum TailwindDirective {
    Base,      // @tailwind base
    Components, // @tailwind components
    Utilities, // @tailwind utilities
    Variants,  // @tailwind variants (legacy)
    Screens,   // @tailwind screens (legacy)
}

impl DirectiveParser {
    pub fn parse(&self, css: &str) -> Result<Vec<TailwindDirective>, ParseError> {
        // Parse @tailwind directives from CSS
        Ok(directives)
    }

    pub fn replace_directives(&self, css: &str, generated_css: &str) -> String {
        // Replace @tailwind directives with generated CSS
        css.replace("@tailwind base;", &format!("/* @tailwind base */\\n{}", generated_css))
           .replace("@tailwind components;", "")
           .replace("@tailwind utilities;", "")
    }
}
```

#### **2. Lazy Evaluation**
```rust
pub struct LazyEvaluator {
    generator: CssGenerator,
    content_cache: HashMap<String, ContentResult>,
    css_cache: HashMap<String, String>,
}

impl LazyEvaluator {
    pub fn evaluate_lazy(&mut self, content_paths: &[String]) -> Result<String, EvalError> {
        // 1. Check cache validity
        // 2. Only scan changed files
        // 3. Generate minimal CSS
        // 4. Cache results
        Ok(css)
    }

    pub fn invalidate_cache(&mut self, changed_files: &[String]) {
        // Mark cache entries as stale
    }
}
```

### **PostCSS-Specific Features**

#### **1. Source Map Integration**
```rust
pub struct PostCssSourceMap {
    generator: SourceMapGenerator,
    input_source_map: Option<String>,
}

impl PostCssSourceMap {
    pub fn generate_postcss_map(&self, css: &str, original_css: &str) -> String {
        // Generate PostCSS-compatible source maps
        // Include original PostCSS transformations
    }
}
```

#### **2. Plugin Compatibility**
```rust
pub struct PostCssPluginAdapter {
    postcss_plugins: Vec<Box<dyn PostCssPlugin>>,
}

impl PostCssPluginAdapter {
    pub fn adapt_plugins(&self, css: &str) -> Result<String, AdapterError> {
        // Allow PostCSS plugins to run on generated CSS
        // Maintain plugin chain compatibility
        Ok(processed_css)
    }
}
```

## 🚀 Implementation Plan

### **Phase 1: WASM Build Setup** (Week 1-2)

#### **Step 1.1: WASM Bindings**
- [ ] Create WASM module interface
- [ ] Implement basic CSS generation API
- [ ] Add configuration passing
- [ ] Set up build pipeline

#### **Step 1.2: Node.js Package**
- [ ] Create `@tailwind-rs/wasm` package
- [ ] Implement TypeScript definitions
- [ ] Add package.json and build scripts
- [ ] Set up npm publishing

#### **Step 1.3: Basic PostCSS Plugin**
- [ ] Create PostCSS plugin skeleton
- [ ] Implement basic directive parsing
- [ ] Add simple CSS generation
- [ ] Test basic functionality

### **Phase 2: Content Processing** (Week 3)

#### **Step 2.1: Directive Handling**
- [ ] Implement @tailwind directive parsing
- [ ] Add directive replacement logic
- [ ] Support all directive types (base, components, utilities)
- [ ] Handle legacy directives

#### **Step 2.2: Content Integration**
- [ ] Connect to content scanning system
- [ ] Implement lazy evaluation
- [ ] Add content caching
- [ ] Optimize for PostCSS workflow

#### **Step 2.3: Configuration**
- [ ] Support PostCSS plugin options
- [ ] Integrate with tailwind-rs config files
- [ ] Add configuration validation
- [ ] Handle config overrides

### **Phase 3: Advanced Features** (Week 4)

#### **Step 3.1: Source Maps**
- [ ] Implement PostCSS source map generation
- [ ] Integrate with existing source map system
- [ ] Handle source map composition
- [ ] Add source map options

#### **Step 3.2: Performance Optimization**
- [ ] Implement lazy CSS generation
- [ ] Add incremental builds
- [ ] Optimize WASM bundle size
- [ ] Add caching layers

#### **Step 3.3: Plugin Ecosystem**
- [ ] Allow other PostCSS plugins to run
- [ ] Maintain plugin chain compatibility
- [ ] Add plugin ordering
- [ ] Handle plugin errors gracefully

### **Phase 4: Production & Testing** (Week 5)

#### **Step 4.1: Error Handling**
- [ ] Comprehensive error reporting
- [ ] PostCSS-compatible error format
- [ ] Graceful degradation
- [ ] Debug logging

#### **Step 4.2: Testing & Compatibility**
- [ ] Test with popular PostCSS setups
- [ ] Ensure drop-in compatibility
- [ ] Performance benchmarking
- [ ] Cross-platform testing

#### **Step 4.3: Documentation & Examples**
- [ ] Create migration guide
- [ ] Add usage examples
- [ ] Document configuration options
- [ ] Create troubleshooting guide

## 🧪 Testing Strategy

### **Unit Tests**
- [ ] WASM bindings functionality
- [ ] Directive parsing accuracy
- [ ] Configuration handling
- [ ] Error conditions

### **Integration Tests**
- [ ] PostCSS plugin loading
- [ ] CSS generation in PostCSS pipeline
- [ ] Source map generation
- [ ] Plugin chain compatibility

### **Compatibility Tests**
- [ ] Drop-in replacement for @tailwindcss/postcss
- [ ] Works with popular build tools (Webpack, Vite, etc.)
- [ ] Compatible with existing Tailwind configurations
- [ ] Performance comparison with original plugin

### **Performance Tests**
- [ ] WASM bundle size optimization
- [ ] CSS generation speed
- [ ] Memory usage in Node.js environment
- [ ] Build time comparison

## 📊 Success Criteria

### **Functional Requirements**
- [ ] ✅ Drop-in replacement for @tailwindcss/postcss
- [ ] ✅ All @tailwind directives supported
- [ ] ✅ Configuration file compatibility
- [ ] ✅ Content scanning integration
- [ ] ✅ Source map generation
- [ ] ✅ PostCSS plugin chain compatibility

### **Performance Requirements**
- [ ] ✅ CSS generation < 2x slower than native
- [ ] ✅ WASM bundle size < 5MB
- [ ] ✅ Memory usage reasonable for Node.js
- [ ] ✅ Build time competitive

### **Compatibility Requirements**
- [ ] ✅ Works with all major build tools
- [ ] ✅ Supports all Tailwind configuration formats
- [ ] ✅ Compatible with existing PostCSS setups
- [ ] ✅ Same CSS output as original plugin

## 🔗 Dependencies

### **Required Before Implementation**
- ✅ Core CSS generation pipeline
- ✅ Configuration system
- ✅ Content scanning
- ✅ WASM build infrastructure

### **Required During Implementation**
- 🔄 CLI tool (for configuration sharing)
- 🔄 Source maps (for debugging)
- 🔄 Plugin system (for extensibility)
- 🔄 Performance optimization

## 📈 Timeline & Milestones

| Week | Milestone | Deliverables |
|------|-----------|--------------|
| 1-2 | WASM foundation | WASM bindings, basic PostCSS plugin |
| 3 | Content processing | Directive parsing, content integration |
| 4 | Advanced features | Source maps, performance optimization |
| 5 | Production ready | Testing, documentation, publishing |

## 🚨 Risk Assessment

### **High Risk**
- **WASM Performance**: JavaScript interop overhead
- **Bundle Size**: WASM module size for web distribution
- **PostCSS Compatibility**: Maintaining plugin chain compatibility

### **Mitigation Strategies**
- **Performance Benchmarking**: Regular performance testing
- **Bundle Optimization**: Code splitting and tree shaking
- **Compatibility Testing**: Extensive testing with real PostCSS setups

## 📚 Related Documents

- [PostCSS Integration Guide](./integration-guide.md)
- [WASM Build Guide](./wasm-build.md)
- [Migration Guide](./migration-guide.md)

---

*This design document outlines a PostCSS plugin that provides seamless integration with existing Tailwind CSS workflows while leveraging tailwind-rs's performance advantages.*
