//! Advanced Plugin System Module
//!
//! Comprehensive plugin system for Tailwind-RS:
//! - Plugin types and priorities for different plugin categories
//! - Plugin configuration and composition strategies
//! - Plugin management and execution framework
//! - Extensible architecture for custom plugins

use crate::classes::ClassBuilder;
use std::collections::HashMap;

// Re-export all plugin system types and traits
pub mod types;

// Re-export all types for easy access
pub use types::*;

/// Plugin definition structure
#[derive(Debug, Clone)]
pub struct PluginDefinition {
    /// Plugin name
    pub name: String,
    /// Plugin type
    pub plugin_type: PluginType,
    /// Plugin priority
    pub priority: PluginPriority,
    /// Plugin configuration
    pub config: PluginConfig,
    /// Plugin composition strategy
    pub composition: PluginComposition,
    /// Plugin metadata
    pub metadata: HashMap<String, String>,
}

impl PluginDefinition {
    /// Create a new plugin definition
    pub fn new(name: String, plugin_type: PluginType) -> Self {
        Self {
            name,
            plugin_type,
            priority: PluginPriority::default(),
            config: PluginConfig::default(),
            composition: PluginComposition::default(),
            metadata: HashMap::new(),
        }
    }

    /// Set the plugin priority
    pub fn with_priority(mut self, priority: PluginPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set the plugin configuration
    pub fn with_config(mut self, config: PluginConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the plugin composition strategy
    pub fn with_composition(mut self, composition: PluginComposition) -> Self {
        self.composition = composition;
        self
    }

    /// Add metadata to the plugin
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if the plugin is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.is_enabled()
    }

    /// Get the plugin's display name
    pub fn display_name(&self) -> String {
        format!("{} ({})", self.name, self.plugin_type)
    }
}

/// Plugin manager for handling plugin lifecycle
#[derive(Debug, Clone)]
pub struct PluginManager {
    plugins: HashMap<String, PluginDefinition>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    /// Register a plugin
    pub fn register_plugin(&mut self, plugin: PluginDefinition) -> Result<(), String> {
        if self.plugins.contains_key(&plugin.name) {
            return Err(format!("Plugin '{}' already exists", plugin.name));
        }

        self.plugins.insert(plugin.name.clone(), plugin);
        Ok(())
    }

    /// Unregister a plugin
    pub fn unregister_plugin(&mut self, name: &str) -> Result<PluginDefinition, String> {
        self.plugins.remove(name)
            .ok_or_else(|| format!("Plugin '{}' not found", name))
    }

    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&PluginDefinition> {
        self.plugins.get(name)
    }

    /// Get all registered plugins
    pub fn get_all_plugins(&self) -> Vec<&PluginDefinition> {
        self.plugins.values().collect()
    }

    /// Get plugins by type
    pub fn get_plugins_by_type(&self, plugin_type: &PluginType) -> Vec<&PluginDefinition> {
        self.plugins.values()
            .filter(|plugin| plugin.plugin_type == *plugin_type)
            .collect()
    }

    /// Get enabled plugins
    pub fn get_enabled_plugins(&self) -> Vec<&PluginDefinition> {
        self.plugins.values()
            .filter(|plugin| plugin.is_enabled())
            .collect()
    }

    /// Get plugins sorted by priority (highest first)
    pub fn get_plugins_by_priority(&self) -> Vec<&PluginDefinition> {
        let mut plugins: Vec<&PluginDefinition> = self.plugins.values().collect();
        plugins.sort_by(|a, b| b.priority.numeric_value().cmp(&a.priority.numeric_value()));
        plugins
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, name: &str) -> Result<(), String> {
        let plugin = self.plugins.get_mut(name)
            .ok_or_else(|| format!("Plugin '{}' not found", name))?;

        plugin.config = PluginConfig::Enable;
        Ok(())
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, name: &str) -> Result<(), String> {
        let plugin = self.plugins.get_mut(name)
            .ok_or_else(|| format!("Plugin '{}' not found", name))?;

        plugin.config = PluginConfig::Disable;
        Ok(())
    }

    /// Update plugin configuration
    pub fn update_plugin_config(&mut self, name: &str, config: PluginConfig) -> Result<(), String> {
        let plugin = self.plugins.get_mut(name)
            .ok_or_else(|| format!("Plugin '{}' not found", name))?;

        plugin.config = config;
        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin execution context
#[derive(Debug, Clone)]
pub struct PluginExecutionContext {
    /// Current plugin being executed
    pub current_plugin: Option<String>,
    /// Execution order
    pub execution_order: Vec<String>,
    /// Execution results
    pub results: HashMap<String, PluginExecutionResult>,
}

impl PluginExecutionContext {
    /// Create a new execution context
    pub fn new() -> Self {
        Self {
            current_plugin: None,
            execution_order: Vec::new(),
            results: HashMap::new(),
        }
    }

    /// Set the current plugin
    pub fn set_current_plugin(&mut self, plugin_name: String) {
        self.current_plugin = Some(plugin_name.clone());
        self.execution_order.push(plugin_name);
    }

    /// Record execution result
    pub fn record_result(&mut self, plugin_name: String, result: PluginExecutionResult) {
        self.results.insert(plugin_name, result);
    }

    /// Get execution result for a plugin
    pub fn get_result(&self, plugin_name: &str) -> Option<&PluginExecutionResult> {
        self.results.get(plugin_name)
    }

    /// Check if execution was successful
    pub fn is_successful(&self) -> bool {
        self.results.values().all(|result| result.is_success())
    }
}

impl Default for PluginExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin execution result
#[derive(Debug, Clone)]
pub enum PluginExecutionResult {
    /// Successful execution
    Success(String),
    /// Failed execution with error message
    Failure(String),
    /// Skipped execution
    Skipped(String),
}

impl PluginExecutionResult {
    /// Check if the result indicates success
    pub fn is_success(&self) -> bool {
        matches!(self, PluginExecutionResult::Success(_))
    }

    /// Check if the result indicates failure
    pub fn is_failure(&self) -> bool {
        matches!(self, PluginExecutionResult::Failure(_))
    }

    /// Check if the result indicates skipping
    pub fn is_skipped(&self) -> bool {
        matches!(self, PluginExecutionResult::Skipped(_))
    }

    /// Get the result message
    pub fn message(&self) -> &str {
        match self {
            PluginExecutionResult::Success(msg) => msg,
            PluginExecutionResult::Failure(msg) => msg,
            PluginExecutionResult::Skipped(msg) => msg,
        }
    }
}

/// Plugin execution engine
#[derive(Debug, Clone)]
pub struct PluginExecutionEngine {
    manager: PluginManager,
}

impl PluginExecutionEngine {
    /// Create a new execution engine
    pub fn new(manager: PluginManager) -> Self {
        Self { manager }
    }

    /// Execute all enabled plugins
    pub fn execute_all(&self) -> PluginExecutionContext {
        let mut context = PluginExecutionContext::new();
        let plugins = self.manager.get_plugins_by_priority();

        for plugin in plugins {
            if plugin.is_enabled() {
                context.set_current_plugin(plugin.name.clone());

                let result = match plugin.plugin_type {
                    PluginType::Utility => self.execute_utility_plugin(plugin),
                    PluginType::Component => self.execute_component_plugin(plugin),
                    PluginType::Base => self.execute_base_plugin(plugin),
                    PluginType::Variant => self.execute_variant_plugin(plugin),
                    PluginType::Custom(_) => self.execute_custom_plugin(plugin),
                };

                context.record_result(plugin.name.clone(), result);
            }
        }

        context
    }

    /// Execute a utility plugin
    fn execute_utility_plugin(&self, plugin: &PluginDefinition) -> PluginExecutionResult {
        // Simulate utility plugin execution
        PluginExecutionResult::Success(format!("Executed utility plugin: {}", plugin.name))
    }

    /// Execute a component plugin
    fn execute_component_plugin(&self, plugin: &PluginDefinition) -> PluginExecutionResult {
        // Simulate component plugin execution
        PluginExecutionResult::Success(format!("Executed component plugin: {}", plugin.name))
    }

    /// Execute a base plugin
    fn execute_base_plugin(&self, plugin: &PluginDefinition) -> PluginExecutionResult {
        // Simulate base plugin execution
        PluginExecutionResult::Success(format!("Executed base plugin: {}", plugin.name))
    }

    /// Execute a variant plugin
    fn execute_variant_plugin(&self, plugin: &PluginDefinition) -> PluginExecutionResult {
        // Simulate variant plugin execution
        PluginExecutionResult::Success(format!("Executed variant plugin: {}", plugin.name))
    }

    /// Execute a custom plugin
    fn execute_custom_plugin(&self, plugin: &PluginDefinition) -> PluginExecutionResult {
        // Simulate custom plugin execution
        PluginExecutionResult::Success(format!("Executed custom plugin: {}", plugin.name))
    }

    /// Get the plugin manager
    pub fn manager(&self) -> &PluginManager {
        &self.manager
    }

    /// Get mutable access to the plugin manager
    pub fn manager_mut(&mut self) -> &mut PluginManager {
        &mut self.manager
    }
}

/// Advanced plugin system utilities trait for extending ClassBuilder
pub trait AdvancedPluginUtilities {
    /// Add plugin-related classes
    fn plugin_type(self, plugin_type: PluginType) -> Self;
    fn plugin_priority(self, priority: PluginPriority) -> Self;
    fn plugin_config(self, config: PluginConfig) -> Self;
    fn plugin_composition(self, composition: PluginComposition) -> Self;
}

impl AdvancedPluginUtilities for ClassBuilder {
    fn plugin_type(self, plugin_type: PluginType) -> Self {
        self.class(&plugin_type.to_class_name())
    }

    fn plugin_priority(self, priority: PluginPriority) -> Self {
        self.class(&priority.to_class_name())
    }

    fn plugin_config(self, config: PluginConfig) -> Self {
        self.class(&config.to_class_name())
    }

    fn plugin_composition(self, composition: PluginComposition) -> Self {
        self.class(&composition.to_class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_definition_creation() {
        let plugin = PluginDefinition::new("test-plugin".to_string(), PluginType::Utility)
            .with_priority(PluginPriority::High)
            .with_config(PluginConfig::Enable);

        assert_eq!(plugin.name, "test-plugin");
        assert!(matches!(plugin.plugin_type, PluginType::Utility));
        assert!(matches!(plugin.priority, PluginPriority::High));
        assert!(plugin.is_enabled());
    }

    #[test]
    fn plugin_definition_display() {
        let plugin = PluginDefinition::new("my-plugin".to_string(), PluginType::Component);
        assert_eq!(plugin.display_name(), "my-plugin (component)");
    }

    #[test]
    fn plugin_manager_operations() {
        let mut manager = PluginManager::new();

        let plugin = PluginDefinition::new("test".to_string(), PluginType::Utility);
        manager.register_plugin(plugin).unwrap();

        assert!(manager.get_plugin("test").is_some());
        assert_eq!(manager.get_all_plugins().len(), 1);

        manager.enable_plugin("test").unwrap();
        assert!(manager.get_plugin("test").unwrap().is_enabled());

        manager.disable_plugin("test").unwrap();
        assert!(!manager.get_plugin("test").unwrap().is_enabled());
    }

    #[test]
    fn plugin_execution_context() {
        let mut context = PluginExecutionContext::new();

        context.set_current_plugin("plugin1".to_string());
        context.record_result("plugin1".to_string(), PluginExecutionResult::Success("OK".to_string()));

        assert_eq!(context.execution_order, vec!["plugin1"]);
        assert!(context.get_result("plugin1").unwrap().is_success());
        assert!(context.is_successful());
    }

    #[test]
    fn plugin_execution_results() {
        let success = PluginExecutionResult::Success("OK".to_string());
        let failure = PluginExecutionResult::Failure("Error".to_string());
        let skipped = PluginExecutionResult::Skipped("Disabled".to_string());

        assert!(success.is_success());
        assert!(failure.is_failure());
        assert!(skipped.is_skipped());

        assert_eq!(success.message(), "OK");
        assert_eq!(failure.message(), "Error");
        assert_eq!(skipped.message(), "Disabled");
    }

    #[test]
    fn plugin_execution_engine() {
        let manager = PluginManager::new();
        let engine = PluginExecutionEngine::new(manager);

        let context = engine.execute_all();
        assert!(context.is_successful()); // No plugins, so successful
    }

    #[test]
    fn advanced_plugin_utilities_trait() {
        let builder = ClassBuilder::new();

        // Test trait methods (simplified - would need actual ClassBuilder implementation)
        let _result = builder
            .plugin_type(PluginType::Utility)
            .plugin_priority(PluginPriority::High)
            .plugin_config(PluginConfig::Enable)
            .plugin_composition(PluginComposition::Merge);
        // In a real implementation, this would add classes to the builder
    }
}
