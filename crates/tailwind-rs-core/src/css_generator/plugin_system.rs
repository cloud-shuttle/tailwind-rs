//! Plugin System for Tailwind-RS
//!
//! This module provides the core plugin architecture that allows developers to extend
//! tailwind-rs with custom utilities, components, and variants.

use super::types::CssProperty;
use crate::enhanced_variants::types::VariantDefinition;
use std::collections::HashMap;
use std::result::Result as StdResult;

/// Core plugin trait that all tailwind-rs plugins must implement
pub trait Plugin: Send + Sync {
    /// Unique name for the plugin
    fn name(&self) -> &str;

    /// Version of the plugin
    fn version(&self) -> &str;

    /// Add custom utilities to the CSS generator
    fn add_utilities(&self, _config: &mut PluginConfig) -> StdResult<(), PluginError> {
        Ok(())
    }

    /// Add custom components to the CSS generator
    fn add_components(&self, _config: &mut PluginConfig) -> StdResult<(), PluginError> {
        Ok(())
    }

    /// Add custom variants to the CSS generator
    fn add_variants(&self, _config: &mut PluginConfig) -> StdResult<(), PluginError> {
        Ok(())
    }

    /// Return theme extensions for this plugin
    fn theme(&self) -> Option<ThemeExtension> {
        None
    }

    /// List of plugin dependencies
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Configuration structure passed to plugins during registration
#[derive(Debug, Default)]
pub struct PluginConfig {
    /// Custom utilities added by plugins
    pub utilities: HashMap<String, Vec<CssProperty>>,
    /// Custom components added by plugins
    pub components: HashMap<String, ComponentDefinition>,
    /// Custom variants added by plugins
    pub variants: HashMap<String, VariantDefinition>,
    /// Theme extensions from plugins
    pub theme: ThemeExtension,
}

/// Definition of a component that can be added by plugins
#[derive(Debug, Clone)]
pub struct ComponentDefinition {
    /// CSS selector for the component
    pub selector: String,
    /// CSS properties for the component
    pub properties: Vec<CssProperty>,
    /// Variants that apply to this component
    pub variants: Vec<String>,
}

/// Theme extensions that plugins can provide
#[derive(Debug, Clone, Default)]
pub struct ThemeExtension {
    /// Extended color palette
    pub colors: Option<HashMap<String, String>>,
    /// Extended spacing values
    pub spacing: Option<HashMap<String, String>>,
    /// Extended font families
    pub font_family: Option<HashMap<String, String>>,
    /// Extended font sizes
    pub font_size: Option<HashMap<String, String>>,
    /// Whether to extend existing theme or replace
    pub extend: bool,
}

/// Errors that can occur during plugin operations
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin validation failed: {message}")]
    ValidationError { message: String },

    #[error("Plugin dependency '{dependency}' not found")]
    MissingDependency { dependency: String },

    #[error("Plugin '{name}' conflicts with existing plugin")]
    Conflict { name: String },

    #[error("Plugin registration failed: {message}")]
    RegistrationError { message: String },

    #[error("Plugin execution failed: {message}")]
    ExecutionError { message: String },
}

/// Manager for registering and coordinating plugins
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    config: PluginConfig,
    dependency_graph: HashMap<String, Vec<String>>,
}

impl std::fmt::Debug for PluginManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginManager")
            .field("plugins", &format!("{} plugins", self.plugins.len()))
            .field("config", &self.config)
            .field("dependency_graph", &self.dependency_graph)
            .finish()
    }
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            config: PluginConfig::default(),
            dependency_graph: HashMap::new(),
        }
    }

    /// Register a plugin with the manager
    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> StdResult<(), PluginError> {
        let name = plugin.name().to_string();

        // Check for conflicts
        if self.plugins.contains_key(&name) {
            return Err(PluginError::Conflict { name });
        }

        // Store dependencies
        self.dependency_graph.insert(name.clone(), plugin.dependencies());

        // Register the plugin
        self.plugins.insert(name, plugin);

        Ok(())
    }

    /// Load and register plugins from configuration
    pub fn load_from_config(&mut self, _config: &serde_json::Value) -> StdResult<(), PluginError> {
        // TODO: Implement plugin loading from configuration
        // This will load plugins specified in tailwind.config.js/ts
        Ok(())
    }

    /// Resolve plugin dependencies and initialize plugins
    pub fn initialize(&mut self) -> StdResult<(), PluginError> {
        // Resolve dependencies (topological sort)
        let resolved_order = self.resolve_dependencies()?;

        // Initialize plugins in dependency order
        for plugin_name in resolved_order {
            if let Some(plugin) = self.plugins.get(&plugin_name) {
                // Call plugin methods to populate config
                plugin.add_utilities(&mut self.config)?;
                plugin.add_components(&mut self.config)?;
                plugin.add_variants(&mut self.config)?;

                // Handle theme extensions
                if let Some(theme_ext) = plugin.theme() {
                    self.merge_theme_extension(theme_ext)?;
                }
            }
        }

        Ok(())
    }

    /// Get the final plugin configuration
    pub fn config(&self) -> &PluginConfig {
        &self.config
    }

    /// Get a mutable reference to the plugin configuration
    pub fn config_mut(&mut self) -> &mut PluginConfig {
        &mut self.config
    }

    /// Resolve plugin dependencies using topological sort
    fn resolve_dependencies(&self) -> StdResult<Vec<String>, PluginError> {
        // Simple topological sort for plugin dependencies
        let mut result = Vec::new();
        let mut visited = HashMap::new();
        let mut visiting = HashMap::new();

        for plugin_name in self.plugins.keys() {
            self.visit_plugin(plugin_name, &mut visited, &mut visiting, &mut result)?;
        }

        Ok(result)
    }

    /// Visit a plugin in the dependency graph
    fn visit_plugin(
        &self,
        plugin_name: &str,
        visited: &mut HashMap<String, bool>,
        visiting: &mut HashMap<String, bool>,
        result: &mut Vec<String>,
    ) -> StdResult<(), PluginError> {
        // Check if already visited
        if let Some(&true) = visited.get(plugin_name) {
            return Ok(());
        }

        // Check for circular dependency
        if let Some(&true) = visiting.get(plugin_name) {
            return Err(PluginError::ValidationError {
                message: format!("Circular dependency detected involving plugin '{}'", plugin_name),
            });
        }

        // Mark as visiting
        visiting.insert(plugin_name.to_string(), true);

        // Visit dependencies
        if let Some(deps) = self.dependency_graph.get(plugin_name) {
            for dep in deps {
                if !self.plugins.contains_key(dep) {
                    return Err(PluginError::MissingDependency {
                        dependency: dep.clone(),
                    });
                }
                self.visit_plugin(dep, visited, visiting, result)?;
            }
        }

        // Mark as visited and add to result
        visiting.insert(plugin_name.to_string(), false);
        visited.insert(plugin_name.to_string(), true);
        result.push(plugin_name.to_string());

        Ok(())
    }

    /// Merge theme extension into the configuration
    fn merge_theme_extension(&mut self, extension: ThemeExtension) -> StdResult<(), PluginError> {
        // Merge colors
        if let Some(colors) = extension.colors {
            if self.config.theme.colors.is_none() {
                self.config.theme.colors = Some(HashMap::new());
            }
            if let Some(existing_colors) = &mut self.config.theme.colors {
                existing_colors.extend(colors);
            }
        }

        // Merge spacing
        if let Some(spacing) = extension.spacing {
            if self.config.theme.spacing.is_none() {
                self.config.theme.spacing = Some(HashMap::new());
            }
            if let Some(existing_spacing) = &mut self.config.theme.spacing {
                existing_spacing.extend(spacing);
            }
        }

        // Merge font families
        if let Some(font_family) = extension.font_family {
            if self.config.theme.font_family.is_none() {
                self.config.theme.font_family = Some(HashMap::new());
            }
            if let Some(existing_fonts) = &mut self.config.theme.font_family {
                existing_fonts.extend(font_family);
            }
        }

        // Merge font sizes
        if let Some(font_size) = extension.font_size {
            if self.config.theme.font_size.is_none() {
                self.config.theme.font_size = Some(HashMap::new());
            }
            if let Some(existing_sizes) = &mut self.config.theme.font_size {
                existing_sizes.extend(font_size);
            }
        }

        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Example utility plugin implementation
pub struct UtilityPlugin {
    name: String,
    utilities: HashMap<String, Vec<CssProperty>>,
}

impl UtilityPlugin {
    pub fn new(name: &str, utilities: HashMap<String, Vec<CssProperty>>) -> Self {
        Self {
            name: name.to_string(),
            utilities,
        }
    }
}

impl Plugin for UtilityPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn add_utilities(&self, config: &mut PluginConfig) -> StdResult<(), PluginError> {
        for (class_name, properties) in &self.utilities {
            config.utilities.insert(class_name.clone(), properties.clone());
        }
        Ok(())
    }
}

/// Example component plugin implementation
pub struct ComponentPlugin {
    name: String,
    components: HashMap<String, ComponentDefinition>,
}

impl ComponentPlugin {
    pub fn new(name: &str, components: HashMap<String, ComponentDefinition>) -> Self {
        Self {
            name: name.to_string(),
            components,
        }
    }
}

impl Plugin for ComponentPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn add_components(&self, config: &mut PluginConfig) -> StdResult<(), PluginError> {
        for (name, component) in &self.components {
            config.components.insert(name.clone(), component.clone());
        }
        Ok(())
    }
}

/// Example theme plugin implementation
pub struct ThemePlugin {
    name: String,
    theme_extension: ThemeExtension,
}

impl ThemePlugin {
    pub fn new(name: &str, theme_extension: ThemeExtension) -> Self {
        Self {
            name: name.to_string(),
            theme_extension,
        }
    }
}

impl Plugin for ThemePlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn theme(&self) -> Option<ThemeExtension> {
        Some(self.theme_extension.clone())
    }
}
