//! # Responsive Builder Pattern
//!
//! This module provides a builder pattern for creating responsive classes.

use super::breakpoints::Breakpoint;
use super::responsive_config::ResponsiveConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Builder for creating responsive classes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveBuilder {
    /// Classes for each breakpoint
    classes: HashMap<Breakpoint, Vec<String>>,
    /// Configuration
    config: ResponsiveConfig,
}

impl ResponsiveBuilder {
    /// Create a new responsive builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a responsive builder with custom configuration
    pub fn with_config(config: ResponsiveConfig) -> Self {
        Self {
            classes: HashMap::new(),
            config,
        }
    }
    
    /// Add a class for a specific breakpoint
    pub fn add_class(&mut self, breakpoint: Breakpoint, class: impl Into<String>) -> &mut Self {
        self.classes
            .entry(breakpoint)
            .or_insert_with(Vec::new)
            .push(class.into());
        self
    }
    
    /// Add classes for a specific breakpoint
    pub fn add_classes(&mut self, breakpoint: Breakpoint, classes: Vec<String>) -> &mut Self {
        self.classes
            .entry(breakpoint)
            .or_insert_with(Vec::new)
            .extend(classes);
        self
    }
    
    /// Add a class for the base breakpoint
    pub fn base(&mut self, class: impl Into<String>) -> &mut Self {
        self.add_class(Breakpoint::Base, class)
    }
    
    /// Add a class for the small breakpoint
    pub fn sm(&mut self, class: impl Into<String>) -> &mut Self {
        self.add_class(Breakpoint::Sm, class)
    }
    
    /// Add a class for the medium breakpoint
    pub fn md(&mut self, class: impl Into<String>) -> &mut Self {
        self.add_class(Breakpoint::Md, class)
    }
    
    /// Add a class for the large breakpoint
    pub fn lg(&mut self, class: impl Into<String>) -> &mut Self {
        self.add_class(Breakpoint::Lg, class)
    }
    
    /// Add a class for the extra large breakpoint
    pub fn xl(&mut self, class: impl Into<String>) -> &mut Self {
        self.add_class(Breakpoint::Xl, class)
    }
    
    /// Add a class for the 2X large breakpoint
    pub fn xl2(&mut self, class: impl Into<String>) -> &mut Self {
        self.add_class(Breakpoint::Xl2, class)
    }
    
    /// Add responsive classes for all breakpoints
    pub fn responsive(&mut self, base: impl Into<String>, sm: Option<String>, md: Option<String>, lg: Option<String>, xl: Option<String>, xl2: Option<String>) -> &mut Self {
        self.base(base);
        
        if let Some(sm_class) = sm {
            self.sm(sm_class);
        }
        if let Some(md_class) = md {
            self.md(md_class);
        }
        if let Some(lg_class) = lg {
            self.lg(lg_class);
        }
        if let Some(xl_class) = xl {
            self.xl(xl_class);
        }
        if let Some(xl2_class) = xl2 {
            self.xl2(xl2_class);
        }
        
        self
    }
    
    /// Get classes for a specific breakpoint
    pub fn get_classes(&self, breakpoint: Breakpoint) -> Vec<String> {
        self.classes.get(&breakpoint).cloned().unwrap_or_default()
    }
    
    /// Get all classes for all breakpoints
    pub fn get_all_classes(&self) -> HashMap<Breakpoint, Vec<String>> {
        self.classes.clone()
    }
    
    /// Check if the builder is empty
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty() || self.classes.values().all(|classes| classes.is_empty())
    }
    
    /// Get the number of breakpoints with classes
    pub fn len(&self) -> usize {
        self.classes.len()
    }
    
    /// Clear all classes
    pub fn clear(&mut self) {
        self.classes.clear();
    }
    
    /// Remove classes for a specific breakpoint
    pub fn remove_breakpoint(&mut self, breakpoint: Breakpoint) -> Vec<String> {
        self.classes.remove(&breakpoint).unwrap_or_default()
    }
    
    /// Build the final CSS classes string
    pub fn build(&self) -> String {
        let mut classes = Vec::new();
        
        // Add classes in breakpoint order
        for breakpoint in Breakpoint::all() {
            if let Some(breakpoint_classes) = self.classes.get(&breakpoint) {
                if !breakpoint_classes.is_empty() {
                    let breakpoint_classes_str = breakpoint_classes.join(" ");
                    if breakpoint == Breakpoint::Base {
                        classes.push(breakpoint_classes_str);
                    } else {
                        classes.push(format!("{}{}", breakpoint.prefix(), breakpoint_classes_str));
                    }
                }
            }
        }
        
        classes.join(" ")
    }
    
    /// Build classes for a specific screen width
    pub fn build_for_width(&self, screen_width: u32) -> String {
        let mut classes = Vec::new();
        
        // Find the appropriate breakpoint for this screen width
        let target_breakpoint = self.config.get_breakpoint_for_width(screen_width);
        
        // Add classes from base up to the target breakpoint
        for breakpoint in Breakpoint::all() {
            if breakpoint.min_width() <= target_breakpoint.min_width() {
                if let Some(breakpoint_classes) = self.classes.get(&breakpoint) {
                    if !breakpoint_classes.is_empty() {
                        let breakpoint_classes_str = breakpoint_classes.join(" ");
                        if breakpoint == Breakpoint::Base {
                            classes.push(breakpoint_classes_str);
                        } else {
                            classes.push(format!("{}{}", breakpoint.prefix(), breakpoint_classes_str));
                        }
                    }
                }
            }
        }
        
        classes.join(" ")
    }
    
    /// Get the configuration
    pub fn get_config(&self) -> &ResponsiveConfig {
        &self.config
    }
    
    /// Update the configuration
    pub fn update_config(&mut self, config: ResponsiveConfig) {
        self.config = config;
    }
}

impl Default for ResponsiveBuilder {
    fn default() -> Self {
        Self {
            classes: HashMap::new(),
            config: ResponsiveConfig::default(),
        }
    }
}

impl std::fmt::Display for ResponsiveBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_builder_new() {
        let builder = ResponsiveBuilder::new();
        assert!(builder.is_empty());
        assert_eq!(builder.len(), 0);
    }

    #[test]
    fn test_responsive_builder_add_class() {
        let mut builder = ResponsiveBuilder::new();
        builder.add_class(Breakpoint::Base, "text-sm");
        builder.add_class(Breakpoint::Sm, "text-base");
        
        assert!(!builder.is_empty());
        assert_eq!(builder.len(), 2);
        assert_eq!(builder.get_classes(Breakpoint::Base), vec!["text-sm"]);
        assert_eq!(builder.get_classes(Breakpoint::Sm), vec!["text-base"]);
    }

    #[test]
    fn test_responsive_builder_add_classes() {
        let mut builder = ResponsiveBuilder::new();
        builder.add_classes(Breakpoint::Base, vec!["text-sm".to_string(), "font-medium".to_string()]);
        
        assert_eq!(builder.get_classes(Breakpoint::Base), vec!["text-sm", "font-medium"]);
    }

    #[test]
    fn test_responsive_builder_breakpoint_methods() {
        let mut builder = ResponsiveBuilder::new();
        builder.base("text-sm");
        builder.sm("text-base");
        builder.md("text-lg");
        builder.lg("text-xl");
        builder.xl("text-2xl");
        builder.xl2("text-3xl");
        
        assert_eq!(builder.get_classes(Breakpoint::Base), vec!["text-sm"]);
        assert_eq!(builder.get_classes(Breakpoint::Sm), vec!["text-base"]);
        assert_eq!(builder.get_classes(Breakpoint::Md), vec!["text-lg"]);
        assert_eq!(builder.get_classes(Breakpoint::Lg), vec!["text-xl"]);
        assert_eq!(builder.get_classes(Breakpoint::Xl), vec!["text-2xl"]);
        assert_eq!(builder.get_classes(Breakpoint::Xl2), vec!["text-3xl"]);
    }

    #[test]
    fn test_responsive_builder_responsive() {
        let mut builder = ResponsiveBuilder::new();
        builder.responsive(
            "text-sm",
            Some("text-base".to_string()),
            Some("text-lg".to_string()),
            None,
            None,
            None,
        );
        
        assert_eq!(builder.get_classes(Breakpoint::Base), vec!["text-sm"]);
        assert_eq!(builder.get_classes(Breakpoint::Sm), vec!["text-base"]);
        assert_eq!(builder.get_classes(Breakpoint::Md), vec!["text-lg"]);
        assert_eq!(builder.get_classes(Breakpoint::Lg), Vec::<String>::new());
    }

    #[test]
    fn test_responsive_builder_build() {
        let mut builder = ResponsiveBuilder::new();
        builder.base("text-sm");
        builder.sm("text-base");
        builder.md("text-lg");
        
        let result = builder.build();
        assert!(result.contains("text-sm"));
        assert!(result.contains("sm:text-base"));
        assert!(result.contains("md:text-lg"));
    }

    #[test]
    fn test_responsive_builder_build_for_width() {
        let mut builder = ResponsiveBuilder::new();
        builder.base("text-sm");
        builder.sm("text-base");
        builder.md("text-lg");
        
        // Test width 0 (base only)
        let result_0 = builder.build_for_width(0);
        assert!(result_0.contains("text-sm"));
        assert!(!result_0.contains("sm:"));
        assert!(!result_0.contains("md:"));
        
        // Test width 640 (sm active)
        let result_640 = builder.build_for_width(640);
        assert!(result_640.contains("text-sm"));
        assert!(result_640.contains("sm:text-base"));
        assert!(!result_640.contains("md:"));
        
        // Test width 768 (md active)
        let result_768 = builder.build_for_width(768);
        assert!(result_768.contains("text-sm"));
        assert!(result_768.contains("sm:text-base"));
        assert!(result_768.contains("md:text-lg"));
    }

    #[test]
    fn test_responsive_builder_clear() {
        let mut builder = ResponsiveBuilder::new();
        builder.base("text-sm");
        builder.sm("text-base");
        
        assert!(!builder.is_empty());
        builder.clear();
        assert!(builder.is_empty());
    }

    #[test]
    fn test_responsive_builder_remove_breakpoint() {
        let mut builder = ResponsiveBuilder::new();
        builder.base("text-sm");
        builder.sm("text-base");
        
        assert_eq!(builder.len(), 2);
        let removed = builder.remove_breakpoint(Breakpoint::Sm);
        assert_eq!(removed, vec!["text-base"]);
        assert_eq!(builder.len(), 1);
        assert_eq!(builder.get_classes(Breakpoint::Sm), Vec::<String>::new());
    }

    #[test]
    fn test_responsive_builder_display() {
        let mut builder = ResponsiveBuilder::new();
        builder.base("text-sm");
        builder.sm("text-base");
        
        let result = format!("{}", builder);
        assert!(result.contains("text-sm"));
        assert!(result.contains("sm:text-base"));
    }
}
