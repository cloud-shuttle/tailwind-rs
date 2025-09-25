//! Main CSS Generator implementation
//!
//! This module contains the core CssGenerator struct and its main functionality.

use crate::error::{Result, TailwindError};
use crate::responsive::Breakpoint;
use std::collections::HashMap;
use super::types::{CssRule, CssProperty, CssGenerationConfig};
use super::variants::VariantParser;
use super::parsers::{SpacingParser, AnimationParser, UtilityParser};

/// CSS generator that converts Tailwind classes to CSS rules
#[derive(Debug, Clone)]
pub struct CssGenerator {
    /// Generated CSS rules
    rules: HashMap<String, CssRule>,
    /// Responsive breakpoints
    breakpoints: HashMap<Breakpoint, String>,
    /// Custom CSS properties
    custom_properties: HashMap<String, String>,
    /// Generation configuration
    config: CssGenerationConfig,
    /// Spacing parser
    spacing_parser: SpacingParser,
    /// Animation parser
    animation_parser: AnimationParser,
    /// Variant parser
    variant_parser: VariantParser,
}

impl CssGenerator {
    /// Create a new CSS generator
    pub fn new() -> Self {
        let mut generator = Self {
            rules: HashMap::new(),
            breakpoints: HashMap::new(),
            custom_properties: HashMap::new(),
            config: CssGenerationConfig::default(),
            spacing_parser: SpacingParser::new(),
            animation_parser: AnimationParser::new(),
            variant_parser: VariantParser::new(),
        };
        
        // Initialize default breakpoints
        generator.breakpoints.insert(Breakpoint::Sm, "(min-width: 640px)".to_string());
        generator.breakpoints.insert(Breakpoint::Md, "(min-width: 768px)".to_string());
        generator.breakpoints.insert(Breakpoint::Lg, "(min-width: 1024px)".to_string());
        generator.breakpoints.insert(Breakpoint::Xl, "(min-width: 1280px)".to_string());
        generator.breakpoints.insert(Breakpoint::Xl2, "(min-width: 1536px)".to_string());
        
        generator
    }

    /// Create a new CSS generator with custom configuration
    pub fn with_config(config: CssGenerationConfig) -> Self {
        let mut generator = Self {
            rules: HashMap::new(),
            breakpoints: HashMap::new(),
            custom_properties: HashMap::new(),
            config,
            spacing_parser: SpacingParser::new(),
            animation_parser: AnimationParser::new(),
            variant_parser: VariantParser::new(),
        };
        
        // Initialize breakpoints (use custom if provided, otherwise defaults)
        if generator.config.custom_breakpoints.is_empty() {
            generator.breakpoints.insert(Breakpoint::Sm, "(min-width: 640px)".to_string());
            generator.breakpoints.insert(Breakpoint::Md, "(min-width: 768px)".to_string());
            generator.breakpoints.insert(Breakpoint::Lg, "(min-width: 1024px)".to_string());
            generator.breakpoints.insert(Breakpoint::Xl, "(min-width: 1280px)".to_string());
            generator.breakpoints.insert(Breakpoint::Xl2, "(min-width: 1536px)".to_string());
        } else {
            generator.breakpoints = generator.config.custom_breakpoints.clone();
        }
        
        generator
    }

    /// Add a class to the generator
    pub fn add_class(&mut self, class: &str) -> Result<()> {
        let rule = self.class_to_css_rule(class)?;
        self.rules.insert(class.to_string(), rule);
        Ok(())
    }

    /// Add a CSS selector directly (for non-Tailwind CSS selectors)
    pub fn add_css_selector(&mut self, selector: &str, properties: &str) -> Result<()> {
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

    /// Get the current configuration
    pub fn config(&self) -> &CssGenerationConfig {
        &self.config
    }

    /// Update the configuration
    pub fn set_config(&mut self, config: CssGenerationConfig) {
        self.config = config;
    }

    /// Get all generated rules
    pub fn rules(&self) -> &HashMap<String, CssRule> {
        &self.rules
    }

    /// Get the number of generated rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Generate CSS from all added classes
    pub fn generate_css(&self) -> String {
        super::css_output::CssOutputGenerator::generate_css(&self.rules, &self.custom_properties)
    }

    /// Generate minified CSS from all added classes
    pub fn generate_minified_css(&self) -> String {
        super::css_output::CssOutputGenerator::generate_minified_css(&self.rules, &self.custom_properties)
    }

    /// Convert a class name to a CSS rule
    fn class_to_css_rule(&self, class: &str) -> Result<CssRule> {
        let (variants, base_class) = self.parse_variants(class);
        let properties = self.class_to_properties(class)?;
        
        // Build selector with variants
        let mut selector = String::new();
        for variant in &variants {
            let variant_selector = self.variant_parser.get_variant_selector(variant);
            if !variant_selector.is_empty() {
                selector.push_str(&variant_selector);
            }
        }
        
        // Add the base class
        selector.push_str(&format!(".{}", base_class));
        
        // Determine media query for responsive and device variants
        let media_query = variants.iter()
            .find_map(|variant| {
                // Try responsive media query first
                if let Some(responsive_query) = self.variant_parser.get_responsive_media_query(variant) {
                    Some(responsive_query)
                } else {
                    // Try device media query
                    self.variant_parser.get_device_media_query(variant)
                }
            });
        
        Ok(CssRule {
            selector,
            properties,
            media_query: media_query.map(|s| s.to_string()),
            specificity: variants.len() as u32 * 10, // Higher specificity for more variants
        })
    }

    /// Parse variants from a class name and return (variants, base_class)
    fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
        self.variant_parser.parse_variants(class)
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
        
        // Fallback to hardcoded classes for backwards compatibility
        match base_class.as_str() {
            // Display utilities
            "block" => Ok(vec![CssProperty { name: "display".to_string(), value: "block".to_string(), important: false }]),
            "inline" => Ok(vec![CssProperty { name: "display".to_string(), value: "inline".to_string(), important: false }]),
            "flex" => Ok(vec![CssProperty { name: "display".to_string(), value: "flex".to_string(), important: false }]),
            "grid" => Ok(vec![CssProperty { name: "display".to_string(), value: "grid".to_string(), important: false }]),
            "hidden" => Ok(vec![CssProperty { name: "display".to_string(), value: "none".to_string(), important: false }]),
            
            _ => Err(TailwindError::class_generation(format!("Unknown class: {}", class))),
        }
    }

    // Parser methods - delegate to the actual parser modules
    fn parse_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        self.spacing_parser.parse_class(class)
    }
    
    fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        self.animation_parser.parse_class(class)
    }
    
    // Stub methods for parsers not yet implemented
    fn parse_color_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_typography_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_layout_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_flexbox_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_grid_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_border_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_effects_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_transform_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_interactivity_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_sizing_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_background_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_filter_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
    fn parse_transition_class(&self, _class: &str) -> Option<Vec<CssProperty>> { None }
}
