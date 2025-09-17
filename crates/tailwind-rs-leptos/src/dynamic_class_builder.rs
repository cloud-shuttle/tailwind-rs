//! # Dynamic Class Builder for Leptos 0.8.8
//!
//! This module provides efficient class generation using simple string building.
//! No signals are needed for static class concatenation.

/// Efficient class generation using simple string building
pub struct DynamicClassBuilder {
    base_classes: String,
    variant_classes: String,
    responsive_classes: String,
    state_classes: String,
    custom_classes: String,
}

impl DynamicClassBuilder {
    /// Create a new dynamic class builder
    pub fn new() -> Self {
        Self {
            base_classes: String::new(),
            variant_classes: String::new(),
            responsive_classes: String::new(),
            state_classes: String::new(),
            custom_classes: String::new(),
        }
    }
    
    /// Set base classes
    pub fn base(mut self, classes: impl Into<String>) -> Self {
        self.base_classes = classes.into();
        self
    }
    
    /// Set variant classes
    pub fn variant(mut self, classes: impl Into<String>) -> Self {
        self.variant_classes = classes.into();
        self
    }
    
    /// Set responsive classes
    pub fn responsive(mut self, classes: impl Into<String>) -> Self {
        self.responsive_classes = classes.into();
        self
    }
    
    /// Set state classes
    pub fn state(mut self, classes: impl Into<String>) -> Self {
        self.state_classes = classes.into();
        self
    }
    
    /// Set custom classes
    pub fn custom(mut self, classes: impl Into<String>) -> Self {
        self.custom_classes = classes.into();
        self
    }
    
    /// Get the computed classes as a string
    pub fn classes(&self) -> String {
        let mut result = String::new();
        
        // Add base classes
        if !self.base_classes.is_empty() {
            result.push_str(&self.base_classes);
        }
        
        // Add variant classes
        if !self.variant_classes.is_empty() {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&self.variant_classes);
        }
        
        // Add responsive classes
        if !self.responsive_classes.is_empty() {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&self.responsive_classes);
        }
        
        // Add state classes
        if !self.state_classes.is_empty() {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&self.state_classes);
        }
        
        // Add custom classes
        if !self.custom_classes.is_empty() {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&self.custom_classes);
        }
        
        result
    }
    
    /// Check if any classes are set
    pub fn is_empty(&self) -> bool {
        self.base_classes.is_empty() &&
        self.variant_classes.is_empty() &&
        self.responsive_classes.is_empty() &&
        self.state_classes.is_empty() &&
        self.custom_classes.is_empty()
    }
}

impl Default for DynamicClassBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility function to create a dynamic class builder
pub fn dynamic_classes() -> DynamicClassBuilder {
    DynamicClassBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_class_builder_creation() {
        let builder = DynamicClassBuilder::new();
        
        // Initially should be empty
        assert!(builder.is_empty());
        assert_eq!(builder.classes(), "");
    }
    
    #[test]
    fn test_dynamic_class_builder_base_classes() {
        let builder = DynamicClassBuilder::new()
            .base("px-4 py-2");
        
        let classes = builder.classes();
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
        assert!(!builder.is_empty());
    }
    
    #[test]
    fn test_dynamic_class_builder_variant_classes() {
        let builder = DynamicClassBuilder::new()
            .variant("bg-blue-600 text-white");
        
        let classes = builder.classes();
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
    }
    
    #[test]
    fn test_dynamic_class_builder_responsive_classes() {
        let builder = DynamicClassBuilder::new()
            .responsive("sm:text-sm md:text-base");
        
        let classes = builder.classes();
        assert!(classes.contains("sm:text-sm"));
        assert!(classes.contains("md:text-base"));
    }
    
    #[test]
    fn test_dynamic_class_builder_state_classes() {
        let builder = DynamicClassBuilder::new()
            .state("hover:bg-blue-700 focus:ring-2");
        
        let classes = builder.classes();
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("focus:ring-2"));
    }
    
    #[test]
    fn test_dynamic_class_builder_custom_classes() {
        let builder = DynamicClassBuilder::new()
            .custom("my-custom-class");
        
        let classes = builder.classes();
        assert!(classes.contains("my-custom-class"));
    }
    
    #[test]
    fn test_dynamic_class_builder_combined_classes() {
        let builder = DynamicClassBuilder::new()
            .base("px-4 py-2")
            .variant("bg-blue-600 text-white")
            .responsive("sm:text-sm md:text-base")
            .state("hover:bg-blue-700")
            .custom("rounded-lg");
        
        let classes = builder.classes();
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("sm:text-sm"));
        assert!(classes.contains("md:text-base"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("rounded-lg"));
    }
    
    #[test]
    fn test_dynamic_class_builder_empty_strings() {
        let builder = DynamicClassBuilder::new()
            .base("")
            .variant("")
            .responsive("")
            .state("")
            .custom("");
        
        assert!(builder.is_empty());
        assert_eq!(builder.classes(), "");
    }
    
    #[test]
    fn test_dynamic_class_builder_whitespace_handling() {
        let builder = DynamicClassBuilder::new()
            .base("  px-4  py-2  ")
            .variant("  bg-blue-600  ");
        
        let classes = builder.classes();
        // Should preserve the whitespace as provided
        assert!(classes.contains("  px-4  py-2  "));
        assert!(classes.contains("  bg-blue-600  "));
    }
    
    #[test]
    fn test_dynamic_class_builder_fluent_api() {
        let builder = DynamicClassBuilder::new()
            .base("px-4")
            .variant("bg-blue-600")
            .responsive("sm:text-sm")
            .state("hover:bg-blue-700")
            .custom("rounded");
        
        // Should be able to chain all methods
        assert!(!builder.is_empty());
        let classes = builder.classes();
        assert!(classes.contains("px-4"));
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("sm:text-sm"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("rounded"));
    }
    
    #[test]
    fn test_dynamic_class_builder_default() {
        let builder = DynamicClassBuilder::default();
        assert!(builder.is_empty());
        assert_eq!(builder.classes(), "");
    }
    
    #[test]
    fn test_dynamic_classes_utility() {
        let builder = dynamic_classes()
            .base("px-4")
            .variant("bg-blue-600");
        
        assert!(!builder.is_empty());
        let classes = builder.classes();
        assert!(classes.contains("px-4"));
        assert!(classes.contains("bg-blue-600"));
    }
}