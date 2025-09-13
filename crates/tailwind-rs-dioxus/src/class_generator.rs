//! # Dioxus Class Generator
//!
//! This module provides Dioxus-specific class generation utilities.

use dioxus::prelude::*;
use std::collections::HashMap;
use tailwind_rs_core::{Breakpoint, ClassBuilder, State};

/// Dioxus-specific class generator
pub struct DioxusClassGenerator {
    class_cache: HashMap<String, String>,
}

impl DioxusClassGenerator {
    /// Create a new Dioxus class generator
    pub fn new() -> Self {
        Self {
            class_cache: HashMap::new(),
        }
    }

    /// Generate classes for Dioxus components
    pub fn generate_dioxus_classes(&mut self, builder: ClassBuilder) -> String {
        let class_string = builder.build();

        // Check cache first
        if let Some(cached_classes) = self.class_cache.get(&class_string) {
            return cached_classes.clone();
        }

        // Cache the result
        self.class_cache
            .insert(class_string.clone(), class_string.clone());

        class_string
    }

    /// Generate conditional classes for Dioxus components
    pub fn generate_conditional_classes(
        &mut self,
        base_classes: &str,
        condition: bool,
        conditional_classes: &str,
    ) -> String {
        let class_string = if condition {
            format!("{} {}", base_classes, conditional_classes)
        } else {
            base_classes.to_string()
        };

        // Check cache first
        if let Some(cached_classes) = self.class_cache.get(&class_string) {
            return cached_classes.clone();
        }

        // Cache the result
        self.class_cache
            .insert(class_string.clone(), class_string.clone());

        class_string
    }

    /// Generate responsive classes for Dioxus components
    pub fn generate_responsive_classes(
        &mut self,
        base_classes: &str,
        responsive_classes: HashMap<Breakpoint, &str>,
    ) -> String {
        let mut class_string = base_classes.to_string();

        for (breakpoint, classes) in &responsive_classes {
            class_string.push(' ');
            class_string.push_str(&format!("{}:{}", breakpoint.prefix(), classes));
        }

        // Check cache first
        if let Some(cached_classes) = self.class_cache.get(&class_string) {
            return cached_classes.clone();
        }

        // Cache the result
        self.class_cache
            .insert(class_string.clone(), class_string.clone());

        class_string
    }

    /// Generate state-based classes for Dioxus components
    pub fn generate_state_classes(
        &mut self,
        base_classes: &str,
        state_classes: HashMap<State, &str>,
    ) -> String {
        let mut class_string = base_classes.to_string();

        for (state, classes) in &state_classes {
            class_string.push(' ');
            class_string.push_str(&format!("{}:{}", state.prefix(), classes));
        }

        // Check cache first
        if let Some(cached_classes) = self.class_cache.get(&class_string) {
            return cached_classes.clone();
        }

        // Cache the result
        self.class_cache
            .insert(class_string.clone(), class_string.clone());

        class_string
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

impl Default for DioxusClassGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dioxus_class_generator_creation() {
        let generator = DioxusClassGenerator::new();
        assert_eq!(generator.cache_size(), 0);
    }

    #[test]
    fn test_generate_dioxus_classes() {
        let mut generator = DioxusClassGenerator::new();
        let builder = ClassBuilder::new()
            .base("px-4 py-2")
            .variant("bg-blue-600 text-white");

        let classes = generator.generate_dioxus_classes(builder);

        assert!(classes.contains("px-4"));
        assert!(classes.contains("bg-blue-600"));
    }

    #[test]
    fn test_conditional_class_generation() {
        let mut generator = DioxusClassGenerator::new();

        let classes = generator.generate_conditional_classes("px-4", true, "bg-blue-600");

        assert!(classes.contains("px-4"));
        assert!(classes.contains("bg-blue-600"));

        let classes = generator.generate_conditional_classes("px-4", false, "bg-blue-600");

        assert!(classes.contains("px-4"));
        assert!(!classes.contains("bg-blue-600"));
    }

    #[test]
    fn test_responsive_class_generation() {
        let mut generator = DioxusClassGenerator::new();
        let mut responsive_classes = HashMap::new();
        responsive_classes.insert(Breakpoint::Sm, "text-sm");
        responsive_classes.insert(Breakpoint::Md, "text-base");

        let classes = generator.generate_responsive_classes("px-4", responsive_classes);

        assert!(classes.contains("sm:text-sm"));
        assert!(classes.contains("md:text-base"));
    }

    #[test]
    fn test_class_caching() {
        let mut generator = DioxusClassGenerator::new();
        let builder = ClassBuilder::new().base("px-4");

        // First generation
        let classes1 = generator.generate_dioxus_classes(builder.clone());
        assert_eq!(generator.cache_size(), 1);

        // Second generation should use cache
        let classes2 = generator.generate_dioxus_classes(builder);
        assert_eq!(generator.cache_size(), 1);

        // Classes should be the same
        assert_eq!(classes1, classes2);
    }
}
