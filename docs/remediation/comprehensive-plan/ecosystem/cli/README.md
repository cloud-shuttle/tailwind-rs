# 🖥️ CLI Tool Architecture Design

## 📋 Overview

**Current Status**: ❌ Not Implemented (0% Complete)  
**Priority**: Critical  
**Timeline**: 3-4 weeks  
**Dependencies**: Core CSS generation, Plugin system

## 🎯 Mission

Implement a comprehensive command-line interface that provides build, watch, and development capabilities equivalent to the official Tailwind CSS CLI, enabling developers to use tailwind-rs in any project environment.

## 📊 Current State Analysis

### ✅ **What's Working**
- Core CSS generation pipeline is functional
- Basic configuration system exists
- File system operations are available

### ❌ **What's Missing**
- No CLI binary or entry point
- No build command (`tailwind build`)
- No watch mode (`tailwind watch`)
- No configuration file parsing
- No content detection/scanning
- No output optimization

## 🏗️ Architecture Design

### **CLI Structure**

#### **1. Main Entry Point**
```rust
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tailwind-rs")]
#[command(about = "A utility-first CSS framework for rapidly building custom user interfaces")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build CSS from source files
    Build(BuildArgs),
    /// Watch for changes and rebuild automatically
    Watch(WatchArgs),
    /// Initialize a new tailwind-rs project
    Init(InitArgs),
    /// Upgrade configuration and migrate from other versions
    Upgrade(UpgradeArgs),
}

#[derive(Parser)]
pub struct BuildArgs {
    /// Input CSS file (default: search for input.css)
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Output CSS file (default: dist/output.css)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Configuration file path
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Content paths to scan for classes
    #[arg(short, long)]
    pub content: Vec<String>,

    /// Watch mode
    #[arg(short, long)]
    pub watch: bool,

    /// Minify output
    #[arg(short, long)]
    pub minify: bool,

    /// Enable source maps
    #[arg(long)]
    pub sourcemap: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
```

#### **2. CLI Runner**
```rust
pub struct CliRunner {
    config: Config,
    generator: CssGenerator,
    content_scanner: ContentScanner,
    file_watcher: Option<FileWatcher>,
}

impl CliRunner {
    pub async fn run(self) -> Result<(), CliError> {
        match self.config.command {
            Commands::Build(args) => self.run_build(args).await,
            Commands::Watch(args) => self.run_watch(args).await,
            Commands::Init(args) => self.run_init(args).await,
            Commands::Upgrade(args) => self.run_upgrade(args).await,
        }
    }

    async fn run_build(&self, args: BuildArgs) -> Result<(), CliError> {
        // 1. Load configuration
        // 2. Scan content files
        // 3. Generate CSS
        // 4. Optimize and write output
        // 5. Generate source maps if requested
        Ok(())
    }

    async fn run_watch(&self, args: WatchArgs) -> Result<(), CliError> {
        // 1. Initial build
        // 2. Set up file watchers
        // 3. Rebuild on changes
        // 4. Handle graceful shutdown
        Ok(())
    }
}
```

### **Core Components**

#### **1. Configuration Loader**
```rust
pub struct ConfigLoader {
    search_paths: Vec<PathBuf>,
}

impl ConfigLoader {
    pub fn load(&self, config_path: Option<&Path>) -> Result<Config, ConfigError> {
        // 1. Search for configuration files
        // 2. Parse configuration (JSON, JS, TS)
        // 3. Validate configuration
        // 4. Load plugins
        // 5. Merge with defaults
        Ok(config)
    }

    fn search_config_files(&self) -> Vec<PathBuf> {
        vec![
            "tailwind-rs.config.js",
            "tailwind-rs.config.ts",
            "tailwind-rs.config.json",
            "tailwind.config.js",
            "tailwind.config.ts",
            "tailwind.config.json",
        ]
    }
}
```

#### **2. Content Scanner**
```rust
pub struct ContentScanner {
    patterns: Vec<GlobPattern>,
    exclude_patterns: Vec<GlobPattern>,
    parsers: HashMap<String, Box<dyn ContentParser>>,
}

impl ContentScanner {
    pub async fn scan(&self, paths: &[String]) -> Result<ContentResult, ScanError> {
        // 1. Expand glob patterns
        // 2. Filter excluded files
        // 3. Parse content files
        // 4. Extract class names
        // 5. Return unique classes
        Ok(result)
    }

    pub async fn scan_incremental(&self, changed_files: &[PathBuf]) -> Result<ContentResult, ScanError> {
        // 1. Only scan changed files
        // 2. Update class cache
        // 3. Return new classes
        Ok(result)
    }
}
```

#### **3. File Watcher**
```rust
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    content_scanner: ContentScanner,
    css_generator: CssGenerator,
    output_writer: OutputWriter,
}

impl FileWatcher {
    pub async fn watch(&mut self, paths: &[PathBuf]) -> Result<(), WatchError> {
        let mut debouncer = new_debouncer(Duration::from_millis(300), None, |res| {
            match res {
                Ok(events) => self.handle_file_changes(events),
                Err(errors) => self.handle_watch_errors(errors),
            }
        })?;

        for path in paths {
            debouncer.watch(path, RecursiveMode::Recursive)?;
        }

        // Keep watching until shutdown signal
        let _ = tokio::signal::ctrl_c().await;
        Ok(())
    }

    fn handle_file_changes(&mut self, events: Vec<notify::Event>) {
        // 1. Determine what changed
        // 2. Update content scan if needed
        // 3. Regenerate CSS
        // 4. Write output
    }
}
```

#### **4. Output Writer**
```rust
pub struct OutputWriter {
    minifier: Option<CssMinifier>,
    source_map_generator: Option<SourceMapGenerator>,
}

impl OutputWriter {
    pub async fn write(&self, css: &str, output_path: &Path, options: &OutputOptions) -> Result<(), WriteError> {
        let mut final_css = css.to_string();

        // 1. Minify if requested
        if options.minify {
            final_css = self.minifier.as_ref()
                .ok_or(WriteError::MinifierNotAvailable)?
                .minify(&final_css)?;
        }

        // 2. Generate source maps if requested
        if options.sourcemap {
            let source_map = self.source_map_generator.as_ref()
                .ok_or(WriteError::SourceMapNotAvailable)?
                .generate(&final_css, output_path)?;
            self.write_source_map(&source_map, output_path).await?;
        }

        // 3. Write CSS file
        tokio::fs::write(output_path, final_css).await?;
        Ok(())
    }
}
```

## 🚀 Implementation Plan

### **Phase 1: Core CLI Structure** (Week 1)

#### **Step 1.1: CLI Framework Setup**
- [ ] Set up clap for argument parsing
- [ ] Create main CLI entry point
- [ ] Define command structure
- [ ] Add basic error handling

#### **Step 1.2: Configuration Loading**
- [ ] Implement config file search
- [ ] Add JSON/JS/TS config parsing
- [ ] Create configuration validation
- [ ] Add default configuration

#### **Step 1.3: Basic Build Command**
- [ ] Implement simple build workflow
- [ ] Add CSS generation integration
- [ ] Create basic output writing
- [ ] Add verbose logging

### **Phase 2: Content Scanning** (Week 2)

#### **Step 2.1: File System Operations**
- [ ] Implement glob pattern expansion
- [ ] Add file filtering and exclusion
- [ ] Create content parser interfaces
- [ ] Add language-specific parsers

#### **Step 2.2: Class Extraction**
- [ ] Implement class name extraction
- [ ] Add deduplication logic
- [ ] Create content result caching
- [ ] Add incremental scanning

#### **Step 2.3: Integration**
- [ ] Connect content scanner to CSS generator
- [ ] Add content-based CSS optimization
- [ ] Implement content watching

### **Phase 3: Watch Mode** (Week 3)

#### **Step 3.1: File Watching**
- [ ] Set up notify file watcher
- [ ] Implement debounced rebuilds
- [ ] Add file change detection
- [ ] Create watch error handling

#### **Step 3.2: Incremental Builds**
- [ ] Implement incremental content scanning
- [ ] Add CSS diffing and patching
- [ ] Create efficient rebuild logic
- [ ] Add watch mode logging

#### **Step 3.3: Development Features**
- [ ] Add hot reload signaling
- [ ] Implement development server integration
- [ ] Add watch mode configuration
- [ ] Create graceful shutdown

### **Phase 4: Advanced Features** (Week 4)

#### **Step 4.1: Output Optimization**
- [ ] Implement CSS minification
- [ ] Add source map generation
- [ ] Create output optimization pipeline
- [ ] Add compression options

#### **Step 4.2: Additional Commands**
- [ ] Implement `init` command
- [ ] Add `upgrade` command
- [ ] Create configuration validation
- [ ] Add help and documentation

#### **Step 4.3: Production Polish**
- [ ] Add comprehensive error handling
- [ ] Implement progress reporting
- [ ] Add performance monitoring
- [ ] Create CLI testing

## 🧪 Testing Strategy

### **Unit Tests**
- [ ] CLI argument parsing
- [ ] Configuration loading
- [ ] Content scanning logic
- [ ] File watching behavior

### **Integration Tests**
- [ ] End-to-end build process
- [ ] Watch mode functionality
- [ ] Configuration file handling
- [ ] Error handling scenarios

### **Performance Tests**
- [ ] Build time benchmarks
- [ ] Memory usage during large builds
- [ ] File watching responsiveness
- [ ] Incremental build efficiency

### **Compatibility Tests**
- [ ] Tailwind CSS CLI compatibility
- [ ] Various project structures
- [ ] Different configuration formats
- [ ] Cross-platform functionality

## 📊 Success Criteria

### **Functional Requirements**
- [ ] ✅ `tailwind-rs build` command works
- [ ] ✅ `tailwind-rs watch` command works
- [ ] ✅ Configuration file loading
- [ ] ✅ Content scanning and class extraction
- [ ] ✅ CSS output generation and writing
- [ ] ✅ Source map generation (optional)
- [ ] ✅ CSS minification (optional)

### **Performance Requirements**
- [ ] ✅ Initial build < 5 seconds for typical projects
- [ ] ✅ Incremental rebuilds < 500ms
- [ ] ✅ Memory usage < 200MB for large projects
- [ ] ✅ File watching responsive (< 300ms debounce)

### **Compatibility Requirements**
- [ ] ✅ Drop-in replacement for `tailwindcss` CLI
- [ ] ✅ Same command-line interface
- [ ] ✅ Same configuration file formats
- [ ] ✅ Same output file structure

## 🔗 Dependencies

### **Required Before Implementation**
- ✅ Core CSS generation pipeline
- ✅ Content scanning foundation
- ✅ Configuration system
- ✅ File system utilities

### **Required During Implementation**
- 🔄 Plugin system (for plugin loading)
- 🔄 CSS optimization (for minification)
- 🔄 Source maps (for debugging)
- 🔄 Content parsers (for class extraction)

## 📈 Timeline & Milestones

| Week | Milestone | Deliverables |
|------|-----------|--------------|
| 1 | CLI foundation | Argument parsing, config loading, basic build |
| 2 | Content scanning | File scanning, class extraction, integration |
| 3 | Watch mode | File watching, incremental builds, development features |
| 4 | Production ready | Optimization, additional commands, polish |

## 🚨 Risk Assessment

### **High Risk**
- **File Watching Complexity**: Cross-platform file watching implementation
- **Performance**: Large project build times and memory usage
- **Compatibility**: Matching Tailwind CLI exactly

### **Mitigation Strategies**
- **Incremental Development**: Start simple, add features iteratively
- **Performance Monitoring**: Benchmark regularly during development
- **Compatibility Testing**: Test against real Tailwind projects

## 📚 Related Documents

- [CLI Usage Guide](./usage-guide.md)
- [Configuration Reference](./configuration.md)
- [CLI Testing](./testing.md)

---

*This design document outlines a comprehensive CLI tool that will provide full compatibility with the official Tailwind CSS CLI while leveraging Rust's performance advantages.*
