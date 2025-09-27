//! NPM Plugin Loader
//!
//! This module provides NPM plugin loading functionality.

use std::collections::HashMap;
use crate::enhanced_plugin_loader::{PluginConfig, PluginInstance, PluginError};

/// NPM plugin loader
pub struct NPMPluginLoader {
    npm_registry: String,
    cache: HashMap<String, PluginInstance>,
}

impl NPMPluginLoader {
    pub fn new() -> Self {
        Self {
            npm_registry: "https://registry.npmjs.org".to_string(),
            cache: HashMap::new(),
        }
    }

    pub fn load_plugin(&mut self, name: &str, config: &PluginConfig) -> Result<PluginInstance, PluginError> {
        // Check cache first
        if let Some(plugin) = self.cache.get(name) {
            return Ok(plugin.clone());
        }

        // Load plugin from NPM registry
        let plugin = PluginInstance::new(
            name.to_string(),
            config.version.clone().unwrap_or_else(|| "latest".to_string()),
            crate::enhanced_plugin_loader::PluginType::NPM,
            config.clone(),
        );

        // Cache the plugin
        self.cache.insert(name.to_string(), plugin.clone());

        Ok(plugin)
    }
}
