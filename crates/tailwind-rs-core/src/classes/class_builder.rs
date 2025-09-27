//! ClassBuilder implementation
//!
//! This module contains the ClassBuilder struct and its methods.
//!
//! ## Example
//!
//! ```rust
//! use tailwind_rs_core::ClassBuilder;
//!
//! // Create type-safe Tailwind classes
//! let class_builder = ClassBuilder::new();
//! let class_set = class_builder
//!     .class("bg-blue-500")
//!     .class("text-white")
//!     .class("px-4")
//!     .class("py-2")
//!     .class("rounded-lg")
//!     .class("hover:bg-blue-600")
//!     .build();
//!
//! // Convert to CSS classes
//! let css_classes = class_set.to_css_classes();
//! // Result: "bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600"
//! ```

use super::ClassSet;
use crate::responsive::Breakpoint;

/// Builder for creating class sets
#[derive(Debug, Clone)]
pub struct ClassBuilder {
    class_set: ClassSet,
}

impl ClassBuilder {
    /// Create a new class builder
    pub fn new() -> Self {
        Self {
            class_set: ClassSet::new(),
        }
    }

    /// Add a base class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class_set.add_class(class);
        self
    }

    /// Add multiple base classes
    pub fn classes(mut self, classes: impl IntoIterator<Item = String>) -> Self {
        self.class_set.add_classes(classes);
        self
    }

    /// Add a responsive class
    pub fn responsive(mut self, breakpoint: Breakpoint, class: impl Into<String>) -> Self {
        self.class_set.add_responsive_class(breakpoint, class);
        self
    }

    /// Add a conditional class
    pub fn conditional(mut self, condition: impl Into<String>, class: impl Into<String>) -> Self {
        self.class_set.add_conditional_class(condition, class);
        self
    }

    /// Add a custom CSS property
    pub fn custom(mut self, property: impl Into<String>, value: impl Into<String>) -> Self {
        self.class_set.add_custom(property, value);
        self
    }

    /// Add a custom variant class (Tailwind v4.1.13 @custom-variant support)
    pub fn custom_variant(mut self, variant: impl Into<String>, class: impl Into<String>) -> Self {
        let variant = variant.into();
        let class = class.into();

        // Add the variant as a conditional class
        self.class_set.add_conditional_class(variant, class);
        self
    }

    /// Add an ARIA variant class
    pub fn aria(self, aria_attr: impl Into<String>, class: impl Into<String>) -> Self {
        let variant = format!("aria-{}", aria_attr.into());
        self.custom_variant(variant, class)
    }

    /// Add a data variant class
    pub fn data(
        self,
        data_attr: impl Into<String>,
        value: Option<String>,
        class: impl Into<String>,
    ) -> Self {
        let variant = if let Some(val) = value {
            format!("data-{}={}", data_attr.into(), val)
        } else {
            format!("data-{}", data_attr.into())
        };
        self.custom_variant(variant, class)
    }

    /// Add a supports variant class
    pub fn supports(self, feature: impl Into<String>, class: impl Into<String>) -> Self {
        let variant = format!("supports-{}", feature.into());
        self.custom_variant(variant, class)
    }

    /// Build the class set
    pub fn build(self) -> ClassSet {
        self.class_set
    }

    /// Build the class set and convert to CSS string
    pub fn build_string(self) -> String {
        self.class_set.to_css_classes()
    }
}

impl Default for ClassBuilder {
    fn default() -> Self {
        Self::new()
    }
}
