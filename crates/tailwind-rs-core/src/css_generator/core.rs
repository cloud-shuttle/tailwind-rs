//! Core CSS Generation Logic
//!
//! This module contains the main CSS generation logic,
//! orchestrating parsers and generating final CSS output.

use crate::css_generator::types::{CssProperty, CssRule};
use crate::error::{Result, TailwindError};
use crate::responsive::Breakpoint;
use std::collections::HashMap;
use super::generator_parsers::CssGeneratorParsers;

// CssRule is now defined in types.rs

// CssProperty is now defined in types.rs

/// CSS generator that converts Tailwind classes to CSS rules
#[derive(Debug, Clone)]
pub struct CssGenerator {
    /// Generated CSS rules
    rules: HashMap<String, CssRule>,
    /// Responsive breakpoints
    breakpoints: HashMap<Breakpoint, String>,
    /// Custom CSS properties
    custom_properties: HashMap<String, String>,
}

impl Default for CssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CssGenerator {
    /// Create a new CSS generator
    pub fn new() -> Self {
        let mut generator = Self {
            rules: HashMap::new(),
            breakpoints: HashMap::new(),
            custom_properties: HashMap::new(),
        };

        // Initialize default breakpoints
        generator
            .breakpoints
            .insert(Breakpoint::Sm, "(min-width: 640px)".to_string());
        generator
            .breakpoints
            .insert(Breakpoint::Md, "(min-width: 768px)".to_string());
        generator
            .breakpoints
            .insert(Breakpoint::Lg, "(min-width: 1024px)".to_string());
        generator
            .breakpoints
            .insert(Breakpoint::Xl, "(min-width: 1280px)".to_string());
        generator
            .breakpoints
            .insert(Breakpoint::Xl2, "(min-width: 1536px)".to_string());

        generator
    }

    /// Add a class to the generator
    pub fn add_class(&mut self, class: &str) -> Result<()> {
        let rule = self.class_to_css_rule(class)?;
        self.rules.insert(class.to_string(), rule);
        Ok(())
    }

    /// Add a responsive class
    pub fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: &str) -> Result<()> {
        let mut rule = self.class_to_css_rule(class)?;
        rule.selector = format!("{}{}", breakpoint.prefix(), class);
        rule.media_query = self.breakpoints.get(&breakpoint).cloned();
        rule.specificity = 20; // Higher specificity for responsive rules

        let responsive_class = format!("{}:{}", breakpoint.prefix().trim_end_matches(':'), class);
        self.rules.insert(responsive_class, rule);
        Ok(())
    }

    /// Add a custom CSS property
    pub fn add_custom_property(&mut self, name: &str, value: &str) {
        self.custom_properties
            .insert(name.to_string(), value.to_string());
    }

    /// Generate CSS from all added classes
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // Add custom properties
        if !self.custom_properties.is_empty() {
            css.push_str(":root {\n");
            for (name, value) in &self.custom_properties {
                css.push_str(&format!("  --{}: {};\n", name, value));
            }
            css.push_str("}\n\n");
        }

        // Group rules by media query
        let mut media_queries: HashMap<Option<String>, Vec<&CssRule>> = HashMap::new();
        for rule in self.rules.values() {
            media_queries
                .entry(rule.media_query.clone())
                .or_default()
                .push(rule);
        }

        // Generate CSS for each media query
        for (media_query, rules) in media_queries {
            let has_media_query = media_query.is_some();
            if let Some(query) = &media_query {
                css.push_str(&format!("@media {} {{\n", query));
            }

            // Sort rules by specificity (higher specificity first)
            let mut sorted_rules = rules;
            sorted_rules.sort_by(|a, b| b.specificity.cmp(&a.specificity));

            for rule in sorted_rules {
                css.push_str(&format!("  {} {{\n", rule.selector));
                for property in &rule.properties {
                    let important = if property.important {
                        " !important"
                    } else {
                        ""
                    };
                    css.push_str(&format!(
                        "    {}: {}{};\n",
                        property.name, property.value, important
                    ));
                }
                css.push_str("  }\n");
            }

            if has_media_query {
                css.push_str("}\n\n");
            }
        }

        css
    }

    /// Generate minified CSS
    pub fn generate_minified_css(&self) -> String {
        let css = self.generate_css();
        // Simple minification: remove extra whitespace and newlines
        css.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Get the number of rules generated
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Clear all rules
    pub fn clear(&mut self) {
        self.rules.clear();
        self.custom_properties.clear();
    }
}