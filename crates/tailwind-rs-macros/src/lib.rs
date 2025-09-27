//! # tailwind-rs-macros
//!
//! Procedural macros for the tailwind-rs library.
//! This crate provides macros for generating type-safe Tailwind CSS classes.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Token};

// Import our modularized components
mod parsers;
mod macros;

// Re-export parser types for use in macros
use parsers::{
    ClassesMacro, ResponsiveMacro, ThemeMacro, 
    ComponentMacro, StateMacro, VariantMacro
};

// Re-export macro implementations
use macros::{
    generate_classes_macro, generate_responsive_macro, generate_theme_macro,
    generate_component_macro, generate_state_macro, generate_variant_macro
};

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
    generate_classes_macro(input)
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
    generate_responsive_macro(input)
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
    generate_theme_macro(input)
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
    generate_component_macro(input)
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
    generate_state_macro(input)
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
    generate_variant_macro(input)
}


