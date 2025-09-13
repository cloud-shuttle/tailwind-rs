//! # Leptos Class Generator
//!
//! This module provides Leptos-specific class generation utilities that integrate
//! with Leptos signals and reactive system.

use leptos::prelude::*;
use std::collections::HashMap;
use tailwind_rs_core::{Breakpoint, ClassBuilder, State};

/// Leptos-specific class generator that integrates with reactive signals
pub struct LeptosClassGenerator {
    reactive_classes: HashMap<String, ReadSignal<String>>,
    class_cache: HashMap<String, String>,
}

impl LeptosClassGenerator {
    /// Create a new Leptos class generator
    pub fn new() -> Self {
        Self {
            reactive_classes: HashMap::new(),
            class_cache: HashMap::new(),
        }
    }

    /// Create reactive classes for Leptos components
    pub fn create_reactive_classes<F>(&mut self, class_fn: F) -> ReadSignal<String>
    where
        F: Fn() -> String + 'static,
    {
        let (classes, set_classes) = signal(class_fn());

        // Store reactive classes
        let key = format!("reactive_{}", self.reactive_classes.len());
        self.reactive_classes.insert(key, classes);

        classes
    }

    /// Generate classes for Leptos components with conditional logic
    pub fn generate_conditional_classes(
        &self,
        base_classes: String,
        conditions: Vec<(ReadSignal<bool>, String)>,
    ) -> ReadSignal<String> {
        let (classes, set_classes) = create_signal(base_classes.clone());

        create_effect(move |_| {
            let mut result = base_classes.clone();

            for (condition, conditional_classes) in &conditions {
                if condition.get() {
                    result.push(' ');
                    result.push_str(conditional_classes);
                }
            }

            set_classes.set(result);
        });

        classes
    }

    /// Generate responsive classes with reactive breakpoints
    pub fn generate_responsive_classes(
        &self,
        base_classes: String,
        responsive_classes: HashMap<Breakpoint, String>,
    ) -> ReadSignal<String> {
        let (classes, set_classes) = create_signal(base_classes.clone());

        create_effect(move |_| {
            let mut result = base_classes.clone();

            for (breakpoint, classes) in &responsive_classes {
                result.push(' ');
                result.push_str(&format!("{}:{}", breakpoint.prefix(), classes));
            }

            set_classes.set(result);
        });

        classes
    }

    /// Generate state-based classes (hover, focus, etc.)
    pub fn generate_state_classes(
        &self,
        base_classes: String,
        state_classes: HashMap<State, String>,
    ) -> ReadSignal<String> {
        let (classes, set_classes) = create_signal(base_classes.clone());

        create_effect(move |_| {
            let mut result = base_classes.clone();

            for (state, classes) in &state_classes {
                result.push(' ');
                result.push_str(&format!("{}:{}", state.prefix(), classes));
            }

            set_classes.set(result);
        });

        classes
    }
}

impl Default for LeptosClassGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leptos_class_generator_creation() {
        let generator = LeptosClassGenerator::new();
        assert_eq!(generator.reactive_classes.len(), 0);
        assert_eq!(generator.class_cache.len(), 0);
    }

    #[test]
    fn test_conditional_class_generation() {
        let generator = LeptosClassGenerator::new();
        let (is_active, set_active) = create_signal(false);

        let conditions = vec![(is_active, "bg-blue-600".to_string())];
        let classes = generator.generate_conditional_classes("px-4 py-2".to_string(), conditions);

        // Initially should not have conditional classes
        assert!(!classes.get().contains("bg-blue-600"));

        // After setting condition to true
        set_active.set(true);
        assert!(classes.get().contains("bg-blue-600"));
    }

    #[test]
    fn test_responsive_class_generation() {
        let generator = LeptosClassGenerator::new();
        let mut responsive_classes = HashMap::new();
        responsive_classes.insert(Breakpoint::Sm, "text-sm".to_string());
        responsive_classes.insert(Breakpoint::Md, "text-base".to_string());

        let classes = generator.generate_responsive_classes("px-4".to_string(), responsive_classes);
        let result = classes.get();

        assert!(result.contains("sm:text-sm"));
        assert!(result.contains("md:text-base"));
    }
}
