//! # Tailwind-rs Yew Integration
//!
//! This crate provides seamless integration between Tailwind CSS and the Yew framework.
//! It follows our TDD-first approach (ADR-001) and comprehensive testing pyramid strategy (ADR-002).
//!
//! ## Features
//!
//! - **Type-safe class generation** - Compile-time validation of Tailwind classes
//! - **Component-based styling** - Yew component integration with Tailwind
//! - **Performance optimized** - Efficient class caching and tree-shaking
//! - **Framework integration** - Native Yew component support
//!
//! ## Quick Start
//!
//! ```rust
//! use yew::prelude::*;
//! use tailwind_rs_yew::{Button, ButtonProps, ButtonVariant};
//!
//! #[function_component]
//! pub fn MyButton(props: &ButtonProps) -> Html {
//!     let classes = yew::classes! {
//!         "px-4", "py-2", "rounded-md", "font-medium", "transition-colors",
//!         match props.variant {
//!             ButtonVariant::Primary => "bg-blue-600",
//!             ButtonVariant::Secondary => "bg-gray-200",
//!             ButtonVariant::Danger => "bg-red-600",
//!             ButtonVariant::Outline => "border border-gray-300",
//!         },
//!     };
//!     
//!     html! {
//!         <button class={classes}>
//!             {props.children.clone()}
//!         </button>
//!     }
//! }
//! ```

pub mod class_generator;
pub mod components;
pub mod props;
pub mod utils;

pub use class_generator::*;
pub use components::*;
pub use props::*;
pub use utils::*;

// Re-export core functionality
pub use tailwind_rs_core::*;
pub use tailwind_rs_macros::*;

/// Yew-specific class generation utilities
pub mod yew_classes {
    use tailwind_rs_core::ClassBuilder;
    use yew::prelude::*;

    /// Create classes for Yew components
    pub fn create_yew_classes(builder: ClassBuilder) -> Classes {
        Classes::from(builder.build().to_css_classes())
    }

    /// Create conditional classes for Yew components
    pub fn create_conditional_classes(
        base_classes: &str,
        condition: bool,
        conditional_classes: &str,
    ) -> Classes {
        let classes = if condition {
            format!("{} {}", base_classes, conditional_classes)
        } else {
            base_classes.to_string()
        };
        Classes::from(classes)
    }

    /// Create responsive classes for Yew components
    pub fn create_responsive_classes(
        base_classes: &str,
        responsive: &[(crate::Breakpoint, &str)],
    ) -> Classes {
        let mut result = base_classes.to_string();

        for (breakpoint, classes) in responsive {
            result.push(' ');
            result.push_str(&format!("{}{}", breakpoint.prefix(), classes));
        }

        Classes::from(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yew_class_generation() {
        let builder = ClassBuilder::new()
            .class("px-4 py-2")
            .class("bg-blue-600 text-white");

        let classes = yew_classes::create_yew_classes(builder);
        let class_string = classes.to_string();

        assert!(class_string.contains("px-4"));
        assert!(class_string.contains("bg-blue-600"));
    }

    #[test]
    fn test_conditional_class_generation() {
        let classes = yew_classes::create_conditional_classes("px-4", true, "bg-blue-600");

        let class_string = classes.to_string();
        assert!(class_string.contains("px-4"));
        assert!(class_string.contains("bg-blue-600"));

        let classes = yew_classes::create_conditional_classes("px-4", false, "bg-blue-600");

        let class_string = classes.to_string();
        assert!(class_string.contains("px-4"));
        assert!(!class_string.contains("bg-blue-600"));
    }

    #[test]
    fn test_responsive_class_generation() {
        let responsive = vec![(Breakpoint::Sm, "text-sm"), (Breakpoint::Md, "text-base")];

        let classes = yew_classes::create_responsive_classes("px-4", &responsive);
        let class_string = classes.to_string();

        assert!(class_string.contains("sm:text-sm"));
        assert!(class_string.contains("md:text-base"));
    }
}
