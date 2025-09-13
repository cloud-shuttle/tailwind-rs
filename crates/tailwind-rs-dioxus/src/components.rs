//! # Dioxus Components
//!
//! This module provides pre-built Dioxus components with Tailwind CSS styling.

use dioxus::prelude::*;
use tailwind_rs_core::ClassBuilder;

/// Button component with Tailwind styling
#[component]
pub fn Button(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    disabled: Option<bool>,
    class: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or(ButtonVariant::Primary);
    let size = size.unwrap_or(ButtonSize::Medium);
    let disabled = disabled.unwrap_or(false);

    let classes = use_memo(move || {
        let mut builder = ClassBuilder::new()
            .class("inline-flex items-center justify-center font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed");

        // Add variant classes
        match variant {
            ButtonVariant::Primary => {
                builder =
                    builder.class("bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500");
            }
            ButtonVariant::Secondary => {
                builder = builder
                    .class("bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-500");
            }
            ButtonVariant::Danger => {
                builder =
                    builder.class("bg-red-600 text-white hover:bg-red-700 focus:ring-red-500");
            }
            ButtonVariant::Outline => {
                builder = builder.class("border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 focus:ring-gray-500");
            }
        }

        // Add size classes
        match size {
            ButtonSize::Small => {
                builder = builder.class("px-3 py-1.5 text-sm rounded-md");
            }
            ButtonSize::Medium => {
                builder = builder.class("px-4 py-2 text-sm rounded-md");
            }
            ButtonSize::Large => {
                builder = builder.class("px-6 py-3 text-base rounded-lg");
            }
        }

        // Add custom classes if provided
        if let Some(custom_class) = &class {
            builder = builder.class(custom_class);
        }

        builder.build_string()
    });

    rsx! {
        button {
            class: classes,
            disabled: disabled,
            onclick: onclick,
            {children}
        }
    }
}

/// Button variant enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Outline,
}

/// Button size enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// Card component with Tailwind styling
#[component]
pub fn Card(class: Option<String>, children: Element) -> Element {
    let classes = use_memo(move || {
        let mut builder =
            ClassBuilder::new().class("bg-white rounded-lg shadow-md border border-gray-200");

        if let Some(custom_class) = &class {
            builder = builder.class(custom_class);
        }

        builder.build_string()
    });

    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

/// Card header component
#[component]
pub fn CardHeader(class: Option<String>, children: Element) -> Element {
    let classes = use_memo(move || {
        let mut builder = ClassBuilder::new().class("px-6 py-4 border-b border-gray-200");

        if let Some(custom_class) = &class {
            builder = builder.class(custom_class);
        }

        builder.build_string()
    });

    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

/// Card body component
#[component]
pub fn CardBody(class: Option<String>, children: Element) -> Element {
    let classes = use_memo(move || {
        let mut builder = ClassBuilder::new().class("px-6 py-4");

        if let Some(custom_class) = &class {
            builder = builder.class(custom_class);
        }

        builder.build_string()
    });

    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

/// Input component with Tailwind styling
#[component]
pub fn Input(
    input_type: Option<String>,
    placeholder: Option<String>,
    disabled: Option<bool>,
    class: Option<String>,
    value: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    let disabled = disabled.unwrap_or(false);

    let classes = use_memo(move || {
        let mut builder = ClassBuilder::new()
            .base("block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500 disabled:bg-gray-50 disabled:text-gray-500");

        if let Some(custom_class) = &class {
            builder = builder.variant(custom_class);
        }

        builder.build()
    });

    rsx! {
        input {
            r#type: input_type,
            class: classes,
            placeholder: placeholder,
            disabled: disabled,
            value: value,
            oninput: oninput,
        }
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
        assert_eq!(ButtonSize::Medium, ButtonSize::Medium);
        assert_ne!(ButtonSize::Small, ButtonSize::Large);
    }

    #[test]
    fn test_class_builder_integration() {
        let builder = ClassBuilder::new()
            .base("px-4 py-2")
            .variant("bg-blue-600 text-white");

        let result = builder.build();
        assert!(result.contains("px-4"));
        assert!(result.contains("bg-blue-600"));
    }
}
