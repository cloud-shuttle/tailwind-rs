//! Core parser implementations for tailwind-rs-macros
//! 
//! This module contains the core parsing logic for the procedural macros.

use syn::{parse::Parse, LitStr, Token};
use tailwind_rs_core::ClassSet;

/// Core parser trait for all macro parsers
pub trait MacroParser {
    fn to_class_set(&self) -> ClassSet;
}

/// Parser for the `classes!` macro
#[derive(Debug, Clone)]
pub struct ClassesMacro {
    pub base: Option<String>,
    pub variant: Option<String>,
    pub responsive: Option<String>,
    pub state: Option<String>,
    pub custom: Vec<(String, String)>,
}

impl Parse for ClassesMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
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

impl MacroParser for ClassesMacro {
    fn to_class_set(&self) -> ClassSet {
        let mut class_set = ClassSet::new();

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
#[derive(Debug, Clone)]
pub struct ResponsiveMacro {
    pub base: Option<String>,
    pub sm: Option<String>,
    pub md: Option<String>,
    pub lg: Option<String>,
    pub xl: Option<String>,
    pub xl2: Option<String>,
}

impl Parse for ResponsiveMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
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

impl MacroParser for ResponsiveMacro {
    fn to_class_set(&self) -> ClassSet {
        let mut class_set = ClassSet::new();

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
