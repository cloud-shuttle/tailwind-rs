//! Class builder utilities for Dioxus

use tailwind_rs_core::ClassBuilder;

/// Simple class builder wrapper for Dioxus
pub struct DioxusClassBuilder {
    builder: ClassBuilder,
}

impl DioxusClassBuilder {
    /// Create a new class builder
    pub fn new() -> Self {
        Self {
            builder: ClassBuilder::new(),
        }
    }

    /// Add a class to the builder
    pub fn class(mut self, class: &str) -> Self {
        self.builder = self.builder.class(class);
        self
    }

    /// Build the final class string
    pub fn build(self) -> String {
        self.builder.build_string()
    }
}

impl Default for DioxusClassBuilder {
    fn default() -> Self {
        Self::new()
    }
}
