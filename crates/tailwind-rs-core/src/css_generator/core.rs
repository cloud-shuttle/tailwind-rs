//! Core CSS Generation Logic
//!
//! This module contains the main CSS generation logic,
//! orchestrating parsers and generating final CSS output.

use crate::css_generator::types::{CssGenerationConfig, CssProperty, CssRule};
use crate::error::{Result, TailwindError};
use crate::responsive::Breakpoint;
use std::collections::HashMap;

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
                .or_insert_with(Vec::new)
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

    /// Convert a class name to a CSS rule
    fn class_to_css_rule(&self, class: &str) -> Result<CssRule> {
        let (variants, base_class) = self.parse_variants(class);
        let properties = self.class_to_properties(class)?;

        // Build selector with variants
        let mut selector = String::new();
        for variant in &variants {
            match variant.as_str() {
                "dark" => selector.push_str(".dark "),
                "hover" => selector.push_str(":hover"),
                "focus" => selector.push_str(":focus"),
                "active" => selector.push_str(":active"),
                "visited" => selector.push_str(":visited"),
                "disabled" => selector.push_str(":disabled"),
                "group-hover" => selector.push_str(".group:hover "),
                "group-focus" => selector.push_str(".group:focus "),
                "group-active" => selector.push_str(".group:active "),
                "group-disabled" => selector.push_str(".group:disabled "),
                "peer-hover" => selector.push_str(".peer:hover "),
                "peer-focus" => selector.push_str(".peer:focus "),
                "peer-active" => selector.push_str(".peer:active "),
                "peer-disabled" => selector.push_str(".peer:disabled "),
                "first" => selector.push_str(":first-child"),
                "last" => selector.push_str(":last-child"),
                "odd" => selector.push_str(":nth-child(odd)"),
                "even" => selector.push_str(":nth-child(even)"),
                // Device variants
                "pointer-coarse" => selector.push_str("@media (pointer: coarse) "),
                "pointer-fine" => selector.push_str("@media (pointer: fine) "),
                "motion-reduce" => selector.push_str("@media (prefers-reduced-motion: reduce) "),
                "motion-safe" => {
                    selector.push_str("@media (prefers-reduced-motion: no-preference) ")
                }
                "light" => selector.push_str("@media (prefers-color-scheme: light) "),
                _ => {} // Responsive variants handled separately
            }
        }

        // Add the base class
        selector.push_str(&format!(".{}", base_class));

        // Determine media query for responsive variants
        let media_query = variants
            .iter()
            .find(|v| matches!(v.as_str(), "sm" | "md" | "lg" | "xl" | "2xl"))
            .and_then(|variant| match variant.as_str() {
                "sm" => Some("(min-width: 640px)"),
                "md" => Some("(min-width: 768px)"),
                "lg" => Some("(min-width: 1024px)"),
                "xl" => Some("(min-width: 1280px)"),
                "2xl" => Some("(min-width: 1536px)"),
                _ => None,
            })
            .map(|s| s.to_string());

        // Determine specificity based on variants
        let specificity = 10 + (variants.len() as u32 * 10);

        Ok(CssRule {
            selector,
            properties,
            media_query,
            specificity,
        })
    }

    /// Parse variants from a class string
    fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
        let mut variants = Vec::new();
        let mut remaining = class.to_string();

        // Parse variants in order of specificity (most specific first)
        let variant_patterns = [
            ("dark:", "dark"),
            ("hover:", "hover"),
            ("focus:", "focus"),
            ("active:", "active"),
            ("visited:", "visited"),
            ("disabled:", "disabled"),
            ("group-hover:", "group-hover"),
            ("group-focus:", "group-focus"),
            ("group-active:", "group-active"),
            ("group-disabled:", "group-disabled"),
            ("peer-hover:", "peer-hover"),
            ("peer-focus:", "peer-focus"),
            ("peer-active:", "peer-active"),
            ("peer-disabled:", "peer-disabled"),
            ("first:", "first"),
            ("last:", "last"),
            ("odd:", "odd"),
            ("even:", "even"),
            // Device variants
            ("pointer-coarse:", "pointer-coarse"),
            ("pointer-fine:", "pointer-fine"),
            ("motion-reduce:", "motion-reduce"),
            ("motion-safe:", "motion-safe"),
            ("light:", "light"),
            // Responsive variants
            ("sm:", "sm"),
            ("md:", "md"),
            ("lg:", "lg"),
            ("xl:", "xl"),
            ("2xl:", "2xl"),
        ];

        for (prefix, variant) in variant_patterns {
            if remaining.starts_with(prefix) {
                variants.push(variant.to_string());
                remaining = remaining
                    .strip_prefix(prefix)
                    .unwrap_or(&remaining)
                    .to_string();
                break; // Only parse one variant at a time for now
            }
        }

        (variants, remaining)
    }

    /// Convert a class name to CSS properties
    fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>> {
        // First, parse variants and get the base class
        let (_variants, base_class) = self.parse_variants(class);

        // Try to parse the base class using comprehensive patterns
        if let Some(properties) = self.parse_spacing_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_color_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_typography_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_layout_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_flexbox_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_grid_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_border_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_effects_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_transform_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_animation_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_interactivity_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_sizing_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_background_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_filter_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_transition_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_text_shadow_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_mask_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_logical_properties_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_enhanced_backdrop_filter_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_modern_css_features_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_device_variant_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_css_nesting_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_advanced_plugin_system_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_enhanced_validation_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_advanced_performance_optimization_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_container_query_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_color_function_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_performance_optimization_class(&base_class) {
            return Ok(properties);
        }

        if let Some(properties) = self.parse_advanced_animation_class(&base_class) {
            return Ok(properties);
        }

        // Fallback to hardcoded classes for backwards compatibility
        match base_class.as_str() {
            // Display utilities
            "block" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "block".to_string(),
                important: false,
            }]),
            "inline" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "inline".to_string(),
                important: false,
            }]),
            "flex" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "flex".to_string(),
                important: false,
            }]),
            "grid" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "grid".to_string(),
                important: false,
            }]),
            "hidden" => Ok(vec![CssProperty {
                name: "display".to_string(),
                value: "none".to_string(),
                important: false,
            }]),

            _ => Err(TailwindError::class_generation(format!(
                "Unknown class: {}",
                class
            ))),
        }
    }

    // Placeholder methods for parser delegation
    // These will be moved to individual parser modules
    fn parse_spacing_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_color_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_typography_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_layout_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_flexbox_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_grid_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_border_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_effects_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_transform_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_animation_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_interactivity_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_sizing_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_background_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_filter_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_transition_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("transition-") {
            let value = &class[11..];
            return Some(vec![CssProperty {
                name: "transition-property".to_string(),
                value: value.to_string(),
                important: false,
            }]);
        }

        if class.starts_with("duration-") {
            let value = &class[9..];
            return Some(vec![CssProperty {
                name: "transition-duration".to_string(),
                value: format!("{}ms", value),
                important: false,
            }]);
        }

        if class.starts_with("ease-") {
            let value = &class[5..];
            return Some(vec![CssProperty {
                name: "transition-timing-function".to_string(),
                value: self.parse_ease_value(value),
                important: false,
            }]);
        }

        if class.starts_with("delay-") {
            let value = &class[6..];
            return Some(vec![CssProperty {
                name: "transition-delay".to_string(),
                value: format!("{}ms", value),
                important: false,
            }]);
        }

        None
    }

    /// Parse ease values for transition timing functions
    fn parse_ease_value(&self, value: &str) -> String {
        match value {
            "linear" => "linear".to_string(),
            "in" => "cubic-bezier(0.4, 0, 1, 1)".to_string(),
            "out" => "cubic-bezier(0, 0, 0.2, 1)".to_string(),
            "in-out" => "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
            _ => value.to_string(),
        }
    }

    fn parse_text_shadow_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_mask_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_logical_properties_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_enhanced_backdrop_filter_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_modern_css_features_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_device_variant_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_css_nesting_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_advanced_plugin_system_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_enhanced_validation_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_advanced_performance_optimization_class(
        &self,
        _class: &str,
    ) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_container_query_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_color_function_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_performance_optimization_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
    fn parse_advanced_animation_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        None
    }
}
