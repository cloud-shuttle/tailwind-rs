# CLI Tool Design Document

## 🎯 **Overview**

The **tailwind-rs CLI tool** provides a command-line interface for CSS generation and development workflow integration. It serves as the primary interface for developers using tailwind-rs in build processes, similar to the official Tailwind CSS CLI.

## 🏗️ **Architecture**

### **Core Components**

```
crates/
├── tailwind-rs-cli/           # Main CLI crate
│   ├── src/
│   │   ├── main.rs           # CLI entry point
│   │   ├── commands/         # Command implementations
│   │   │   ├── build.rs      # Build command
│   │   │   ├── init.rs       # Init command
│   │   │   ├── watch.rs      # Watch command
│   │   │   └── mod.rs
│   │   ├── config.rs         # Configuration handling
│   │   ├── file_watcher.rs   # File watching utilities
│   │   └── css_processor.rs  # CSS processing logic
│   └── Cargo.toml
```

### **Command Structure**

```bash
tailwind-rs [COMMAND] [OPTIONS]

Commands:
  build     Build CSS from source files
  init      Create configuration file
  watch     Watch mode for development
  help      Display help information

Options:
  -i, --input <FILE>     Input CSS file
  -o, --output <FILE>    Output CSS file
  -c, --config <FILE>    Configuration file
  -w, --watch            Watch mode
  --content <PATTERN>    Content file patterns
  --purge <PATTERN>      Purge file patterns
  --minify               Minify output
  --verbose              Verbose output
```

## 🔧 **Core Commands**

### **1. Build Command**

**Purpose**: Generate CSS from Tailwind classes found in source files

**Implementation**:
- Scan content files for Tailwind classes
- Generate CSS using `CssGenerator`
- Write output to specified file
- Support for minification and source maps

**Example**:
```bash
tailwind-rs build -i input.css -o output.css --content "src/**/*.{html,js,ts,jsx,tsx}"
```

### **2. Init Command**

**Purpose**: Create initial configuration file

**Implementation**:
- Generate `tailwind.config.js` or `tailwind.config.ts`
- Create basic `input.css` with Tailwind directives
- Set up default content patterns

**Example**:
```bash
tailwind-rs init
```

### **3. Watch Command**

**Purpose**: Monitor source files and rebuild CSS on changes

**Implementation**:
- File system watcher using `notify` crate
- Automatic rebuild on file changes
- Live reload integration (optional)
- Debounced rebuilds to prevent excessive regeneration

**Example**:
```bash
tailwind-rs watch -i input.css -o output.css --content "src/**/*.{html,js}"
```

## ⚙️ **Configuration**

### **Configuration File Support**

The CLI supports multiple configuration formats:

1. **tailwind.config.js** - JavaScript configuration
2. **tailwind.config.ts** - TypeScript configuration
3. **tailwind-rs.toml** - TOML configuration (native)

### **Configuration Structure**

```javascript
// tailwind.config.js
module.exports = {
  content: [
    "./src/**/*.{js,ts,jsx,tsx}",
    "./public/index.html"
  ],
  theme: {
    extend: {
      colors: {
        primary: "#3b82f6"
      }
    }
  },
  plugins: [
    // Plugin configurations
  ]
}
```

## 🔍 **Class Extraction**

### **Content Scanning**

The CLI implements sophisticated content scanning:

- **File Pattern Matching**: Support for globs and multiple file types
- **Class Extraction**: Parse HTML, JavaScript, and other source files
- **Dynamic Class Detection**: Handle dynamically constructed class names
- **Plugin Class Support**: Recognize custom plugin-generated classes

### **Supported File Types**

- HTML: `<div class="bg-blue-500 hover:bg-blue-700">`
- JavaScript/TypeScript: `className: "flex items-center"`
- Vue/Svelte: `class="text-lg {active ? 'font-bold' : ''}"`
- Markdown: CSS classes in code blocks

## 🎨 **CSS Generation**

### **Build Process**

1. **Initialization**: Load configuration and plugins
2. **Content Analysis**: Scan files for Tailwind classes
3. **CSS Generation**: Generate rules using `CssGenerator`
4. **Optimization**: Apply minification and deduplication
5. **Output**: Write final CSS file

### **Performance Optimizations**

- **Incremental Builds**: Only rebuild changed content
- **Parallel Processing**: Concurrent file scanning
- **Caching**: Cache parsed classes and generated CSS
- **Memory Efficiency**: Stream processing for large codebases

## 🔌 **Plugin Integration**

### **Plugin Loading**

- **Dynamic Plugin Loading**: Load plugins from configuration
- **Plugin Manager Integration**: Use core plugin system
- **Custom Utilities**: Support for plugin-defined utilities
- **Theme Extensions**: Apply plugin theme modifications

## 🧪 **Testing Strategy**

### **Unit Tests**

- Command parsing and validation
- File scanning accuracy
- CSS generation correctness
- Plugin loading and execution

### **Integration Tests**

- End-to-end build workflows
- Watch mode functionality
- Configuration file parsing
- Plugin integration testing

## 📊 **Success Metrics**

### **Performance Targets**

- **Build Time**: < 2 seconds for typical projects
- **Memory Usage**: < 100MB for large codebases
- **File Watching**: < 100ms rebuild latency
- **CSS Size**: Optimized output size

### **Compatibility Goals**

- **Drop-in Replacement**: Work with existing Tailwind workflows
- **Configuration Compatibility**: Support existing config formats
- **Plugin Ecosystem**: Compatible with popular Tailwind plugins
- **Build Tool Integration**: Work with Vite, Webpack, etc.

## 🚀 **Implementation Plan**

### **Phase 1: Core CLI Framework** (Week 7)
- [ ] CLI argument parsing with `clap`
- [ ] Basic project structure
- [ ] Configuration file loading
- [ ] Core build command skeleton

### **Phase 2: Build Command** (Week 8)
- [ ] File scanning implementation
- [ ] Class extraction logic
- [ ] CSS generation integration
- [ ] Output file writing

### **Phase 3: Watch Mode** (Week 9)
- [ ] File watcher implementation
- [ ] Change detection logic
- [ ] Incremental rebuilds
- [ ] Live reload support

### **Phase 4: Advanced Features** (Week 10)
- [ ] Plugin support
- [ ] Minification
- [ ] Source maps
- [ ] Performance optimizations

## 🔗 **Integration Points**

### **With Core Engine**

- Uses `CssGenerator` for CSS generation
- Integrates with `PluginManager` for extensibility
- Leverages `ColorCache` for performance
- Supports all core Tailwind features

### **With Ecosystem**

- **PostCSS Plugin**: Shared configuration and logic
- **Framework Integrations**: Consistent API across integrations
- **Build Tools**: Standard CLI interface for automation

## 📝 **API Design**

### **Public API**

```rust
// Main CLI entry point
pub fn run() -> Result<(), CliError>

// Command implementations
pub mod commands {
    pub fn build(args: BuildArgs) -> Result<(), CliError>
    pub fn init(args: InitArgs) -> Result<(), CliError>
    pub fn watch(args: WatchArgs) -> Result<(), CliError>
}
```

### **Error Handling**

```rust
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Build error: {0}")]
    Build(#[from] BuildError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

## 🎯 **Next Steps**

1. **Create CLI crate structure**
2. **Implement basic build command**
3. **Add configuration file support**
4. **Integrate with core CSS generator**
5. **Add watch mode functionality**
6. **Implement plugin loading**
7. **Add performance optimizations**
8. **Create comprehensive tests**

---

*This document outlines the complete design for the tailwind-rs CLI tool, providing a comprehensive development workflow interface for the CSS framework.*
