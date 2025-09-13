//! # Yew Components
//!
//! This module provides pre-built Yew components with Tailwind CSS styling.

use tailwind_rs_core::ClassBuilder;
use yew::prelude::*;

/// Button component with Tailwind styling
#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<String>,
    pub children: Children,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let classes = use_memo(
        (props.variant, props.size, props.class.clone()),
        |(variant, size, class)| {
            let mut builder = ClassBuilder::new()
                .class("inline-flex items-center justify-center font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed");

            // Add variant classes
            match variant {
                ButtonVariant::Primary => {
                    builder = builder
                        .class("bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500");
                }
                ButtonVariant::Secondary => {
                    builder = builder
                        .class("bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-500");
                }
                ButtonVariant::Danger => {
                    builder = builder
                        .class("bg-red-600 text-white hover:bg-red-700 focus:ring-red-500");
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
            if let Some(custom_class) = class {
                builder = builder.class(custom_class);
            }

            Classes::from(builder.build().to_css_classes())
        },
    );

    html! {
        <button
            class={(*classes).clone()}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
        >
            {props.children.clone()}
        </button>
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

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

/// Button size enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Card component with Tailwind styling
#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    #[prop_or_default]
    pub class: Option<String>,
    pub children: Children,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    let classes = use_memo(
        props.class.clone(),
        |class| {
            let mut builder =
                ClassBuilder::new().class("bg-white rounded-lg shadow-md border border-gray-200");

            if let Some(custom_class) = class {
                builder = builder.class(custom_class);
            }

            Classes::from(builder.build().to_css_classes())
        },
    );

    html! {
        <div class={(*classes).clone()}>
            {props.children.clone()}
        </div>
    }
}

/// Card header component
#[derive(Properties, PartialEq, Clone)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub class: Option<String>,
    pub children: Children,
}

#[function_component]
pub fn CardHeader(props: &CardHeaderProps) -> Html {
    let classes = use_memo(
        props.class.clone(),
        |class| {
            let mut builder = ClassBuilder::new().class("px-6 py-4 border-b border-gray-200");

            if let Some(custom_class) = class {
                builder = builder.class(custom_class);
            }

            Classes::from(builder.build().to_css_classes())
        },
    );

    html! {
        <div class={(*classes).clone()}>
            {props.children.clone()}
        </div>
    }
}

/// Card body component
#[derive(Properties, PartialEq, Clone)]
pub struct CardBodyProps {
    #[prop_or_default]
    pub class: Option<String>,
    pub children: Children,
}

#[function_component]
pub fn CardBody(props: &CardBodyProps) -> Html {
    let classes = use_memo(
        props.class.clone(),
        |class| {
            let mut builder = ClassBuilder::new().class("px-6 py-4");

            if let Some(custom_class) = class {
                builder = builder.class(custom_class);
            }

            Classes::from(builder.build().to_css_classes())
        },
    );

    html! {
        <div class={(*classes).clone()}>
            {props.children.clone()}
        </div>
    }
}

/// Input component with Tailwind styling
#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    #[prop_or_default]
    pub input_type: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,
}

#[function_component]
pub fn Input(props: &InputProps) -> Html {
    let classes = use_memo(
        props.class.clone(),
        |class| {
            let mut builder = ClassBuilder::new()
                .class("block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500 disabled:bg-gray-50 disabled:text-gray-500");

            if let Some(custom_class) = class {
                builder = builder.class(custom_class);
            }

            Classes::from(builder.build().to_css_classes())
        },
    );

    html! {
        <input
            type={props.input_type.clone().unwrap_or_else(|| "text".to_string())}
            class={(*classes).clone()}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            value={props.value.clone()}
            oninput={props.oninput.clone()}
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::prelude::*;

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
            .class("px-4 py-2")
            .class("bg-blue-600 text-white");

        let result = builder.build();
        let class_string = result.to_css_classes();
        assert!(class_string.contains("px-4"));
        assert!(class_string.contains("bg-blue-600"));
    }
}
