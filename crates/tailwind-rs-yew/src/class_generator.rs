//! # Yew Class Generator
//!
//! This module provides Yew-specific class generation utilities.

use std::collections::HashMap;
use tailwind_rs_core::{Breakpoint, ClassBuilder, State};
use yew::prelude::*;

/// Yew-specific class generator
pub struct YewClassGenerator {
    class_cache: HashMap<String, Classes>,
}

impl YewClassGenerator {
    /// Create a new Yew class generator
    pub fn new() -> Self {
        Self {
            class_cache: HashMap::new(),
        }
    }

    /// Generate classes for Yew components
    pub fn generate_yew_classes(&mut self, builder: ClassBuilder) -> Classes {
        let class_set = builder.build();
        let class_string = class_set.to_css_classes();

        // Check cache first
        if let Some(cached_classes) = self.class_cache.get(&class_string) {
            return cached_classes.clone();
        }

        // Create new classes and cache them
        let classes = Classes::from(class_string.clone());
        self.class_cache.insert(class_string, classes.clone());

        classes
    }

    /// Generate conditional classes for Yew components
    pub fn generate_conditional_classes(
        &mut self,
        base_classes: &str,
        condition: bool,
        conditional_classes: &str,
    ) -> Classes {
        let class_string = if condition {
            format!("{} {}", base_classes, conditional_classes)
        } else {
            base_classes.to_string()
        };

        // Check cache first
        if let Some(cached_classes) = self.class_cache.get(&class_string) {
            return cached_classes.clone();
        }

        // Create new classes and cache them
        let classes = Classes::from(class_string.clone());
        self.class_cache.insert(class_string, classes.clone());

        classes
    }

    /// Generate responsive classes for Yew components
    pub fn generate_responsive_classes(
        &mut self,
        base_classes: &str,
        responsive_classes: HashMap<Breakpoint, &str>,
    ) -> Classes {
        let mut class_string = base_classes.to_string();

        for (breakpoint, classes) in &responsive_classes {
            class_string.push(' ');
            class_string.push_str(&format!("{}{}", breakpoint.prefix(), classes));
        }

        // Check cache first
        if let Some(cached_classes) = self.class_cache.get(&class_string) {
            return cached_classes.clone();
        }

        // Create new classes and cache them
        let classes = Classes::from(class_string.clone());
        self.class_cache.insert(class_string, classes.clone());

        classes
    }

    /// Generate state-based classes for Yew components
    pub fn generate_state_classes(
        &mut self,
        base_classes: &str,
        state_classes: HashMap<State, &str>,
    ) -> Classes {
        let mut class_string = base_classes.to_string();

        for (state, classes) in &state_classes {
            class_string.push(' ');
            class_string.push_str(&format!("{}:{}", state.prefix(), classes));
        }

        // Check cache first
        if let Some(cached_classes) = self.class_cache.get(&class_string) {
            return cached_classes.clone();
        }

        // Create new classes and cache them
        let classes = Classes::from(class_string.clone());
        self.class_cache.insert(class_string, classes.clone());

        classes
    }

    /// Clear the class cache
    pub fn clear_cache(&mut self) {
        self.class_cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.class_cache.len()
    }
}

impl Default for YewClassGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yew_class_generator_creation() {
        let generator = YewClassGenerator::new();
        assert_eq!(generator.cache_size(), 0);
    }

    #[test]
    fn test_generate_yew_classes() {
        let mut generator = YewClassGenerator::new();
        let builder = ClassBuilder::new()
            .class("px-4 py-2")
            .class("bg-blue-600 text-white");

        let classes = generator.generate_yew_classes(builder);
        let class_string = classes.to_string();

        assert!(class_string.contains("px-4"));
        assert!(class_string.contains("bg-blue-600"));
    }

    #[test]
    fn test_conditional_class_generation() {
        let mut generator = YewClassGenerator::new();

        let classes = generator.generate_conditional_classes("px-4", true, "bg-blue-600");

        let class_string = classes.to_string();
        assert!(class_string.contains("px-4"));
        assert!(class_string.contains("bg-blue-600"));

        let classes = generator.generate_conditional_classes("px-4", false, "bg-blue-600");

        let class_string = classes.to_string();
        assert!(class_string.contains("px-4"));
        assert!(!class_string.contains("bg-blue-600"));
    }

    #[test]
    fn test_responsive_class_generation() {
        let mut generator = YewClassGenerator::new();
        let mut responsive_classes = HashMap::new();
        responsive_classes.insert(Breakpoint::Sm, "text-sm");
        responsive_classes.insert(Breakpoint::Md, "text-base");

        let classes = generator.generate_responsive_classes("px-4", responsive_classes);

        let class_string = classes.to_string();
        assert!(class_string.contains("sm:text-sm"));
        assert!(class_string.contains("md:text-base"));
    }

    #[test]
    fn test_class_caching() {
        let mut generator = YewClassGenerator::new();
        let builder = ClassBuilder::new().class("px-4");

        // First generation
        let classes1 = generator.generate_yew_classes(builder.clone());
        assert_eq!(generator.cache_size(), 1);

        // Second generation should use cache
        let classes2 = generator.generate_yew_classes(builder);
        assert_eq!(generator.cache_size(), 1);

        // Classes should be the same
        assert_eq!(classes1.to_string(), classes2.to_string());
    }
}
