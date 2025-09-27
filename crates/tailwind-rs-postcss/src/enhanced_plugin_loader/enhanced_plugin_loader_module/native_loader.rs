//! Native Plugin Loader
//!
//! This module provides native plugin loading functionality.

use std::collections::HashMap;
use crate::enhanced_plugin_loader::enhanced_plugin_loader_module::{PluginConfig, PluginInstance, PluginError, PluginType};

/// Native plugin loader
pub struct NativePluginLoader {
    plugins: HashMap<String, PluginInstance>,
}

impl NativePluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn load_plugin(&mut self, name: &str, config: &PluginConfig) -> Result<PluginInstance, PluginError> {
        // Check if plugin is already loaded
        if let Some(plugin) = self.plugins.get(name) {
            return Ok(plugin.clone());
        }

        // Load native plugin
        let plugin = PluginInstance::new(
            name.to_string(),
            config.version.clone().unwrap_or_else(|| "1.0.0".to_string()),
            PluginType::Native,
            config.clone(),
        );

        // Store the plugin
        self.plugins.insert(name.to_string(), plugin.clone());

        Ok(plugin)
    }
}
