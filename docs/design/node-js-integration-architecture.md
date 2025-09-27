# Node.js Integration Architecture

## Overview

This document details the technical architecture for integrating Node.js and PostCSS.js into the Rust `tailwind-rs-postcss` crate, enabling real CSS processing capabilities.

## Integration Strategies

### Strategy 1: N-API (Recommended)
**Pros**: Native performance, direct memory access, mature ecosystem
**Cons**: Complex setup, platform-specific compilation

```rust
// Using napi-rs for Node.js integration
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct PostCSSProcessor {
    runtime: NodeRuntime,
}

#[napi]
impl PostCSSProcessor {
    #[napi(constructor)]
    pub fn new() -> Result<Self, napi::Error> {
        // Initialize Node.js runtime
        Ok(Self {
            runtime: NodeRuntime::new()?,
        })
    }
    
    #[napi]
    pub async fn process_css(&self, css: String, plugins: Vec<String>) -> Result<String, napi::Error> {
        // Call PostCSS.js through N-API
        self.runtime.execute_postcss(css, plugins).await
    }
}
```

### Strategy 2: Child Process Communication
**Pros**: Simple implementation, isolated execution
**Cons**: Higher overhead, serialization costs

```rust
// Using std::process for Node.js subprocess
use std::process::{Command, Stdio};
use serde_json::{json, Value};

pub struct PostCSSSubprocess {
    node_path: PathBuf,
    script_path: PathBuf,
}

impl PostCSSSubprocess {
    pub fn process_css(&self, css: &str, plugins: &[String]) -> Result<String, PostCSSError> {
        let input = json!({
            "css": css,
            "plugins": plugins
        });
        
        let output = Command::new(&self.node_path)
            .arg(&self.script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()?;
            
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            Err(PostCSSError::ProcessingFailed(String::from_utf8(output.stderr)?))
        }
    }
}
```

### Strategy 3: WebAssembly Bridge
**Pros**: No Node.js dependency, cross-platform
**Cons**: Limited PostCSS.js compatibility, performance overhead

```rust
// Using wasm-bindgen for JavaScript integration
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = postcss)]
    fn process(css: &str, plugins: &JsValue) -> JsValue;
}

pub struct PostCSSWasm {
    plugins: Vec<JsValue>,
}

impl PostCSSWasm {
    pub fn process_css(&self, css: &str) -> Result<String, PostCSSError> {
        let result = process(css, &JsValue::from_serde(&self.plugins)?);
        Ok(result.as_string().unwrap_or_default())
    }
}
```

## Recommended Architecture: N-API Integration

### Core Components

```rust
// src/node_bridge/mod.rs
pub mod runtime;
pub mod postcss_wrapper;
pub mod error_handling;

pub use runtime::NodeRuntime;
pub use postcss_wrapper::PostCSSWrapper;
pub use error_handling::NodeBridgeError;

// src/node_bridge/runtime.rs
use napi::bindgen_prelude::*;
use tokio::runtime::Runtime;

pub struct NodeRuntime {
    rt: Runtime,
    postcss_module: Module,
}

impl NodeRuntime {
    pub fn new() -> Result<Self, NodeBridgeError> {
        let rt = Runtime::new()?;
        let postcss_module = Self::load_postcss_module()?;
        
        Ok(Self {
            rt,
            postcss_module,
        })
    }
    
    pub async fn process_css(
        &self,
        css: &str,
        plugins: &[PluginConfig],
        options: &ProcessingOptions,
    ) -> Result<ProcessResult, NodeBridgeError> {
        // Execute PostCSS.js processing
        let result = self.postcss_module
            .call_method("process", (css, plugins, options))
            .await?;
            
        Ok(ProcessResult::from_js_value(result)?)
    }
    
    fn load_postcss_module() -> Result<Module, NodeBridgeError> {
        // Load PostCSS.js module
        // Initialize plugin registry
        // Set up error handling
    }
}
```

### Plugin System Integration

```rust
// src/plugins/node_plugins.rs
use napi::bindgen_prelude::*;

pub struct NodePlugin {
    name: String,
    config: JsValue,
    instance: PluginInstance,
}

impl NodePlugin {
    pub fn new(name: &str, config: &serde_json::Value) -> Result<Self, PluginError> {
        let instance = Self::load_plugin(name)?;
        Ok(Self {
            name: name.to_string(),
            config: JsValue::from_serde(config)?,
            instance,
        })
    }
    
    pub async fn process(&self, ast: &mut CSSNode) -> Result<(), PluginError> {
        // Convert Rust AST to JavaScript AST
        let js_ast = self.convert_to_js_ast(ast)?;
        
        // Execute plugin in Node.js
        let result = self.instance
            .call_method("process", (js_ast, &self.config))
            .await?;
            
        // Convert JavaScript AST back to Rust
        self.convert_from_js_ast(result, ast)?;
        
        Ok(())
    }
}
```

### CSS AST Conversion

```rust
// src/css_ast/conversion.rs
use napi::bindgen_prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JSCSSNode {
    pub node_type: String,
    pub selector: Option<String>,
    pub declarations: Vec<JSDeclaration>,
    pub children: Vec<JSCSSNode>,
}

#[derive(Serialize, Deserialize)]
pub struct JSDeclaration {
    pub property: String,
    pub value: String,
    pub important: bool,
}

impl CSSNode {
    pub fn to_js_value(&self) -> Result<JsValue, ConversionError> {
        let js_node = JSCSSNode::from(self);
        Ok(JsValue::from_serde(&js_node)?)
    }
    
    pub fn from_js_value(js_value: &JsValue) -> Result<Self, ConversionError> {
        let js_node: JSCSSNode = js_value.into_serde()?;
        Ok(js_node.into())
    }
}
```

## Implementation Details

### 1. Node.js Runtime Management

```rust
// src/node_bridge/runtime_manager.rs
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct NodeRuntimeManager {
    runtime: Arc<Mutex<NodeRuntime>>,
    plugin_registry: Arc<Mutex<PluginRegistry>>,
}

impl NodeRuntimeManager {
    pub fn new() -> Result<Self, NodeBridgeError> {
        let runtime = Arc::new(Mutex::new(NodeRuntime::new()?));
        let plugin_registry = Arc::new(Mutex::new(PluginRegistry::new()?));
        
        Ok(Self {
            runtime,
            plugin_registry,
        })
    }
    
    pub async fn process_css(
        &self,
        css: &str,
        plugins: &[String],
    ) -> Result<String, NodeBridgeError> {
        let runtime = self.runtime.lock().await;
        let plugin_configs = self.load_plugin_configs(plugins).await?;
        
        let result = runtime
            .process_css(css, &plugin_configs, &ProcessingOptions::default())
            .await?;
            
        Ok(result.css)
    }
}
```

### 2. Plugin Loading and Management

```rust
// src/plugins/plugin_loader.rs
use std::collections::HashMap;
use std::path::PathBuf;

pub struct PluginLoader {
    plugin_paths: HashMap<String, PathBuf>,
    loaded_plugins: HashMap<String, Box<dyn NodePlugin>>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugin_paths: HashMap::new(),
            loaded_plugins: HashMap::new(),
        }
    }
    
    pub fn register_plugin(&mut self, name: &str, path: PathBuf) {
        self.plugin_paths.insert(name.to_string(), path);
    }
    
    pub async fn load_plugin(&mut self, name: &str) -> Result<&dyn NodePlugin, PluginError> {
        if let Some(plugin) = self.loaded_plugins.get(name) {
            return Ok(plugin.as_ref());
        }
        
        let path = self.plugin_paths.get(name)
            .ok_or_else(|| PluginError::PluginNotFound(name.to_string()))?;
            
        let plugin = self.load_plugin_from_path(path).await?;
        self.loaded_plugins.insert(name.to_string(), plugin);
        
        Ok(self.loaded_plugins.get(name).unwrap().as_ref())
    }
}
```

### 3. Error Handling and Recovery

```rust
// src/node_bridge/error_handling.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeBridgeError {
    #[error("Node.js runtime error: {0}")]
    RuntimeError(String),
    
    #[error("PostCSS processing failed: {0}")]
    ProcessingError(String),
    
    #[error("Plugin loading failed: {0}")]
    PluginError(String),
    
    #[error("CSS parsing error: {0}")]
    ParseError(String),
    
    #[error("Memory allocation failed")]
    MemoryError,
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl NodeBridgeError {
    pub fn is_recoverable(&self) -> bool {
        matches!(self, 
            NodeBridgeError::ProcessingError(_) |
            NodeBridgeError::PluginError(_)
        )
    }
    
    pub fn retry_after(&self) -> Option<std::time::Duration> {
        match self {
            NodeBridgeError::ProcessingError(_) => Some(std::time::Duration::from_millis(100)),
            NodeBridgeError::PluginError(_) => Some(std::time::Duration::from_millis(500)),
            _ => None,
        }
    }
}
```

## Performance Considerations

### 1. Memory Management

```rust
// src/node_bridge/memory_manager.rs
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct MemoryManager {
    allocated: AtomicUsize,
    max_memory: usize,
}

impl MemoryManager {
    pub fn new(max_memory: usize) -> Self {
        Self {
            allocated: AtomicUsize::new(0),
            max_memory,
        }
    }
    
    pub fn allocate(&self, size: usize) -> Result<(), MemoryError> {
        let current = self.allocated.load(Ordering::Relaxed);
        if current + size > self.max_memory {
            return Err(MemoryError::OutOfMemory);
        }
        
        self.allocated.fetch_add(size, Ordering::Relaxed);
        Ok(())
    }
    
    pub fn deallocate(&self, size: usize) {
        self.allocated.fetch_sub(size, Ordering::Relaxed);
    }
}
```

### 2. Caching Strategy

```rust
// src/cache/postcss_cache.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PostCSSCache {
    cache: Arc<RwLock<HashMap<String, CachedResult>>>,
    ttl: std::time::Duration,
}

impl PostCSSCache {
    pub async fn get(&self, key: &str) -> Option<CachedResult> {
        let cache = self.cache.read().await;
        cache.get(key).and_then(|result| {
            if result.is_expired() {
                None
            } else {
                Some(result.clone())
            }
        })
    }
    
    pub async fn set(&self, key: String, result: CachedResult) {
        let mut cache = self.cache.write().await;
        cache.insert(key, result);
    }
}
```

## Testing Strategy

### 1. Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_node_runtime_initialization() {
        let runtime = NodeRuntime::new().await.unwrap();
        assert!(runtime.is_initialized());
    }
    
    #[tokio::test]
    async fn test_css_processing() {
        let runtime = NodeRuntime::new().await.unwrap();
        let css = "body { color: red; }";
        let plugins = vec!["autoprefixer".to_string()];
        
        let result = runtime.process_css(css, &plugins, &ProcessingOptions::default()).await;
        assert!(result.is_ok());
    }
}
```

### 2. Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_full_postcss_pipeline() {
        let processor = PostCSSProcessor::new().await.unwrap();
        let css = include_str!("../test_data/input.css");
        let expected = include_str!("../test_data/expected.css");
        
        let result = processor.process_css(css, &["autoprefixer", "cssnano"]).await.unwrap();
        assert_eq!(result, expected);
    }
}
```

## Deployment Considerations

### 1. Node.js Dependency Management

```toml
# Cargo.toml
[dependencies]
napi = "2.0"
napi-derive = "2.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[build-dependencies]
napi-build = "2.0"
```

### 2. Build Script

```rust
// build.rs
use napi_build::setup;

fn main() {
    setup();
    
    // Ensure Node.js is available
    if !std::process::Command::new("node")
        .arg("--version")
        .output()
        .is_ok() {
        panic!("Node.js is required but not found");
    }
}
```

### 3. Runtime Requirements

```rust
// src/node_bridge/requirements.rs
pub struct NodeRequirements {
    pub min_version: String,
    pub required_modules: Vec<String>,
}

impl NodeRequirements {
    pub fn check(&self) -> Result<(), RequirementError> {
        // Check Node.js version
        // Verify required modules are installed
        // Validate PostCSS.js availability
    }
}
```

This architecture provides a robust foundation for integrating Node.js and PostCSS.js into the Rust ecosystem while maintaining performance and reliability.
