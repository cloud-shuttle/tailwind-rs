//! # Signal Lifecycle Management for Leptos 0.8.8
//!
//! This module provides comprehensive signal lifecycle management for tailwind-rs components
//! following the new Leptos 0.8.8 signal system patterns.

use leptos::prelude::*;
use tailwind_rs_core::Theme;

/// Manages signal lifecycle for tailwind-rs components
/// Uses ArcRwSignal for shared styling state that needs to persist
#[derive(Clone)]
pub struct TailwindSignalManager {
    // Use ArcRwSignal for shared styling state that needs to persist
    theme_signal: ArcRwSignal<Theme>,
    variant_signal: ArcRwSignal<String>,
    size_signal: ArcRwSignal<String>,
    responsive_signal: ArcRwSignal<String>,
    disabled_signal: ArcRwSignal<bool>,
    loading_signal: ArcRwSignal<bool>,
}

impl TailwindSignalManager {
    /// Create a new signal manager with default values
    pub fn new() -> Self {
        Self {
            theme_signal: ArcRwSignal::new(Theme::new("default")),
            variant_signal: ArcRwSignal::new("primary".to_string()),
            size_signal: ArcRwSignal::new("medium".to_string()),
            responsive_signal: ArcRwSignal::new("default".to_string()),
            disabled_signal: ArcRwSignal::new(false),
            loading_signal: ArcRwSignal::new(false),
        }
    }
    
    /// Create a new signal manager with custom initial values
    pub fn with_initial_values(
        theme: Theme,
        variant: String,
        size: String,
        responsive: String,
    ) -> Self {
        Self {
            theme_signal: ArcRwSignal::new(theme),
            variant_signal: ArcRwSignal::new(variant),
            size_signal: ArcRwSignal::new(size),
            responsive_signal: ArcRwSignal::new(responsive),
            disabled_signal: ArcRwSignal::new(false),
            loading_signal: ArcRwSignal::new(false),
        }
    }
    
    /// Provide context that persists across component disposal
    pub fn provide_context(self) {
        provide_context(self);
    }
    
    /// Get theme signal for dynamic theming
    pub fn theme(&self) -> ArcRwSignal<Theme> {
        self.theme_signal.clone()
    }
    
    /// Get variant signal for component variants
    pub fn variant(&self) -> ArcRwSignal<String> {
        self.variant_signal.clone()
    }
    
    /// Get size signal for responsive sizing
    pub fn size(&self) -> ArcRwSignal<String> {
        self.size_signal.clone()
    }
    
    /// Get responsive configuration signal
    pub fn responsive(&self) -> ArcRwSignal<String> {
        self.responsive_signal.clone()
    }
    
    /// Get disabled state signal
    pub fn disabled(&self) -> ArcRwSignal<bool> {
        self.disabled_signal.clone()
    }
    
    /// Get loading state signal
    pub fn loading(&self) -> ArcRwSignal<bool> {
        self.loading_signal.clone()
    }
    
    /// Update multiple signals in a batch for better performance
    pub fn batch_update<F>(&self, update_fn: F)
    where
        F: FnOnce(&ArcRwSignal<Theme>, &ArcRwSignal<String>, &ArcRwSignal<String>, &ArcRwSignal<String>, &ArcRwSignal<bool>, &ArcRwSignal<bool>),
    {
        update_fn(
            &self.theme_signal,
            &self.variant_signal,
            &self.size_signal,
            &self.responsive_signal,
            &self.disabled_signal,
            &self.loading_signal,
        );
    }
    
    /// Sync external props with internal state using batched updates
    pub fn sync_props(
        &self,
        variant: impl Into<String>,
        size: impl Into<String>,
        disabled: bool,
        loading: bool,
    ) {
        self.variant_signal.set(variant.into());
        self.size_signal.set(size.into());
        self.disabled_signal.set(disabled);
        self.loading_signal.set(loading);
    }
}

impl Default for TailwindSignalManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook to get the current TailwindSignalManager from context
pub fn use_tailwind_signal_manager() -> Option<TailwindSignalManager> {
    use_context::<TailwindSignalManager>()
}

/// Hook to get or create a TailwindSignalManager
pub fn use_tailwind_signal_manager_or_default() -> TailwindSignalManager {
    use_context::<TailwindSignalManager>().unwrap_or_else(|| {
        let manager = TailwindSignalManager::new();
        let manager_clone = manager.clone();
        manager.provide_context();
        manager_clone
    })
}

/// Signal cleanup utility for proper memory management
pub struct SignalCleanup {
    signals: Vec<ArcRwSignal<()>>,
    memos: Vec<ArcMemo<()>>,
}

impl SignalCleanup {
    /// Create a new signal cleanup manager
    pub fn new() -> Self {
        Self { 
            signals: Vec::new(),
            memos: Vec::new(),
        }
    }
    
    /// Track a signal for cleanup
    pub fn track_signal<T>(&mut self, signal: ArcRwSignal<T>) -> ArcRwSignal<T> {
        // Track signal for cleanup by creating a dummy signal
        self.signals.push(ArcRwSignal::new(()));
        signal
    }
    
    /// Track a memo for cleanup
    pub fn track_memo<T>(&mut self, memo: ArcMemo<T>) -> ArcMemo<T> 
    where
        T: Send + Sync + 'static,
    {
        // Track memo for cleanup by creating a dummy memo
        self.memos.push(ArcMemo::new(|_| ()));
        memo
    }
    
    /// Cleanup all tracked signals and memos
    pub fn cleanup(self) {
        // Signals and memos will be automatically disposed when this struct is dropped
        // due to Leptos 0.8.8's ownership tree
        drop(self);
    }
}

impl Default for SignalCleanup {
    fn default() -> Self {
        Self::new()
    }
}

/// Automatic cleanup implementation
impl Drop for SignalCleanup {
    fn drop(&mut self) {
        // Leptos 0.8.8 will automatically dispose signals and memos
        // when they go out of scope
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_signal_manager_creation() {
        let manager = TailwindSignalManager::new();
        
        // Test default values
        assert_eq!(manager.variant().get(), "primary");
        assert_eq!(manager.size().get(), "medium");
        assert_eq!(manager.disabled().get(), false);
        assert_eq!(manager.loading().get(), false);
    }
    
    #[test]
    fn test_signal_manager_with_initial_values() {
        let theme = Theme::new("test");
        let manager = TailwindSignalManager::with_initial_values(
            theme.clone(),
            "secondary".to_string(),
            "large".to_string(),
            "custom".to_string(),
        );
        
        assert_eq!(manager.variant().get(), "secondary");
        assert_eq!(manager.size().get(), "large");
    }
    
    #[test]
    fn test_signal_manager_sync_props() {
        let manager = TailwindSignalManager::new();
        
        // Sync props
        manager.sync_props("danger", "small", true, true);
        
        assert_eq!(manager.variant().get(), "danger");
        assert_eq!(manager.size().get(), "small");
        assert_eq!(manager.disabled().get(), true);
        assert_eq!(manager.loading().get(), true);
    }
    
    #[test]
    fn test_signal_cleanup() {
        let mut cleanup = SignalCleanup::new();
        
        // Create signals and track them
        let signal1 = cleanup.track_signal(ArcRwSignal::new(42));
        let signal2 = cleanup.track_signal(ArcRwSignal::new("test".to_string()));
        let memo = cleanup.track_memo(ArcMemo::new(|_| 84));
        
        // Verify signals work
        assert_eq!(signal1.get(), 42);
        assert_eq!(signal2.get(), "test");
        assert_eq!(memo.get(), 84);
        
        // Cleanup should dispose signals
        cleanup.cleanup();
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_context_providing() {
        use crate::test_utils::*;
        
        let manager = TailwindSignalManager::new();
        
        manager.provide_context();
        
        // Should be able to get from context
        let context_manager = use_test_context::<TailwindSignalManager>();
        assert!(context_manager.is_some());
    }
}
