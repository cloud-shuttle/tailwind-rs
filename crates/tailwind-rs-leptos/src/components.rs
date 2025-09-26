//! # Leptos Components with Tailwind-RS
//!
//! This module provides pre-built Leptos components that use Tailwind-RS for styling.

use crate::dynamic_class_builder::DynamicClassBuilder;
use leptos::prelude::*;

/// Button variant options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Outline,
}

/// Button size options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Medium,
    Small,
    Large,
}

/// Simplified Button component using input signals directly
#[component]
pub fn Button(
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] loading: Signal<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Use input signals directly - no cloning or manual effects needed
    let classes = move || {
        let builder = DynamicClassBuilder::new()
            .base("inline-flex items-center justify-center font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed");

        // Variant classes - use input signal directly
        let builder = match variant.get() {
            ButtonVariant::Primary => builder.variant("bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500"),
            ButtonVariant::Secondary => builder.variant("bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-500"),
            ButtonVariant::Danger => builder.variant("bg-red-600 text-white hover:bg-red-700 focus:ring-red-500"),
            ButtonVariant::Outline => builder.variant("border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 focus:ring-gray-500"),
        };

        // Size classes - use input signal directly
        let builder = match size.get() {
            ButtonSize::Small => builder.responsive("px-3 py-1.5 text-sm rounded-md"),
            ButtonSize::Medium => builder.responsive("px-4 py-2 text-sm rounded-md"),
            ButtonSize::Large => builder.responsive("px-6 py-3 text-base rounded-lg"),
        };

        // State classes - use input signals directly
        let builder = if disabled.get() {
            builder.state("opacity-50 cursor-not-allowed")
        } else if loading.get() {
            builder.state("opacity-75 cursor-wait")
        } else {
            builder
        };

        // Add custom classes if provided
        let builder = if let Some(custom_class) = &class {
            builder.custom(custom_class)
        } else {
            builder
        };

        builder.classes()
    };

    view! {
        <button
            class=classes
            disabled=move || disabled.get() || loading.get()
        >
            {move || if loading.get() {
                view! { <span class="animate-spin mr-2">"..."</span> }.into_any()
            } else {
                ().into_any()
            }}
            {children.map(|c| c())}
        </button>
    }
}

/// Card component
#[component]
pub fn Card(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = move || {
        let builder =
            DynamicClassBuilder::new().base("bg-white rounded-lg shadow-md border border-gray-200");

        if let Some(custom_class) = &class {
            builder.custom(custom_class).classes()
        } else {
            builder.classes()
        }
    };

    view! {
        <div class=classes>
            {children.map(|c| c())}
        </div>
    }
}

/// Card header component
#[component]
pub fn CardHeader(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = move || {
        let builder = DynamicClassBuilder::new().base("px-6 py-4 border-b border-gray-200");

        if let Some(custom_class) = &class {
            builder.custom(custom_class).classes()
        } else {
            builder.classes()
        }
    };

    view! {
        <div class=classes>
            {children.map(|c| c())}
        </div>
    }
}

/// Card body component
#[component]
pub fn CardBody(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = move || {
        let builder = DynamicClassBuilder::new().base("px-6 py-4");

        if let Some(custom_class) = &class {
            builder.custom(custom_class).classes()
        } else {
            builder.classes()
        }
    };

    view! {
        <div class=classes>
            {children.map(|c| c())}
        </div>
    }
}

/// Input component
#[component]
pub fn Input(
    #[prop(into, optional)] input_type: Signal<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = move || {
        let builder = DynamicClassBuilder::new()
            .base("block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500 disabled:bg-gray-50 disabled:text-gray-500");

        if let Some(custom_class) = &class {
            builder.custom(custom_class).classes()
        } else {
            builder.classes()
        }
    };

    view! {
        <input
            type=move || input_type.get()
            class=classes
            placeholder=placeholder
            disabled=move || disabled.get()
        />
        {children.map(|c| c())}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_variant_enum() {
        assert_eq!(ButtonVariant::Primary, ButtonVariant::Primary);
        assert_ne!(ButtonVariant::Primary, ButtonVariant::Secondary);
    }

    #[test]
    fn test_button_size_enum() {
        assert_eq!(ButtonSize::Small, ButtonSize::Small);
        assert_ne!(ButtonSize::Small, ButtonSize::Medium);
    }

    #[test]
    fn test_dynamic_class_builder_usage() {
        let builder = DynamicClassBuilder::new()
            .base("px-4 py-2")
            .variant("bg-blue-600 text-white")
            .responsive("sm:text-sm md:text-base")
            .state("hover:bg-blue-700")
            .custom("rounded-lg");

        let classes = builder.classes();
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("sm:text-sm"));
        assert!(classes.contains("md:text-base"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("rounded-lg"));
    }
}
