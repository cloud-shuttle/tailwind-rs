//! # Dynamic Class Builder for Leptos 0.8.8
//!
//! This module provides enhanced class generation with proper Leptos 0.8.8 signal management.

use leptos::prelude::*;

/// Enhanced class generation with Leptos 0.8.8 signal management
pub struct DynamicClassBuilder {
    base_classes: ArcRwSignal<String>,
    variant_classes: ArcRwSignal<String>,
    responsive_classes: ArcRwSignal<String>,
    state_classes: ArcRwSignal<String>,
    custom_classes: ArcRwSignal<String>,
    computed_classes: ArcMemo<String>,
}

impl DynamicClassBuilder {
    /// Create a new dynamic class builder
    pub fn new() -> Self {
        let base_classes = ArcRwSignal::new(String::new());
        let variant_classes = ArcRwSignal::new(String::new());
        let responsive_classes = ArcRwSignal::new(String::new());
        let state_classes = ArcRwSignal::new(String::new());
        let custom_classes = ArcRwSignal::new(String::new());
        
        // Clone signals for the closure
        let base_clone = base_classes.clone();
        let variant_clone = variant_classes.clone();
        let responsive_clone = responsive_classes.clone();
        let state_clone = state_classes.clone();
        let custom_clone = custom_classes.clone();
        
        // Use ArcMemo for computed classes that depend on multiple signals
        let computed_classes = ArcMemo::new(move |_| {
            let mut result = String::new();
            
            // Add base classes
            let base = base_clone.get();
            if !base.is_empty() {
                result.push_str(&base);
            }
            
            // Add variant classes
            let variant = variant_clone.get();
            if !variant.is_empty() {
                if !result.is_empty() {
                    result.push(' ');
                }
                result.push_str(&variant);
            }
            
            // Add responsive classes
            let responsive = responsive_clone.get();
            if !responsive.is_empty() {
                if !result.is_empty() {
                    result.push(' ');
                }
                result.push_str(&responsive);
            }
            
            // Add state classes
            let state = state_clone.get();
            if !state.is_empty() {
                if !result.is_empty() {
                    result.push(' ');
                }
                result.push_str(&state);
            }
            
            // Add custom classes
            let custom = custom_clone.get();
            if !custom.is_empty() {
                if !result.is_empty() {
                    result.push(' ');
                }
                result.push_str(&custom);
            }
            
            result.trim().to_string()
        });
        
        Self {
            base_classes,
            variant_classes,
            responsive_classes,
            state_classes,
            custom_classes,
            computed_classes,
        }
    }
    
    /// Set base classes for the component
    pub fn base(&self, classes: impl Into<String>) -> &Self {
        self.base_classes.set(classes.into());
        self
    }
    
    /// Set variant classes
    pub fn variant(&self, classes: impl Into<String>) -> &Self {
        self.variant_classes.set(classes.into());
        self
    }
    
    /// Set responsive classes
    pub fn responsive(&self, classes: impl Into<String>) -> &Self {
        self.responsive_classes.set(classes.into());
        self
    }
    
    /// Set state classes (hover, focus, disabled, etc.)
    pub fn state(&self, classes: impl Into<String>) -> &Self {
        self.state_classes.set(classes.into());
        self
    }
    
    /// Set custom classes
    pub fn custom(&self, classes: impl Into<String>) -> &Self {
        self.custom_classes.set(classes.into());
        self
    }
    
    /// Get the computed classes signal
    pub fn classes(&self) -> ArcMemo<String> {
        self.computed_classes.clone()
    }
    
    /// Get the computed classes as a string (for immediate use)
    pub fn classes_string(&self) -> String {
        self.computed_classes.get()
    }
    
    /// Update multiple class categories in a batch
    pub fn batch_update<F>(&self, update_fn: F)
    where
        F: FnOnce(&ArcRwSignal<String>, &ArcRwSignal<String>, &ArcRwSignal<String>, &ArcRwSignal<String>, &ArcRwSignal<String>),
    {
        update_fn(
            &self.base_classes,
            &self.variant_classes,
            &self.responsive_classes,
            &self.state_classes,
            &self.custom_classes,
        );
    }
    
    /// Clear all classes
    pub fn clear(&self) {
        self.base_classes.set(String::new());
        self.variant_classes.set(String::new());
        self.responsive_classes.set(String::new());
        self.state_classes.set(String::new());
        self.custom_classes.set(String::new());
    }
    
    /// Check if any classes are set
    pub fn is_empty(&self) -> bool {
        self.base_classes.get().is_empty() &&
        self.variant_classes.get().is_empty() &&
        self.responsive_classes.get().is_empty() &&
        self.state_classes.get().is_empty() &&
        self.custom_classes.get().is_empty()
    }
}

impl Default for DynamicClassBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Batched signal updates for better performance
pub struct BatchedSignalUpdater {
    update_queue: ArcRwSignal<Vec<Box<dyn Fn() + Send + Sync>>>,
    is_batching: ArcRwSignal<bool>,
}

impl BatchedSignalUpdater {
    /// Create a new batched signal updater
    pub fn new() -> Self {
        Self {
            update_queue: ArcRwSignal::new(Vec::new()),
            is_batching: ArcRwSignal::new(false),
        }
    }
    
    /// Queue an update for batched execution
    pub fn queue_update<F>(&self, update: F) 
    where 
        F: Fn() + Send + Sync + 'static 
    {
        self.update_queue.update(|queue| {
            queue.push(Box::new(update));
        });
    }
    
    /// Flush all queued updates
    pub fn flush_updates(&self) {
        self.update_queue.with(|updates| {
            for update in updates {
                update();
            }
        });
        self.update_queue.set(Vec::new());
    }
    
    /// Start batching updates
    pub fn start_batching(&self) {
        self.is_batching.set(true);
    }
    
    /// End batching and flush updates
    pub fn end_batching(&self) {
        self.is_batching.set(false);
        self.flush_updates();
    }
    
    /// Check if currently batching
    pub fn is_batching(&self) -> bool {
        self.is_batching.get()
    }
    
    /// Execute updates in a batch
    pub fn batch<F>(&self, updates: F)
    where
        F: FnOnce(&Self),
    {
        self.start_batching();
        updates(self);
        self.end_batching();
    }
}

impl Default for BatchedSignalUpdater {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility function to create a dynamic class builder
pub fn dynamic_classes() -> DynamicClassBuilder {
    DynamicClassBuilder::new()
}

/// Utility function to create a batched signal updater
pub fn batched_updates() -> BatchedSignalUpdater {
    BatchedSignalUpdater::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_dynamic_class_builder_creation() {
        let builder = DynamicClassBuilder::new();
        
        // Initially should be empty
        assert!(builder.is_empty());
        assert_eq!(builder.classes_string(), "");
    }
    
    #[test]
    fn test_dynamic_class_builder_base_classes() {
        let builder = DynamicClassBuilder::new();
        
        // Set base classes
        builder.base("px-4 py-2");
        
        let classes = builder.classes_string();
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
    }
    
    #[test]
    fn test_dynamic_class_builder_variant_classes() {
        let builder = DynamicClassBuilder::new();
        
        // Set variant classes
        builder.variant("bg-blue-600 text-white");
        
        let classes = builder.classes_string();
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
    }
    
    #[test]
    fn test_dynamic_class_builder_responsive_classes() {
        let builder = DynamicClassBuilder::new();
        
        // Set responsive classes
        builder.responsive("sm:text-sm md:text-base");
        
        let classes = builder.classes_string();
        assert!(classes.contains("sm:text-sm"));
        assert!(classes.contains("md:text-base"));
    }
    
    #[test]
    fn test_dynamic_class_builder_state_classes() {
        let builder = DynamicClassBuilder::new();
        
        // Set state classes
        builder.state("hover:bg-blue-700 focus:ring-2");
        
        let classes = builder.classes_string();
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("focus:ring-2"));
    }
    
    #[test]
    fn test_dynamic_class_builder_combined_classes() {
        let builder = DynamicClassBuilder::new();
        
        // Set multiple class types
        builder
            .base("px-4 py-2")
            .variant("bg-blue-600 text-white")
            .responsive("sm:text-sm md:text-base")
            .state("hover:bg-blue-700 focus:ring-2");
        
        let classes = builder.classes_string();
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("sm:text-sm"));
        assert!(classes.contains("md:text-base"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("focus:ring-2"));
    }
    
    #[test]
    fn test_dynamic_class_builder_clear() {
        let builder = DynamicClassBuilder::new();
        
        // Set some classes
        builder
            .base("px-4 py-2")
            .variant("bg-blue-600 text-white");
        
        // Verify classes are set
        assert!(!builder.is_empty());
        assert!(!builder.classes_string().is_empty());
        
        // Clear classes
        builder.clear();
        
        // Verify classes are cleared
        assert!(builder.is_empty());
        assert_eq!(builder.classes_string(), "");
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_batched_signal_updater() {
        use crate::test_utils::*;
        
        let updater = BatchedSignalUpdater::new();
        let (counter, set_counter) = create_test_signal(0);
        
        // Queue multiple updates
        updater.queue_update(move || set_counter.update(|c| *c += 1));
        updater.queue_update(move || set_counter.update(|c| *c += 1));
        updater.queue_update(move || set_counter.update(|c| *c += 1));
        
        // Counter should still be 0 before flush
        assert_eq!(counter.get(), 0);
        
        // Flush updates
        updater.flush_updates();
        
        // Counter should now be 3
        assert_eq!(counter.get(), 3);
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_batched_signal_updater_batch_method() {
        use crate::test_utils::*;
        
        let updater = BatchedSignalUpdater::new();
        let (counter, set_counter) = create_test_signal(0);
        
        // Use batch method
        updater.batch(|batch_updater| {
            batch_updater.queue_update(move || set_counter.update(|c| *c += 1));
            batch_updater.queue_update(move || set_counter.update(|c| *c += 1));
            batch_updater.queue_update(move || set_counter.update(|c| *c += 1));
        });
        
        // Counter should now be 3
        assert_eq!(counter.get(), 3);
    }
}
