//! Macro-specific parser implementations
//! 
//! This module contains parsers for specific macro types like theme, component, state, and variant.

use syn::{parse::Parse, LitStr, Token};
use tailwind_rs_core::ClassSet;
use super::core_parsers::MacroParser;

/// Parser for the `theme!` macro
#[derive(Debug, Clone)]
pub struct ThemeMacro {
    pub color: Option<String>,
    pub spacing: Option<String>,
    pub border_radius: Option<String>,
    pub box_shadow: Option<String>,
    pub custom: Vec<(String, String)>,
}

impl Parse for ThemeMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut color = None;
        let mut spacing = None;
        let mut border_radius = None;
        let mut box_shadow = None;
        let mut custom = Vec::new();

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: LitStr = input.parse()?;

            match key.to_string().as_str() {
                "color" => color = Some(value.value()),
                "spacing" => spacing = Some(value.value()),
                "border_radius" => border_radius = Some(value.value()),
                "box_shadow" => box_shadow = Some(value.value()),
                _ => custom.push((key.to_string(), value.value())),
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(ThemeMacro {
            color,
            spacing,
            border_radius,
            box_shadow,
            custom,
        })
    }
}

impl MacroParser for ThemeMacro {
    fn to_class_set(&self) -> ClassSet {
        let mut class_set = ClassSet::new();

        if let Some(ref color) = self.color {
            class_set.add_class(format!("bg-{}-500", color));
        }

        if let Some(ref spacing) = self.spacing {
            class_set.add_class(format!("p-{}", spacing));
        }

        if let Some(ref border_radius) = self.border_radius {
            class_set.add_class(format!("rounded-{}", border_radius));
        }

        if let Some(ref box_shadow) = self.box_shadow {
            class_set.add_class(format!("shadow-{}", box_shadow));
        }

        for (key, value) in &self.custom {
            class_set.add_custom(key.clone(), value.clone());
        }

        class_set
    }
}

/// Parser for the `component!` macro
#[derive(Debug, Clone)]
pub struct ComponentMacro {
    pub name: Option<String>,
    pub variant: Option<String>,
    pub size: Option<String>,
    pub state: Option<String>,
    pub custom: Vec<(String, String)>,
}

impl Parse for ComponentMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut variant = None;
        let mut size = None;
        let mut state = None;
        let mut custom = Vec::new();

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: LitStr = input.parse()?;

            match key.to_string().as_str() {
                "name" => name = Some(value.value()),
                "variant" => variant = Some(value.value()),
                "size" => size = Some(value.value()),
                "state" => state = Some(value.value()),
                _ => custom.push((key.to_string(), value.value())),
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(ComponentMacro {
            name,
            variant,
            size,
            state,
            custom,
        })
    }
}

impl MacroParser for ComponentMacro {
    fn to_class_set(&self) -> ClassSet {
        let mut class_set = ClassSet::new();

        // Add base component classes
        if let Some(ref name) = self.name {
            class_set.add_class(name.to_string());
        }

        // Add variant classes
        if let Some(ref variant) = self.variant {
            class_set.add_class(format!(
                "{}-{}",
                self.name.as_deref().unwrap_or("component"),
                variant
            ));
        }

        // Add size classes
        if let Some(ref size) = self.size {
            class_set.add_class(format!(
                "{}-{}",
                self.name.as_deref().unwrap_or("component"),
                size
            ));
        }

        // Add state classes
        if let Some(ref state) = self.state {
            class_set.add_class(format!(
                "{}-{}",
                self.name.as_deref().unwrap_or("component"),
                state
            ));
        }

        for (key, value) in &self.custom {
            class_set.add_custom(key.clone(), value.clone());
        }

        class_set
    }
}

/// Parser for the `state!` macro
#[derive(Debug, Clone)]
pub struct StateMacro {
    pub base: Option<String>,
    pub hover: Option<String>,
    pub focus: Option<String>,
    pub active: Option<String>,
    pub disabled: Option<String>,
    pub custom: Vec<(String, String)>,
}

impl Parse for StateMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut base = None;
        let mut hover = None;
        let mut focus = None;
        let mut active = None;
        let mut disabled = None;
        let mut custom = Vec::new();

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: LitStr = input.parse()?;

            match key.to_string().as_str() {
                "base" => base = Some(value.value()),
                "hover" => hover = Some(value.value()),
                "focus" => focus = Some(value.value()),
                "active" => active = Some(value.value()),
                "disabled" => disabled = Some(value.value()),
                _ => custom.push((key.to_string(), value.value())),
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(StateMacro {
            base,
            hover,
            focus,
            active,
            disabled,
            custom,
        })
    }
}

impl MacroParser for StateMacro {
    fn to_class_set(&self) -> ClassSet {
        let mut class_set = ClassSet::new();

        if let Some(ref base) = self.base {
            class_set.add_classes(base.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref hover) = self.hover {
            class_set.add_classes(hover.split_whitespace().map(|s| format!("hover:{}", s)));
        }

        if let Some(ref focus) = self.focus {
            class_set.add_classes(focus.split_whitespace().map(|s| format!("focus:{}", s)));
        }

        if let Some(ref active) = self.active {
            class_set.add_classes(active.split_whitespace().map(|s| format!("active:{}", s)));
        }

        if let Some(ref disabled) = self.disabled {
            class_set.add_classes(
                disabled
                    .split_whitespace()
                    .map(|s| format!("disabled:{}", s)),
            );
        }

        for (key, value) in &self.custom {
            class_set.add_custom(key.clone(), value.clone());
        }

        class_set
    }
}

/// Parser for the `variant!` macro
#[derive(Debug, Clone)]
pub struct VariantMacro {
    pub base: Option<String>,
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub danger: Option<String>,
    pub success: Option<String>,
    pub warning: Option<String>,
    pub info: Option<String>,
    pub custom: Vec<(String, String)>,
}

impl Parse for VariantMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut base = None;
        let mut primary = None;
        let mut secondary = None;
        let mut danger = None;
        let mut success = None;
        let mut warning = None;
        let mut info = None;
        let mut custom = Vec::new();

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: LitStr = input.parse()?;

            match key.to_string().as_str() {
                "base" => base = Some(value.value()),
                "primary" => primary = Some(value.value()),
                "secondary" => secondary = Some(value.value()),
                "danger" => danger = Some(value.value()),
                "success" => success = Some(value.value()),
                "warning" => warning = Some(value.value()),
                "info" => info = Some(value.value()),
                _ => custom.push((key.to_string(), value.value())),
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(VariantMacro {
            base,
            primary,
            secondary,
            danger,
            success,
            warning,
            info,
            custom,
        })
    }
}

impl MacroParser for VariantMacro {
    fn to_class_set(&self) -> ClassSet {
        let mut class_set = ClassSet::new();

        if let Some(ref base) = self.base {
            class_set.add_classes(base.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref primary) = self.primary {
            class_set.add_classes(primary.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref secondary) = self.secondary {
            class_set.add_classes(secondary.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref danger) = self.danger {
            class_set.add_classes(danger.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref success) = self.success {
            class_set.add_classes(success.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref warning) = self.warning {
            class_set.add_classes(warning.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref info) = self.info {
            class_set.add_classes(info.split_whitespace().map(|s| s.to_string()));
        }

        for (key, value) in &self.custom {
            class_set.add_custom(key.clone(), value.clone());
        }

        class_set
    }
}
