//! Component macro implementations
//! 
//! This module contains the component-specific macro logic.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parsers::{
    ComponentMacro, StateMacro, VariantMacro, MacroParser
};

/// Generate component classes from a ComponentMacro
pub fn generate_component_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ComponentMacro);
    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}

/// Generate state classes from a StateMacro
pub fn generate_state_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StateMacro);
    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}

/// Generate variant classes from a VariantMacro
pub fn generate_variant_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as VariantMacro);
    let class_set = input.to_class_set();
    let class_string = class_set.to_css_classes();

    quote! {
        #class_string
    }
    .into()
}
