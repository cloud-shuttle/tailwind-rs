//! Macro implementations for tailwind-rs-macros
//! 
//! This module contains the actual procedural macro implementations.
//! Due to proc-macro crate limitations, the macro functions themselves
//! must remain in the root lib.rs file, but the supporting logic
//! can be organized here.

pub mod core_macros;
pub mod component_macros;

// Re-export macro implementations
pub use core_macros::*;
pub use component_macros::*;
