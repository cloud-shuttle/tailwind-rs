//! # Tailwind-rs Dioxus Integration
//!
//! This crate provides seamless integration between Tailwind CSS and the Dioxus framework.
//! It follows our TDD-first approach (ADR-001) and comprehensive testing pyramid strategy (ADR-002).
//!
//! ## Features
//!
//! - **Type-safe class generation** - Compile-time validation of Tailwind classes
//! - **Component-based styling** - Dioxus component integration with Tailwind
//! - **Performance optimized** - Efficient class caching and tree-shaking
//! - **Framework integration** - Native Dioxus component support
//!
//! ## Quick Start
//!
//! ```rust
//! use dioxus::prelude::*;
//! use tailwind_rs_dioxus::*;
//!
//! #[component]
//! pub fn Button(variant: ButtonVariant) -> Element {
//!     let classes = classes! {
//!         base: "px-4 py-2 rounded-md font-medium transition-colors",
//!         variant: match variant {
//!             ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
//!             ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
//!         },
//!     };
//!     
//!     rsx! {
//!         button { class: classes, "Click me" }
//!     }
//! }
//! ```

pub mod class_generator;
pub mod components;
pub mod hooks;
pub mod utils;

pub use class_generator::*;
pub use components::*;
pub use hooks::*;
pub use utils::*;

// Re-export core functionality
pub use tailwind_rs_core::*;
pub use tailwind_rs_macros::*;

/// Dioxus-specific class generation utilities
pub mod dioxus_classes {
    use dioxus::prelude::*;
    use tailwind_rs_core::ClassBuilder;

    /// Create classes for Dioxus components
    pub fn create_dioxus_classes(builder: ClassBuilder) -> String {
        builder.build_string()
    }

    /// Create conditional classes for Dioxus components
    pub fn create_conditional_classes(
        base_classes: &str,
        condition: bool,
        conditional_classes: &str,
    ) -> String {
        if condition {
            format!("{} {}", base_classes, conditional_classes)
        } else {
            base_classes.to_string()
        }
    }

    /// Create responsive classes for Dioxus components
    pub fn create_responsive_classes(
        base_classes: &str,
        responsive: &[(crate::Breakpoint, &str)],
    ) -> String {
        let mut result = base_classes.to_string();

        for (breakpoint, classes) in responsive {
            result.push(' ');
            result.push_str(&format!("{}:{}", breakpoint.prefix(), classes));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_dioxus_class_generation() {
        let builder = ClassBuilder::new()
            .base("px-4 py-2")
            .variant("bg-blue-600 text-white");

        let classes = dioxus_classes::create_dioxus_classes(builder);

        assert!(classes.contains("px-4"));
        assert!(classes.contains("bg-blue-600"));
    }

    #[test]
    fn test_conditional_class_generation() {
        let classes = dioxus_classes::create_conditional_classes("px-4", true, "bg-blue-600");

        assert!(classes.contains("px-4"));
        assert!(classes.contains("bg-blue-600"));

        let classes = dioxus_classes::create_conditional_classes("px-4", false, "bg-blue-600");

        assert!(classes.contains("px-4"));
        assert!(!classes.contains("bg-blue-600"));
    }

    #[test]
    fn test_responsive_class_generation() {
        let responsive = vec![(Breakpoint::Sm, "text-sm"), (Breakpoint::Md, "text-base")];

        let classes = dioxus_classes::create_responsive_classes("px-4", &responsive);

        assert!(classes.contains("sm:text-sm"));
        assert!(classes.contains("md:text-base"));
    }
}
