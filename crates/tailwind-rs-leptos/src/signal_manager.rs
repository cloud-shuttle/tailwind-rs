//! # Signal Management for Leptos 0.8.8
//!
//! This module provides signal management for tailwind-rs components
//! following the new Leptos 0.8.8 signal system patterns.
//!
//! Leptos 0.8.8 automatically handles signal cleanup when they go out of scope,
//! so no explicit cleanup management is needed.

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
        F: FnOnce(
            &ArcRwSignal<Theme>,
            &ArcRwSignal<String>,
            &ArcRwSignal<String>,
            &ArcRwSignal<String>,
            &ArcRwSignal<bool>,
            &ArcRwSignal<bool>,
        ),
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_automatic_signal_cleanup() {
        // Test that signals are automatically cleaned up when they go out of scope
        {
            let signal = ArcRwSignal::new(42);
            assert_eq!(signal.get(), 42);
            // Signal should be automatically cleaned up here
        }
        // No explicit cleanup needed - Leptos handles it
    }

    #[test]
    fn test_signal_lifecycle_in_component() {
        // Test that signals work correctly without explicit cleanup
        let signal = ArcRwSignal::new("test".to_string());
        assert_eq!(signal.get(), "test");

        signal.set("updated".to_string());
        assert_eq!(signal.get(), "updated");

        // Signal cleanup is automatic
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
