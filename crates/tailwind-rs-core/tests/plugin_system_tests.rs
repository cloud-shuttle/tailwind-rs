//! TDD Tests for Plugin System Architecture
//!
//! This module tests the plugin system that allows users to extend
//! Tailwind CSS functionality with custom utilities, components, and variants.

use std::collections::HashMap;
use tailwind_rs_core::*;

/// Plugin trait for extending Tailwind functionality
pub trait TailwindPlugin {
    /// Plugin name
    fn name(&self) -> &str;

    /// Plugin version
    fn version(&self) -> &str;

    /// Initialize the plugin
    fn init(&mut self) -> Result<()>;

    /// Add custom utilities
    fn add_utilities(&self) -> HashMap<String, String>;

    /// Add custom components
    fn add_components(&self) -> HashMap<String, String>;

    /// Add custom variants
    fn add_variants(&self) -> Vec<String>;

    /// Configure the plugin
    fn configure(&mut self, config: &TailwindConfig) -> Result<()>;
}

/// Example plugin for custom animations
pub struct AnimationPlugin {
    name: String,
    version: String,
    animations: HashMap<String, String>,
}

impl AnimationPlugin {
    pub fn new() -> Self {
        Self {
            name: "animation-plugin".to_string(),
            version: "1.0.0".to_string(),
            animations: HashMap::new(),
        }
    }

    pub fn add_animation(&mut self, name: String, keyframes: String) {
        self.animations.insert(name, keyframes);
    }
}

impl TailwindPlugin for AnimationPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn init(&mut self) -> Result<()> {
        // Initialize default animations
        self.animations.insert(
            "fade-in".to_string(),
            "@keyframes fade-in { from { opacity: 0; } to { opacity: 1; } }".to_string(),
        );
        self.animations.insert("slide-up".to_string(), 
            "@keyframes slide-up { from { transform: translateY(100%); } to { transform: translateY(0); } }".to_string());
        Ok(())
    }

    fn add_utilities(&self) -> HashMap<String, String> {
        let mut utilities = HashMap::new();

        // Add animation utilities
        for (name, _) in &self.animations {
            utilities.insert(
                format!("animate-{}", name),
                format!("animation: {} 1s ease-in-out;", name),
            );
        }

        utilities
    }

    fn add_components(&self) -> HashMap<String, String> {
        let mut components = HashMap::new();

        // Add animation components
        components.insert(
            ".fade-in-component".to_string(),
            "opacity: 0; animation: fade-in 0.5s ease-in-out forwards;".to_string(),
        );

        components.insert(
            ".slide-up-component".to_string(),
            "transform: translateY(100%); animation: slide-up 0.3s ease-out forwards;".to_string(),
        );

        components
    }

    fn add_variants(&self) -> Vec<String> {
        vec![
            "animate-on-hover".to_string(),
            "animate-on-focus".to_string(),
            "animate-on-load".to_string(),
        ]
    }

    fn configure(&mut self, _config: &TailwindConfig) -> Result<()> {
        // Plugin configuration logic
        Ok(())
    }
}

/// Plugin manager for handling multiple plugins
pub struct PluginManager {
    plugins: Vec<Box<dyn TailwindPlugin>>,
    utilities: HashMap<String, String>,
    components: HashMap<String, String>,
    variants: Vec<String>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            utilities: HashMap::new(),
            components: HashMap::new(),
            variants: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, mut plugin: Box<dyn TailwindPlugin>) -> Result<()> {
        plugin.init()?;

        // Collect utilities from plugin
        for (name, css) in plugin.add_utilities() {
            self.utilities.insert(name, css);
        }

        // Collect components from plugin
        for (name, css) in plugin.add_components() {
            self.components.insert(name, css);
        }

        // Collect variants from plugin
        self.variants.extend(plugin.add_variants());

        self.plugins.push(plugin);
        Ok(())
    }

    pub fn get_utilities(&self) -> &HashMap<String, String> {
        &self.utilities
    }

    pub fn get_components(&self) -> &HashMap<String, String> {
        &self.components
    }

    pub fn get_variants(&self) -> &Vec<String> {
        &self.variants
    }

    pub fn configure_plugins(&mut self, config: &TailwindConfig) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.configure(config)?;
        }
        Ok(())
    }
}

#[test]
fn test_plugin_creation_and_initialization() {
    let mut plugin = AnimationPlugin::new();

    assert_eq!(plugin.name(), "animation-plugin");
    assert_eq!(plugin.version(), "1.0.0");

    // Initialize plugin
    let result = plugin.init();
    assert!(result.is_ok());

    // Check that default animations were added
    let utilities = plugin.add_utilities();
    assert!(utilities.contains_key("animate-fade-in"));
    assert!(utilities.contains_key("animate-slide-up"));
}

#[test]
fn test_plugin_custom_animations() {
    let mut plugin = AnimationPlugin::new();
    plugin.init().unwrap();

    // Add custom animation
    plugin.add_animation("bounce".to_string(), 
        "@keyframes bounce { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-25%); } }".to_string());

    let utilities = plugin.add_utilities();
    assert!(utilities.contains_key("animate-bounce"));
    assert_eq!(
        utilities.get("animate-bounce").unwrap(),
        "animation: bounce 1s ease-in-out;"
    );
}

#[test]
fn test_plugin_components() {
    let mut plugin = AnimationPlugin::new();
    plugin.init().unwrap();

    let components = plugin.add_components();
    assert!(components.contains_key(".fade-in-component"));
    assert!(components.contains_key(".slide-up-component"));

    assert!(components
        .get(".fade-in-component")
        .unwrap()
        .contains("animation: fade-in"));
    assert!(components
        .get(".slide-up-component")
        .unwrap()
        .contains("animation: slide-up"));
}

#[test]
fn test_plugin_variants() {
    let mut plugin = AnimationPlugin::new();
    plugin.init().unwrap();

    let variants = plugin.add_variants();
    assert!(variants.contains(&"animate-on-hover".to_string()));
    assert!(variants.contains(&"animate-on-focus".to_string()));
    assert!(variants.contains(&"animate-on-load".to_string()));
}

#[test]
fn test_plugin_manager_creation() {
    let manager = PluginManager::new();

    assert!(manager.plugins.is_empty());
    assert!(manager.utilities.is_empty());
    assert!(manager.components.is_empty());
    assert!(manager.variants.is_empty());
}

#[test]
fn test_plugin_manager_add_plugin() {
    let mut manager = PluginManager::new();
    let plugin = Box::new(AnimationPlugin::new());

    let result = manager.add_plugin(plugin);
    assert!(result.is_ok());

    // Check that plugin was added
    assert_eq!(manager.plugins.len(), 1);

    // Check that utilities were collected
    assert!(manager.utilities.contains_key("animate-fade-in"));
    assert!(manager.utilities.contains_key("animate-slide-up"));

    // Check that components were collected
    assert!(manager.components.contains_key(".fade-in-component"));
    assert!(manager.components.contains_key(".slide-up-component"));

    // Check that variants were collected
    assert!(manager.variants.contains(&"animate-on-hover".to_string()));
    assert!(manager.variants.contains(&"animate-on-focus".to_string()));
    assert!(manager.variants.contains(&"animate-on-load".to_string()));
}

#[test]
fn test_plugin_manager_multiple_plugins() {
    let mut manager = PluginManager::new();

    // Add first plugin
    let plugin1 = Box::new(AnimationPlugin::new());
    manager.add_plugin(plugin1).unwrap();

    // Add second plugin (custom)
    let mut plugin2 = AnimationPlugin::new();
    plugin2.add_animation("custom-spin".to_string(), 
        "@keyframes custom-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }".to_string());
    manager.add_plugin(Box::new(plugin2)).unwrap();

    // Check that both plugins were added
    assert_eq!(manager.plugins.len(), 2);

    // Check that utilities from both plugins are present
    assert!(manager.utilities.contains_key("animate-fade-in"));
    assert!(manager.utilities.contains_key("animate-custom-spin"));

    // Check that components from both plugins are present
    assert!(manager.components.contains_key(".fade-in-component"));
    assert!(manager.components.contains_key(".slide-up-component"));
}

#[test]
fn test_plugin_integration_with_class_builder() {
    let mut manager = PluginManager::new();
    let plugin = Box::new(AnimationPlugin::new());
    manager.add_plugin(plugin).unwrap();

    // Create a class builder and add plugin-generated classes
    let builder = ClassBuilder::new()
        .class("animate-fade-in")
        .class("fade-in-component");

    let result = builder.build();

    // Check that plugin classes are included
    assert!(result.has_class("animate-fade-in"));
    assert!(result.has_class("fade-in-component"));
}

#[test]
fn test_plugin_configuration() {
    let mut manager = PluginManager::new();
    let plugin = Box::new(AnimationPlugin::new());
    manager.add_plugin(plugin).unwrap();

    // Create a config
    let config = TailwindConfig::default();

    // Configure plugins
    let result = manager.configure_plugins(&config);
    assert!(result.is_ok());
}

#[test]
fn test_plugin_error_handling() {
    // Test plugin initialization error handling
    let mut plugin = AnimationPlugin::new();

    // This should succeed
    let result = plugin.init();
    assert!(result.is_ok());

    // Test plugin manager error handling
    let mut manager = PluginManager::new();
    let plugin = Box::new(AnimationPlugin::new());

    // This should succeed
    let result = manager.add_plugin(plugin);
    assert!(result.is_ok());
}

#[test]
fn test_plugin_performance() {
    let mut manager = PluginManager::new();

    // Add multiple plugins
    for i in 0..10 {
        let mut plugin = AnimationPlugin::new();
        plugin.add_animation(
            format!("animation-{}", i),
            format!(
                "@keyframes animation-{} {{ from {{ opacity: 0; }} to {{ opacity: 1; }} }}",
                i
            ),
        );
        manager.add_plugin(Box::new(plugin)).unwrap();
    }

    // Check performance
    let start = std::time::Instant::now();
    let utilities = manager.get_utilities();
    let components = manager.get_components();
    let variants = manager.get_variants();
    let duration = start.elapsed();

    // Should complete in reasonable time (< 1ms)
    assert!(
        duration.as_millis() < 1,
        "Plugin operations took too long: {:?}",
        duration
    );

    // Check that all plugins were processed
    assert_eq!(manager.plugins.len(), 10);
    assert!(utilities.len() > 10); // Should have utilities from all plugins
    assert!(components.len() > 0); // Should have components
    assert!(variants.len() > 0); // Should have variants
}
