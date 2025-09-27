//! Plugin system for extending Tailwind-RS functionality
//!
//! This module provides a plugin system that allows users to extend Tailwind-RS
//! with custom utilities, components, and optimizations.

use crate::css_generator::{CssGenerator, CssProperty, CssRule};
use crate::error::{Result, TailwindError};
use std::collections::HashMap;
use std::sync::Arc;

/// Plugin hook types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginHook {
    /// Hook called before CSS generation
    BeforeGenerate,
    /// Hook called after CSS generation
    AfterGenerate,
    /// Hook called when a class is added
    OnClassAdd,
    /// Hook called when a rule is created
    OnRuleCreate,
    /// Hook called during optimization
    OnOptimize,
}

/// Plugin context containing current state
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// Current CSS generator
    pub generator: Arc<CssGenerator>,
    /// Plugin data storage
    pub data: HashMap<String, serde_json::Value>,
    /// Configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Get the plugin name
    fn name(&self) -> &str;

    /// Get the plugin version
    fn version(&self) -> &str;

    /// Get the plugin description
    fn description(&self) -> &str;

    /// Initialize the plugin
    fn initialize(&mut self, context: &mut PluginContext) -> Result<()>;

    /// Handle plugin hooks
    fn handle_hook(&mut self, hook: PluginHook, context: &mut PluginContext) -> Result<()>;

    /// Get plugin configuration schema
    fn get_config_schema(&self) -> Option<serde_json::Value>;

    /// Validate plugin configuration
    fn validate_config(&self, config: &serde_json::Value) -> Result<()>;
}

/// Plugin registry for managing plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    hooks: HashMap<PluginHook, Vec<String>>,
    context: PluginContext,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            hooks: HashMap::new(),
            context: PluginContext {
                generator: Arc::new(CssGenerator::new()),
                data: HashMap::new(),
                config: HashMap::new(),
            },
        }
    }

    /// Register a plugin
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<()> {
        let name = plugin.name().to_string();

        if self.plugins.contains_key(&name) {
            return Err(TailwindError::build(format!(
                "Plugin '{}' is already registered",
                name
            )));
        }

        // Initialize the plugin
        let mut plugin_box = plugin;
        plugin_box.initialize(&mut self.context)?;

        // Register the plugin
        self.plugins.insert(name.clone(), plugin_box);

        // Register default hooks
        self.register_default_hooks(&name);

        Ok(())
    }

    /// Unregister a plugin
    pub fn unregister_plugin(&mut self, name: &str) -> Result<()> {
        if !self.plugins.contains_key(name) {
            return Err(TailwindError::build(format!(
                "Plugin '{}' is not registered",
                name
            )));
        }

        // Remove from plugins
        self.plugins.remove(name);

        // Remove from hooks
        for hook_list in self.hooks.values_mut() {
            hook_list.retain(|plugin_name| plugin_name != name);
        }

        Ok(())
    }

    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// Get a mutable plugin by name
    pub fn get_plugin_mut(&mut self, name: &str) -> Option<&mut (dyn Plugin + '_)> {
        if let Some(plugin) = self.plugins.get_mut(name) {
            Some(plugin.as_mut())
        } else {
            None
        }
    }

    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// Execute a hook for all registered plugins
    pub fn execute_hook(&mut self, hook: PluginHook) -> Result<()> {
        if let Some(plugin_names) = self.hooks.get(&hook) {
            for plugin_name in plugin_names {
                if let Some(plugin) = self.plugins.get_mut(plugin_name) {
                    plugin.handle_hook(hook.clone(), &mut self.context)?;
                }
            }
        }
        Ok(())
    }

    /// Set plugin configuration
    pub fn set_plugin_config(
        &mut self,
        plugin_name: &str,
        config: serde_json::Value,
    ) -> Result<()> {
        if let Some(plugin) = self.plugins.get(plugin_name) {
            plugin.validate_config(&config)?;
        }

        self.context.config.insert(plugin_name.to_string(), config);
        Ok(())
    }

    /// Get plugin configuration
    pub fn get_plugin_config(&self, plugin_name: &str) -> Option<&serde_json::Value> {
        self.context.config.get(plugin_name)
    }

    /// Set plugin data
    pub fn set_plugin_data(&mut self, key: String, value: serde_json::Value) {
        self.context.data.insert(key, value);
    }

    /// Get plugin data
    pub fn get_plugin_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.context.data.get(key)
    }

    /// Update the CSS generator
    pub fn update_generator(&mut self, generator: CssGenerator) {
        self.context.generator = Arc::new(generator);
    }

    /// Get the current CSS generator
    pub fn get_generator(&self) -> Arc<CssGenerator> {
        self.context.generator.clone()
    }

    /// Register default hooks for a plugin
    fn register_default_hooks(&mut self, plugin_name: &str) {
        let default_hooks = vec![
            PluginHook::BeforeGenerate,
            PluginHook::AfterGenerate,
            PluginHook::OnClassAdd,
            PluginHook::OnRuleCreate,
            PluginHook::OnOptimize,
        ];

        for hook in default_hooks {
            self.hooks
                .entry(hook)
                .or_default()
                .push(plugin_name.to_string());
        }
    }
}

/// Example plugin: Custom utilities
#[derive(Debug)]
pub struct CustomUtilitiesPlugin {
    name: String,
    version: String,
    description: String,
    custom_utilities: HashMap<String, CssRule>,
}

impl Default for CustomUtilitiesPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomUtilitiesPlugin {
    /// Create a new custom utilities plugin
    pub fn new() -> Self {
        Self {
            name: "custom-utilities".to_string(),
            version: "1.0.0".to_string(),
            description: "Adds custom utility classes".to_string(),
            custom_utilities: HashMap::new(),
        }
    }

    /// Add a custom utility
    pub fn add_utility(&mut self, class_name: String, rule: CssRule) {
        self.custom_utilities.insert(class_name, rule);
    }
}

impl Plugin for CustomUtilitiesPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn initialize(&mut self, _context: &mut PluginContext) -> Result<()> {
        // Add some default custom utilities
        self.add_utility(
            "custom-shadow".to_string(),
            CssRule {
                selector: ".custom-shadow".to_string(),
                properties: vec![CssProperty {
                    name: "box-shadow".to_string(),
                    value: "0 4px 6px -1px rgba(0, 0, 0, 0.1)".to_string(),
                    important: false,
                }],
                media_query: None,
                specificity: 10,
            },
        );

        Ok(())
    }

    fn handle_hook(&mut self, hook: PluginHook, _context: &mut PluginContext) -> Result<()> {
        match hook {
            PluginHook::BeforeGenerate => {
                // Add custom utilities to the generator
                // Note: This is a simplified implementation
                // In a real implementation, we would need to modify the generator
                println!(
                    "Custom utilities plugin: Adding {} custom utilities",
                    self.custom_utilities.len()
                );
            }
            PluginHook::AfterGenerate => {
                println!("Custom utilities plugin: CSS generation completed");
            }
            _ => {}
        }
        Ok(())
    }

    fn get_config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "utilities": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"},
                            "properties": {"type": "object"}
                        }
                    }
                }
            }
        }))
    }

    fn validate_config(&self, config: &serde_json::Value) -> Result<()> {
        if !config.is_object() {
            return Err(TailwindError::build(
                "Plugin config must be an object".to_string(),
            ));
        }
        Ok(())
    }
}

/// Example plugin: CSS minifier
#[derive(Debug)]
pub struct MinifierPlugin {
    name: String,
    version: String,
    description: String,
    minify: bool,
}

impl Default for MinifierPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl MinifierPlugin {
    /// Create a new minifier plugin
    pub fn new() -> Self {
        Self {
            name: "minifier".to_string(),
            version: "1.0.0".to_string(),
            description: "Minifies CSS output".to_string(),
            minify: true,
        }
    }
}

impl Plugin for MinifierPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn initialize(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    fn handle_hook(&mut self, hook: PluginHook, _context: &mut PluginContext) -> Result<()> {
        if hook == PluginHook::OnOptimize && self.minify {
            println!("Minifier plugin: Applying minification");
        }
        Ok(())
    }

    fn get_config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "enabled": {"type": "boolean"}
            }
        }))
    }

    fn validate_config(&self, config: &serde_json::Value) -> Result<()> {
        if let Some(enabled) = config.get("enabled") {
            if !enabled.is_boolean() {
                return Err(TailwindError::build(
                    "Minifier enabled must be a boolean".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_registry_creation() {
        let registry = PluginRegistry::new();
        assert!(registry.list_plugins().is_empty());
    }

    #[test]
    fn test_register_plugin() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(CustomUtilitiesPlugin::new());

        registry.register_plugin(plugin).unwrap();

        assert_eq!(registry.list_plugins().len(), 1);
        assert!(registry
            .list_plugins()
            .contains(&"custom-utilities".to_string()));
    }

    #[test]
    fn test_duplicate_plugin_registration() {
        let mut registry = PluginRegistry::new();
        let plugin1 = Box::new(CustomUtilitiesPlugin::new());
        let plugin2 = Box::new(CustomUtilitiesPlugin::new());

        registry.register_plugin(plugin1).unwrap();
        let result = registry.register_plugin(plugin2);

        assert!(result.is_err());
    }

    #[test]
    fn test_unregister_plugin() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(CustomUtilitiesPlugin::new());

        registry.register_plugin(plugin).unwrap();
        assert_eq!(registry.list_plugins().len(), 1);

        registry.unregister_plugin("custom-utilities").unwrap();
        assert!(registry.list_plugins().is_empty());
    }

    #[test]
    fn test_plugin_config() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(MinifierPlugin::new());

        registry.register_plugin(plugin).unwrap();

        let config = serde_json::json!({"enabled": true});
        registry
            .set_plugin_config("minifier", config.clone())
            .unwrap();

        assert_eq!(registry.get_plugin_config("minifier"), Some(&config));
    }

    #[test]
    fn test_plugin_data() {
        let mut registry = PluginRegistry::new();

        let data = serde_json::json!({"key": "value"});
        registry.set_plugin_data("test_key".to_string(), data.clone());

        assert_eq!(registry.get_plugin_data("test_key"), Some(&data));
    }

    #[test]
    fn test_execute_hook() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(MinifierPlugin::new());

        registry.register_plugin(plugin).unwrap();

        // This should not panic
        registry.execute_hook(PluginHook::OnOptimize).unwrap();
    }

    #[test]
    fn test_custom_utilities_plugin() {
        let mut plugin = CustomUtilitiesPlugin::new();
        let mut context = PluginContext {
            generator: Arc::new(CssGenerator::new()),
            data: HashMap::new(),
            config: HashMap::new(),
        };

        plugin.initialize(&mut context).unwrap();
        assert_eq!(plugin.name(), "custom-utilities");
        assert_eq!(plugin.version(), "1.0.0");
    }

    #[test]
    fn test_minifier_plugin() {
        let mut plugin = MinifierPlugin::new();
        let mut context = PluginContext {
            generator: Arc::new(CssGenerator::new()),
            data: HashMap::new(),
            config: HashMap::new(),
        };

        plugin.initialize(&mut context).unwrap();
        assert_eq!(plugin.name(), "minifier");
        assert_eq!(plugin.version(), "1.0.0");
    }
}
