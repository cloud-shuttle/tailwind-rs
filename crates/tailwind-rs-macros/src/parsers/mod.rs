//! Parser modules for tailwind-rs-macros
//! 
//! This module contains all the parser structs and implementations
//! for the procedural macros. Due to proc-macro crate limitations,
//! these are organized as modules but the actual macro functions
//! must remain in the root lib.rs file.

pub mod core_parsers;
pub mod macro_parsers;

// Re-export the main parser types for use in lib.rs
pub use core_parsers::*;
pub use macro_parsers::*;
