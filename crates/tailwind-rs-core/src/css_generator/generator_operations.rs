//! CSS Generator Operations
//!
//! This module contains the add/remove/update operations for CssGenerator.

use super::types::{CssProperty, CssRule};
use super::generator_parsers::CssGeneratorParsers;
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
        // Generate individual rule for this class only - "One Class = One CSS Rule"
        let rule = self.generate_individual_css_rule(class)?;
        self.rules.insert(class.to_string(), rule);
        Ok(())
    }

    fn add_classes_for_element(&mut self, classes: &[&str]) -> Result<()> {
        // Process each class individually using the "One Class = One CSS Rule" architecture
        for &class in classes {
            // Skip gradient stops - they only set CSS variables and don't generate individual rules
            if Self::extract_gradient_stop_type(class).is_some() {
                continue;
            }

            // Generate individual rule for each class (including gradient directions)
            let rule = self.generate_individual_css_rule(class)?;
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
