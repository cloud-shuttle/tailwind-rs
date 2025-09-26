//! CSS Generator Operations
//!
//! This module contains the add/remove/update operations for CssGenerator.

use crate::error::Result;
use crate::responsive::Breakpoint;
use super::types::{CssRule, CssProperty};

/// Operations trait for CssGenerator
pub trait CssGeneratorOperations {
    /// Add a class to the generator
    fn add_class(&mut self, class: &str) -> Result<()>;
    
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

impl CssGeneratorOperations for super::CssGenerator {
    fn add_class(&mut self, class: &str) -> Result<()> {
        let rule = self.class_to_css_rule(class)?;
        self.rules.insert(class.to_string(), rule);
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
        self.custom_properties.insert(name.to_string(), value.to_string());
    }

    fn remove_rule(&mut self, selector: &str) {
        self.rules.remove(selector);
    }

    fn update_rule(&mut self, selector: &str, rule: CssRule) {
        self.rules.insert(selector.to_string(), rule);
    }
}
