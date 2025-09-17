//! # Responsive Value Handling
//!
//! This module provides utilities for handling responsive values across different breakpoints.

use super::breakpoints::Breakpoint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A responsive value that can have different values for different breakpoints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveValue<T> {
    /// Values for each breakpoint
    pub values: HashMap<Breakpoint, T>,
}

impl<T> ResponsiveValue<T> {
    /// Create a new responsive value
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    /// Create a responsive value with a base value
    pub fn with_base(base: T) -> Self {
        let mut values = HashMap::new();
        values.insert(Breakpoint::Base, base);
        Self { values }
    }
    
    /// Set a value for a specific breakpoint
    pub fn set_breakpoint(&mut self, breakpoint: Breakpoint, value: T) {
        self.values.insert(breakpoint, value);
    }
    
    /// Get a value for a specific breakpoint
    pub fn get_breakpoint(&self, breakpoint: Breakpoint) -> Option<&T> {
        self.values.get(&breakpoint)
    }
    
    /// Get a value for a specific breakpoint, falling back to the base value
    pub fn get_breakpoint_or_base(&self, breakpoint: Breakpoint) -> Option<&T> {
        self.values.get(&breakpoint).or_else(|| self.values.get(&Breakpoint::Base))
    }
    
    /// Get the base value
    pub fn get_base(&self) -> Option<&T> {
        self.values.get(&Breakpoint::Base)
    }
    
    /// Check if a breakpoint has a value
    pub fn has_breakpoint(&self, breakpoint: Breakpoint) -> bool {
        self.values.contains_key(&breakpoint)
    }
    
    /// Get all breakpoints that have values
    pub fn get_breakpoints(&self) -> Vec<Breakpoint> {
        self.values.keys().cloned().collect()
    }
    
    /// Check if the responsive value is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    
    /// Get the number of breakpoints with values
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    /// Clear all values
    pub fn clear(&mut self) {
        self.values.clear();
    }
    
    /// Remove a value for a specific breakpoint
    pub fn remove_breakpoint(&mut self, breakpoint: Breakpoint) -> Option<T> {
        self.values.remove(&breakpoint)
    }
    
    /// Get the value for the most appropriate breakpoint based on screen width
    pub fn get_for_width(&self, screen_width: u32) -> Option<&T> {
        // Find the highest breakpoint that is active for this screen width
        let active_breakpoints: Vec<Breakpoint> = self.values
            .keys()
            .filter(|&&bp| screen_width >= bp.min_width())
            .cloned()
            .collect();
        
        if active_breakpoints.is_empty() {
            return self.get_base();
        }
        
        // Find the breakpoint with the highest min_width among active ones
        let best_breakpoint = active_breakpoints
            .into_iter()
            .max_by_key(|bp| bp.min_width())?;
        
        self.get_breakpoint(best_breakpoint)
    }
    
    /// Generate CSS classes for all breakpoints
    pub fn to_css_classes<F>(&self, class_generator: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut classes = Vec::new();
        
        for breakpoint in Breakpoint::all() {
            if let Some(value) = self.get_breakpoint(breakpoint) {
                let class = class_generator(value);
                if !class.is_empty() {
                    if breakpoint == Breakpoint::Base {
                        classes.push(class);
                    } else {
                        classes.push(format!("{}{}", breakpoint.prefix(), class));
                    }
                }
            }
        }
        
        classes.join(" ")
    }
}

impl<T> Default for ResponsiveValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<T> for ResponsiveValue<T> {
    fn from(value: T) -> Self {
        Self::with_base(value)
    }
}

impl<T> From<HashMap<Breakpoint, T>> for ResponsiveValue<T> {
    fn from(values: HashMap<Breakpoint, T>) -> Self {
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_value_new() {
        let rv = ResponsiveValue::<String>::new();
        assert!(rv.is_empty());
        assert_eq!(rv.len(), 0);
    }

    #[test]
    fn test_responsive_value_with_base() {
        let rv = ResponsiveValue::with_base("base".to_string());
        assert!(!rv.is_empty());
        assert_eq!(rv.len(), 1);
        assert_eq!(rv.get_base(), Some(&"base".to_string()));
    }

    #[test]
    fn test_responsive_value_set_get_breakpoint() {
        let mut rv = ResponsiveValue::new();
        rv.set_breakpoint(Breakpoint::Base, "base".to_string());
        rv.set_breakpoint(Breakpoint::Sm, "sm".to_string());
        
        assert_eq!(rv.get_breakpoint(Breakpoint::Base), Some(&"base".to_string()));
        assert_eq!(rv.get_breakpoint(Breakpoint::Sm), Some(&"sm".to_string()));
        assert_eq!(rv.get_breakpoint(Breakpoint::Md), None);
    }

    #[test]
    fn test_responsive_value_get_breakpoint_or_base() {
        let mut rv = ResponsiveValue::new();
        rv.set_breakpoint(Breakpoint::Base, "base".to_string());
        rv.set_breakpoint(Breakpoint::Sm, "sm".to_string());
        
        assert_eq!(rv.get_breakpoint_or_base(Breakpoint::Base), Some(&"base".to_string()));
        assert_eq!(rv.get_breakpoint_or_base(Breakpoint::Sm), Some(&"sm".to_string()));
        assert_eq!(rv.get_breakpoint_or_base(Breakpoint::Md), Some(&"base".to_string()));
    }

    #[test]
    fn test_responsive_value_has_breakpoint() {
        let mut rv = ResponsiveValue::new();
        rv.set_breakpoint(Breakpoint::Base, "base".to_string());
        rv.set_breakpoint(Breakpoint::Sm, "sm".to_string());
        
        assert!(rv.has_breakpoint(Breakpoint::Base));
        assert!(rv.has_breakpoint(Breakpoint::Sm));
        assert!(!rv.has_breakpoint(Breakpoint::Md));
    }

    #[test]
    fn test_responsive_value_get_breakpoints() {
        let mut rv = ResponsiveValue::new();
        rv.set_breakpoint(Breakpoint::Base, "base".to_string());
        rv.set_breakpoint(Breakpoint::Sm, "sm".to_string());
        rv.set_breakpoint(Breakpoint::Lg, "lg".to_string());
        
        let breakpoints = rv.get_breakpoints();
        assert_eq!(breakpoints.len(), 3);
        assert!(breakpoints.contains(&Breakpoint::Base));
        assert!(breakpoints.contains(&Breakpoint::Sm));
        assert!(breakpoints.contains(&Breakpoint::Lg));
    }

    #[test]
    fn test_responsive_value_clear() {
        let mut rv = ResponsiveValue::with_base("base".to_string());
        rv.set_breakpoint(Breakpoint::Sm, "sm".to_string());
        
        assert!(!rv.is_empty());
        rv.clear();
        assert!(rv.is_empty());
    }

    #[test]
    fn test_responsive_value_remove_breakpoint() {
        let mut rv = ResponsiveValue::with_base("base".to_string());
        rv.set_breakpoint(Breakpoint::Sm, "sm".to_string());
        
        assert_eq!(rv.len(), 2);
        let removed = rv.remove_breakpoint(Breakpoint::Sm);
        assert_eq!(removed, Some("sm".to_string()));
        assert_eq!(rv.len(), 1);
        assert!(!rv.has_breakpoint(Breakpoint::Sm));
    }

    #[test]
    fn test_responsive_value_get_for_width() {
        let mut rv = ResponsiveValue::new();
        rv.set_breakpoint(Breakpoint::Base, "base".to_string());
        rv.set_breakpoint(Breakpoint::Sm, "sm".to_string());
        rv.set_breakpoint(Breakpoint::Md, "md".to_string());
        
        // Test width 0 (base only)
        assert_eq!(rv.get_for_width(0), Some(&"base".to_string()));
        
        // Test width 640 (sm active)
        assert_eq!(rv.get_for_width(640), Some(&"sm".to_string()));
        
        // Test width 768 (md active)
        assert_eq!(rv.get_for_width(768), Some(&"md".to_string()));
        
        // Test width 1000 (md still active)
        assert_eq!(rv.get_for_width(1000), Some(&"md".to_string()));
    }

    #[test]
    fn test_responsive_value_to_css_classes() {
        let mut rv = ResponsiveValue::new();
        rv.set_breakpoint(Breakpoint::Base, "text-sm".to_string());
        rv.set_breakpoint(Breakpoint::Sm, "text-base".to_string());
        rv.set_breakpoint(Breakpoint::Md, "text-lg".to_string());
        
        let classes = rv.to_css_classes(|v| v.clone());
        assert!(classes.contains("text-sm"));
        assert!(classes.contains("sm:text-base"));
        assert!(classes.contains("md:text-lg"));
    }

    #[test]
    fn test_responsive_value_from() {
        let rv = ResponsiveValue::from("base".to_string());
        assert_eq!(rv.get_base(), Some(&"base".to_string()));
        
        let mut map = HashMap::new();
        map.insert(Breakpoint::Base, "base".to_string());
        map.insert(Breakpoint::Sm, "sm".to_string());
        
        let rv = ResponsiveValue::from(map);
        assert_eq!(rv.len(), 2);
        assert_eq!(rv.get_breakpoint(Breakpoint::Base), Some(&"base".to_string()));
        assert_eq!(rv.get_breakpoint(Breakpoint::Sm), Some(&"sm".to_string()));
    }
}
