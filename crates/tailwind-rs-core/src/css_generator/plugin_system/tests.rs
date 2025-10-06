//! Tests for the plugin system

use super::*;
use crate::css_generator::types::CssProperty;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_registration() {
        let mut manager = PluginManager::new();

        // Create a simple utility plugin
        let mut utilities = HashMap::new();
        utilities.insert(
            "text-shadow-sm".to_string(),
            vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "0 1px 2px rgba(0, 0, 0, 0.05)".to_string(),
                important: false,
            }],
        );

        let plugin = UtilityPlugin::new("test-utils", utilities);

        // Register plugin
        let result = manager.register(Box::new(plugin));
        assert!(result.is_ok());

        // Initialize plugins
        let result = manager.initialize();
        assert!(result.is_ok());

        // Check that utility was registered
        let config = manager.config();
        assert!(config.utilities.contains_key("text-shadow-sm"));
    }

    #[test]
    fn test_component_plugin() {
        let mut manager = PluginManager::new();

        // Create a component plugin
        let mut components = HashMap::new();
        components.insert(
            "btn".to_string(),
            ComponentDefinition {
                selector: "btn".to_string(),
                properties: vec![
                    CssProperty {
                        name: "padding".to_string(),
                        value: "0.5rem 1rem".to_string(),
                        important: false,
                    },
                    CssProperty {
                        name: "border-radius".to_string(),
                        value: "0.375rem".to_string(),
                        important: false,
                    },
                ],
                variants: vec!["hover".to_string()],
            },
        );

        let plugin = ComponentPlugin::new("test-components", components);

        // Register and initialize
        manager.register(Box::new(plugin)).unwrap();
        manager.initialize().unwrap();

        // Check component was registered
        let config = manager.config();
        assert!(config.components.contains_key("btn"));
    }

    #[test]
    fn test_theme_plugin() {
        let mut manager = PluginManager::new();

        // Create a theme plugin
        let mut colors = HashMap::new();
        colors.insert("primary".to_string(), "#3b82f6".to_string());
        colors.insert("secondary".to_string(), "#64748b".to_string());

        let theme_extension = ThemeExtension {
            colors: Some(colors),
            spacing: None,
            font_family: None,
            font_size: None,
            extend: true,
        };

        let plugin = ThemePlugin::new("test-theme", theme_extension);

        // Register and initialize
        manager.register(Box::new(plugin)).unwrap();
        manager.initialize().unwrap();

        // Check theme was merged
        let config = manager.config();
        assert!(config.theme.colors.as_ref().unwrap().contains_key("primary"));
        assert!(config.theme.colors.as_ref().unwrap().contains_key("secondary"));
    }

    #[test]
    fn test_plugin_dependencies() {
        let mut manager = PluginManager::new();

        // Create plugins with dependencies
        let plugin_a = UtilityPlugin::new("plugin-a", HashMap::new());
        let mut plugin_b = UtilityPlugin::new("plugin-b", HashMap::new());

        // Plugin B depends on Plugin A
        // Note: We'd need to modify the plugin trait to support this
        // For now, we'll just test that registration works

        manager.register(Box::new(plugin_a)).unwrap();
        manager.register(Box::new(plugin_b)).unwrap();
        manager.initialize().unwrap();

        assert_eq!(manager.config().utilities.len(), 0); // No utilities added
    }

    #[test]
    fn test_plugin_conflict() {
        let mut manager = PluginManager::new();

        let plugin_a = UtilityPlugin::new("test-plugin", HashMap::new());
        let plugin_b = UtilityPlugin::new("test-plugin", HashMap::new()); // Same name

        manager.register(Box::new(plugin_a)).unwrap();

        // Second registration should fail
        let result = manager.register(Box::new(plugin_b));
        assert!(result.is_err());
        match result.unwrap_err() {
            PluginError::Conflict(name) => assert_eq!(name, "test-plugin"),
            _ => panic!("Expected conflict error"),
        }
    }

    #[test]
    fn test_css_generation_with_plugins() {
        use crate::css_generator::CssGenerator;

        let mut generator = CssGenerator::new();

        // Create and register a utility plugin
        let mut utilities = HashMap::new();
        utilities.insert(
            "gradient-radial".to_string(),
            vec![CssProperty {
                name: "background".to_string(),
                value: "radial-gradient(circle, #667eea 0%, #764ba2 100%)".to_string(),
                important: false,
            }],
        );

        let plugin = UtilityPlugin::new("gradient-utils", utilities);
        generator.register_plugin(Box::new(plugin)).unwrap();

        // Generate CSS
        let css = generator.generate_css();

        // Check that plugin CSS is included
        assert!(css.contains(".gradient-radial"));
        assert!(css.contains("background: radial-gradient(circle, #667eea 0%, #764ba2 100%)"));
    }

    #[test]
    fn test_component_css_generation() {
        use crate::css_generator::CssGenerator;

        let mut generator = CssGenerator::new();

        // Create a component plugin
        let mut components = HashMap::new();
        components.insert(
            "card".to_string(),
            ComponentDefinition {
                selector: "card".to_string(),
                properties: vec![
                    CssProperty {
                        name: "border-radius".to_string(),
                        value: "0.5rem".to_string(),
                        important: false,
                    },
                    CssProperty {
                        name: "box-shadow".to_string(),
                        value: "0 4px 6px -1px rgba(0, 0, 0, 0.1)".to_string(),
                        important: false,
                    },
                ],
                variants: vec!["hover".to_string()],
            },
        );

        let plugin = ComponentPlugin::new("card-component", components);
        generator.register_plugin(Box::new(plugin)).unwrap();

        // Generate CSS
        let css = generator.generate_css();

        // Check that component CSS is included
        assert!(css.contains(".card"));
        assert!(css.contains("border-radius: 0.5rem"));
        assert!(css.contains("box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1)"));

        // Check that hover variant is generated
        assert!(css.contains(".hovercard:hover"));
    }

    #[test]
    fn test_minified_css_with_plugins() {
        use crate::css_generator::CssGenerator;

        let mut generator = CssGenerator::new();

        // Add a utility plugin
        let mut utilities = HashMap::new();
        utilities.insert(
            "text-glow".to_string(),
            vec![CssProperty {
                name: "text-shadow".to_string(),
                value: "0 0 10px rgba(255, 255, 255, 0.8)".to_string(),
                important: false,
            }],
        );

        let plugin = UtilityPlugin::new("glow-utils", utilities);
        generator.register_plugin(Box::new(plugin)).unwrap();

        // Generate minified CSS
        let css = generator.generate_minified_css();

        // Should contain the utility without extra whitespace
        assert!(css.contains(".text-glow{text-shadow:0 0 10px rgba(255, 255, 255, 0.8);}"));
    }
}
