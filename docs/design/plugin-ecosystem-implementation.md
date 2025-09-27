# Plugin Ecosystem Implementation

## Overview

This document outlines the comprehensive implementation plan for building a full-featured plugin ecosystem for `tailwind-rs-postcss`, enabling real CSS processing capabilities.

## Plugin Architecture

### Core Plugin Interface

```rust
// src/plugins/traits.rs
use async_trait::async_trait;
use crate::css_ast::CSSNode;
use crate::error::PluginError;

#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    
    async fn process(&self, ast: &mut CSSNode) -> Result<(), PluginError>;
    
    fn dependencies(&self) -> Vec<String>;
    fn conflicts(&self) -> Vec<String>;
    fn priority(&self) -> PluginPriority;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PluginPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}

#[async_trait]
pub trait ConfigurablePlugin: Plugin {
    type Config: Send + Sync;
    
    fn configure(&mut self, config: Self::Config) -> Result<(), PluginError>;
    fn get_config(&self) -> &Self::Config;
}
```

### Plugin Registry

```rust
// src/plugins/registry.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PluginRegistry {
    plugins: Arc<RwLock<HashMap<String, Box<dyn Plugin>>>>,
    execution_order: Arc<RwLock<Vec<String>>>,
    dependencies: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            execution_order: Arc::new(RwLock::new(Vec::new())),
            dependencies: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn register_plugin(&self, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let name = plugin.name().to_string();
        let dependencies = plugin.dependencies();
        
        // Check for conflicts
        self.check_conflicts(&name, &plugin.conflicts()).await?;
        
        // Register dependencies
        {
            let mut deps = self.dependencies.write().await;
            deps.insert(name.clone(), dependencies);
        }
        
        // Register plugin
        {
            let mut plugins = self.plugins.write().await;
            plugins.insert(name.clone(), plugin);
        }
        
        // Recalculate execution order
        self.recalculate_execution_order().await?;
        
        Ok(())
    }
    
    async fn recalculate_execution_order(&self) -> Result<(), PluginError> {
        let plugins = self.plugins.read().await;
        let dependencies = self.dependencies.read().await;
        
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();
        
        for (name, plugin) in plugins.iter() {
            self.topological_sort(name, &plugins, &dependencies, &mut order, &mut visited)?;
        }
        
        let mut execution_order = self.execution_order.write().await;
        *execution_order = order;
        
        Ok(())
    }
}
```

## Core Plugin Implementations

### 1. Autoprefixer Plugin

```rust
// src/plugins/autoprefixer.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoprefixerConfig {
    pub browsers: Vec<String>,
    pub cascade: bool,
    pub add: bool,
    pub remove: bool,
    pub supports: bool,
    pub flexbox: bool,
    pub grid: bool,
}

impl Default for AutoprefixerConfig {
    fn default() -> Self {
        Self {
            browsers: vec!["last 2 versions".to_string()],
            cascade: true,
            add: true,
            remove: true,
            supports: true,
            flexbox: true,
            grid: true,
        }
    }
}

pub struct AutoprefixerPlugin {
    config: AutoprefixerConfig,
    browser_data: BrowserData,
    prefix_data: PrefixData,
}

impl AutoprefixerPlugin {
    pub fn new(config: AutoprefixerConfig) -> Result<Self, PluginError> {
        let browser_data = BrowserData::load()?;
        let prefix_data = PrefixData::load()?;
        
        Ok(Self {
            config,
            browser_data,
            prefix_data,
        })
    }
    
    fn add_vendor_prefixes(&self, node: &mut CSSNode) -> Result<(), PluginError> {
        match &node.node_type {
            NodeType::Rule(rule) => {
                for declaration in &mut rule.declarations {
                    if let Some(prefixes) = self.get_prefixes(&declaration.property) {
                        for prefix in prefixes {
                            let prefixed_property = format!("{}{}", prefix, declaration.property);
                            // Add prefixed declaration
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
    
    fn get_prefixes(&self, property: &str) -> Option<Vec<String>> {
        self.prefix_data.get_prefixes(property, &self.config.browsers)
    }
}

#[async_trait]
impl Plugin for AutoprefixerPlugin {
    fn name(&self) -> &str { "autoprefixer" }
    fn version(&self) -> &str { "10.4.0" }
    fn description(&self) -> &str { "Add vendor prefixes to CSS" }
    
    async fn process(&self, ast: &mut CSSNode) -> Result<(), PluginError> {
        self.add_vendor_prefixes(ast)
    }
    
    fn dependencies(&self) -> Vec<String> { vec![] }
    fn conflicts(&self) -> Vec<String> { vec![] }
    fn priority(&self) -> PluginPriority { PluginPriority::High }
}
```

### 2. CSSNano Plugin

```rust
// src/plugins/cssnano.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSSNanoConfig {
    pub preset: String,
    pub discard_comments: bool,
    pub normalize_whitespace: bool,
    pub minify_selectors: bool,
    pub minify_font_values: bool,
    pub minify_params: bool,
}

impl Default for CSSNanoConfig {
    fn default() -> Self {
        Self {
            preset: "default".to_string(),
            discard_comments: true,
            normalize_whitespace: true,
            minify_selectors: true,
            minify_font_values: true,
            minify_params: true,
        }
    }
}

pub struct CSSNanoPlugin {
    config: CSSNanoConfig,
    minifier: CSSMinifier,
}

impl CSSNanoPlugin {
    pub fn new(config: CSSNanoConfig) -> Result<Self, PluginError> {
        let minifier = CSSMinifier::new(&config)?;
        
        Ok(Self {
            config,
            minifier,
        })
    }
    
    fn minify_css(&self, node: &mut CSSNode) -> Result<(), PluginError> {
        // Remove unnecessary whitespace
        if self.config.normalize_whitespace {
            self.minifier.normalize_whitespace(node)?;
        }
        
        // Minify selectors
        if self.config.minify_selectors {
            self.minifier.minify_selectors(node)?;
        }
        
        // Minify font values
        if self.config.minify_font_values {
            self.minifier.minify_font_values(node)?;
        }
        
        // Discard comments
        if self.config.discard_comments {
            self.minifier.discard_comments(node)?;
        }
        
        Ok(())
    }
}

#[async_trait]
impl Plugin for CSSNanoPlugin {
    fn name(&self) -> &str { "cssnano" }
    fn version(&self) -> &str { "5.1.0" }
    fn description(&self) -> &str { "CSS minification" }
    
    async fn process(&self, ast: &mut CSSNode) -> Result<(), PluginError> {
        self.minify_css(ast)
    }
    
    fn dependencies(&self) -> Vec<String> { vec![] }
    fn conflicts(&self) -> Vec<String> { vec![] }
    fn priority(&self) -> PluginPriority { PluginPriority::Low }
}
```

### 3. PostCSS Preset Env Plugin

```rust
// src/plugins/preset_env.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetEnvConfig {
    pub stage: u8,
    pub features: HashMap<String, bool>,
    pub browsers: Vec<String>,
    pub autoprefixer: Option<AutoprefixerConfig>,
    pub preserve: bool,
}

impl Default for PresetEnvConfig {
    fn default() -> Self {
        Self {
            stage: 2,
            features: HashMap::new(),
            browsers: vec!["last 2 versions".to_string()],
            autoprefixer: Some(AutoprefixerConfig::default()),
            preserve: false,
        }
    }
}

pub struct PresetEnvPlugin {
    config: PresetEnvConfig,
    features: FeatureSet,
    polyfills: Vec<Box<dyn Polyfill>>,
}

impl PresetEnvPlugin {
    pub fn new(config: PresetEnvConfig) -> Result<Self, PluginError> {
        let features = FeatureSet::load(&config)?;
        let polyfills = Self::load_polyfills(&config)?;
        
        Ok(Self {
            config,
            features,
            polyfills,
        })
    }
    
    fn apply_polyfills(&self, node: &mut CSSNode) -> Result<(), PluginError> {
        for polyfill in &self.polyfills {
            if polyfill.should_apply(node, &self.config.browsers) {
                polyfill.apply(node)?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Plugin for PresetEnvPlugin {
    fn name(&self) -> &str { "postcss-preset-env" }
    fn version(&self) -> &str { "7.0.0" }
    fn description(&self) -> &str { "Use future CSS features today" }
    
    async fn process(&self, ast: &mut CSSNode) -> Result<(), PluginError> {
        self.apply_polyfills(ast)
    }
    
    fn dependencies(&self) -> Vec<String> { vec!["autoprefixer".to_string()] }
    fn conflicts(&self) -> Vec<String> { vec![] }
    fn priority(&self) -> PluginPriority { PluginPriority::Normal }
}
```

## Advanced Plugin Features

### 1. Custom Plugin Development

```rust
// src/plugins/custom.rs
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CustomPlugin {
    name: String,
    version: String,
    processor: Arc<dyn Fn(&mut CSSNode) -> Result<(), PluginError> + Send + Sync>,
    config: serde_json::Value,
}

impl CustomPlugin {
    pub fn new<F>(
        name: String,
        version: String,
        processor: F,
        config: serde_json::Value,
    ) -> Self
    where
        F: Fn(&mut CSSNode) -> Result<(), PluginError> + Send + Sync + 'static,
    {
        Self {
            name,
            version,
            processor: Arc::new(processor),
            config,
        }
    }
}

#[async_trait]
impl Plugin for CustomPlugin {
    fn name(&self) -> &str { &self.name }
    fn version(&self) -> &str { &self.version }
    fn description(&self) -> &str { "Custom plugin" }
    
    async fn process(&self, ast: &mut CSSNode) -> Result<(), PluginError> {
        (self.processor)(ast)
    }
    
    fn dependencies(&self) -> Vec<String> { vec![] }
    fn conflicts(&self) -> Vec<String> { vec![] }
    fn priority(&self) -> PluginPriority { PluginPriority::Normal }
}
```

### 2. Plugin Chaining

```rust
// src/plugins/chain.rs
pub struct PluginChain {
    plugins: Vec<Box<dyn Plugin>>,
    execution_strategy: ExecutionStrategy,
}

pub enum ExecutionStrategy {
    Sequential,
    Parallel,
    Conditional,
}

impl PluginChain {
    pub async fn execute(&self, ast: &mut CSSNode) -> Result<(), PluginError> {
        match self.execution_strategy {
            ExecutionStrategy::Sequential => self.execute_sequential(ast).await,
            ExecutionStrategy::Parallel => self.execute_parallel(ast).await,
            ExecutionStrategy::Conditional => self.execute_conditional(ast).await,
        }
    }
    
    async fn execute_sequential(&self, ast: &mut CSSNode) -> Result<(), PluginError> {
        for plugin in &self.plugins {
            plugin.process(ast).await?;
        }
        Ok(())
    }
    
    async fn execute_parallel(&self, ast: &mut CSSNode) -> Result<(), PluginError> {
        let tasks: Vec<_> = self.plugins.iter().map(|plugin| {
            let ast_clone = ast.clone();
            async move { plugin.process(&mut ast_clone).await }
        }).collect();
        
        let results = futures::future::join_all(tasks).await;
        
        for result in results {
            result?;
        }
        
        Ok(())
    }
}
```

### 3. Plugin Configuration Management

```rust
// src/plugins/config.rs
use std::collections::HashMap;
use serde_json::Value;

pub struct PluginConfigManager {
    configs: HashMap<String, Value>,
    defaults: HashMap<String, Value>,
    validators: HashMap<String, Box<dyn ConfigValidator>>,
}

impl PluginConfigManager {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            defaults: HashMap::new(),
            validators: HashMap::new(),
        }
    }
    
    pub fn set_config(&mut self, plugin_name: &str, config: Value) -> Result<(), ConfigError> {
        if let Some(validator) = self.validators.get(plugin_name) {
            validator.validate(&config)?;
        }
        
        self.configs.insert(plugin_name.to_string(), config);
        Ok(())
    }
    
    pub fn get_config(&self, plugin_name: &str) -> Option<&Value> {
        self.configs.get(plugin_name)
            .or_else(|| self.defaults.get(plugin_name))
    }
    
    pub fn merge_configs(&self, plugin_name: &str) -> Result<Value, ConfigError> {
        let mut merged = self.defaults.get(plugin_name)
            .cloned()
            .unwrap_or(Value::Object(serde_json::Map::new()));
            
        if let Some(user_config) = self.configs.get(plugin_name) {
            self.deep_merge(&mut merged, user_config)?;
        }
        
        Ok(merged)
    }
}
```

## Plugin Testing Framework

### 1. Plugin Unit Tests

```rust
// src/plugins/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_ast::*;
    
    #[tokio::test]
    async fn test_autoprefixer_plugin() {
        let config = AutoprefixerConfig::default();
        let plugin = AutoprefixerPlugin::new(config).unwrap();
        
        let mut ast = CSSNode::parse("display: flex;").unwrap();
        plugin.process(&mut ast).await.unwrap();
        
        // Verify vendor prefixes were added
        assert!(ast.contains_declaration("-webkit-display"));
        assert!(ast.contains_declaration("-ms-display"));
    }
    
    #[tokio::test]
    async fn test_cssnano_plugin() {
        let config = CSSNanoConfig::default();
        let plugin = CSSNanoPlugin::new(config).unwrap();
        
        let mut ast = CSSNode::parse("body { color: red; /* comment */ }").unwrap();
        plugin.process(&mut ast).await.unwrap();
        
        // Verify comments were removed
        assert!(!ast.contains_comment());
    }
}
```

### 2. Plugin Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_plugin_chain() {
        let mut registry = PluginRegistry::new();
        
        let autoprefixer = Box::new(AutoprefixerPlugin::new(AutoprefixerConfig::default()).unwrap());
        let cssnano = Box::new(CSSNanoPlugin::new(CSSNanoConfig::default()).unwrap());
        
        registry.register_plugin(autoprefixer).await.unwrap();
        registry.register_plugin(cssnano).await.unwrap();
        
        let css = "body { display: flex; }";
        let mut ast = CSSNode::parse(css).unwrap();
        
        registry.execute_plugins(&mut ast).await.unwrap();
        
        // Verify both plugins were applied
        assert!(ast.contains_declaration("-webkit-display"));
        assert!(!ast.contains_comment());
    }
}
```

## Performance Optimization

### 1. Plugin Caching

```rust
// src/plugins/cache.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PluginCache {
    cache: Arc<RwLock<HashMap<String, CachedResult>>>,
    ttl: std::time::Duration,
}

impl PluginCache {
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

### 2. Parallel Plugin Execution

```rust
// src/plugins/parallel.rs
use rayon::prelude::*;

pub struct ParallelPluginExecutor {
    plugins: Vec<Box<dyn Plugin>>,
    max_workers: usize,
}

impl ParallelPluginExecutor {
    pub async fn execute(&self, ast: &mut CSSNode) -> Result<(), PluginError> {
        let tasks: Vec<_> = self.plugins.iter().map(|plugin| {
            let ast_clone = ast.clone();
            async move { plugin.process(&mut ast_clone).await }
        }).collect();
        
        let results = futures::future::join_all(tasks).await;
        
        for result in results {
            result?;
        }
        
        Ok(())
    }
}
```

This comprehensive plugin ecosystem provides the foundation for a fully functional PostCSS implementation with real CSS processing capabilities.
