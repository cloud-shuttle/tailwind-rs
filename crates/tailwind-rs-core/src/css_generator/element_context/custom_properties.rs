//! Custom Properties Context Module
//!
//! Handles CSS custom properties (CSS variables) like [--my-var:red]

use crate::css_generator::types::CssProperty;

/// Context for managing CSS custom properties within an element
#[derive(Debug, Clone, Default)]
pub struct CustomPropertyContext {
    /// Custom CSS properties defined on this element
    pub properties: std::collections::HashMap<String, String>,
}

impl CustomPropertyContext {
    /// Update context from a custom property class like "[--my-var:red]"
    pub fn update_from_class(&mut self, class: &str) {
        if let Some((var_name, value)) = Self::parse_custom_property(class) {
            self.properties.insert(var_name, value);
        }
    }

    /// Parse custom property from class like "[--my-var:red]" -> ("--my-var", "red")
    pub fn parse_custom_property(class: &str) -> Option<(String, String)> {
        if !class.starts_with("[--") || !class.ends_with("]") {
            return None;
        }

        // Remove brackets and split on first colon
        let content = &class[1..class.len() - 1]; // Remove [ and ]
        if let Some(colon_pos) = content.find(':') {
            let var_name = content[..colon_pos].trim().to_string();
            let value = content[colon_pos + 1..].trim().to_string();
            if var_name.starts_with("--") && !value.is_empty() {
                return Some((var_name, value));
            }
        }
        None
    }

    /// Generate CSS properties for custom properties
    pub fn to_css_properties(&self) -> Vec<CssProperty> {
        self.properties.iter().map(|(name, value)| {
            CssProperty {
                name: name.clone(),
                value: value.clone(),
                important: false,
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_property_parsing() {
        let mut context = CustomPropertyContext::default();

        // Test custom property classes
        context.update_from_class("[--my-color:red]");
        context.update_from_class("[--spacing:16px]");
        context.update_from_class("[--radius:8px]");

        // Generate CSS
        let properties = context.to_css_properties();

        // Should have custom property definitions
        assert_eq!(properties.len(), 3);

        // Check specific properties
        let has_my_color = properties.iter().any(|p| p.name == "--my-color" && p.value == "red");
        let has_spacing = properties.iter().any(|p| p.name == "--spacing" && p.value == "16px");
        let has_radius = properties.iter().any(|p| p.name == "--radius" && p.value == "8px");

        assert!(has_my_color, "Should have --my-color property");
        assert!(has_spacing, "Should have --spacing property");
        assert!(has_radius, "Should have --radius property");

        // Test parsing individual custom properties
        assert_eq!(CustomPropertyContext::parse_custom_property("[--my-color:red]"), Some(("--my-color".to_string(), "red".to_string())));
        assert_eq!(CustomPropertyContext::parse_custom_property("[--spacing:16px]"), Some(("--spacing".to_string(), "16px".to_string())));

        // Test invalid custom properties
        assert_eq!(CustomPropertyContext::parse_custom_property("[my-color:red]"), None); // Missing --
        assert_eq!(CustomPropertyContext::parse_custom_property("[--my-color]"), None); // Missing colon and value
        assert_eq!(CustomPropertyContext::parse_custom_property("--my-color:red"), None); // Missing brackets
    }
}
