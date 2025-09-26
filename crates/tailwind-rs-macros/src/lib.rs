//! # tailwind-rs-macros
//!
//! Procedural macros for the tailwind-rs library.
//! This crate provides macros for generating type-safe Tailwind CSS classes.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, parse_macro_input, LitStr, Token};

/// The main `classes!` macro for generating Tailwind CSS classes
///
/// # Examples
///
/// ```rust
/// use tailwind_rs_macros::classes;
///
/// let classes = classes! {
///     base: "bg-blue-500 text-white",
///     variant: "px-4 py-2",
///     responsive: "sm:text-sm md:text-base",
///     state: "hover:bg-blue-600 focus:ring-2"
/// };
/// ```
#[proc_macro]
pub fn classes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClassesMacro);

    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}

/// The `responsive!` macro for generating responsive classes
///
/// # Examples
///
/// ```rust
/// use tailwind_rs_macros::responsive;
///
/// let responsive_classes = responsive! {
///     base: "text-sm",
///     sm: "text-base",
///     md: "text-lg",
///     lg: "text-xl"
/// };
/// ```
#[proc_macro]
pub fn responsive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ResponsiveMacro);

    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}

/// The `theme!` macro for generating theme-based classes
///
/// # Examples
///
/// ```rust
/// use tailwind_rs_macros::theme;
///
/// let theme_classes = theme! {
///     color: "primary",
///     spacing: "md",
///     border_radius: "lg"
/// };
/// ```
#[proc_macro]
pub fn theme(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ThemeMacro);

    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}

/// The `component!` macro for generating component classes
///
/// # Examples
///
/// ```rust
/// use tailwind_rs_macros::component;
///
/// let component_classes = component! {
///     name: "button",
///     variant: "primary",
///     size: "md"
/// };
/// ```
#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ComponentMacro);

    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}

/// The `state!` macro for generating state-based classes
///
/// # Examples
///
/// ```rust
/// use tailwind_rs_macros::state;
///
/// let state_classes = state! {
///     base: "px-4 py-2 rounded-md",
///     hover: "bg-blue-700",
///     focus: "ring-2 ring-blue-500",
///     active: "bg-blue-800"
/// };
/// ```
#[proc_macro]
pub fn state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StateMacro);

    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}

/// The `variant!` macro for generating component variants
///
/// # Examples
///
/// ```rust
/// use tailwind_rs_macros::variant;
///
/// let variant_classes = variant! {
///     base: "px-4 py-2 rounded-md font-medium",
///     primary: "bg-blue-600 text-white hover:bg-blue-700",
///     secondary: "bg-gray-200 text-gray-900 hover:bg-gray-300",
///     danger: "bg-red-600 text-white hover:bg-red-700"
/// };
/// ```
#[proc_macro]
pub fn variant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as VariantMacro);

    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}

/// Parser for the `classes!` macro
struct ClassesMacro {
    base: Option<String>,
    variant: Option<String>,
    responsive: Option<String>,
    state: Option<String>,
    custom: Vec<(String, String)>,
}

impl syn::parse::Parse for ClassesMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut base = None;
        let mut variant = None;
        let mut responsive = None;
        let mut state = None;
        let mut custom = Vec::new();

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: LitStr = input.parse()?;

            match key.to_string().as_str() {
                "base" => base = Some(value.value()),
                "variant" => variant = Some(value.value()),
                "responsive" => responsive = Some(value.value()),
                "state" => state = Some(value.value()),
                _ => custom.push((key.to_string(), value.value())),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(ClassesMacro {
            base,
            variant,
            responsive,
            state,
            custom,
        })
    }
}

impl ClassesMacro {
    fn to_class_set(&self) -> tailwind_rs_core::ClassSet {
        let mut class_set = tailwind_rs_core::ClassSet::new();

        if let Some(ref base) = self.base {
            class_set.add_classes(base.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref variant) = self.variant {
            class_set.add_classes(variant.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref responsive) = self.responsive {
            class_set.add_classes(responsive.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref state) = self.state {
            class_set.add_classes(state.split_whitespace().map(|s| s.to_string()));
        }

        for (key, value) in &self.custom {
            class_set.add_custom(key.clone(), value.clone());
        }

        class_set
    }
}

/// Parser for the `responsive!` macro
struct ResponsiveMacro {
    base: Option<String>,
    sm: Option<String>,
    md: Option<String>,
    lg: Option<String>,
    xl: Option<String>,
    xl2: Option<String>,
}

impl syn::parse::Parse for ResponsiveMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut base = None;
        let mut sm = None;
        let mut md = None;
        let mut lg = None;
        let mut xl = None;
        let mut xl2 = None;

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: LitStr = input.parse()?;

            match key.to_string().as_str() {
                "base" => base = Some(value.value()),
                "sm" => sm = Some(value.value()),
                "md" => md = Some(value.value()),
                "lg" => lg = Some(value.value()),
                "xl" => xl = Some(value.value()),
                "2xl" => xl2 = Some(value.value()),
                _ => {
                    return Err(syn::Error::new_spanned(
                        key,
                        "Invalid responsive breakpoint",
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(ResponsiveMacro {
            base,
            sm,
            md,
            lg,
            xl,
            xl2,
        })
    }
}

impl ResponsiveMacro {
    fn to_class_set(&self) -> tailwind_rs_core::ClassSet {
        let mut class_set = tailwind_rs_core::ClassSet::new();

        if let Some(ref base) = self.base {
            class_set.add_classes(base.split_whitespace().map(|s| s.to_string()));
        }

        if let Some(ref sm) = self.sm {
            class_set.add_responsive_class(tailwind_rs_core::Breakpoint::Sm, sm.clone());
        }

        if let Some(ref md) = self.md {
            class_set.add_responsive_class(tailwind_rs_core::Breakpoint::Md, md.clone());
        }

        if let Some(ref lg) = self.lg {
            class_set.add_responsive_class(tailwind_rs_core::Breakpoint::Lg, lg.clone());
        }

        if let Some(ref xl) = self.xl {
            class_set.add_responsive_class(tailwind_rs_core::Breakpoint::Xl, xl.clone());
        }

        if let Some(ref xl2) = self.xl2 {
            class_set.add_responsive_class(tailwind_rs_core::Breakpoint::Xl2, xl2.clone());
        }

        class_set
    }
}

/// Parser for the `theme!` macro
struct ThemeMacro {
    color: Option<String>,
    spacing: Option<String>,
    border_radius: Option<String>,
    box_shadow: Option<String>,
    custom: Vec<(String, String)>,
}

impl syn::parse::Parse for ThemeMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
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

impl ThemeMacro {
    fn to_class_set(&self) -> tailwind_rs_core::ClassSet {
        let mut class_set = tailwind_rs_core::ClassSet::new();

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
struct ComponentMacro {
    name: Option<String>,
    variant: Option<String>,
    size: Option<String>,
    state: Option<String>,
    custom: Vec<(String, String)>,
}

impl syn::parse::Parse for ComponentMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
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

impl ComponentMacro {
    fn to_class_set(&self) -> tailwind_rs_core::ClassSet {
        let mut class_set = tailwind_rs_core::ClassSet::new();

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
struct StateMacro {
    base: Option<String>,
    hover: Option<String>,
    focus: Option<String>,
    active: Option<String>,
    disabled: Option<String>,
    custom: Vec<(String, String)>,
}

impl syn::parse::Parse for StateMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
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

impl StateMacro {
    fn to_class_set(&self) -> tailwind_rs_core::ClassSet {
        let mut class_set = tailwind_rs_core::ClassSet::new();

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
struct VariantMacro {
    base: Option<String>,
    primary: Option<String>,
    secondary: Option<String>,
    danger: Option<String>,
    success: Option<String>,
    warning: Option<String>,
    info: Option<String>,
    custom: Vec<(String, String)>,
}

impl syn::parse::Parse for VariantMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
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

impl VariantMacro {
    fn to_class_set(&self) -> tailwind_rs_core::ClassSet {
        let mut class_set = tailwind_rs_core::ClassSet::new();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classes_macro_parser() {
        let input = quote! {
            base: "bg-blue-500 text-white",
            variant: "px-4 py-2",
            responsive: "sm:text-sm md:text-base",
            state: "hover:bg-blue-600 focus:ring-2"
        };

        let parsed = syn::parse2::<ClassesMacro>(input).unwrap();
        assert_eq!(parsed.base, Some("bg-blue-500 text-white".to_string()));
        assert_eq!(parsed.variant, Some("px-4 py-2".to_string()));
        assert_eq!(
            parsed.responsive,
            Some("sm:text-sm md:text-base".to_string())
        );
        assert_eq!(
            parsed.state,
            Some("hover:bg-blue-600 focus:ring-2".to_string())
        );
    }

    #[test]
    fn test_responsive_macro_parser() {
        let input = quote! {
            base: "text-sm",
            sm: "text-base",
            md: "text-lg",
            lg: "text-xl"
        };

        let parsed = syn::parse2::<ResponsiveMacro>(input).unwrap();
        assert_eq!(parsed.base, Some("text-sm".to_string()));
        assert_eq!(parsed.sm, Some("text-base".to_string()));
        assert_eq!(parsed.md, Some("text-lg".to_string()));
        assert_eq!(parsed.lg, Some("text-xl".to_string()));
    }

    #[test]
    fn test_theme_macro_parser() {
        let input = quote! {
            color: "primary",
            spacing: "md",
            border_radius: "lg"
        };

        let parsed = syn::parse2::<ThemeMacro>(input).unwrap();
        assert_eq!(parsed.color, Some("primary".to_string()));
        assert_eq!(parsed.spacing, Some("md".to_string()));
        assert_eq!(parsed.border_radius, Some("lg".to_string()));
    }

    #[test]
    fn test_component_macro_parser() {
        let input = quote! {
            name: "button",
            variant: "primary",
            size: "md"
        };

        let parsed = syn::parse2::<ComponentMacro>(input).unwrap();
        assert_eq!(parsed.name, Some("button".to_string()));
        assert_eq!(parsed.variant, Some("primary".to_string()));
        assert_eq!(parsed.size, Some("md".to_string()));
    }

    #[test]
    fn test_state_macro_parser() {
        let input = quote! {
            base: "px-4 py-2 rounded-md",
            hover: "bg-blue-700",
            focus: "ring-2 ring-blue-500",
            active: "bg-blue-800"
        };

        let parsed = syn::parse2::<StateMacro>(input).unwrap();
        assert_eq!(parsed.base, Some("px-4 py-2 rounded-md".to_string()));
        assert_eq!(parsed.hover, Some("bg-blue-700".to_string()));
        assert_eq!(parsed.focus, Some("ring-2 ring-blue-500".to_string()));
        assert_eq!(parsed.active, Some("bg-blue-800".to_string()));
    }

    #[test]
    fn test_variant_macro_parser() {
        let input = quote! {
            base: "px-4 py-2 rounded-md font-medium",
            primary: "bg-blue-600 text-white hover:bg-blue-700",
            secondary: "bg-gray-200 text-gray-900 hover:bg-gray-300",
            danger: "bg-red-600 text-white hover:bg-red-700"
        };

        let parsed = syn::parse2::<VariantMacro>(input).unwrap();
        assert_eq!(
            parsed.base,
            Some("px-4 py-2 rounded-md font-medium".to_string())
        );
        assert_eq!(
            parsed.primary,
            Some("bg-blue-600 text-white hover:bg-blue-700".to_string())
        );
        assert_eq!(
            parsed.secondary,
            Some("bg-gray-200 text-gray-900 hover:bg-gray-300".to_string())
        );
        assert_eq!(
            parsed.danger,
            Some("bg-red-600 text-white hover:bg-red-700".to_string())
        );
    }
}
