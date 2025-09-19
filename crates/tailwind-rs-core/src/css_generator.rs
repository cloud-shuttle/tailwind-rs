//! CSS generation system for tailwind-rs
//!
//! This module provides the core CSS generation functionality, converting
//! Tailwind class names into actual CSS rules.

use crate::error::{Result, TailwindError};
use crate::responsive::Breakpoint;
use std::collections::HashMap;

/// Represents a CSS rule with selector and properties
#[derive(Debug, Clone, PartialEq)]
pub struct CssRule {
    /// CSS selector (e.g., ".p-4", ".md:bg-blue-500")
    pub selector: String,
    /// CSS properties for this rule
    pub properties: Vec<CssProperty>,
    /// Media query for responsive rules
    pub media_query: Option<String>,
    /// CSS specificity score
    pub specificity: u32,
}

/// Represents a CSS property
#[derive(Debug, Clone, PartialEq)]
pub struct CssProperty {
    /// Property name (e.g., "padding", "background-color")
    pub name: String,
    /// Property value (e.g., "1rem", "#3b82f6")
    pub value: String,
    /// Whether the property is marked as !important
    pub important: bool,
}

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

impl CssGenerator {
    /// Create a new CSS generator
    pub fn new() -> Self {
        let mut generator = Self {
            rules: HashMap::new(),
            breakpoints: HashMap::new(),
            custom_properties: HashMap::new(),
        };
        
        // Initialize default breakpoints
        generator.breakpoints.insert(Breakpoint::Sm, "(min-width: 640px)".to_string());
        generator.breakpoints.insert(Breakpoint::Md, "(min-width: 768px)".to_string());
        generator.breakpoints.insert(Breakpoint::Lg, "(min-width: 1024px)".to_string());
        generator.breakpoints.insert(Breakpoint::Xl, "(min-width: 1280px)".to_string());
        generator.breakpoints.insert(Breakpoint::Xl2, "(min-width: 1536px)".to_string());
        
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
        self.custom_properties.insert(name.to_string(), value.to_string());
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
        let mut base_rules = Vec::new();
        let mut responsive_rules: HashMap<String, Vec<&CssRule>> = HashMap::new();
        
        for rule in self.rules.values() {
            if let Some(ref media_query) = rule.media_query {
                responsive_rules.entry(media_query.clone()).or_default().push(rule);
            } else {
                base_rules.push(rule);
            }
        }
        
        // Generate base rules
        for rule in base_rules {
            css.push_str(&self.rule_to_css(rule));
        }
        
        // Generate responsive rules
        for (media_query, rules) in responsive_rules {
            css.push_str(&format!("@media {} {{\n", media_query));
            for rule in rules {
                css.push_str(&format!("  {}\n", self.rule_to_css(rule)));
            }
            css.push_str("}\n\n");
        }
        
        css
    }

    /// Generate minified CSS
    pub fn generate_minified_css(&self) -> String {
        let css = self.generate_css();
        self.minify_css(&css)
    }

    /// Get all generated rules
    pub fn get_rules(&self) -> &HashMap<String, CssRule> {
        &self.rules
    }

    /// Get the number of generated rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Remove a CSS rule by selector
    pub fn remove_rule(&mut self, selector: &str) -> Option<CssRule> {
        self.rules.remove(selector)
    }

    /// Update a CSS rule
    pub fn update_rule(&mut self, selector: &str, rule: CssRule) {
        self.rules.insert(selector.to_string(), rule);
    }

    /// Convert a class name to a CSS rule
    fn class_to_css_rule(&self, class: &str) -> Result<CssRule> {
        let selector = format!(".{}", class);
        let properties = self.class_to_properties(class)?;
        
        Ok(CssRule {
            selector,
            properties,
            media_query: None,
            specificity: 10,
        })
    }

    /// Convert a class name to CSS properties
    fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>> {
        match class {
            // Spacing utilities
            "p-0" => Ok(vec![CssProperty { name: "padding".to_string(), value: "0px".to_string(), important: false }]),
            "p-1" => Ok(vec![CssProperty { name: "padding".to_string(), value: "0.25rem".to_string(), important: false }]),
            "p-2" => Ok(vec![CssProperty { name: "padding".to_string(), value: "0.5rem".to_string(), important: false }]),
            "p-3" => Ok(vec![CssProperty { name: "padding".to_string(), value: "0.75rem".to_string(), important: false }]),
            "p-4" => Ok(vec![CssProperty { name: "padding".to_string(), value: "1rem".to_string(), important: false }]),
            "p-5" => Ok(vec![CssProperty { name: "padding".to_string(), value: "1.25rem".to_string(), important: false }]),
            "p-6" => Ok(vec![CssProperty { name: "padding".to_string(), value: "1.5rem".to_string(), important: false }]),
            "p-8" => Ok(vec![CssProperty { name: "padding".to_string(), value: "2rem".to_string(), important: false }]),
            
            // Margin utilities
            "m-0" => Ok(vec![CssProperty { name: "margin".to_string(), value: "0px".to_string(), important: false }]),
            "m-1" => Ok(vec![CssProperty { name: "margin".to_string(), value: "0.25rem".to_string(), important: false }]),
            "m-2" => Ok(vec![CssProperty { name: "margin".to_string(), value: "0.5rem".to_string(), important: false }]),
            "m-3" => Ok(vec![CssProperty { name: "margin".to_string(), value: "0.75rem".to_string(), important: false }]),
            "m-4" => Ok(vec![CssProperty { name: "margin".to_string(), value: "1rem".to_string(), important: false }]),
            "m-5" => Ok(vec![CssProperty { name: "margin".to_string(), value: "1.25rem".to_string(), important: false }]),
            "m-6" => Ok(vec![CssProperty { name: "margin".to_string(), value: "1.5rem".to_string(), important: false }]),
            "m-8" => Ok(vec![CssProperty { name: "margin".to_string(), value: "2rem".to_string(), important: false }]),
            
            // Background colors
            "bg-white" => Ok(vec![CssProperty { name: "background-color".to_string(), value: "#ffffff".to_string(), important: false }]),
            "bg-black" => Ok(vec![CssProperty { name: "background-color".to_string(), value: "#000000".to_string(), important: false }]),
            "bg-blue-500" => Ok(vec![CssProperty { name: "background-color".to_string(), value: "#3b82f6".to_string(), important: false }]),
            "bg-red-500" => Ok(vec![CssProperty { name: "background-color".to_string(), value: "#ef4444".to_string(), important: false }]),
            "bg-green-500" => Ok(vec![CssProperty { name: "background-color".to_string(), value: "#22c55e".to_string(), important: false }]),
            
            // Text colors
            "text-white" => Ok(vec![CssProperty { name: "color".to_string(), value: "#ffffff".to_string(), important: false }]),
            "text-black" => Ok(vec![CssProperty { name: "color".to_string(), value: "#000000".to_string(), important: false }]),
            "text-blue-500" => Ok(vec![CssProperty { name: "color".to_string(), value: "#3b82f6".to_string(), important: false }]),
            "text-red-500" => Ok(vec![CssProperty { name: "color".to_string(), value: "#ef4444".to_string(), important: false }]),
            "text-green-500" => Ok(vec![CssProperty { name: "color".to_string(), value: "#22c55e".to_string(), important: false }]),
            
            // Display utilities
            "block" => Ok(vec![CssProperty { name: "display".to_string(), value: "block".to_string(), important: false }]),
            "inline" => Ok(vec![CssProperty { name: "display".to_string(), value: "inline".to_string(), important: false }]),
            "flex" => Ok(vec![CssProperty { name: "display".to_string(), value: "flex".to_string(), important: false }]),
            "grid" => Ok(vec![CssProperty { name: "display".to_string(), value: "grid".to_string(), important: false }]),
            "hidden" => Ok(vec![CssProperty { name: "display".to_string(), value: "none".to_string(), important: false }]),
            
            // Border radius
            "rounded-md" => Ok(vec![CssProperty { name: "border-radius".to_string(), value: "0.375rem".to_string(), important: false }]),
            
            _ => Err(TailwindError::class_generation(format!("Unknown class: {}", class))),
        }
    }

    /// Convert a CSS rule to CSS string
    fn rule_to_css(&self, rule: &CssRule) -> String {
        let mut css = format!("{} {{\n", rule.selector);
        for property in &rule.properties {
            let important = if property.important { " !important" } else { "" };
            css.push_str(&format!("  {}: {}{};\n", property.name, property.value, important));
        }
        css.push_str("}\n");
        css
    }

    /// Minify CSS by removing unnecessary whitespace
    fn minify_css(&self, css: &str) -> String {
        css.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("")
            .replace(" {", "{")
            .replace("} ", "}")
            .replace("; ", ";")
            .replace(" ", "")
    }
}

impl Default for CssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_generator_creation() {
        let generator = CssGenerator::new();
        assert_eq!(generator.rule_count(), 0);
        assert!(!generator.breakpoints.is_empty());
    }

    #[test]
    fn test_add_class() {
        let mut generator = CssGenerator::new();
        generator.add_class("p-4").unwrap();
        
        assert_eq!(generator.rule_count(), 1);
        let rules = generator.get_rules();
        assert!(rules.contains_key("p-4"));
    }

    #[test]
    fn test_generate_css() {
        let mut generator = CssGenerator::new();
        generator.add_class("p-4").unwrap();
        generator.add_class("bg-blue-500").unwrap();
        
        let css = generator.generate_css();
        assert!(css.contains(".p-4"));
        assert!(css.contains("padding: 1rem"));
        assert!(css.contains(".bg-blue-500"));
        assert!(css.contains("background-color: #3b82f6"));
    }

    #[test]
    fn test_responsive_class() {
        let mut generator = CssGenerator::new();
        generator.add_responsive_class(Breakpoint::Md, "p-4").unwrap();
        
        let css = generator.generate_css();
        assert!(css.contains("@media (min-width: 768px)"));
        assert!(css.contains("md:p-4"));
    }

    #[test]
    fn test_custom_properties() {
        let mut generator = CssGenerator::new();
        generator.add_custom_property("primary-color", "#3b82f6");
        
        let css = generator.generate_css();
        assert!(css.contains(":root"));
        assert!(css.contains("--primary-color: #3b82f6"));
    }

    #[test]
    fn test_minified_css() {
        let mut generator = CssGenerator::new();
        generator.add_class("p-4").unwrap();
        
        let minified = generator.generate_minified_css();
        assert!(!minified.contains('\n'));
        assert!(!minified.contains(' '));
    }

    #[test]
    fn test_unknown_class() {
        let mut generator = CssGenerator::new();
        let result = generator.add_class("unknown-class");
        assert!(result.is_err());
    }
}
