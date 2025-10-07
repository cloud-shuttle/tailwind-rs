//! CSS Generator Operations
//!
//! This module contains the add/remove/update operations for CssGenerator.

use super::types::{CssProperty, CssRule};
use crate::error::Result;
use crate::responsive::Breakpoint;

/// Operations trait for CssGenerator
pub trait CssGeneratorOperations {
    /// Add a class to the generator
    fn add_class(&mut self, class: &str) -> Result<()>;

    /// Add multiple classes for an element (useful for gradient combinations)
    fn add_classes_for_element(&mut self, classes: &[&str]) -> Result<()>;

    /// Add a CSS selector directly (for non-Tailwind CSS selectors)
    fn add_css_selector(&mut self, selector: &str, properties: &str) -> Result<()>;

    /// Add a responsive class
    fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: &str) -> Result<()>;

    /// Add a custom CSS property
    fn add_custom_property(&mut self, name: &str, value: &str);

    /// Remove a rule by selector
    fn remove_rule(&mut self, selector: &str);

    /// Update a rule
    fn update_rule(&mut self, selector: &str, rule: CssRule);
}

impl super::CssGenerator {
    /// Extract gradient stop type from a class name (from, via, to)
    pub fn extract_gradient_stop_type(class: &str) -> Option<&'static str> {
        if class.starts_with("from-") {
            Some("from")
        } else if class.starts_with("via-") {
            Some("via")
        } else if class.starts_with("to-") {
            Some("to")
        } else {
            None
        }
    }

    /// Extract color from a gradient stop class
    pub fn extract_gradient_color(color_cache: &mut super::color_cache::ColorCache, class: &str, stop_type: &str) -> Option<String> {
              let color_part = class.strip_prefix(&format!("{}-", stop_type))?;

              // Split color and opacity (e.g., "blue-900/30" -> "blue-900", "30")
              let (color_name, opacity) = if let Some((color, opacity_str)) = color_part.split_once('/') {
                  (color, Some(opacity_str))
              } else {
                  (color_part, None)
              };

        // Use the color cache for efficient lookup
        match color_cache.get_or_parse(color_name, opacity) {
            Ok(color) => Some(color),
            Err(_) => None,
        }
    }

    /// Extract gradient direction from a class name
    pub fn extract_gradient_direction(class: &str) -> Option<&'static str> {
        match class {
            "bg-gradient-to-t" => Some("to top"),
            "bg-gradient-to-tr" => Some("to top right"),
            "bg-gradient-to-r" => Some("to right"),
            "bg-gradient-to-br" => Some("to bottom right"),
            "bg-gradient-to-b" => Some("to bottom"),
            "bg-gradient-to-bl" => Some("to bottom left"),
            "bg-gradient-to-l" => Some("to left"),
            "bg-gradient-to-tl" => Some("to top left"),
            _ => None,
        }
    }
}

impl CssGeneratorOperations for super::CssGenerator {
    fn add_class(&mut self, class: &str) -> Result<()> {
        // Generate individual rule and store it
        // For the core generator, delegate to the core operations
        let rule = <Self as super::core::operations::CssGeneratorOperations>::generate_individual_css_rule(self, class)?;
        self.rules.insert(class.to_string(), rule);
        Ok(())
    }

    fn add_classes_for_element(&mut self, classes: &[&str]) -> Result<()> {
        // Process each class individually
        for &class in classes {
            self.add_class(class)?;
        }
        Ok(())
    }

    fn add_css_selector(&mut self, selector: &str, properties: &str) -> Result<()> {
        // Parse properties and create a rule
        let properties_vec = properties.split(';')
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                let parts: Vec<&str> = s.splitn(2, ':').collect();
                CssProperty {
                    name: parts[0].trim().to_string(),
                    value: parts[1].trim().to_string(),
                    important: false,
                }
            })
            .collect();

        let rule = CssRule {
            selector: selector.to_string(),
            properties: properties_vec,
            media_query: None,
            specificity: 1,
        };
        self.rules.insert(selector.to_string(), rule);
        Ok(())
    }

    fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: &str) -> Result<()> {
        // Construct responsive class and add it
        let responsive_class = format!("{}:{}", breakpoint.to_string().to_lowercase(), class);
        self.add_class(&responsive_class)
    }

    fn add_custom_property(&mut self, name: &str, value: &str) {
        if let Some(ref mut props) = self.custom_properties {
            props.insert(name.to_string(), value.to_string());
        }
    }

    fn remove_rule(&mut self, selector: &str) {
        self.rules.remove(selector);
    }

    fn update_rule(&mut self, selector: &str, rule: CssRule) {
        self.rules.insert(selector.to_string(), rule);
    }
}
