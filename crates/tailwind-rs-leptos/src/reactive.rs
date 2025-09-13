//! # Reactive Styling
//!
//! This module provides reactive styling utilities that integrate with Leptos signals.

use leptos::prelude::*;
use std::collections::HashMap;
use tailwind_rs_core::ClassBuilder;

/// Reactive class manager that handles dynamic class updates
pub struct ReactiveClassManager {
    base_classes: String,
    dynamic_classes: HashMap<String, ReadSignal<String>>,
}

impl ReactiveClassManager {
    /// Create a new reactive class manager
    pub fn new(base_classes: &str) -> Self {
        Self {
            base_classes: base_classes.to_string(),
            dynamic_classes: HashMap::new(),
        }
    }

    /// Add a reactive class that updates based on a signal
    pub fn add_reactive_class<S>(
        &mut self,
        key: &str,
        signal: ReadSignal<S>,
        class_fn: impl Fn(S) -> String + 'static,
    ) where
        S: Clone + 'static,
    {
        let (classes, set_classes) = create_signal(class_fn(signal.get()));

        create_effect(move |_| {
            let value = signal.get();
            set_classes.set(class_fn(value));
        });

        self.dynamic_classes.insert(key.to_string(), classes);
    }

    /// Add a conditional class that appears based on a boolean signal
    pub fn add_conditional_class(&mut self, key: &str, condition: ReadSignal<bool>, class: String) {
        let (classes, set_classes) = create_signal(String::new());

        create_effect(move |_| {
            if condition.get() {
                set_classes.set(class.clone());
            } else {
                set_classes.set(String::new());
            }
        });

        self.dynamic_classes.insert(key.to_string(), classes);
    }

    /// Get the combined classes as a reactive signal
    pub fn get_combined_classes(&self) -> ReadSignal<String> {
        let base_classes = self.base_classes.clone();
        let dynamic_classes = self.dynamic_classes.clone();

        let (combined, set_combined) = create_signal(self.build_classes());

        // Update combined classes when any dynamic class changes
        for (_, signal) in &dynamic_classes {
            let set_combined = set_combined.clone();
            let base_classes = base_classes.clone();
            let dynamic_classes = dynamic_classes.clone();

            create_effect(move |_| {
                signal.get(); // Trigger on signal change
                let mut result = base_classes.clone();

                for (_, class_signal) in &dynamic_classes {
                    let class_value = class_signal.get();
                    if !class_value.is_empty() {
                        result.push(' ');
                        result.push_str(&class_value);
                    }
                }

                set_combined.set(result);
            });
        }

        combined
    }

    /// Build the current classes string
    fn build_classes(&self) -> String {
        let mut result = self.base_classes.clone();

        for (_, signal) in &self.dynamic_classes {
            let class_value = signal.get();
            if !class_value.is_empty() {
                result.push(' ');
                result.push_str(&class_value);
            }
        }

        result
    }
}

/// Utility for creating reactive classes with common patterns
pub struct ReactiveClassBuilder {
    manager: ReactiveClassManager,
}

impl ReactiveClassBuilder {
    /// Create a new reactive class builder
    pub fn new(base_classes: &str) -> Self {
        Self {
            manager: ReactiveClassManager::new(base_classes),
        }
    }

    /// Add a reactive class based on a signal
    pub fn reactive<S>(
        mut self,
        key: &str,
        signal: ReadSignal<S>,
        class_fn: impl Fn(S) -> String + 'static,
    ) -> Self
    where
        S: Clone + 'static,
    {
        self.manager.add_reactive_class(key, signal, class_fn);
        self
    }

    /// Add a conditional class
    pub fn conditional(mut self, key: &str, condition: ReadSignal<bool>, class: &str) -> Self {
        self.manager.add_conditional_class(key, condition, class.to_string());
        self
    }

    /// Build the reactive classes
    pub fn build(self) -> ReadSignal<String> {
        self.manager.get_combined_classes()
    }
}

/// Hook for creating reactive classes
pub fn use_reactive_classes<F>(class_fn: F) -> ReadSignal<String>
where
    F: Fn() -> String + 'static,
{
    let (classes, set_classes) = create_signal(class_fn());

    create_effect(move |_| {
        set_classes.set(class_fn());
    });

    classes
}

/// Hook for creating conditional classes
pub fn use_conditional_classes(
    base_classes: String,
    condition: ReadSignal<bool>,
    conditional_classes: String,
) -> ReadSignal<String> {
    let (classes, set_classes) = create_signal(base_classes.clone());

    create_effect(move |_| {
        let mut result = base_classes.clone();
        if condition.get() {
            result.push(' ');
            result.push_str(&conditional_classes);
        }
        set_classes.set(result);
    });

    classes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reactive_class_manager_creation() {
        let manager = ReactiveClassManager::new("px-4 py-2");
        assert_eq!(manager.base_classes, "px-4 py-2");
        assert_eq!(manager.dynamic_classes.len(), 0);
    }

    #[test]
    fn test_conditional_class_addition() {
        let mut manager = ReactiveClassManager::new("px-4");
        let (is_active, set_active) = signal(false);

        manager.add_conditional_class("active", is_active, "bg-blue-600".to_string());

        // Initially should not have conditional class
        let classes = manager.get_combined_classes();
        assert!(!classes.get().contains("bg-blue-600"));

        // After setting condition to true
        set_active.set(true);
        assert!(classes.get().contains("bg-blue-600"));
    }

    #[test]
    fn test_reactive_class_builder() {
        let (count, set_count) = signal(0);
        let (is_visible, _) = signal(true);

        let classes = ReactiveClassBuilder::new("px-4")
            .reactive("count", count, |c| {
                if c > 5 {
                    "text-lg".to_string()
                } else {
                    "text-sm".to_string()
                }
            })
            .conditional("visible", is_visible, "opacity-100")
            .build();

        // Test initial state
        let result = classes.get();
        assert!(result.contains("px-4"));
        assert!(result.contains("text-sm"));
        assert!(result.contains("opacity-100"));

        // Test reactive update
        set_count.set(10);
        let result = classes.get();
        assert!(result.contains("text-lg"));
        assert!(!result.contains("text-sm"));
    }
}
