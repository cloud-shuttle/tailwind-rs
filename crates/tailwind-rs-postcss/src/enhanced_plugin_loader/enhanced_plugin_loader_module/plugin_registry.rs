//! Plugin Registry
//!
//! This module provides plugin registry functionality.

use std::collections::HashMap;
use crate::enhanced_plugin_loader::enhanced_plugin_loader_module::{PluginConfig, PluginInstance};

/// Plugin registry
pub struct PluginRegistry {
    plugins: HashMap<String, PluginInstance>,
    categories: HashMap<String, Vec<String>>,
    compatibility_matrix: HashMap<String, Vec<String>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            categories: HashMap::new(),
            compatibility_matrix: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: PluginInstance) {
        self.plugins.insert(plugin.name().to_string(), plugin);
    }

    pub fn get_plugin(&self, name: &str) -> Option<&PluginInstance> {
        self.plugins.get(name)
    }

    pub fn list_plugins(&self) -> Vec<&PluginInstance> {
        self.plugins.values().collect()
    }
}
