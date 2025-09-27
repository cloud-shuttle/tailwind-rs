//! Tailwind CSS integration for Dioxus framework
//!
//! This crate provides utilities for integrating Tailwind CSS with Dioxus applications.
//!
//! # Example
//!
//! ```rust
//! use tailwind_rs_dioxus::DioxusClassBuilder;
//!
//! let classes = DioxusClassBuilder::new()
//!     .class("bg-blue-500")
//!     .class("text-white")
//!     .class("p-4")
//!     .class("rounded-lg")
//!     .build();
//!
//! // Use the classes string in your Dioxus component
//! ```

pub mod class_builder;
pub mod utils;
pub mod integration_tests;

pub use class_builder::*;
pub use utils::*;

/// Re-export core tailwind-rs functionality
pub use tailwind_rs_core::*;
