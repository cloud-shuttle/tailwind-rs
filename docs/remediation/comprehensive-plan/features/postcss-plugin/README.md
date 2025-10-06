# PostCSS Plugin Design Document

## 🎯 **Overview**

The **tailwind-rs-postcss plugin** provides seamless integration with existing PostCSS build pipelines, enabling developers to use Tailwind-RS in Webpack, Vite, Rollup, and other build tools that support PostCSS.

## 🏗️ **Architecture**

### **Core Components**

```
crates/tailwind-rs-postcss/
├── src/
│   ├── lib.rs              # Main PostCSS plugin interface
│   ├── plugin.rs           # Plugin implementation
│   ├── processor.rs        # CSS processing logic
│   ├── extractor.rs        # Class extraction from content
│   ├── cache.rs            # Build caching and optimization
│   └── config.rs           # PostCSS-specific configuration
├── pkg/                    # WebAssembly build output
│   └── pkg.js             # JavaScript bindings
└── tests/                  # Integration tests
```

### **Plugin Interface**

The plugin exposes a JavaScript API compatible with PostCSS:

```javascript
const tailwindRs = require('tailwind-rs-postcss');

// Basic usage
postcss([
  tailwindRs({
    content: ['./src/**/*.{html,js,ts,jsx,tsx}'],
    theme: {
      extend: {}
    },
    plugins: []
  })
])

// Advanced usage with custom config
postcss([
  tailwindRs({
    config: './tailwind.config.js',
    content: ['./src/**/*.{js,jsx,ts,tsx}'],
    safelist: ['custom-class'],
    blocklist: ['unused-class']
  })
])
```

## 🔧 **Core Functionality**

### **1. CSS Processing Pipeline**

```
Input CSS (@tailwind directives)
    ↓
Content Scanning (HTML/JS/TS files)
    ↓
Class Extraction & Deduplication
    ↓
CSS Generation (Tailwind-RS Core)
    ↓
Output CSS (Optimized & Minified)
```

### **2. Content Processing**

#### **File Types Supported**
- HTML: `<div class="bg-blue-500 hover:bg-blue-700">`
- JavaScript/TypeScript: `className: "flex items-center"`
- Vue/Svelte: `class="text-lg {active ? 'font-bold' : ''}"`
- Markdown: CSS classes in code blocks
- Custom file types via configuration

#### **Dynamic Class Handling**
- Template literals: `className: \`bg-${color}-${intensity}\``
- Conditional classes: `className: active ? 'text-red' : 'text-blue'`
- Runtime class construction: `classNames('base', { active: isActive })`

### **3. Configuration System**

#### **Configuration Sources**
1. **JavaScript Config**: `tailwind.config.js`
2. **TypeScript Config**: `tailwind.config.ts`
3. **TOML Config**: `tailwind-rs.toml`
4. **Inline Config**: Plugin options

#### **Configuration Merging**
```javascript
// tailwind.config.js
module.exports = {
  content: ['./src/**/*.{js,ts}'],
  theme: {
    extend: {
      colors: {
        primary: '#3b82f6'
      }
    }
  }
}

// Plugin usage merges with config
postcss([
  tailwindRs({
    content: ['./pages/**/*.{js,jsx}'], // Merges with config
    theme: {
      extend: {
        fontFamily: {
          sans: ['Inter', 'sans-serif']
        }
      }
    } // Merges with config theme
  })
])
```

## 🎨 **CSS Generation**

### **Directive Processing**

#### **@tailwind base**
```css
/* Input */
@tailwind base;

/* Output */
*, ::before, ::after {
  box-sizing: border-box;
}

html {
  line-height: 1.15;
  -webkit-text-size-adjust: 100%;
}

/* Additional base styles... */
```

#### **@tailwind components**
```css
/* Input */
@tailwind components;

/* Output - Reserved for component classes */
```

#### **@tailwind utilities**
```css
/* Input */
@tailwind utilities;

/* Output - Generated from content analysis */
.flex { display: flex; }
.items-center { align-items: center; }
.bg-blue-500 { background-color: rgb(59 130 246); }
.hover\:bg-blue-700:hover { background-color: rgb(29 78 216); }
/* ... thousands more utilities */
```

### **Optimization Features**

- **Tree Shaking**: Only includes used utilities
- **CSS Deduplication**: Merges identical rules
- **Selector Optimization**: Minimizes selector specificity
- **Minification**: Removes whitespace and comments

## 🔌 **Plugin Ecosystem**

### **Plugin Loading**

```javascript
// tailwind.config.js
module.exports = {
  plugins: [
    require('tailwindcss-typography'),
    require('@tailwindcss/forms'),
    // Custom plugins...
  ]
}
```

### **Plugin Architecture**

```rust
// Plugin trait for extending functionality
pub trait PostCssPlugin {
    fn name(&self) -> &str;
    fn process_css(&self, css: &mut CssAst, config: &Config) -> Result<(), PostCssError>;
    fn add_utilities(&self, utilities: &mut HashMap<String, CssRule>) -> Result<(), PostCssError>;
}
```

## ⚡ **Performance Optimizations**

### **Build Caching**

- **Content Hashing**: Cache based on file contents
- **Incremental Builds**: Only rebuild changed files
- **Dependency Tracking**: Track file relationships
- **Memory Caching**: In-memory cache for repeated builds

### **Parallel Processing**

- **Concurrent File Scanning**: Process multiple files simultaneously
- **Parallel CSS Generation**: Distribute rule generation across cores
- **Async I/O**: Non-blocking file operations

### **Memory Management**

- **Streaming Processing**: Process large files without loading entirely
- **Garbage Collection**: Automatic cleanup of unused resources
- **Memory Pool**: Reuse allocated objects

## 🌐 **WebAssembly Integration**

### **Browser Support**

```javascript
import { tailwindRs } from 'tailwind-rs-postcss';

// Browser-compatible API
const processor = tailwindRs({
  content: ['./src/**/*.{js,ts}'],
  theme: { /* ... */ }
});

// Process CSS
const result = await processor.process(`
  @tailwind base;
  @tailwind utilities;
`, {
  from: 'input.css',
  to: 'output.css'
});
```

### **Build Tool Integration**

#### **Vite Plugin**
```javascript
// vite.config.js
import tailwindRs from 'tailwind-rs-postcss/vite'

export default {
  plugins: [
    tailwindRs({
      content: ['./src/**/*.{vue,js,ts}']
    })
  ]
}
```

#### **Webpack Loader**
```javascript
// webpack.config.js
module.exports = {
  module: {
    rules: [{
      test: /\.css$/,
      use: [
        'style-loader',
        'css-loader',
        {
          loader: 'tailwind-rs-postcss',
          options: {
            content: ['./src/**/*.{js,jsx}']
          }
        }
      ]
    }]
  }
}
```

## 🧪 **Testing Strategy**

### **Unit Tests**

- CSS processing accuracy
- Class extraction reliability
- Configuration parsing
- Plugin loading and execution

### **Integration Tests**

- End-to-end PostCSS pipeline
- Build tool integration (Vite, Webpack)
- Browser compatibility
- Performance benchmarks

### **Browser Tests**

- WebAssembly loading
- CSS generation in browser
- Error handling and recovery

## 📊 **Success Metrics**

### **Performance Targets**

- **Cold Build**: < 5 seconds for typical projects
- **Hot Reload**: < 500ms rebuild time
- **Memory Usage**: < 200MB for large codebases
- **Bundle Size**: < 500KB for WebAssembly build

### **Compatibility Goals**

- **PostCSS Compatibility**: 100% PostCSS API compliance
- **Build Tool Support**: Major tools (Vite, Webpack, Rollup, etc.)
- **Browser Support**: Modern browsers with WASM support
- **Configuration**: 100% compatibility with Tailwind CSS configs

## 🚀 **Implementation Plan**

### **Phase 1: Core PostCSS Interface** (Week 11)
- [ ] PostCSS plugin trait and interface
- [ ] Basic CSS processing pipeline
- [ ] Configuration loading and merging
- [ ] File content scanning

### **Phase 2: CSS Generation** (Week 12)
- [ ] CSS AST processing
- [ ] Tailwind directive handling
- [ ] Utility class generation
- [ ] Optimization and minification

### **Phase 3: Advanced Features** (Week 13)
- [ ] Plugin ecosystem support
- [ ] Caching and incremental builds
- [ ] Parallel processing
- [ ] Error handling and diagnostics

### **Phase 4: WebAssembly & Integration** (Week 14)
- [ ] WASM build and JavaScript bindings
- [ ] Build tool plugins (Vite, Webpack)
- [ ] Browser testing and optimization
- [ ] Documentation and examples

## 🔗 **Integration Points**

### **With Core Engine**

- Uses `CssGenerator` for rule generation
- Integrates with `PluginManager` for extensions
- Leverages `ColorCache` for performance
- Supports all core Tailwind features

### **With CLI Tool**

- Shared configuration system
- Consistent file scanning logic
- Unified caching strategy
- Cross-platform compatibility

### **With Build Tools**

- Standard PostCSS plugin interface
- Compatible with existing workflows
- Drop-in replacement for tailwindcss
- Enhanced performance and features

## 🎯 **Next Steps**

1. **Create PostCSS plugin structure**
2. **Implement basic CSS processing**
3. **Add content scanning and class extraction**
4. **Integrate with Tailwind-RS core**
5. **Add configuration support**
6. **Implement optimization features**
7. **Create WebAssembly build**
8. **Add build tool integrations**
9. **Comprehensive testing**
10. **Documentation and examples**

---

*This document outlines the complete design for the PostCSS plugin integration, providing seamless compatibility with existing build toolchains while leveraging Tailwind-RS's advanced features and performance.*
