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
    fn extract_gradient_stop_type(class: &str) -> Option<&'static str> {
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
    fn extract_gradient_color(class: &str, stop_type: &str) -> Option<String> {
        let color_part = class.strip_prefix(&format!("{}-", stop_type))?;

        // Use the color parser to get the actual color value
        // For now, return a simple mapping - this could be enhanced
        match color_part {
            "blue-500" => Some("#3b82f6".to_string()),
            "red-500" => Some("#ef4444".to_string()),
            "green-500" => Some("#22c55e".to_string()),
            "purple-500" => Some("#a855f7".to_string()),
            "pink-500" => Some("#ec4899".to_string()),
            "slate-900" => Some("#0f172a".to_string()),
            "emerald-600" => Some("#059669".to_string()),
            "gray-900" => Some("#111827".to_string()),
            "cyan-500" => Some("#06b6d4".to_string()),
            // Add more colors as needed
            _ => {
                // Try to parse as a basic color name
                match color_part {
                    "black" => Some("#000000".to_string()),
                    "white" => Some("#ffffff".to_string()),
                    "transparent" => Some("transparent".to_string()),
                    _ => None, // Unknown color, let fallback handle it
                }
            }
        }
    }

    /// Extract gradient direction from a class name
    fn extract_gradient_direction(class: &str) -> Option<&'static str> {
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
        // Handle gradient stops specially - they don't generate CSS by themselves
        // but add to the gradient context for later use
        if let Some(stop_type) = Self::extract_gradient_stop_type(class) {
            if let Some(color) = Self::extract_gradient_color(class, stop_type) {
                self.add_gradient_stop(stop_type, color);
                // Gradient stops don't generate CSS rules by themselves
                return Ok(());
            }
        }

        // Handle gradient directions - they may generate CSS using context
        if let Some(direction) = Self::extract_gradient_direction(class) {
            if let Some(gradient_css) = self.generate_gradient_css(direction) {
                // Create a rule with the gradient CSS
                let rule = super::types::CssRule {
                    selector: format!(".{}", class),
                    properties: vec![super::types::CssProperty {
                        name: "background-image".to_string(),
                        value: gradient_css,
                        important: false,
                    }],
                    media_query: None,
                    specificity: 10,
                };
                self.rules.insert(class.to_string(), rule);
                return Ok(());
            }
        }

        // Handle all other classes normally
        let rule = self.class_to_css_rule(class)?;
        self.rules.insert(class.to_string(), rule);
        Ok(())
    }

    fn add_classes_for_element(&mut self, classes: &[&str]) -> Result<()> {
        // Clear gradient context for this element
        self.clear_gradient_context();

        // First pass: collect gradient stops
        for &class in classes {
            if let Some(stop_type) = Self::extract_gradient_stop_type(class) {
                if let Some(color) = Self::extract_gradient_color(class, stop_type) {
                    self.add_gradient_stop(stop_type, color);
                }
            }
        }

        // Second pass: process all classes, handling gradients specially
        for &class in classes {
            // Handle gradient directions - they may generate CSS using collected context
            if let Some(direction) = Self::extract_gradient_direction(class) {
                if let Some(gradient_css) = self.generate_gradient_css(direction) {
                    // Create a rule with the gradient CSS
                    let rule = super::types::CssRule {
                        selector: format!(".{}", class),
                        properties: vec![super::types::CssProperty {
                            name: "background-image".to_string(),
                            value: gradient_css,
                            important: false,
                        }],
                        media_query: None,
                        specificity: 10,
                    };
                    self.rules.insert(class.to_string(), rule);
                    continue; // Skip normal processing for this class
                }
            }

            // Handle gradient stops - skip them as they were collected above
            if Self::extract_gradient_stop_type(class).is_some() {
                continue; // Skip gradient stops, they don't generate individual rules
            }

            // Handle all other classes normally
            let rule = self.class_to_css_rule(class)?;
            self.rules.insert(class.to_string(), rule);
        }

        Ok(())
    }

    fn add_css_selector(&mut self, selector: &str, properties: &str) -> Result<()> {
        let rule = CssRule {
            selector: selector.to_string(),
            properties: vec![CssProperty {
                name: "content".to_string(),
                value: properties.to_string(),
                important: false,
            }],
            media_query: None,
            specificity: 0, // CSS selectors have low specificity
        };
        self.rules.insert(selector.to_string(), rule);
        Ok(())
    }

    fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: &str) -> Result<()> {
        let mut rule = self.class_to_css_rule(class)?;
        rule.selector = format!("{}{}", breakpoint.prefix(), class);
        rule.media_query = self.breakpoints.get(&breakpoint).cloned();
        rule.specificity = 20; // Higher specificity for responsive rules

        let responsive_class = format!("{}:{}", breakpoint.prefix().trim_end_matches(':'), class);
        self.rules.insert(responsive_class, rule);
        Ok(())
    }

    fn add_custom_property(&mut self, name: &str, value: &str) {
        self.custom_properties
            .insert(name.to_string(), value.to_string());
    }

    fn remove_rule(&mut self, selector: &str) {
        self.rules.remove(selector);
    }

    fn update_rule(&mut self, selector: &str, rule: CssRule) {
        self.rules.insert(selector.to_string(), rule);
    }
}
