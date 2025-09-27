//! Plugin Config Manager
//!
//! This module provides plugin configuration management functionality.

use std::collections::HashMap;
use crate::enhanced_plugin_loader::enhanced_plugin_loader_module::PluginConfig;

/// Plugin config manager
pub struct PluginConfigManager {
    configs: HashMap<String, PluginConfig>,
    templates: HashMap<String, ConfigTemplate>,
}

impl PluginConfigManager {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            templates: HashMap::new(),
        }
    }

    pub fn get_config(&self, name: &str) -> Option<&PluginConfig> {
        self.configs.get(name)
    }

    pub fn set_config(&mut self, name: String, config: PluginConfig) {
        self.configs.insert(name, config);
    }
}

/// Configuration template
pub struct ConfigTemplate {
    pub name: String,
    pub template: PluginConfig,
}
