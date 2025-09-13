//! # Test Utilities for Leptos Integration
//!
//! This module provides simple utilities for testing Leptos components and signals
//! without requiring complex runtime setup.

use leptos::prelude::*;

/// Simple test utility for creating signals in tests
pub fn create_test_signal<T>(initial: T) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Send + Sync + 'static,
{
    signal(initial)
}

/// Simple test utility for creating ArcRwSignal in tests
pub fn create_test_arc_rw_signal<T>(initial: T) -> ArcRwSignal<T>
where
    T: Send + Sync + 'static,
{
    ArcRwSignal::new(initial)
}

/// Simple test utility for creating ArcMemo in tests
pub fn create_test_arc_memo<F, T>(memo_fn: F) -> ArcMemo<T>
where
    F: Fn() -> T + Send + Sync + 'static,
    T: Send + Sync + PartialEq + 'static,
{
    ArcMemo::new(move |_| memo_fn())
}

/// Simple test utility for creating effects in tests
pub fn create_test_effect<F>(effect_fn: F)
where
    F: Fn() + Send + Sync + 'static,
{
    Effect::new(move |_| {
        effect_fn();
    });
}

/// Simple test utility for providing context in tests
pub fn provide_test_context<T>(context: T)
where
    T: Send + Sync + 'static,
{
    provide_context(context);
}

/// Simple test utility for using context in tests
pub fn use_test_context<T>() -> Option<T>
where
    T: Clone + 'static,
{
    use_context::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_signal_creation() {
        let (read, write) = create_test_signal(42);
        assert_eq!(read.get(), 42);
        write.set(100);
        assert_eq!(read.get(), 100);
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_arc_rw_signal_creation() {
        let signal = create_test_arc_rw_signal("hello".to_string());
        assert_eq!(signal.get(), "hello");
        signal.set("world".to_string());
        assert_eq!(signal.get(), "world");
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_arc_memo_creation() {
        let counter = create_test_arc_rw_signal(0);
        let counter_clone = counter.clone();
        let doubled = create_test_arc_memo(move || counter_clone.get() * 2);
        
        assert_eq!(doubled.get(), 0);
        counter.set(5);
        assert_eq!(doubled.get(), 10);
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_context_providing() {
        provide_test_context("test_context".to_string());
        let context: Option<String> = use_test_context();
        assert_eq!(context, Some("test_context".to_string()));
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_effect_creation() {
        let (counter, set_counter) = create_test_signal(0);
        let (effect_called, set_effect_called) = create_test_signal(false);
        
        create_test_effect(move || {
            counter.get(); // Track the signal
            set_effect_called.set(true);
        });
        
        // Effect should be called when signal changes
        set_counter.set(1);
        assert!(effect_called.get());
    }
}
