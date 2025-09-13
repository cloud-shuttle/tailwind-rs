//! Tailwind CSS integration for Dioxus framework
//!
//! This crate provides utilities for integrating Tailwind CSS with Dioxus applications.
//! 
//! # Example
//!
//! ```rust
//! use tailwind_rs_dioxus::*;
//! use dioxus::prelude::*;
//!
//! fn App() -> Element {
//!     rsx! {
//!         div {
//!             class: "bg-blue-500 text-white p-4 rounded-lg",
//!             "Hello from Tailwind + Dioxus!"
//!         }
//!     }
//! }
//! ```

pub mod class_builder;
pub mod utils;

pub use class_builder::*;
pub use utils::*;

/// Re-export core tailwind-rs functionality
pub use tailwind_rs_core::*;