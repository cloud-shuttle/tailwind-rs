//! Core Enhanced Plugin Loader functionality
//!
//! This module contains the main EnhancedPluginLoader struct and its core methods.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::error::PostCSSError;
use crate::enhanced_plugin_loader::{
    npm_loader::NPMPluginLoader, native_loader::NativePluginLoader, plugin_registry::PluginRegistry,
    config_manager::PluginConfigManager, plugin_cache::PluginCache
};

/// Enhanced plugin loader with NPM and native plugin support
pub struct EnhancedPluginLoader {
    npm_loader: NPMPluginLoader,
    native_loader: NativePluginLoader,
    plugin_registry: PluginRegistry,
    config_manager: PluginConfigManager,
    performance_monitor: PluginPerformanceMonitor,
    cache: PluginCache,
}

impl EnhancedPluginLoader {
    /// Create a new enhanced plugin loader
    pub fn new() -> Self {
        Self {
            npm_loader: NPMPluginLoader::new(),
            native_loader: NativePluginLoader::new(),
            plugin_registry: PluginRegistry::new(),
            config_manager: PluginConfigManager::new(),
            performance_monitor: PluginPerformanceMonitor::new(),
            cache: PluginCache::new(),
        }
    }

    /// Load and execute NPM plugins
    pub fn load_npm_plugin(
        &mut self,
        plugin_name: &str,
        config: &PluginConfig,
    ) -> Result<PluginInstance, PostCSSError> {
        let start_time = Instant::now();

        // Check cache first
        if let Some(cached) = self
            .cache
            .get_plugin(plugin_name, config.version.as_deref())
        {
            return Ok(cached.clone());
        }

        // Load plugin from NPM
        let plugin = self.npm_loader.load_plugin(plugin_name, config)?;
        
        // Cache the plugin
        self.cache.cache_plugin(plugin_name.to_string(), config.version.clone(), plugin.clone());
        
        // Monitor performance
        let load_time = start_time.elapsed();
        self.performance_monitor.record_load_time(plugin_name, load_time);

        Ok(plugin)
    }

    /// Load and execute native plugins
    pub fn load_native_plugin(
        &mut self,
        plugin_name: &str,
        config: &PluginConfig,
    ) -> Result<PluginInstance, PostCSSError> {
        let start_time = Instant::now();

        // Check cache first
        if let Some(cached) = self
            .cache
            .get_plugin(plugin_name, config.version.as_deref())
        {
            return Ok(cached.clone());
        }

        // Load plugin from native loader
        let plugin = self.native_loader.load_plugin(plugin_name, config)?;
        
        // Cache the plugin
        self.cache.cache_plugin(plugin_name.to_string(), config.version.clone(), plugin.clone());
        
        // Monitor performance
        let load_time = start_time.elapsed();
        self.performance_monitor.record_load_time(plugin_name, load_time);

        Ok(plugin)
    }

    /// Execute plugin with given input
    pub fn execute_plugin(
        &mut self,
        plugin: &PluginInstance,
        input: &str,
        options: &PluginOptions,
    ) -> Result<String, PostCSSError> {
        let start_time = Instant::now();

        // Execute plugin
        let result = plugin.execute(input, options)?;
        
        // Monitor performance
        let execution_time = start_time.elapsed();
        self.performance_monitor.record_execution_time(plugin.name(), execution_time);

        Ok(result)
    }

    /// Get plugin registry
    pub fn registry(&self) -> &PluginRegistry {
        &self.plugin_registry
    }

    /// Get config manager
    pub fn config_manager(&self) -> &PluginConfigManager {
        &self.config_manager
    }

    /// Get performance monitor
    pub fn performance_monitor(&self) -> &PluginPerformanceMonitor {
        &self.performance_monitor
    }

    /// Get cache
    pub fn cache(&self) -> &PluginCache {
        &self.cache
    }
}

/// Plugin instance
#[derive(Clone)]
pub struct PluginInstance {
    name: String,
    version: String,
    plugin_type: PluginType,
    config: PluginConfig,
}

impl PluginInstance {
    pub fn new(name: String, version: String, plugin_type: PluginType, config: PluginConfig) -> Self {
        Self {
            name,
            version,
            plugin_type,
            config,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn plugin_type(&self) -> &PluginType {
        &self.plugin_type
    }

    pub fn execute(&self, input: &str, options: &PluginOptions) -> Result<String, PostCSSError> {
        match self.plugin_type {
            PluginType::NPM => {
                // Execute NPM plugin
                Ok(format!("NPM Plugin {} executed: {}", self.name, input))
            }
            PluginType::Native => {
                // Execute native plugin
                Ok(format!("Native Plugin {} executed: {}", self.name, input))
            }
        }
    }
}

/// Plugin type
#[derive(Debug, Clone)]
pub enum PluginType {
    NPM,
    Native,
}

/// Plugin configuration
#[derive(Clone)]
pub struct PluginConfig {
    pub version: Option<String>,
    pub options: HashMap<String, serde_json::Value>,
    pub dependencies: Vec<String>,
}

impl PluginConfig {
    pub fn new() -> Self {
        Self {
            version: None,
            options: HashMap::new(),
            dependencies: Vec::new(),
        }
    }
}

/// Plugin performance monitor
pub struct PluginPerformanceMonitor {
    pub metrics: HashMap<String, PluginMetrics>,
}

impl PluginPerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }

    pub fn record_load_time(&mut self, plugin_name: &str, duration: Duration) {
        let metrics = self.metrics.entry(plugin_name.to_string()).or_insert(PluginMetrics::new());
        metrics.load_time = duration;
    }

    pub fn record_execution_time(&mut self, plugin_name: &str, duration: Duration) {
        let metrics = self.metrics.entry(plugin_name.to_string()).or_insert(PluginMetrics::new());
        metrics.execution_time = duration;
    }
}

/// Plugin performance metrics
pub struct PluginMetrics {
    pub load_time: Duration,
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub error_count: usize,
}

impl PluginMetrics {
    pub fn new() -> Self {
        Self {
            load_time: Duration::new(0, 0),
            execution_time: Duration::new(0, 0),
            memory_usage: 0,
            error_count: 0,
        }
    }
}

/// Plugin options
pub struct PluginOptions {
    pub input_format: String,
    pub output_format: String,
    pub options: HashMap<String, serde_json::Value>,
}

impl PluginOptions {
    pub fn new() -> Self {
        Self {
            input_format: "css".to_string(),
            output_format: "css".to_string(),
            options: HashMap::new(),
        }
    }
}
