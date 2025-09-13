//! # Signal Integration
//!
//! This module provides utilities for integrating Tailwind classes with Leptos signals.

use leptos::prelude::*;
use tailwind_rs_core::ClassBuilder;

/// Signal-based class manager that automatically updates classes when signals change
pub struct SignalClassManager {
    base_builder: ClassBuilder,
    signal_updates: Vec<Box<dyn Fn() -> String>>,
}

impl SignalClassManager {
    /// Create a new signal class manager
    pub fn new() -> Self {
        Self {
            base_builder: ClassBuilder::new(),
            signal_updates: Vec::new(),
        }
    }

    /// Add base classes
    pub fn base(mut self, classes: &str) -> Self {
        self.base_builder = self.base_builder.class(classes);
        self
    }

    /// Add reactive classes based on a signal
    pub fn signal<S>(
        mut self,
        signal: ReadSignal<S>,
        class_fn: impl Fn(S) -> String + 'static,
    ) -> Self
    where
        S: Clone + 'static,
    {
        let update_fn = Box::new(move || class_fn(signal.get()));
        self.signal_updates.push(update_fn);
        self
    }

    /// Add conditional classes based on a boolean signal
    pub fn conditional(mut self, condition: ReadSignal<bool>, classes: &str) -> Self {
        let classes = classes.to_string();
        let update_fn = Box::new(move || {
            if condition.get() {
                classes.clone()
            } else {
                String::new()
            }
        });
        self.signal_updates.push(update_fn);
        self
    }

    /// Build the reactive class signal
    pub fn build(self) -> ReadSignal<String> {
        let (classes, set_classes) = create_signal(self.build_current_classes());

        // Create effect to update classes when any signal changes
        create_effect(move |_| {
            // Trigger on any signal change by calling all update functions
            for update_fn in &self.signal_updates {
                update_fn();
            }
            set_classes.set(self.build_current_classes());
        });

        classes
    }

    /// Build the current classes string
    fn build_current_classes(&self) -> String {
        let mut result = self.base_builder.clone().build().to_css_classes();

        for update_fn in &self.signal_updates {
            let additional_classes = update_fn();
            if !additional_classes.is_empty() {
                result.push(' ');
                result.push_str(&additional_classes);
            }
        }

        result
    }
}

impl Default for SignalClassManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for creating signal-based classes with a fluent API
pub fn signal_classes() -> SignalClassManager {
    SignalClassManager::new()
}

/// Hook for creating classes that react to multiple signals
pub fn use_multi_signal_classes<F>(class_fn: F) -> ReadSignal<String>
where
    F: Fn() -> String + 'static,
{
    let (classes, set_classes) = create_signal(class_fn());

    create_effect(move |_| {
        set_classes.set(class_fn());
    });

    classes
}

/// Hook for creating classes that depend on a specific signal
pub fn use_signal_classes<S, F>(signal: ReadSignal<S>, class_fn: F) -> ReadSignal<String>
where
    S: Clone + 'static,
    F: Fn(S) -> String + 'static,
{
    let (classes, set_classes) = create_signal(class_fn(signal.get()));

    create_effect(move |_| {
        let value = signal.get();
        set_classes.set(class_fn(value));
    });

    classes
}

/// Hook for creating conditional classes based on a boolean signal
pub fn use_conditional_signal_classes(
    condition: ReadSignal<bool>,
    true_classes: &str,
    false_classes: &str,
) -> ReadSignal<String> {
    let (classes, set_classes) = create_signal(if condition.get() {
        true_classes
    } else {
        false_classes
    });

    create_effect(move |_| {
        let class_value = if condition.get() {
            true_classes
        } else {
            false_classes
        };
        set_classes.set(class_value);
    });

    classes
}

/// Hook for creating classes that cycle through multiple states
pub fn use_cycling_classes(
    states: Vec<String>,
    current_state: ReadSignal<usize>,
) -> ReadSignal<String> {
        let (classes, set_classes) =
        create_signal(states.get(current_state.get()).cloned().unwrap_or_default());

    create_effect(move |_| {
        let state_index = current_state.get();
        let class_value = states.get(state_index).cloned().unwrap_or_default();
        set_classes.set(class_value);
    });

    classes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_class_manager_creation() {
        let manager = SignalClassManager::new();
        assert_eq!(manager.signal_updates.len(), 0);
    }

    #[test]
    fn test_signal_class_manager_with_base() {
        let manager = SignalClassManager::new().base("px-4 py-2");

        let classes = manager.build();
        assert!(classes.get().contains("px-4"));
        assert!(classes.get().contains("py-2"));
    }

    #[test]
    fn test_conditional_signal_classes() {
        let (is_active, set_active) = signal(false);

        let classes = use_conditional_signal_classes(
            is_active,
            "bg-blue-600 text-white",
            "bg-gray-200 text-gray-900",
        );

        // Initially should have false classes
        let result = classes.get();
        assert!(result.contains("bg-gray-200"));
        assert!(!result.contains("bg-blue-600"));

        // After setting to true
        set_active.set(true);
        let result = classes.get();
        assert!(result.contains("bg-blue-600"));
        assert!(!result.contains("bg-gray-200"));
    }

    #[test]
    fn test_cycling_classes() {
        let states = vec!["state-1".to_string(), "state-2".to_string(), "state-3".to_string()];
        let (current_state, set_state) = signal(0);

        let classes = use_cycling_classes(states, current_state);

        // Test initial state
        assert_eq!(classes.get(), "state-1");

        // Test cycling
        set_state.set(1);
        assert_eq!(classes.get(), "state-2");

        set_state.set(2);
        assert_eq!(classes.get(), "state-3");

        // Test out of bounds
        set_state.set(5);
        assert_eq!(classes.get(), "");
    }
}
