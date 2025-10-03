//! Core CSS Generation Logic
//!
//! This module contains the main CSS generation logic,
//! orchestrating parsers and generating final CSS output.

use crate::css_generator::types::{CssProperty, CssRule};
use crate::error::{Result, TailwindError};
use crate::responsive::Breakpoint;
use std::collections::HashMap;
use super::generator_parsers::CssGeneratorParsers;
use super::element_context::ElementContext;

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
    /// Context for element state (gradients, shadows, transforms, etc.)
    element_context: ElementContext,
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
            element_context: ElementContext::default(),
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

    /// Add a gradient stop to the current gradient context
    pub fn add_gradient_stop(&mut self, stop_type: &str, color: String) {
        match stop_type {
            "from" => self.element_context.gradients.from_color = Some(color),
            "via" => self.element_context.gradients.via_color = Some(color),
            "to" => self.element_context.gradients.to_color = Some(color),
            _ => {}
        }
    }

    /// Generate gradient CSS using current context and direction
    pub fn generate_gradient_css(&mut self, direction: &str) -> Option<String> {
        self.element_context.gradients.direction = Some(direction.to_string());

        let mut colors = Vec::new();

        // Add colors in order: from, via, to
        if let Some(from) = &self.element_context.gradients.from_color {
            colors.push(from.clone());
        }
        if let Some(via) = &self.element_context.gradients.via_color {
            colors.push(via.clone());
        }
        if let Some(to) = &self.element_context.gradients.to_color {
            colors.push(to.clone());
        }

        // If no colors collected, return None (let fallback handle it)
        if colors.is_empty() {
            return None;
        }

        // Generate the gradient CSS
        let gradient_css = format!("linear-gradient({}, {})", direction, colors.join(", "));

        // Reset context for next gradient
        self.element_context = ElementContext::default();

        Some(gradient_css)
    }

    /// Clear gradient context (useful for resetting between elements)
    pub fn clear_gradient_context(&mut self) {
        self.element_context = ElementContext::default();
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

    /// PRIMARY API: Process multiple classes for an element (handles stateful combinations)
    ///
    /// This is the new architecture that matches how real Tailwind CSS works.
    /// Classes are processed in context, allowing gradients and other stateful utilities
    /// to work correctly together.
    pub fn process_element_classes(&mut self, classes: &[&str]) -> String {
        // Reset element context for new element
        self.element_context = ElementContext::default();

        // First pass: collect stateful information from all classes
        for class in classes {
            self.element_context.update_from_class(class);
        }

        // Generate variant-aware CSS rules for all stateful classes
        let mut css_output = String::new();

        // Collect all stateful classes (gradients, shadows, transforms)
        let stateful_classes: Vec<&str> = classes.iter()
            .filter(|class| {
                class.starts_with("bg-gradient-") || class.starts_with("bg-linear-") ||
                class.starts_with("bg-conic") || class.starts_with("bg-radial") ||
                class.starts_with("from-") || class.starts_with("via-") || class.starts_with("to-") ||
                class.starts_with("shadow-") || class.starts_with("scale-") ||
                class.starts_with("rotate-") || class.starts_with("translate-") || class.starts_with("skew-")
            })
            .copied()
            .collect();

        // Generate variant-aware CSS for each stateful class
        for class in stateful_classes {
            let rules = self.element_context.generate_variant_css(class);
            for rule in rules {
                css_output.push_str(&self.rule_to_css(&rule));
                css_output.push('\n');
            }
        }

        // Handle non-stateful classes with existing logic
        for class in classes {
            if !class.starts_with("bg-gradient-") && !class.starts_with("bg-linear-") &&
               !class.starts_with("bg-conic") && !class.starts_with("bg-radial") &&
               !class.starts_with("from-") && !class.starts_with("via-") && !class.starts_with("to-") &&
               !class.starts_with("shadow-") && !class.starts_with("scale-") &&
               !class.starts_with("rotate-") && !class.starts_with("translate-") && !class.starts_with("skew-") {

                if let Some(css) = self.generate_class_css_with_context(class) {
                    css_output.push_str(&css);
                    css_output.push('\n');
                }
            }
        }

        css_output.trim().to_string()
    }

    /// Generate CSS for a class using full element context
    fn generate_class_css_with_context(&mut self, class: &str) -> Option<String> {
        // Handle stateful classes that require element context
        if class.starts_with("bg-gradient-") || class.starts_with("bg-linear-") ||
           class.starts_with("bg-conic") || class.starts_with("bg-radial") ||
           class.starts_with("from-") || class.starts_with("via-") || class.starts_with("to-") ||
           class.starts_with("shadow-") || class.starts_with("scale-") ||
           class.starts_with("rotate-") || class.starts_with("translate-") || class.starts_with("skew-") {

            // Generate CSS if we have any stateful context (gradients, shadows, transforms)
            let has_context = self.element_context.gradients.has_gradient() ||
                            self.element_context.shadows.has_shadow() ||
                            self.element_context.transforms.has_transform();

            if has_context {
                let properties = self.element_context.generate_css();
                return Some(self.format_css_properties(class, &properties));
            }
            return None;
        }

        // For non-stateful classes, use existing logic
        match self.class_to_css_rule(class) {
            Ok(rule) => Some(self.rule_to_css(&rule)),
            Err(_) => None,
        }
    }

    /// Format CSS properties into CSS rule string
    fn format_css_properties(&self, class: &str, properties: &[CssProperty]) -> String {
        if properties.is_empty() {
            return String::new();
        }

        let selector = format!(".{}", class.replace(":", "\\:"));
        let mut css = format!("{} {{\n", selector);

        for property in properties {
            css.push_str(&format!("  {}: {};\n", property.name, property.value));
        }

        css.push('}');
        css
    }

    /// LEGACY API: For backward compatibility (limited functionality)
    ///
    /// WARNING: This method cannot handle stateful utilities like gradients correctly.
    /// Use process_element_classes() for full functionality.
    pub fn class_to_css_rule(&self, class: &str) -> Result<CssRule> {
        eprintln!("Warning: class_to_css_rule is deprecated. Use process_element_classes for full functionality.");

        // For now, delegate to the parsers (limited functionality)
        CssGeneratorParsers::class_to_css_rule(self, class)
    }

    /// LEGACY API: Convert CssRule to CSS string
    ///
    /// This is kept for backward compatibility but is deprecated.
    pub fn rule_to_css(&self, rule: &CssRule) -> String {
        let mut css = format!("{} {{\n", rule.selector);
        for property in &rule.properties {
            let important = if property.important { " !important" } else { "" };
            css.push_str(&format!("  {}: {}{};\n", property.name, property.value, important));
        }
        css.push('}');
        css
    }

    /// Clear all rules
    pub fn clear(&mut self) {
        self.rules.clear();
        self.custom_properties.clear();
        self.element_context = ElementContext::default();
    }
}