//! Core macro implementations
//! 
//! This module contains the core macro logic that can be called
//! from the procedural macro functions in lib.rs.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parsers::{
    ClassesMacro, ResponsiveMacro, ThemeMacro, MacroParser
};

/// Generate CSS classes from a ClassesMacro
pub fn generate_classes_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClassesMacro);
    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}

/// Generate responsive classes from a ResponsiveMacro
pub fn generate_responsive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ResponsiveMacro);
    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}

/// Generate theme classes from a ThemeMacro
pub fn generate_theme_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ThemeMacro);
    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}
