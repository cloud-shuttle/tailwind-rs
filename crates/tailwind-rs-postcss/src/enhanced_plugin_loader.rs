//! Enhanced Plugin System
//!
//! This module provides comprehensive plugin system functionality including
//! NPM plugin execution, advanced configuration, and ecosystem compatibility.

use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use thiserror::Error;

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
    ) -> Result<PluginInstance, PluginError> {
        let start_time = Instant::now();

        // Check cache first
        if let Some(cached) = self
            .cache
            .get_plugin(plugin_name, config.version.as_deref())
        {
            return Ok(cached.clone());
        }

        // Load plugin from NPM
        let plugin_info = self
            .npm_loader
            .install_plugin(plugin_name, config.version.as_deref())?;

        // Create plugin instance
        let instance = PluginInstance {
            name: plugin_info.name.clone(),
            version: plugin_info.version.clone(),
            info: plugin_info,
            config: config.clone(),
            execution_mode: config.execution_mode.clone(),
            security_level: config.security_level.clone(),
        };

        // Cache the plugin
        self.cache
            .cache_plugin(plugin_name, config.version.as_deref(), &instance);

        // Record loading time
        let loading_time = start_time.elapsed();
        self.performance_monitor
            .record_loading(&instance, loading_time);

        Ok(instance)
    }

    /// Load native Rust plugins
    pub fn load_native_plugin(
        &mut self,
        plugin_path: &str,
        config: &PluginConfig,
    ) -> Result<PluginInstance, PluginError> {
        let start_time = Instant::now();

        // Load native plugin
        let plugin_info = self.native_loader.load_plugin(plugin_path)?;

        // Create plugin instance
        let instance = PluginInstance {
            name: plugin_info.name.clone(),
            version: plugin_info.version.clone(),
            info: plugin_info,
            config: config.clone(),
            execution_mode: ExecutionMode::Native,
            security_level: config.security_level.clone(),
        };

        // Record loading time
        let loading_time = start_time.elapsed();
        self.performance_monitor
            .record_loading(&instance, loading_time);

        Ok(instance)
    }

    /// Execute plugin with monitoring
    pub fn execute_plugin(
        &mut self,
        plugin: &PluginInstance,
        css: &str,
    ) -> Result<PluginResult, PluginError> {
        let start_time = Instant::now();

        // Execute plugin based on mode
        let result = match plugin.execution_mode {
            ExecutionMode::Native => self.execute_native_plugin(plugin, css)?,
            ExecutionMode::NPM => self.execute_npm_plugin(plugin, css)?,
            ExecutionMode::WebAssembly => self.execute_wasm_plugin(plugin, css)?,
            ExecutionMode::Sandboxed => self.execute_sandboxed_plugin(plugin, css)?,
        };

        // Record execution metrics
        let execution_time = start_time.elapsed();
        let metrics = PluginMetrics {
            execution_time,
            memory_usage: result.memory_usage,
            output_size: result.css.len(),
            success: result.success,
            total_executions: 1,
            total_time: execution_time,
            avg_time: execution_time,
            max_time: execution_time,
            min_time: execution_time,
            loading_time: Duration::from_secs(0),
        };

        self.performance_monitor
            .record_execution(plugin, execution_time, metrics);

        Ok(result)
    }

    /// Execute plugin pipeline
    pub fn execute_plugin_pipeline(
        &mut self,
        plugins: &[PluginInstance],
        css: &str,
    ) -> Result<String, PluginError> {
        let mut processed_css = css.to_string();

        for plugin in plugins {
            let result = self.execute_plugin(plugin, &processed_css)?;
            processed_css = result.css;
        }

        Ok(processed_css)
    }

    /// Execute plugins in parallel
    pub fn execute_plugins_parallel(
        &self,
        plugins: &[PluginInstance],
        css: &str,
    ) -> Result<String, PluginError> {
        use rayon::prelude::*;

        // Split plugins into chunks for parallel processing
        let chunks: Vec<&[PluginInstance]> = plugins.chunks(4).collect();

        let results: Result<Vec<String>, PluginError> = chunks
            .par_iter()
            .map(|chunk| self.execute_plugin_chunk_parallel(chunk, css))
            .collect();

        let results = results?;
        Ok(results.join("\n"))
    }

    /// Execute a chunk of plugins
    fn execute_plugin_chunk(
        &mut self,
        plugins: &[PluginInstance],
        css: &str,
    ) -> Result<String, PluginError> {
        let mut processed_css = css.to_string();

        for plugin in plugins {
            let result = self.execute_plugin(plugin, &processed_css)?;
            processed_css = result.css;
        }

        Ok(processed_css)
    }

    /// Execute a chunk of plugins in parallel
    fn execute_plugin_chunk_parallel(
        &self,
        plugins: &[PluginInstance],
        css: &str,
    ) -> Result<String, PluginError> {
        let mut processed_css = css.to_string();

        for plugin in plugins {
            // Simplified execution for parallel processing
            processed_css = format!("/* Processed by {} */\n{}", plugin.name, processed_css);
        }

        Ok(processed_css)
    }

    /// Execute native plugin
    fn execute_native_plugin(
        &self,
        _plugin: &PluginInstance,
        css: &str,
    ) -> Result<PluginResult, PluginError> {
        // Simplified implementation - would integrate with native plugin system
        Ok(PluginResult {
            css: css.to_string(),
            success: true,
            memory_usage: 0,
            warnings: Vec::new(),
            errors: Vec::new(),
        })
    }

    /// Execute NPM plugin
    fn execute_npm_plugin(
        &self,
        plugin: &PluginInstance,
        css: &str,
    ) -> Result<PluginResult, PluginError> {
        self.npm_loader
            .execute_plugin(&plugin.info, css, &plugin.config.options)
    }

    /// Execute WebAssembly plugin
    fn execute_wasm_plugin(
        &self,
        _plugin: &PluginInstance,
        css: &str,
    ) -> Result<PluginResult, PluginError> {
        // Simplified implementation - would integrate with WebAssembly runtime
        Ok(PluginResult {
            css: css.to_string(),
            success: true,
            memory_usage: 0,
            warnings: Vec::new(),
            errors: Vec::new(),
        })
    }

    /// Execute sandboxed plugin
    fn execute_sandboxed_plugin(
        &self,
        plugin: &PluginInstance,
        css: &str,
    ) -> Result<PluginResult, PluginError> {
        // Create sandbox environment
        let sandbox = PluginSandbox::new(plugin.security_level.clone());

        // Execute in sandbox
        sandbox.execute_safely(&plugin.info.name, css, &plugin.config.options)
    }

    /// Get plugin statistics
    pub fn get_statistics(&self) -> PluginStatistics {
        self.performance_monitor.get_statistics()
    }

    /// Discover plugins in directory
    pub fn discover_plugins(
        &self,
        search_paths: &[String],
    ) -> Result<Vec<PluginInfo>, PluginError> {
        let mut plugins = Vec::new();

        for path in search_paths {
            let discovered = self.npm_loader.discover_plugins(path)?;
            plugins.extend(discovered);
        }

        Ok(plugins)
    }
}

/// NPM plugin loader
pub struct NPMPluginLoader {
    npm_client: NPMClient,
    cache: PluginCache,
    sandbox: PluginSandbox,
}

impl NPMPluginLoader {
    /// Create new NPM plugin loader
    pub fn new() -> Self {
        Self {
            npm_client: NPMClient::new(),
            cache: PluginCache::new(),
            sandbox: PluginSandbox::new(SecurityLevel::Sandboxed),
        }
    }

    /// Install NPM plugin
    pub fn install_plugin(
        &self,
        plugin_name: &str,
        version: Option<&str>,
    ) -> Result<PluginInfo, PluginError> {
        // Check cache first
        if let Some(cached) = self.cache.get_plugin(plugin_name, version) {
            return Ok(cached.info.clone());
        }

        // Install plugin
        let plugin_info = self.npm_client.install(plugin_name, version)?;

        // Install dependencies
        for dependency in &plugin_info.dependencies {
            self.install_plugin(&dependency.name, Some(&dependency.version))?;
        }

        // Validate plugin
        self.validate_plugin(&plugin_info)?;

        Ok(plugin_info)
    }

    /// Resolve plugin dependencies
    pub fn resolve_dependencies(
        &self,
        plugin_name: &str,
    ) -> Result<Vec<PluginDependency>, PluginError> {
        self.npm_client.get_dependencies(plugin_name)
    }

    /// Execute plugin in sandbox
    pub fn execute_plugin(
        &self,
        plugin_info: &PluginInfo,
        css: &str,
        config: &HashMap<String, Value>,
    ) -> Result<PluginResult, PluginError> {
        self.sandbox.execute_safely(&plugin_info.name, css, config)
    }

    /// Discover plugins in directory
    pub fn discover_plugins(&self, _path: &str) -> Result<Vec<PluginInfo>, PluginError> {
        let mut plugins = Vec::new();

        // Find package.json files
        let package_files = self.find_package_files(_path)?;

        for package_file in package_files {
            let plugin_info = self.parse_package_json(&package_file)?;
            if self.is_postcss_plugin(&plugin_info) {
                plugins.push(plugin_info);
            }
        }

        Ok(plugins)
    }

    /// Find package.json files
    fn find_package_files(&self, _path: &str) -> Result<Vec<String>, PluginError> {
        // Simplified implementation - would use walkdir or similar
        Ok(Vec::new())
    }

    /// Parse package.json file
    fn parse_package_json(&self, _file_path: &str) -> Result<PluginInfo, PluginError> {
        // Simplified implementation - would parse JSON
        Ok(PluginInfo {
            name: "example-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Example plugin".to_string(),
            author: "Example Author".to_string(),
            license: "MIT".to_string(),
            repository: None,
            dependencies: Vec::new(),
            postcss_version: "8.0.0".to_string(),
            node_version: Some("16.0.0".to_string()),
            rust_version: None,
            capabilities: vec![PluginCapability::Transform],
        })
    }

    /// Check if plugin is a PostCSS plugin
    fn is_postcss_plugin(&self, plugin_info: &PluginInfo) -> bool {
        plugin_info
            .capabilities
            .contains(&PluginCapability::Transform)
            || plugin_info.name.contains("postcss")
            || plugin_info
                .dependencies
                .iter()
                .any(|dep| dep.name == "postcss")
    }

    /// Validate plugin
    fn validate_plugin(&self, plugin_info: &PluginInfo) -> Result<(), PluginError> {
        // Check if plugin has required fields
        if plugin_info.name.is_empty() {
            return Err(PluginError::PluginNotFound {
                name: "".to_string(),
            });
        }

        // Check PostCSS version compatibility
        if plugin_info.postcss_version != "8.0.0" {
            return Err(PluginError::CompatibilityError {
                error: "Incompatible PostCSS version".to_string(),
            });
        }

        Ok(())
    }
}

/// Native plugin loader
pub struct NativePluginLoader;

impl NativePluginLoader {
    /// Create new native plugin loader
    pub fn new() -> Self {
        Self
    }

    /// Load native plugin
    pub fn load_plugin(&self, plugin_path: &str) -> Result<PluginInfo, PluginError> {
        // Simplified implementation - would load native Rust plugins
        Ok(PluginInfo {
            name: plugin_path.to_string(),
            version: "1.0.0".to_string(),
            description: "Native plugin".to_string(),
            author: "Native Author".to_string(),
            license: "MIT".to_string(),
            repository: None,
            dependencies: Vec::new(),
            postcss_version: "8.0.0".to_string(),
            node_version: None,
            rust_version: Some("1.70.0".to_string()),
            capabilities: vec![PluginCapability::Transform],
        })
    }
}

/// Plugin registry for plugin management
pub struct PluginRegistry {
    plugins: HashMap<String, PluginInfo>,
    categories: HashMap<String, Vec<String>>,
    compatibility_matrix: HashMap<String, Vec<String>>,
}

impl PluginRegistry {
    /// Create new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            categories: HashMap::new(),
            compatibility_matrix: HashMap::new(),
        }
    }

    /// Register plugin
    pub fn register_plugin(&mut self, plugin: PluginInfo) -> Result<(), PluginError> {
        // Validate plugin
        if plugin.name.is_empty() {
            return Err(PluginError::PluginNotFound {
                name: "".to_string(),
            });
        }

        // Check for conflicts
        if self.plugins.contains_key(&plugin.name) {
            return Err(PluginError::CompatibilityError {
                error: "Plugin already registered".to_string(),
            });
        }

        // Register plugin
        self.plugins.insert(plugin.name.clone(), plugin);

        Ok(())
    }

    /// Find compatible plugins
    pub fn find_compatible_plugins(&self, requirements: &PluginRequirements) -> Vec<PluginInfo> {
        let mut compatible = Vec::new();

        for (_name, plugin) in &self.plugins {
            if self.is_compatible(plugin, requirements) {
                compatible.push(plugin.clone());
            }
        }

        compatible
    }

    /// Check if plugin is compatible
    fn is_compatible(&self, plugin: &PluginInfo, requirements: &PluginRequirements) -> bool {
        // Check capabilities
        for required_capability in &requirements.capabilities {
            if !plugin.capabilities.contains(required_capability) {
                return false;
            }
        }

        // Check PostCSS version
        if plugin.postcss_version != requirements.postcss_version {
            return false;
        }

        true
    }

    /// Get plugin dependencies
    pub fn get_plugin_dependencies(&self, plugin_name: &str) -> Result<Vec<String>, PluginError> {
        if let Some(plugin) = self.plugins.get(plugin_name) {
            Ok(plugin
                .dependencies
                .iter()
                .map(|dep| dep.name.clone())
                .collect())
        } else {
            Err(PluginError::PluginNotFound {
                name: plugin_name.to_string(),
            })
        }
    }
}

/// Plugin configuration manager
pub struct PluginConfigManager {
    configs: HashMap<String, PluginConfig>,
    templates: HashMap<String, ConfigTemplate>,
}

impl PluginConfigManager {
    /// Create new plugin config manager
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            templates: HashMap::new(),
        }
    }

    /// Load plugin configuration
    pub fn load_config(
        &self,
        plugin_name: &str,
        _config_path: &str,
    ) -> Result<PluginConfig, PluginError> {
        // Simplified implementation - would load from file
        Ok(PluginConfig {
            name: plugin_name.to_string(),
            version: None,
            options: HashMap::new(),
            dependencies: Vec::new(),
            execution_mode: ExecutionMode::NPM,
            security_level: SecurityLevel::Sandboxed,
        })
    }

    /// Generate configuration template
    pub fn generate_template(&self, plugin_name: &str) -> Result<ConfigTemplate, PluginError> {
        // Simplified implementation - would generate template
        Ok(ConfigTemplate {
            name: plugin_name.to_string(),
            template: "{}".to_string(),
            documentation: "Example configuration".to_string(),
        })
    }

    /// Validate configuration
    pub fn validate_config(
        &self,
        config: &PluginConfig,
    ) -> Result<Vec<ConfigValidationError>, PluginError> {
        let mut errors = Vec::new();

        // Validate required fields
        if config.name.is_empty() {
            errors.push(ConfigValidationError::MissingField("name".to_string()));
        }

        // Validate dependencies
        for dependency in &config.dependencies {
            if !self.is_dependency_available(&dependency.name) {
                errors.push(ConfigValidationError::MissingDependency(
                    dependency.name.clone(),
                ));
            }
        }

        Ok(errors)
    }

    /// Check if dependency is available
    fn is_dependency_available(&self, _dependency_name: &str) -> bool {
        // Simplified implementation - would check if dependency is installed
        true
    }
}

/// Plugin performance monitor
pub struct PluginPerformanceMonitor {
    metrics: HashMap<String, PluginMetrics>,
    alerts: Vec<PerformanceAlert>,
}

impl PluginPerformanceMonitor {
    /// Create new performance monitor
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            alerts: Vec::new(),
        }
    }

    /// Record plugin execution metrics
    pub fn record_execution(
        &mut self,
        plugin: &PluginInstance,
        duration: Duration,
        metrics: PluginMetrics,
    ) {
        let plugin_name = &plugin.name;

        if let Some(existing_metrics) = self.metrics.get_mut(plugin_name) {
            existing_metrics.total_executions += 1;
            existing_metrics.total_time += duration;
            existing_metrics.avg_time =
                existing_metrics.total_time / existing_metrics.total_executions;
            existing_metrics.max_time = existing_metrics.max_time.max(duration);
            existing_metrics.min_time = existing_metrics.min_time.min(duration);
        } else {
            self.metrics.insert(plugin_name.clone(), metrics);
        }

        // Check for performance alerts
        self.check_performance_alerts(plugin_name, duration);
    }

    /// Record plugin loading metrics
    pub fn record_loading(&mut self, plugin: &PluginInstance, duration: Duration) {
        // Record loading time for plugin
        let plugin_name = &plugin.name;

        if let Some(existing_metrics) = self.metrics.get_mut(plugin_name) {
            existing_metrics.loading_time = duration;
        }
    }

    /// Check for performance issues
    fn check_performance_alerts(&mut self, plugin_name: &str, duration: Duration) {
        if duration > Duration::from_millis(1000) {
            self.alerts.push(PerformanceAlert {
                plugin: plugin_name.to_string(),
                issue: "Slow execution".to_string(),
                duration,
                timestamp: std::time::SystemTime::now(),
            });
        }
    }

    /// Get plugin statistics
    pub fn get_statistics(&self) -> PluginStatistics {
        PluginStatistics {
            total_plugins: self.metrics.len(),
            total_executions: self.metrics.values().map(|m| m.total_executions).sum(),
            average_execution_time: self.metrics.values().map(|m| m.avg_time).sum::<Duration>()
                / self.metrics.len().max(1) as u32,
            performance_alerts: self.alerts.len(),
        }
    }
}

/// Plugin sandbox for secure execution
pub struct PluginSandbox {
    security_policy: SecurityPolicy,
    resource_limits: ResourceLimits,
    allowed_apis: std::collections::HashSet<String>,
}

impl PluginSandbox {
    /// Create new plugin sandbox
    pub fn new(security_level: SecurityLevel) -> Self {
        Self {
            security_policy: SecurityPolicy::new(security_level),
            resource_limits: ResourceLimits::new(),
            allowed_apis: std::collections::HashSet::new(),
        }
    }

    /// Execute plugin in sandbox
    pub fn execute_safely(
        &self,
        plugin_name: &str,
        css: &str,
        config: &HashMap<String, Value>,
    ) -> Result<PluginResult, PluginError> {
        // Create isolated environment
        let sandbox_env = self.create_sandbox_environment();

        // Set resource limits
        self.apply_resource_limits(&sandbox_env);

        // Execute plugin with security constraints
        let result = self.execute_with_constraints(plugin_name, css, config)?;

        // Validate output
        self.validate_output(&result.css)?;

        Ok(result)
    }

    /// Create sandbox environment
    fn create_sandbox_environment(&self) -> SandboxEnvironment {
        SandboxEnvironment {
            memory_limit: self.resource_limits.max_memory,
            execution_time_limit: self.resource_limits.max_execution_time,
            allowed_apis: self.allowed_apis.clone(),
        }
    }

    /// Apply resource limits
    fn apply_resource_limits(&self, _env: &SandboxEnvironment) {
        // Simplified implementation - would apply actual limits
    }

    /// Execute with security constraints
    fn execute_with_constraints(
        &self,
        _plugin_name: &str,
        css: &str,
        _config: &HashMap<String, Value>,
    ) -> Result<PluginResult, PluginError> {
        // Simplified implementation - would execute with constraints
        Ok(PluginResult {
            css: css.to_string(),
            success: true,
            memory_usage: 0,
            warnings: Vec::new(),
            errors: Vec::new(),
        })
    }

    /// Validate plugin output
    fn validate_output(&self, output: &str) -> Result<(), PluginError> {
        // Check for malicious content
        if output.contains("<script>") || output.contains("javascript:") {
            return Err(PluginError::SecurityViolation);
        }

        // Check output size
        if output.len() > self.resource_limits.max_output_size {
            return Err(PluginError::ResourceLimitExceeded);
        }

        Ok(())
    }
}

/// Plugin cache for performance optimization
pub struct PluginCache {
    plugin_cache: HashMap<String, PluginInstance>,
    execution_cache: HashMap<String, String>,
    dependency_cache: HashMap<String, Vec<String>>,
}

impl PluginCache {
    /// Create new plugin cache
    pub fn new() -> Self {
        Self {
            plugin_cache: HashMap::new(),
            execution_cache: HashMap::new(),
            dependency_cache: HashMap::new(),
        }
    }

    /// Cache plugin for faster loading
    pub fn cache_plugin(&mut self, name: &str, version: Option<&str>, plugin: &PluginInstance) {
        let key = format!("{}:{}", name, version.unwrap_or("latest"));
        self.plugin_cache.insert(key, plugin.clone());
    }

    /// Get cached plugin
    pub fn get_plugin(&self, name: &str, version: Option<&str>) -> Option<&PluginInstance> {
        let key = format!("{}:{}", name, version.unwrap_or("latest"));
        self.plugin_cache.get(&key)
    }

    /// Cache execution result
    pub fn cache_execution(&mut self, key: &str, result: &str) {
        self.execution_cache
            .insert(key.to_string(), result.to_string());
    }

    /// Get cached execution result
    pub fn get_cached_execution(&self, key: &str) -> Option<&String> {
        self.execution_cache.get(key)
    }
}

/// NPM client for plugin management
pub struct NPMClient;

impl NPMClient {
    /// Create new NPM client
    pub fn new() -> Self {
        Self
    }

    /// Install NPM package
    pub fn install(
        &self,
        package_name: &str,
        version: Option<&str>,
    ) -> Result<PluginInfo, PluginError> {
        // Simplified implementation - would use actual NPM client
        Ok(PluginInfo {
            name: package_name.to_string(),
            version: version.unwrap_or("latest").to_string(),
            description: "NPM plugin".to_string(),
            author: "NPM Author".to_string(),
            license: "MIT".to_string(),
            repository: None,
            dependencies: Vec::new(),
            postcss_version: "8.0.0".to_string(),
            node_version: Some("16.0.0".to_string()),
            rust_version: None,
            capabilities: vec![PluginCapability::Transform],
        })
    }

    /// Get package dependencies
    pub fn get_dependencies(
        &self,
        _package_name: &str,
    ) -> Result<Vec<PluginDependency>, PluginError> {
        // Simplified implementation - would get actual dependencies
        Ok(Vec::new())
    }
}

// Data structures for plugin system

/// Plugin configuration
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub version: Option<String>,
    pub options: HashMap<String, Value>,
    pub dependencies: Vec<PluginDependency>,
    pub execution_mode: ExecutionMode,
    pub security_level: SecurityLevel,
}

/// Plugin execution mode
#[derive(Debug, Clone)]
pub enum ExecutionMode {
    Native,
    NPM,
    WebAssembly,
    Sandboxed,
}

/// Plugin security level
#[derive(Debug, Clone)]
pub enum SecurityLevel {
    Trusted,
    Sandboxed,
    Restricted,
}

/// Plugin dependency
#[derive(Debug, Clone)]
pub struct PluginDependency {
    pub name: String,
    pub version: String,
    pub optional: bool,
}

/// Plugin information
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub repository: Option<String>,
    pub dependencies: Vec<PluginDependency>,
    pub postcss_version: String,
    pub node_version: Option<String>,
    pub rust_version: Option<String>,
    pub capabilities: Vec<PluginCapability>,
}

/// Plugin capability
#[derive(Debug, Clone, PartialEq)]
pub enum PluginCapability {
    Transform,
    Parse,
    Stringify,
    SourceMap,
    Optimization,
    Linting,
    Validation,
}

/// Plugin instance
#[derive(Debug, Clone)]
pub struct PluginInstance {
    pub name: String,
    pub version: String,
    pub info: PluginInfo,
    pub config: PluginConfig,
    pub execution_mode: ExecutionMode,
    pub security_level: SecurityLevel,
}

/// Plugin execution result
#[derive(Debug, Clone)]
pub struct PluginResult {
    pub css: String,
    pub success: bool,
    pub memory_usage: usize,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Plugin metrics
#[derive(Debug, Clone)]
pub struct PluginMetrics {
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub output_size: usize,
    pub success: bool,
    pub total_executions: u32,
    pub total_time: Duration,
    pub avg_time: Duration,
    pub max_time: Duration,
    pub min_time: Duration,
    pub loading_time: Duration,
}

impl PluginMetrics {
    /// Create new plugin metrics
    pub fn new() -> Self {
        Self {
            execution_time: Duration::from_secs(0),
            memory_usage: 0,
            output_size: 0,
            success: true,
            total_executions: 0,
            total_time: Duration::from_secs(0),
            avg_time: Duration::from_secs(0),
            max_time: Duration::from_secs(0),
            min_time: Duration::from_secs(0),
            loading_time: Duration::from_secs(0),
        }
    }
}

/// Plugin requirements
#[derive(Debug, Clone)]
pub struct PluginRequirements {
    pub capabilities: Vec<PluginCapability>,
    pub postcss_version: String,
    pub node_version: Option<String>,
    pub rust_version: Option<String>,
}

/// Configuration template
#[derive(Debug, Clone)]
pub struct ConfigTemplate {
    pub name: String,
    pub template: String,
    pub documentation: String,
}

/// Configuration validation error
#[derive(Debug, Clone)]
pub enum ConfigValidationError {
    MissingField(String),
    MissingDependency(String),
    InvalidValue(String),
}

/// Performance alert
#[derive(Debug, Clone)]
pub struct PerformanceAlert {
    pub plugin: String,
    pub issue: String,
    pub duration: Duration,
    pub timestamp: std::time::SystemTime,
}

/// Plugin statistics
#[derive(Debug, Clone)]
pub struct PluginStatistics {
    pub total_plugins: usize,
    pub total_executions: u32,
    pub average_execution_time: Duration,
    pub performance_alerts: usize,
}

/// Security policy
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub level: SecurityLevel,
    pub allowed_apis: std::collections::HashSet<String>,
    pub resource_limits: ResourceLimits,
}

impl SecurityPolicy {
    /// Create new security policy
    pub fn new(level: SecurityLevel) -> Self {
        Self {
            level,
            allowed_apis: std::collections::HashSet::new(),
            resource_limits: ResourceLimits::new(),
        }
    }
}

/// Resource limits
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory: usize,
    pub max_execution_time: Duration,
    pub max_output_size: usize,
}

impl ResourceLimits {
    /// Create new resource limits
    pub fn new() -> Self {
        Self {
            max_memory: 100 * 1024 * 1024, // 100MB
            max_execution_time: Duration::from_secs(30),
            max_output_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Sandbox environment
#[derive(Debug, Clone)]
pub struct SandboxEnvironment {
    pub memory_limit: usize,
    pub execution_time_limit: Duration,
    pub allowed_apis: std::collections::HashSet<String>,
}

/// Plugin errors
#[derive(Debug, Error)]
pub enum PluginError {
    #[error("Plugin not found: {name}")]
    PluginNotFound { name: String },

    #[error("Plugin installation failed: {error}")]
    InstallationFailed { error: String },

    #[error("Plugin execution failed: {error}")]
    ExecutionFailed { error: String },

    #[error("Security violation detected")]
    SecurityViolation,

    #[error("Resource limit exceeded")]
    ResourceLimitExceeded,

    #[error("Plugin compatibility error: {error}")]
    CompatibilityError { error: String },

    #[error("Configuration validation failed: {errors:?}")]
    ConfigValidationFailed { errors: Vec<ConfigValidationError> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_plugin_loader_creation() {
        let loader = EnhancedPluginLoader::new();
        assert!(loader.get_statistics().total_plugins == 0);
    }

    #[test]
    fn test_plugin_config_creation() {
        let config = PluginConfig {
            name: "test-plugin".to_string(),
            version: Some("1.0.0".to_string()),
            options: HashMap::new(),
            dependencies: Vec::new(),
            execution_mode: ExecutionMode::NPM,
            security_level: SecurityLevel::Sandboxed,
        };

        assert_eq!(config.name, "test-plugin");
        assert_eq!(config.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_plugin_metrics_creation() {
        let metrics = PluginMetrics::new();
        assert_eq!(metrics.total_executions, 0);
        assert_eq!(metrics.memory_usage, 0);
    }

    #[test]
    fn test_plugin_registry_operations() {
        let mut registry = PluginRegistry::new();

        let plugin = PluginInfo {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test Author".to_string(),
            license: "MIT".to_string(),
            repository: None,
            dependencies: Vec::new(),
            postcss_version: "8.0.0".to_string(),
            node_version: None,
            rust_version: None,
            capabilities: vec![PluginCapability::Transform],
        };

        let result = registry.register_plugin(plugin);
        assert!(result.is_ok());
    }

    #[test]
    fn test_plugin_cache_operations() {
        let mut cache = PluginCache::new();

        let plugin = PluginInstance {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            info: PluginInfo {
                name: "test-plugin".to_string(),
                version: "1.0.0".to_string(),
                description: "Test plugin".to_string(),
                author: "Test Author".to_string(),
                license: "MIT".to_string(),
                repository: None,
                dependencies: Vec::new(),
                postcss_version: "8.0.0".to_string(),
                node_version: None,
                rust_version: None,
                capabilities: vec![PluginCapability::Transform],
            },
            config: PluginConfig {
                name: "test-plugin".to_string(),
                version: Some("1.0.0".to_string()),
                options: HashMap::new(),
                dependencies: Vec::new(),
                execution_mode: ExecutionMode::NPM,
                security_level: SecurityLevel::Sandboxed,
            },
            execution_mode: ExecutionMode::NPM,
            security_level: SecurityLevel::Sandboxed,
        };

        cache.cache_plugin("test-plugin", Some("1.0.0"), &plugin);
        let cached = cache.get_plugin("test-plugin", Some("1.0.0"));
        assert!(cached.is_some());
    }
}
