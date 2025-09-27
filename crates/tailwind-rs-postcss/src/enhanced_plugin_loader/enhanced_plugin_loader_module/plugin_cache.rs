//! Plugin Cache
//!
//! This module provides plugin caching functionality.

use std::collections::HashMap;
use crate::enhanced_plugin_loader::enhanced_plugin_loader_module::{PluginInstance, PluginConfig};

/// Plugin cache
pub struct PluginCache {
    plugins: HashMap<String, PluginInstance>,
    dependency_cache: HashMap<String, Vec<String>>,
}

impl PluginCache {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            dependency_cache: HashMap::new(),
        }
    }

    pub fn get_plugin(&self, name: &str, version: Option<&str>) -> Option<&PluginInstance> {
        let key = if let Some(version) = version {
            format!("{}@{}", name, version)
        } else {
            name.to_string()
        };
        self.plugins.get(&key)
    }

    pub fn cache_plugin(&mut self, name: String, version: Option<String>, plugin: PluginInstance) {
        let key = if let Some(version) = version {
            format!("{}@{}", name, version)
        } else {
            name
        };
        self.plugins.insert(key, plugin);
    }
}
