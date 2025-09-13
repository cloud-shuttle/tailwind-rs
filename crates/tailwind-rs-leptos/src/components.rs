//! # Leptos Components
//!
//! This module provides pre-built Leptos components with Tailwind CSS styling.
//! Updated for Leptos 0.8.8 signal system with proper lifecycle management.

use leptos::prelude::*;
use crate::DynamicClassBuilder;

/// Enhanced Button component with proper Leptos 0.8.8 signal management
#[component]
pub fn Button(
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] loading: Signal<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Use ArcRwSignal for internal state that needs to persist
    let internal_variant = ArcRwSignal::new(variant.get());
    let internal_size = ArcRwSignal::new(size.get());
    let internal_disabled = ArcRwSignal::new(disabled.get());
    let internal_loading = ArcRwSignal::new(loading.get());
    
    // Sync external props with internal state using reactive effects
    {
        let internal_variant = internal_variant.clone();
        let internal_size = internal_size.clone();
        let internal_disabled = internal_disabled.clone();
        let internal_loading = internal_loading.clone();
        
        Effect::new(move |_| {
            internal_variant.set(variant.get());
            internal_size.set(size.get());
            internal_disabled.set(disabled.get());
            internal_loading.set(loading.get());
        });
    }
    
    // Clone signals for use in memos and closures
    let internal_variant_memo = internal_variant.clone();
    let internal_size_memo = internal_size.clone();
    let internal_disabled_memo = internal_disabled.clone();
    let internal_loading_memo = internal_loading.clone();
    let internal_disabled_button = internal_disabled.clone();
    let internal_loading_button = internal_loading.clone();
    let internal_loading_view = internal_loading.clone();
    
    // Use ArcMemo for computed classes
    let classes = ArcMemo::new(move |_| {
        let builder = DynamicClassBuilder::new();
        
        builder.base("inline-flex items-center justify-center font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed");
        
        // Variant classes
        match internal_variant_memo.get() {
            ButtonVariant::Primary => builder.variant("bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500"),
            ButtonVariant::Secondary => builder.variant("bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-500"),
            ButtonVariant::Danger => builder.variant("bg-red-600 text-white hover:bg-red-700 focus:ring-red-500"),
            ButtonVariant::Outline => builder.variant("border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 focus:ring-gray-500"),
        };
        
        // Size classes
        match internal_size_memo.get() {
            ButtonSize::Small => builder.responsive("px-3 py-1.5 text-sm rounded-md"),
            ButtonSize::Medium => builder.responsive("px-4 py-2 text-sm rounded-md"),
            ButtonSize::Large => builder.responsive("px-6 py-3 text-base rounded-lg"),
        };
        
        // State classes
        if internal_disabled_memo.get() {
            builder.state("opacity-50 cursor-not-allowed");
        } else if internal_loading_memo.get() {
            builder.state("opacity-75 cursor-wait");
        }
        
        // Add custom classes if provided
        if let Some(custom_class) = &class {
            builder.custom(custom_class);
        }
        
        builder.classes().get()
    });
    
    view! {
        <button 
            class=classes
            disabled=move || internal_disabled_button.get() || internal_loading_button.get()
        >
            {move || if internal_loading_view.get() {
                view! { <span class="animate-spin mr-2">"..."</span> }.into_any()
            } else {
                ().into_any()
            }}
            {children.map(|c| c())}
        </button>
    }
}

/// Button variant enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Outline,
}

/// Button size enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Medium,
    Small,
    Large,
}

/// Enhanced Card component with proper Leptos 0.8.8 signal management
#[component]
pub fn Card(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Use ArcMemo for computed classes
    let classes = ArcMemo::new(move |_| {
        let builder = DynamicClassBuilder::new();
        
        builder.base("bg-white rounded-lg shadow-md border border-gray-200");
        
        // Add custom classes if provided
        if let Some(custom_class) = &class {
            builder.custom(custom_class);
        }
        
        builder.classes().get()
    });

    view! {
        <div class=classes>
            {children.map(|c| c())}
        </div>
    }
}

/// Enhanced Card header component with proper Leptos 0.8.8 signal management
#[component]
pub fn CardHeader(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Use ArcMemo for computed classes
    let classes = ArcMemo::new(move |_| {
        let builder = DynamicClassBuilder::new();
        
        builder.base("px-6 py-4 border-b border-gray-200");
        
        // Add custom classes if provided
        if let Some(custom_class) = &class {
            builder.custom(custom_class);
        }
        
        builder.classes().get()
    });

    view! {
        <div class=classes>
            {children.map(|c| c())}
        </div>
    }
}

/// Enhanced Card body component with proper Leptos 0.8.8 signal management
#[component]
pub fn CardBody(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Use ArcMemo for computed classes
    let classes = ArcMemo::new(move |_| {
        let builder = DynamicClassBuilder::new();
        
        builder.base("px-6 py-4");
        
        // Add custom classes if provided
        if let Some(custom_class) = &class {
            builder.custom(custom_class);
        }
        
        builder.classes().get()
    });

    view! {
        <div class=classes>
            {children.map(|c| c())}
        </div>
    }
}

/// Enhanced Input component with proper Leptos 0.8.8 signal management
#[component]
pub fn Input(
    #[prop(into, optional)] input_type: Signal<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] value: Option<ReadSignal<String>>,
    #[prop(optional)] on_input: Option<Box<dyn FnMut(web_sys::Event) + 'static>>,
) -> impl IntoView {
    // Use ArcRwSignal for internal state that needs to persist
    let internal_type = ArcRwSignal::new(input_type.get());
    let internal_disabled = ArcRwSignal::new(disabled.get());
    
    // Sync external props with internal state using reactive effects
    {
        let internal_type = internal_type.clone();
        let internal_disabled = internal_disabled.clone();
        
        Effect::new(move |_| {
            internal_type.set(input_type.get());
            internal_disabled.set(disabled.get());
        });
    }

    // Use ArcMemo for computed classes
    let classes = ArcMemo::new(move |_| {
        let builder = DynamicClassBuilder::new();
        
        builder.base("block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500 disabled:bg-gray-50 disabled:text-gray-500");
        
        // Add custom classes if provided
        if let Some(custom_class) = &class {
            builder.custom(custom_class);
        }
        
        builder.classes().get()
    });

    view! {
        <input
            type=move || internal_type.get()
            class=classes
            placeholder=placeholder
            disabled=move || internal_disabled.get()
            value=value.map(|v| v.get())
            on:input=on_input.unwrap_or_else(|| Box::new(|_| {}))
        />
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
        let builder = tailwind_rs_core::ClassBuilder::new()
            .class("px-4 py-2")
            .class("bg-blue-600 text-white");

        let result = builder.build().to_css_classes();
        assert!(result.contains("px-4"));
        assert!(result.contains("bg-blue-600"));
    }
}
