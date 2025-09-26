//! # Yew Props
//!
//! This module provides common prop types for Yew components with Tailwind styling.

use std::collections::HashMap;
use tailwind_rs_core::ClassBuilder;
use yew::prelude::*;

/// Common props for styled components
#[derive(Properties, PartialEq, Clone)]
pub struct StyledProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

/// Props for components with variants
#[derive(Properties, PartialEq, Clone)]
pub struct VariantProps<T: Default + PartialEq> {
    #[prop_or_default]
    pub variant: T,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

/// Props for components with sizes
#[derive(Properties, PartialEq, Clone)]
pub struct SizeProps<T: Default + PartialEq> {
    #[prop_or_default]
    pub size: T,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

/// Props for components with variants and sizes
#[derive(Properties, PartialEq, Clone)]
pub struct VariantSizeProps<T: Default + PartialEq, U: Default + PartialEq> {
    #[prop_or_default]
    pub variant: T,
    #[prop_or_default]
    pub size: U,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

/// Props for interactive components
#[derive(Properties, PartialEq, Clone)]
pub struct InteractiveProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

/// Props for form components
#[derive(Properties, PartialEq, Clone)]
pub struct FormProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,
}

/// Props for layout components
#[derive(Properties, PartialEq, Clone)]
pub struct LayoutProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub style: Option<String>,
}

/// Props for responsive components
#[derive(Properties, PartialEq, Clone)]
pub struct ResponsiveProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub responsive_classes: Option<HashMap<String, String>>,
}

/// Utility for creating class props
pub fn create_class_props(base_classes: &str, custom_class: Option<String>) -> Classes {
    let mut builder = ClassBuilder::new().class(base_classes);

    if let Some(custom) = custom_class {
        builder = builder.class(&custom);
    }

    Classes::from(builder.build().to_css_classes())
}

/// Utility for creating variant class props
pub fn create_variant_class_props<T>(
    base_classes: &str,
    variant: T,
    variant_classes: impl Fn(T) -> &'static str,
    custom_class: Option<String>,
) -> Classes
where
    T: Copy,
{
    let mut builder = ClassBuilder::new()
        .class(base_classes)
        .class(variant_classes(variant));

    if let Some(custom) = custom_class {
        builder = builder.class(&custom);
    }

    Classes::from(builder.build().to_css_classes())
}

/// Utility for creating size class props
pub fn create_size_class_props<T>(
    base_classes: &str,
    size: T,
    size_classes: impl Fn(T) -> &'static str,
    custom_class: Option<String>,
) -> Classes
where
    T: Copy,
{
    let mut builder = ClassBuilder::new()
        .class(base_classes)
        .class(size_classes(size));

    if let Some(custom) = custom_class {
        builder = builder.class(&custom);
    }

    Classes::from(builder.build().to_css_classes())
}

/// Utility for creating conditional class props
pub fn create_conditional_class_props(
    base_classes: &str,
    condition: bool,
    conditional_classes: &str,
    custom_class: Option<String>,
) -> Classes {
    let mut builder = ClassBuilder::new().class(base_classes);

    if condition {
        builder = builder.class(conditional_classes);
    }

    if let Some(custom) = custom_class {
        builder = builder.class(&custom);
    }

    Classes::from(builder.build().to_css_classes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_class_props() {
        let classes = create_class_props("px-4 py-2", Some("bg-blue-600".to_string()));
        let class_string = classes.to_string();

        assert!(class_string.contains("px-4"));
        assert!(class_string.contains("py-2"));
        assert!(class_string.contains("bg-blue-600"));
    }

    #[test]
    fn test_create_variant_class_props() {
        #[derive(Clone, Copy, PartialEq)]
        enum TestVariant {
            Primary,
            Secondary,
        }

        let classes = create_variant_class_props(
            "px-4",
            TestVariant::Primary,
            |v| match v {
                TestVariant::Primary => "bg-blue-600",
                TestVariant::Secondary => "bg-gray-600",
            },
            None,
        );

        let class_string = classes.to_string();
        assert!(class_string.contains("px-4"));
        assert!(class_string.contains("bg-blue-600"));
    }

    #[test]
    fn test_create_size_class_props() {
        #[derive(Clone, Copy, PartialEq)]
        enum TestSize {
            Small,
            Large,
        }

        let classes = create_size_class_props(
            "px-4",
            TestSize::Small,
            |s| match s {
                TestSize::Small => "py-1",
                TestSize::Large => "py-3",
            },
            None,
        );

        let class_string = classes.to_string();
        assert!(class_string.contains("px-4"));
        assert!(class_string.contains("py-1"));
    }

    #[test]
    fn test_create_conditional_class_props() {
        let classes = create_conditional_class_props("px-4", true, "bg-blue-600", None);

        let class_string = classes.to_string();
        assert!(class_string.contains("px-4"));
        assert!(class_string.contains("bg-blue-600"));

        let classes = create_conditional_class_props("px-4", false, "bg-blue-600", None);

        let class_string = classes.to_string();
        assert!(class_string.contains("px-4"));
        assert!(!class_string.contains("bg-blue-600"));
    }
}
